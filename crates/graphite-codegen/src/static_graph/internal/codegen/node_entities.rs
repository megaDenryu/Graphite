// 生成物2: Nodes (旧 ノード達)。ノードの実体の唯一の所有者。値あり宣言
// (`node 名前 = ..;` / `node 名前: 型 = 式;`) はフィールドの初期化式に
// ノード宣言の右辺をそのまま埋め込む。値なし宣言 (`node 名前: 型;`) は
// 宣言順の位置引数として `new` に加え、その引数をそのままフィールドへ渡す
// (実行時供給)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::literal::input::ノード宣言;

pub(super) fn ノード達を生成する(ノード宣言達: &[ノード宣言]) -> TokenStream {
    let フィールド達 = ノード宣言達.iter().map(|ノード| {
        let 名前 = &ノード.名前;
        let 実体型 = &ノード.実体型;
        quote! { #名前: #実体型 }
    });
    let 引数達 = ノード宣言達.iter().filter(|ノード| ノード.値.is_none()).map(|ノード| {
        let 名前 = &ノード.名前;
        let 実体型 = &ノード.実体型;
        quote! { #名前: #実体型 }
    });
    let 初期化達 = ノード宣言達.iter().map(|ノード| {
        let 名前 = &ノード.名前;
        let 初期化式 = match &ノード.値 {
            Some(式) => quote! { #式 },
            None => quote! { #名前 },
        };
        quote! { #名前: #初期化式 }
    });
    quote! {
        struct Nodes {
            #(#フィールド達,)*
        }
        impl Nodes {
            fn new(#(#引数達),*) -> Self {
                Self { #(#初期化達,)* }
            }
        }
    }
}
