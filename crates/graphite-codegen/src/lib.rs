//! Graphite の schema DSL を通常の Rust ソースへ変換する純粋層。
//!
//! ファイル探索と読み書きは `xtask`、コンパイル時の入口は
//! `graphite-macros` が担当する。このクレートはどちらも行わない。

mod declaration_site;
mod generated_path;
pub mod naming;
mod schema_codegen;
mod schema_dsl;
mod schema_validate;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{Ident, LitStr, Token};

pub use declaration_site::DeclarationSite;
pub use generated_path::validate_generated_relative_path;

/// 追跡対象の schema 宣言を検証して生成に使える形へしたもの。
pub struct TrackedSchema {
    generated_path: LitStr,
    schema: schema_dsl::SchemaInput,
    fingerprint: [u64; 4],
}

impl TrackedSchema {
    /// DSL が宣言する schema module 名を返す。
    pub fn schema_name(&self) -> &Ident {
        &self.schema.schema_name
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
        let body = schema_codegen::generate_module_body(&self.schema);
        let fingerprint = self.fingerprint;
        let generated: syn::File = syn::parse2(quote! {
            #[allow(unused_imports)]
            use super::*;

            #[doc(hidden)]
            pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
                #(#fingerprint),*
            ];

            #body
        })?;
        let formatted = prettyplease::unparse(&generated);
        // 再生成コマンドの実行場所は絶対パスで書かない。ここに機械固有のパスを
        // 埋めると、別の作業環境で生成した内容が一致せず `--check` が落ちる。
        let site = site.display();
        Ok(format!(
            "// このファイルは Graphite が生成したため手編集しないこと。\n\
             // 生成元: {site}\n\
             // 再生成: リポジトリルートで `cargo xtask generate` を実行する。\n\n\
             {formatted}"
        ))
    }
}

struct TrackedInput {
    generated_path: LitStr,
    schema_tokens: TokenStream,
}

impl syn::parse::Parse for TrackedInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        if key != "generated" {
            return Err(syn::Error::new_spanned(
                key,
                "追跡可能な生成先が指定されていません。最初の行に `generated = \"...\";` を書いてください",
            ));
        }
        input.parse::<Token![=]>()?;
        let generated_path = input.parse()?;
        input.parse::<Token![;]>()?;
        let schema_tokens = input.parse()?;
        Ok(Self {
            generated_path,
            schema_tokens,
        })
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
    let parsed = schema_dsl::SchemaInput::parse_recovering
        .parse2(tracked.schema_tokens.clone())
        .map_err(|error| vec![error])?;
    let schema = validate(parsed)?;
    let generated = schema_codegen::generate_module_body(&schema);
    let normalized: syn::File = syn::parse2(quote! {
        #[allow(unused_imports)]
        use super::*;
        #generated
    })
    .map_err(|error| vec![error])?;
    // 指紋は prettyplease が整形した後のテキストをハッシュするため、整形結果の
    // 揺れ (prettyplease の版差) がそのまま指紋の揺れになる。ルート
    // Cargo.toml で prettyplease を厳密ピン止め (`=0.2.37`) してこの依存を
    // 抑えているが、将来的には整形前のトークン列を正規化してハッシュする
    // 方式へ移行し、フォーマッタの版に依存しない指紋にすべきである。
    let fingerprint = fingerprint(
        &tracked.generated_path.value(),
        &prettyplease::unparse(&normalized),
    );
    Ok(TrackedSchema {
        generated_path: tracked.generated_path,
        schema,
        fingerprint,
    })
}

/// 回復診断テスト専用のインライン schema 展開を返す。
pub fn expand_inline_for_test(input: TokenStream) -> TokenStream {
    let parsed = match schema_dsl::SchemaInput::parse_recovering.parse2(input) {
        Ok(parsed) => parsed,
        Err(error) => return error.to_compile_error(),
    };
    match validate_recovering(parsed) {
        ValidationResult::Generated { schema, errors } => {
            let diagnostics: TokenStream =
                errors.iter().map(syn::Error::to_compile_error).collect();
            let generated = schema_codegen::generate(&schema);
            quote! { #diagnostics #generated }
        }
        ValidationResult::Rejected(errors) => {
            errors.iter().map(syn::Error::to_compile_error).collect()
        }
    }
}

fn validate(parsed: schema_dsl::SchemaParse) -> Result<schema_dsl::SchemaInput, Vec<syn::Error>> {
    match validate_recovering(parsed) {
        ValidationResult::Generated { schema, errors } if errors.is_empty() => Ok(schema),
        ValidationResult::Generated { errors, .. } | ValidationResult::Rejected(errors) => {
            Err(errors)
        }
    }
}

enum ValidationResult {
    Generated {
        schema: schema_dsl::SchemaInput,
        errors: Vec<syn::Error>,
    },
    Rejected(Vec<syn::Error>),
}

fn validate_recovering(parsed: schema_dsl::SchemaParse) -> ValidationResult {
    let schema_dsl::SchemaParse {
        schema,
        errors: parse_errors,
    } = parsed;
    let has_parse_errors = !parse_errors.is_empty();
    let edges = if has_parse_errors {
        schema_validate::filter_edges_with_known_endpoints(&schema.nodes, schema.edges)
    } else {
        schema.edges
    };

    let mut validate_errors = Vec::new();
    let node_names_are_unique = collect_validation(
        schema_validate::validate_unique_node_names(&schema.nodes),
        &mut validate_errors,
    );
    if !has_parse_errors {
        collect_validation(
            schema_validate::validate_edge_endpoints(&schema.nodes, &edges),
            &mut validate_errors,
        );
    }
    let edge_names_are_unique = collect_validation(
        schema_validate::validate_unique_edge_kinds(&edges),
        &mut validate_errors,
    );
    if node_names_are_unique && edge_names_are_unique {
        collect_validation(
            schema_validate::validate_generated_type_names(
                &schema.schema_name,
                &schema.nodes,
                &edges,
            ),
            &mut validate_errors,
        );
    }
    collect_validation(
        schema_validate::validate_undirected_same_type(&edges),
        &mut validate_errors,
    );
    collect_validation(
        schema_validate::validate_edge_roles(&edges),
        &mut validate_errors,
    );
    collect_validation(
        schema_validate::validate_each_reference(&edges),
        &mut validate_errors,
    );

    if !validate_errors.is_empty() {
        let mut errors = parse_errors;
        errors.extend(validate_errors);
        return ValidationResult::Rejected(errors);
    }
    ValidationResult::Generated {
        schema: schema_dsl::SchemaInput {
            schema_name: schema.schema_name,
            nodes: schema.nodes,
            edges,
        },
        errors: parse_errors,
    }
}

fn collect_validation(result: syn::Result<()>, errors: &mut Vec<syn::Error>) -> bool {
    match result {
        Ok(()) => true,
        Err(error) => {
            errors.push(error);
            false
        }
    }
}

fn fingerprint(path: &str, normalized_schema: &str) -> [u64; 4] {
    let canonical = format!("{path}\0{normalized_schema}");
    [
        fnv1a(canonical.as_bytes(), 0xcbf29ce484222325),
        fnv1a(canonical.as_bytes(), 0x84222325cbf29ce4),
        fnv1a(canonical.as_bytes(), 0x9e3779b185ebca87),
        fnv1a(canonical.as_bytes(), 0xd6e8feb86659fd93),
    ]
}

fn fnv1a(bytes: &[u8], seed: u64) -> u64 {
    bytes.iter().fold(seed, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
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
            16028462957885294
        );
    }
}
