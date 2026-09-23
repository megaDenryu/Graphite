// 追跡対象の静的グラフschema宣言。動的グラフの `TrackedSchema` (crate直下
// `lib.rs`) と同じ役割を果たす公開型 (issue #41 §2)。
//
// 段階2ではこの型の生成・単体試験だけを行い、
// `static_graph_schema!`/`__static_graph_impl` の公開の振る舞いはまだこれへ
// 切り替えない (段階3の仕事)。

use proc_macro2::{Ident, TokenStream};
use syn::LitStr;

use crate::declaration_site::DeclarationSite;
use crate::fingerprint::fingerprint;
use crate::fingerprint_check::静的schema指紋定数名;
use crate::generated_path::validate_generated_relative_path;
use crate::generated_source::{指紋の材料になる整形済み本文, 生成ファイルの本文};
use crate::schema::codegen::宣言元ファイルの綴り;
use crate::tracked_input::TrackedInput;

use crate::static_graph::file::schema本体を組み立てる;
use crate::static_graph::schema::input::静的グラフ型入力;

pub struct TrackedStaticSchema {
    generated_path: LitStr,
    型入力: 静的グラフ型入力,
    fingerprint: [u64; 4],
}

impl TrackedStaticSchema {
    pub fn schema_name(&self) -> &Ident {
        &self.型入力.schema名
    }

    pub fn generated_path(&self) -> &LitStr {
        &self.generated_path
    }

    pub fn fingerprint(&self) -> [u64; 4] {
        self.fingerprint
    }

    // 相互検証・意味モデルの組み立てに使う、検証済みのschema構文木。
    // `parse_tracked_static_instance` へ渡す。cliの2段階解決 (静的schema名簿
    // からinstanceを見つける、issue #41 §2) が公開マクロの探索へ配線されて
    // いない間は、単体試験だけが呼ぶ。
    #[allow(dead_code)]
    pub(crate) fn 型入力(&self) -> &静的グラフ型入力 {
        &self.型入力
    }

    pub fn render_module_source(&self, site: &DeclarationSite) -> syn::Result<String> {
        let 宣言元 =
            宣言元ファイルの綴り::パッケージ相対で分かっている(site.宣言ファイルの綴り().to_string());
        let body = schema本体を組み立てる(&self.型入力, &宣言元);
        生成ファイルの本文(&body, self.fingerprint, site, &静的schema指紋定数名())
    }
}

pub fn parse_tracked_static_schema(input: TokenStream) -> Result<TrackedStaticSchema, Vec<syn::Error>> {
    let tracked = syn::parse2::<TrackedInput>(input).map_err(|error| vec![error])?;
    if let Err(reason) = validate_generated_relative_path(&tracked.generated_path.value()) {
        return Err(vec![syn::Error::new_spanned(&tracked.generated_path, reason)]);
    }
    let 型入力: 静的グラフ型入力 =
        syn::parse2(tracked.schema_tokens.clone()).map_err(|error| vec![error])?;
    型入力.検証する().map_err(|error| vec![error])?;

    let body = schema本体を組み立てる(&型入力, &宣言元ファイルの綴り::分かっていない);
    let 整形済み本文 = 指紋の材料になる整形済み本文(&body).map_err(|error| vec![error])?;
    let fingerprint = fingerprint(&tracked.generated_path.value(), &整形済み本文);

    Ok(TrackedStaticSchema { generated_path: tracked.generated_path, 型入力, fingerprint })
}
