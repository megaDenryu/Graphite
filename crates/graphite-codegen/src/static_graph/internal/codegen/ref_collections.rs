// 生成物6: 参照の層の集まり (`NodeRefs`/`EdgeRefs`、旧 `ノード参照達`/
// `辺参照達`)。Nodes・Edges (実体の層) から作る。内部の個体名・辺名の
// フィールド名は利用者がinstance宣言に書いた名前をそのまま使う (README
// 「生成される名前の公開契約」参照)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::naming::{個体参照型名, 辺参照型名};
use crate::static_graph::semantic::意味モデル;

pub(super) fn ノード参照達を生成する(意味モデル: &意味モデル) -> TokenStream {
    let フィールド達 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 参照型 = 個体参照型名(意味モデル, 個体, &宣言元ファイルの綴り::分かっていない);
        quote! { #名前: #参照型<'a> }
    });
    let 初期化達 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 参照型 = 個体参照型名(意味モデル, 個体, &宣言元ファイルの綴り::分かっていない);
        quote! { #名前: #参照型 { entity: &nodes.#名前, nodes, edges } }
    });
    quote! {
        struct NodeRefs<'a> {
            #(#フィールド達,)*
        }
        impl<'a> NodeRefs<'a> {
            fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
                Self { #(#初期化達,)* }
            }
        }
    }
}

pub(super) fn 辺参照達を生成する(意味モデル: &意味モデル) -> TokenStream {
    let フィールド達 = 意味モデル.具体辺列().iter().map(|辺| {
        let 名前 = 辺.名前();
        let 参照型 = 辺参照型名(意味モデル, 辺, &宣言元ファイルの綴り::分かっていない);
        quote! { #名前: #参照型<'a> }
    });
    let 初期化達 = 意味モデル.具体辺列().iter().map(|辺| {
        let 名前 = 辺.名前();
        let 参照型 = 辺参照型名(意味モデル, 辺, &宣言元ファイルの綴り::分かっていない);
        quote! { #名前: #参照型 { entity: &edges.#名前, nodes, edges } }
    });
    quote! {
        struct EdgeRefs<'a> {
            #(#フィールド達,)*
        }
        impl<'a> EdgeRefs<'a> {
            fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
                Self { #(#初期化達,)* }
            }
        }
    }
}
