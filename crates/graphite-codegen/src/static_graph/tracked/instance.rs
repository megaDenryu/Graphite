// 追跡対象の静的グラフinstance宣言 (issue #41 §2)。schemaと同じ入口
// (`generated = "..."` + 本体) を受け取るが、相互検証・意味モデルの組み立て
// にschemaを要るため、`parse_tracked_static_instance` はschemaの構文木への
// 参照を引数に取る (設計書 §2「第2段階」の
// `parse_tracked_static_instance(&schema, tokens)` と同じ形)。
//
// 段階2ではこの型の生成・単体試験だけを行い、`__static_graph_impl` の公開の
// 振る舞いはまだこれへ切り替えない (段階3の仕事)。

use proc_macro2::TokenStream;
use syn::LitStr;

use crate::declaration_site::DeclarationSite;
use crate::fingerprint::fingerprint;
use crate::fingerprint_check::静的instance指紋定数名;
use crate::generated_path::validate_generated_relative_path;
use crate::generated_source::{指紋の材料になる整形済み本文, 生成ファイルの本文};
use crate::schema::codegen::宣言元ファイルの綴り;
use crate::tracked_input::TrackedInput;

use crate::static_graph::file::instance本体を組み立てる;
use crate::static_graph::internal::静的グラフ内部入力;
use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::schema::input::静的グラフ型入力;
use crate::static_graph::semantic::意味モデル;

pub struct TrackedStaticInstance {
    generated_path: LitStr,
    意味モデル: 意味モデル,
    fingerprint: [u64; 4],
}

impl TrackedStaticInstance {
    pub fn instance_name(&self) -> &proc_macro2::Ident {
        self.意味モデル.グラフ名()
    }

    pub fn generated_path(&self) -> &LitStr {
        &self.generated_path
    }

    pub fn fingerprint(&self) -> [u64; 4] {
        self.fingerprint
    }

    pub fn render_module_source(&self, site: &DeclarationSite) -> syn::Result<String> {
        let 宣言元 =
            宣言元ファイルの綴り::パッケージ相対で分かっている(site.宣言ファイルの綴り().to_string());
        let body = instance本体を組み立てる(&self.意味モデル, &宣言元);
        生成ファイルの本文(&body, self.fingerprint, site, &静的instance指紋定数名())
    }
}

pub fn parse_tracked_static_instance(
    schema: &静的グラフ型入力,
    input: TokenStream,
) -> Result<TrackedStaticInstance, Vec<syn::Error>> {
    let tracked = syn::parse2::<TrackedInput>(input).map_err(|error| vec![error])?;
    if let Err(reason) = validate_generated_relative_path(&tracked.generated_path.value()) {
        return Err(vec![syn::Error::new_spanned(&tracked.generated_path, reason)]);
    }
    let instance: 静的グラフ入力 =
        syn::parse2(tracked.schema_tokens.clone()).map_err(|error| vec![error])?;

    // schemaとinstanceの相互検証 (多重度・対一意・種別の存在等) は
    // `静的グラフ内部入力` が既に持つ組み立て済みの検証手順をそのまま使う
    // (`internal::mod` 参照。ここで検証手順を複製しない)。`検証する` は
    // `self` を消費して検証済みの型を返し、検証前に意味モデルを組み立てる
    // ことを型状態で防ぐ (issue #41 是正8)。
    let 内部入力 = 静的グラフ内部入力 { schema: schema.clone(), instance };
    let 検証済み = 内部入力.検証する().map_err(|error| vec![error])?;
    let 意味モデル = 検証済み.意味モデルを組み立てる();

    let body = instance本体を組み立てる(&意味モデル, &宣言元ファイルの綴り::分かっていない);
    let 整形済み本文 = 指紋の材料になる整形済み本文(&body).map_err(|error| vec![error])?;
    let fingerprint = fingerprint(&tracked.generated_path.value(), &整形済み本文);

    Ok(TrackedStaticInstance { generated_path: tracked.generated_path, 意味モデル, fingerprint })
}
