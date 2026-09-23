// このファイルは参照の層の集まり (`NodeRefs`/`EdgeRefs`) を組み立てる。
// フィールドは個体名・辺名をそのまま使い、`pub` + 意味カードを付ける
// (issue #41 §5.3)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::naming::{
    個体参照型名, 個体参照集合型名, 構築メソッド名, 辺参照型名, 辺参照集合型名, edge_refsフィールドの追跡情報を作る,
    node_refsフィールドの追跡情報を作る,
};
use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::固定語彙の所有者;

use super::super::doc_render::doc属性を組み立てる;

pub(super) fn node_refs本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元ファイルの綴り) -> TokenStream {
    let 型名 = 個体参照集合型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let 構築名 = 構築メソッド名(固定語彙の所有者::NodeRefs, 意味モデル);
    let 構築doc = doc属性を組み立てる(構築名.追跡());

    let フィールド列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 参照型 = 個体参照型名(意味モデル, 個体, 宣言元);
        let doc = doc属性を組み立てる(&node_refsフィールドの追跡情報を作る(意味モデル, 個体, 宣言元));
        quote! { #doc pub #名前: #参照型<'a> }
    });
    let 初期化列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 参照型 = 個体参照型名(意味モデル, 個体, 宣言元);
        quote! { #名前: #参照型 { entity: &nodes.#名前, nodes, edges } }
    });
    quote! {
        #型doc
        pub struct #型名<'a> {
            #(#フィールド列,)*
        }
        impl<'a> #型名<'a> {
            #構築doc
            pub fn #構築名(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
                Self { #(#初期化列,)* }
            }
        }
    }
}

pub(super) fn edge_refs本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元ファイルの綴り) -> TokenStream {
    let 型名 = 辺参照集合型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let 構築名 = 構築メソッド名(固定語彙の所有者::EdgeRefs, 意味モデル);
    let 構築doc = doc属性を組み立てる(構築名.追跡());

    let フィールド列 = 意味モデル.具体辺列().iter().map(|辺| {
        let 名前 = 辺.名前();
        let 参照型 = 辺参照型名(意味モデル, 辺, 宣言元);
        let doc = doc属性を組み立てる(&edge_refsフィールドの追跡情報を作る(意味モデル, 辺, 宣言元));
        quote! { #doc pub #名前: #参照型<'a> }
    });
    let 初期化列 = 意味モデル.具体辺列().iter().map(|辺| {
        let 名前 = 辺.名前();
        let 参照型 = 辺参照型名(意味モデル, 辺, 宣言元);
        quote! { #名前: #参照型 { entity: &edges.#名前, nodes, edges } }
    });
    quote! {
        #型doc
        pub struct #型名<'a> {
            #(#フィールド列,)*
        }
        impl<'a> #型名<'a> {
            #構築doc
            pub fn #構築名(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
                Self { #(#初期化列,)* }
            }
        }
    }
}
