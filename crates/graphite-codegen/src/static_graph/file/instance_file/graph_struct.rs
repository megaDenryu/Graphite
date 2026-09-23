// このファイルは `Graph` (具体グラフ本体、issue #41 確定判断1で固定名に
// した) の本体を組み立てる。`Graph` は `node_refs`/`edge_refs` の2
// フィールドだけを持つ (issue #41 §5.3、B分類)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::naming::{個体参照フィールド名, 構築メソッド名, グラフ型名, 辺参照フィールド名};
use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::固定語彙の所有者;

use super::super::doc_render::doc属性を組み立てる;

pub(super) fn graph本体を組み立てる(意味モデル: &意味モデル) -> TokenStream {
    let 型名 = グラフ型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let 構築名 = 構築メソッド名(固定語彙の所有者::Graph, 意味モデル);
    let 構築doc = doc属性を組み立てる(構築名.追跡());
    let node_refsフィールド = 個体参照フィールド名(意味モデル);
    let node_refsdoc = doc属性を組み立てる(node_refsフィールド.追跡());
    let edge_refsフィールド = 辺参照フィールド名(意味モデル);
    let edge_refsdoc = doc属性を組み立てる(edge_refsフィールド.追跡());

    quote! {
        #型doc
        pub struct #型名<'a> {
            #node_refsdoc
            pub #node_refsフィールド: NodeRefs<'a>,
            #edge_refsdoc
            pub #edge_refsフィールド: EdgeRefs<'a>,
        }
        impl<'a> #型名<'a> {
            #構築doc
            pub fn #構築名(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
                Self {
                    #node_refsフィールド: NodeRefs::new(nodes, edges),
                    #edge_refsフィールド: EdgeRefs::new(nodes, edges),
                }
            }
        }
    }
}
