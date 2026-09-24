// このファイルは参照の層の集まり (`NodeRefs`/`EdgeRefs`) を組み立てる。
// フィールドは借用した `graph: &'a Graph` の1つだけであり非公開にし、
// 個体名・辺名をそのまま名前にした読み出し専用のアクセサメソッドへ
// `pub` + 意味カードを付ける (issue #41 §5.3)。非公開にすることで、利用者
// が構造体リテラルで`NodeRefs`/`EdgeRefs`を直接作り、由来の異なる`Graph`
// を混ぜた不整合な値を組み立てる迂回を防ぐ (`node_ref.rs`の冒頭コメント
// 参照)。構築は `graph_struct.rs` の `Graph::node_refs()`/`edge_refs()` が
// 同じmodule内から直接構造体リテラルで行うため、旧`Nodes`/`Edges`の内部
// 構築子と違い `construct!` の展開 (呼び出し位置というmodule外) から
// 呼ばれる橋渡し用の `new` は要らない。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{
    edge_refsメソッドの追跡情報を作る, node_refsメソッドの追跡情報を作る, 個体参照型名, 個体参照集合型名, 辺参照型名,
    辺参照集合型名, graphフィールド名,
};
use crate::static_graph::semantic::意味モデル;

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn node_refs本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let 型名 = 個体参照集合型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let graph = graphフィールド名();

    let メソッド列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 参照型 = 個体参照型名(意味モデル, 個体, 宣言元);
        let doc = doc属性を組み立てる(&node_refsメソッドの追跡情報を作る(意味モデル, 個体, 宣言元));
        quote! {
            #doc
            pub fn #名前(&self) -> #参照型<'a> { #参照型 { #graph: self.#graph } }
        }
    });
    quote! {
        #型doc
        pub struct #型名<'a> {
            #graph: &'a Graph,
        }
        impl<'a> #型名<'a> {
            #(#メソッド列)*
        }
    }
}

pub(super) fn edge_refs本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let 型名 = 辺参照集合型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let graph = graphフィールド名();

    let メソッド列 = 意味モデル.具体辺列().iter().map(|辺| {
        let 名前 = 辺.名前();
        let 参照型 = 辺参照型名(意味モデル, 辺, 宣言元);
        let doc = doc属性を組み立てる(&edge_refsメソッドの追跡情報を作る(意味モデル, 辺, 宣言元));
        quote! {
            #doc
            pub fn #名前(&self) -> #参照型<'a> { #参照型 { #graph: self.#graph } }
        }
    });
    quote! {
        #型doc
        pub struct #型名<'a> {
            #graph: &'a Graph,
        }
        impl<'a> #型名<'a> {
            #(#メソッド列)*
        }
    }
}
