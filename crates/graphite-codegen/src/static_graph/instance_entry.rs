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
use crate::schema::codegen::宣言元ファイルの綴り;

use super::declaration_sites::宣言元の対;
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

    // このその場展開はコンパイル時のマクロ実行中であり、呼び出し元の
    // ソースファイルパスを取得する安定APIが無い (`tracked::instance展開用に
    // 解析する` が自分の指紋計算に使う宣言元と同じ理由で「分かっていない」
    // にする)。意味カードの「宣言:」段落だけが省かれ、他の項目は影響しない。
    let 宣言元 = 宣言元の対::new(宣言元ファイルの綴り::分かっていない, 宣言元ファイルの綴り::分かっていない);
    let 個体組み立て関数 = inline::個体組み立て関数を組み立てる(instance名, 意味モデル, &宣言元);
    let 辺組み立て関数 = inline::辺組み立て関数を組み立てる(instance名, 意味モデル, &宣言元);

    quote! {
        #指紋照合
        #型参照
        #個体組み立て関数
        #辺組み立て関数
    }
}
