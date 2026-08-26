//! Graphite の schema DSL を通常の Rust ソースへ変換する純粋層。
//!
//! ファイル探索と読み書きは `xtask`、コンパイル時の入口は
//! `graphite-macros` が担当する。このクレートはどちらも行わない。
//!
//! ## この module の役割
//!
//! ここは工程を配線するだけの Composition Root である。入力を受け取ってから
//! 生成物を返すまでの順序 (構文解析 → 意味検査 → 意味モデルの組み立て →
//! コード生成 → 指紋の計算) を知るのはここだけであり、各工程の中身は
//! [`schema`] 配下の各層が持つ。工程どうしが互いを直接呼ぶことはしない。

mod declaration_site;
mod fingerprint;
mod generated_path;
mod generated_source;
pub mod naming;
mod schema;
mod tracked_input;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{Ident, LitStr};

use crate::fingerprint::fingerprint;
use crate::generated_source::{
    指紋の材料になる整形済み本文, 生成ファイルの本文
};
use crate::tracked_input::TrackedInput;

pub use declaration_site::DeclarationSite;
pub use generated_path::validate_generated_relative_path;

/// 追跡対象の schema 宣言を検証し、意味モデルまで確定させたもの。
///
/// 構文モデルではなく意味モデルを保持するのは、指紋の計算とソース生成が同じ
/// 解析結果を読むようにするためである (解析を2回走らせない)。
pub struct TrackedSchema {
    generated_path: LitStr,
    スキーマ定義: schema::semantic::スキーマ定義,
    fingerprint: [u64; 4],
}

impl TrackedSchema {
    /// DSL が宣言する schema module 名を返す。
    pub fn schema_name(&self) -> &Ident {
        self.スキーマ定義.スキーマ名()
    }

    /// 宣言元ファイルから見た生成先の相対パスを返す。
    pub fn generated_path(&self) -> &LitStr {
        &self.generated_path
    }

    /// schema の意味と生成先を表す決定的な指紋を返す。
    pub fn fingerprint(&self) -> [u64; 4] {
        self.fingerprint
    }

    /// schema module の通常の Rust ソース本文を生成する。
    pub fn render_module_source(&self, site: &DeclarationSite) -> syn::Result<String> {
        let body = schema::codegen::generate_module_body(&self.スキーマ定義);
        生成ファイルの本文(&body, self.fingerprint, site)
    }
}

/// 追跡形式の `graph_schema!` 入力を解析・検証する。
pub fn parse_tracked_schema(input: TokenStream) -> Result<TrackedSchema, Vec<syn::Error>> {
    let tracked = syn::parse2::<TrackedInput>(input).map_err(|error| vec![error])?;
    if let Err(reason) = validate_generated_relative_path(&tracked.generated_path.value()) {
        return Err(vec![syn::Error::new_spanned(
            &tracked.generated_path,
            reason,
        )]);
    }
    let parsed = schema::syntax::SchemaInput::parse_recovering
        .parse2(tracked.schema_tokens.clone())
        .map_err(|error| vec![error])?;
    let 検証済み構文 = schema::validate::validate(parsed)?;
    let スキーマ定義 =
        schema::semantic::検証済み構文からスキーマ定義を組み立てる(
            &検証済み構文,
        );
    let 生成コード = schema::codegen::generate_module_body(&スキーマ定義);
    let 整形済み本文 = 指紋の材料になる整形済み本文(&生成コード).map_err(|error| vec![error])?;
    let fingerprint = fingerprint(&tracked.generated_path.value(), &整形済み本文);
    Ok(TrackedSchema {
        generated_path: tracked.generated_path,
        スキーマ定義,
        fingerprint,
    })
}

/// 回復診断テスト専用のインライン schema 展開を返す。
pub fn expand_inline_for_test(input: TokenStream) -> TokenStream {
    let parsed = match schema::syntax::SchemaInput::parse_recovering.parse2(input) {
        Ok(parsed) => parsed,
        Err(error) => return error.to_compile_error(),
    };
    match schema::validate::validate_recovering(parsed) {
        schema::validate::ValidationResult::Generated {
            schema: 検証済み構文,
            errors,
        } => {
            let diagnostics: TokenStream =
                errors.iter().map(syn::Error::to_compile_error).collect();
            let スキーマ定義 =
                schema::semantic::検証済み構文からスキーマ定義を組み立てる(
                    &検証済み構文,
                );
            let generated = schema::codegen::generate(&スキーマ定義);
            quote! { #diagnostics #generated }
        }
        schema::validate::ValidationResult::Rejected(errors) => {
            errors.iter().map(syn::Error::to_compile_error).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fingerprint::fnv1a;
    use quote::quote;

    #[test]
    fn 同じ入力の生成結果はバイト単位で一致する() {
        let input = quote! {
            generated = "generated/world.rs";
            schema World {
                node Person;
                edge Knows = (source: Person) -> (target: Person);
            }
        };
        let first = parse_tracked_schema(input.clone()).unwrap();
        let second = parse_tracked_schema(input).unwrap();
        // 固定値は生成物の意図しない変化を検出するための錨である。生成器を
        // 意図して変えたときは `cargo xtask generate` の差分と併せて更新する。
        let site = DeclarationSite::new("tests/schema.rs".to_string(), 10);
        let rendered = first.render_module_source(&site).unwrap();
        assert_eq!(rendered, second.render_module_source(&site).unwrap());
        assert_eq!(
            fnv1a(rendered.as_bytes(), 0xcbf29ce484222325),
            10892219771106859126
        );
    }
}
