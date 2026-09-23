// 追跡対象の静的グラフinstance宣言。schemaと同じ入口 (`generated = "..."` +
// 本体) を受け取るが、相互検証・意味モデルの組み立てにschemaを要る。
//
// 公開の `parse_tracked_static_instance` は `&TrackedStaticSchema` を受け
// 取り、crateの外 (`graphite-cli` の2段階の解決) から呼べる形にする。
// `graphite-macros::__static_graph_impl` は、schemaの生成先・指紋を検査
// 済みの `TrackedStaticSchema` を持たず、macro_rules!転送で届いたschemaの
// 生の構文木 (`静的グラフ型入力`) だけを持つため、その場展開専用の
// crate内部版 (`instance展開用に解析する`) を別に持つ。両方とも実装は
// `構文木から組み立てる` を共有する。

use proc_macro2::TokenStream;
use syn::LitStr;

use crate::declaration_site::DeclarationSite;
use crate::fingerprint::fingerprint;
use crate::fingerprint_check::静的instance指紋定数名;
use crate::generated_path::validate_generated_relative_path;
use crate::generated_source::{指紋の材料になる整形済み本文, 生成ファイルの本文};
use crate::schema::codegen::宣言元ファイルの綴り;
use crate::tracked_input::TrackedInput;

use crate::static_graph::declaration_sites::宣言元の対;
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

    // その場展開が読む意味モデル (DSLトークンの型参照・値供給関数の組み立てに
    // 要る。issue #41 段階3)。
    pub(crate) fn 意味モデル(&self) -> &意味モデル {
        &self.意味モデル
    }

    // `site` はinstance自身の宣言元 (指紋照合コードのspanと生成ファイル先頭
    // の案内コメントに使う)、`schema_site` はこのinstanceが由来するschemaの
    // 宣言元である。schemaとinstanceが別ファイルの場合に、意味カードの
    // 「関係する schema 宣言」の段落が実在する場所を指すよう、cliの2段階の
    // 解決が両方を渡す (issue #41)。
    pub fn render_module_source(
        &self,
        site: &DeclarationSite,
        schema_site: &DeclarationSite,
    ) -> syn::Result<String> {
        let 宣言元 = 宣言元の対::new(
            宣言元ファイルの綴り::パッケージ相対で分かっている(site.宣言ファイルの綴り().to_string()),
            宣言元ファイルの綴り::パッケージ相対で分かっている(
                schema_site.宣言ファイルの綴り().to_string(),
            ),
        );
        let body = instance本体を組み立てる(&self.意味モデル, &宣言元);
        生成ファイルの本文(&body, self.fingerprint, site, &静的instance指紋定数名())
    }
}

// crate の外 (`graphite-cli`) から呼ぶ公開の入口。CLIは2段階の解決で
// schemaを検証・指紋計算まで済ませた `TrackedStaticSchema` を先に持つため、
// これをそのまま受け取る。
pub fn parse_tracked_static_instance(
    schema: &super::TrackedStaticSchema,
    input: TokenStream,
) -> Result<TrackedStaticInstance, Vec<syn::Error>> {
    instance展開用に解析する(schema.型入力(), input)
}

// `__static_graph_impl` のその場展開専用。schemaの生の構文木だけを持つ
// (schema自身の `TrackedStaticSchema` は静的グラフ内マクロの手前
// (`static_graph_schema!`) で既に検証済みであり、ここでは二重に検証しない)。
pub(crate) fn instance展開用に解析する(
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
    // ことを型状態で防ぐ。
    let 内部入力 = 静的グラフ内部入力 { schema: schema.clone(), instance };
    let 検証済み = 内部入力.検証する().map_err(|error| vec![error])?;
    let 意味モデル = 検証済み.意味モデルを組み立てる();

    let 宣言元不明の対 = 宣言元の対::new(宣言元ファイルの綴り::分かっていない, 宣言元ファイルの綴り::分かっていない);
    let body = instance本体を組み立てる(&意味モデル, &宣言元不明の対);
    let 整形済み本文 = 指紋の材料になる整形済み本文(&body).map_err(|error| vec![error])?;
    let fingerprint = fingerprint(&tracked.generated_path.value(), &整形済み本文);

    Ok(TrackedStaticInstance { generated_path: tracked.generated_path, 意味モデル, fingerprint })
}
