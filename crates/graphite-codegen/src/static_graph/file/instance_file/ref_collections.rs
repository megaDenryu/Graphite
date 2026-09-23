// このファイルは参照の層の集まり (`NodeRefs`/`EdgeRefs`) を組み立てる。
// フィールドは個体名・辺名をそのまま使い、`pub` + 意味カードを付ける
// (issue #41 §5.3)。`new` (C分類、非公開) は `graph_struct.rs` の
// `Graph::new` だけが呼ぶ内部専用の素の構築子であり、PR #45レビューCの
// 対策 (`Graph::new` が `&Edges` だけを起点にする) により、由来の異なる
// `Nodes`/`Edges` の組を渡す経路自体が存在しない。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{
    edge_refsフィールドの追跡情報を作る, edges変数名, entityフィールド名, node_refsフィールドの追跡情報を作る, nodes変数名,
    個体参照型名, 個体参照集合型名, 辺参照型名, 辺参照集合型名,
};
use crate::static_graph::semantic::意味モデル;

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn node_refs本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let 型名 = 個体参照集合型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let entity = entityフィールド名();
    let nodes = nodes変数名();
    let edges = edges変数名();

    let フィールド列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 参照型 = 個体参照型名(意味モデル, 個体, 宣言元);
        let doc = doc属性を組み立てる(&node_refsフィールドの追跡情報を作る(意味モデル, 個体, 宣言元));
        quote! { #doc pub #名前: #参照型<'a> }
    });
    let 初期化列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 参照型 = 個体参照型名(意味モデル, 個体, 宣言元);
        quote! { #名前: #参照型 { #entity: &#nodes.#名前, #nodes, #edges } }
    });
    quote! {
        #型doc
        pub struct #型名<'a> {
            #(#フィールド列,)*
        }
        impl<'a> #型名<'a> {
            fn new(#nodes: &'a Nodes, #edges: &'a Edges<'a>) -> Self {
                Self { #(#初期化列,)* }
            }
        }
    }
}

pub(super) fn edge_refs本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let 型名 = 辺参照集合型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let entity = entityフィールド名();
    let nodes = nodes変数名();
    let edges = edges変数名();

    let フィールド列 = 意味モデル.具体辺列().iter().map(|辺| {
        let 名前 = 辺.名前();
        let 参照型 = 辺参照型名(意味モデル, 辺, 宣言元);
        let doc = doc属性を組み立てる(&edge_refsフィールドの追跡情報を作る(意味モデル, 辺, 宣言元));
        quote! { #doc pub #名前: #参照型<'a> }
    });
    let 初期化列 = 意味モデル.具体辺列().iter().map(|辺| {
        let 名前 = 辺.名前();
        let 参照型 = 辺参照型名(意味モデル, 辺, 宣言元);
        quote! { #名前: #参照型 { #entity: &#edges.#名前, #nodes, #edges } }
    });
    quote! {
        #型doc
        pub struct #型名<'a> {
            #(#フィールド列,)*
        }
        impl<'a> #型名<'a> {
            fn new(#nodes: &'a Nodes, #edges: &'a Edges<'a>) -> Self {
                Self { #(#初期化列,)* }
            }
        }
    }
}
