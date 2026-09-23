// `__static_graph_impl!` の展開本体 (issue #41 段階3)。`static_graph_schema!`
// が macro_rules!転送で焼き込んだschemaの構文木と、利用側が
// `<schema名>! { .. }` で書いたinstanceの生トークン (`generated = "..."` を
// 含む) の組を受け取り、相互検証・指紋照合・値の供給関数・DSLトークンの
// 型参照をその場展開の出力として返す。公開の `Nodes`・`Edges`・`{個体名}Ref`・
// `{辺名}Ref`・`Graph` は生成ファイルだけにあり、ここでは出力しない。
//
// `instance` 側は `generated = "..."` の切り出しが挟まるため、
// `schema <名前> { .. } instance { .. }` を1回で構造化した型へは直接parse
// せず、instance本体を生トークンのまま `instance展開用に解析する` へ渡す。

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};

use crate::fingerprint_check::{指紋照合コードを生成する, 静的instance対象文言, 静的instance指紋定数名};

use super::naming::指紋照合パスの起点;
use super::{inline, instance展開用に解析する, schema};

struct 静的グラフ転送入力 {
    schema: schema::input::静的グラフ型入力,
    instance_tokens: TokenStream,
}

syn::custom_keyword!(instance);

impl Parse for 静的グラフ転送入力 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let schema: schema::input::静的グラフ型入力 = input.parse()?;
        input.parse::<instance>()?;
        let 本体;
        syn::braced!(本体 in input);
        let instance_tokens: TokenStream = 本体.parse()?;
        Ok(静的グラフ転送入力 { schema, instance_tokens })
    }
}

pub fn expand_static_graph_internal(input: TokenStream) -> TokenStream {
    let 転送入力 = match syn::parse2::<静的グラフ転送入力>(input) {
        Ok(転送入力) => 転送入力,
        Err(エラー) => return エラー.to_compile_error(),
    };
    let tracked = match instance展開用に解析する(&転送入力.schema, 転送入力.instance_tokens) {
        Ok(tracked) => tracked,
        Err(errors) => {
            return errors.iter().map(syn::Error::to_compile_error).collect();
        }
    };

    let instance名 = tracked.instance_name();
    let 定数名 = 静的instance指紋定数名();
    let 定数パスの起点 = 指紋照合パスの起点(instance名, tracked.generated_path().span());
    let 指紋照合 = 指紋照合コードを生成する(
        quote! { #定数パスの起点::#定数名 },
        tracked.fingerprint(),
        &静的instance対象文言(instance名),
        tracked.generated_path().span(),
    );

    let 意味モデル = tracked.意味モデル();
    let 型参照 = inline::dslトークンの型参照を組み立てる(意味モデル);

    let nodes型 = quote! { #instance名::Nodes };
    let 個体供給関数列 = 意味モデル
        .個体列()
        .iter()
        .filter(|個体| !個体.値なし宣言か())
        .map(|個体| inline::個体供給関数を組み立てる(&nodes型, 個体));

    // `Edges<'a>` は生成ファイルの中で生存期間パラメータを持つため、
    // 呼び出し元 (このマクロ展開位置) からの修飾パスでは明示のライフタイム
    // 引数が要る (`impl 開発チーム::Edges { .. }` は E0726 で拒否される)。
    let edges型 = quote! { #instance名::Edges<'_> };
    let 積み荷供給関数列 = 意味モデル
        .具体辺列()
        .iter()
        .filter(|辺| 辺.積み荷式().is_some())
        .map(|辺| inline::積み荷供給関数を組み立てる(&edges型, 辺));

    quote! {
        #指紋照合
        #型参照
        #(#個体供給関数列)*
        #(#積み荷供給関数列)*
    }
}
