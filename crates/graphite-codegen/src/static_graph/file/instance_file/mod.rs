//! instanceファイル本体 (issue #41 §2)。`internal/codegen` (その場展開用)
//! と同じ並び順 (Nodes → Edges → 個体参照 → 辺インスタンス参照 → 参照の
//! 層の集まり → グラフ本体) で、`pub` 可視性と意味カードを添えて並べる。
//! 各生成物の中身は配下のmoduleが持ち、この module本体は並び順だけを知る。
//! instanceの指紋定数は呼び出し側 (`static_graph::tracked::instance`) が
//! `crate::generated_source::生成ファイルの本文` 経由で別途足す。

mod edge_entities;
mod edge_ref;
mod graph_struct;
mod node_entities;
mod node_ref;
mod ref_collections;

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::semantic::意味モデル;

pub(crate) fn instance本体を組み立てる(
    意味モデル: &意味モデル,
    宣言元: &宣言元ファイルの綴り,
) -> TokenStream {
    let nodes = node_entities::nodes本体を組み立てる(意味モデル, 宣言元);
    let edges = edge_entities::edges本体を組み立てる(意味モデル, 宣言元);
    let 個体参照列 = node_ref::個体参照列を組み立てる(意味モデル, 宣言元);
    let 辺インスタンス参照列 = edge_ref::辺インスタンス参照列を組み立てる(意味モデル, 宣言元);
    let node_refs = ref_collections::node_refs本体を組み立てる(意味モデル, 宣言元);
    let edge_refs = ref_collections::edge_refs本体を組み立てる(意味モデル, 宣言元);
    let graph = graph_struct::graph本体を組み立てる(意味モデル);

    quote! {
        #nodes
        #edges
        #個体参照列
        #辺インスタンス参照列
        #node_refs
        #edge_refs
        #graph
    }
}
