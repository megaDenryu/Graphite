// 生成物2: node宣言の型アンカー。`node 社員;` は識別子の宣言であって型として
// の使用ではないため、rust-analyzerの色付け・定義ジャンプが効かない。全node
// 型を引数型として使うだけの、呼ばれない関数を生成してこれを補う。
// `const _: () = { .. };` で囲み、周囲のスコープを汚さない。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::schema::input::静的グラフ型入力;

pub(super) fn 型アンカーを生成する(schema: &静的グラフ型入力) -> TokenStream {
    let 引数達 = schema.ノード宣言達.iter().map(|ノード| {
        let 型 = &ノード.名前;
        quote! { _: &#型 }
    });
    quote! {
        const _: () = {
            #[allow(dead_code)]
            fn ノード型を使う(#(#引数達),*) {}
        };
    }
}
