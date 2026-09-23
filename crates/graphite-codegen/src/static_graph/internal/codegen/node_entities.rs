// 生成物2: Nodes (旧 ノード達)。ノードの実体の唯一の所有者。値あり宣言
// (`node 名前 = ..;` / `node 名前: 型 = 式;`) はフィールドの初期化式に
// ノード宣言の右辺をそのまま埋め込む。値なし宣言 (`node 名前: 型;`) は
// 宣言順の位置引数として `new` に加え、その引数をそのままフィールドへ渡す
// (実行時供給)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::semantic::個体;

pub(super) fn ノード達を生成する(個体列: &[個体]) -> TokenStream {
    let フィールド達 = 個体列.iter().map(|個体| {
        let 名前 = 個体.名前();
        let 実体型 = 個体.実体型();
        quote! { #名前: #実体型 }
    });
    let 引数達 = 個体列.iter().filter(|個体| 個体.値なし宣言か()).map(|個体| {
        let 名前 = 個体.名前();
        let 実体型 = 個体.実体型();
        quote! { #名前: #実体型 }
    });
    let 初期化達 = 個体列.iter().map(|個体| {
        let 名前 = 個体.名前();
        let 初期化式 = match 個体.値() {
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
