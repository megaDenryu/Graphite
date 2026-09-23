// このファイルは `Graph` (具体グラフ本体、Graph型名を固定名にする確定判断
// による) の本体を組み立てる。`Graph` は `node_refs`/`edge_refs` の2
// フィールドだけを持つ (固定語彙、B分類)。フィールドの型・構築メソッド名
// (`NodeRefs`/`EdgeRefs`・それぞれの `new`) も `naming::fixed_vocabulary`
// から読み、この生成器の他の箇所と綴りを1箇所で揃える。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::naming::{
    edges変数名, nodes変数名, 個体参照フィールド名, 個体参照集合型名, 構築メソッド名, グラフ型名, 辺参照フィールド名,
    辺参照集合型名,
};
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
    let node_refs型 = 個体参照集合型名(意味モデル);
    let edge_refs型 = 辺参照集合型名(意味モデル);
    let node_refs構築名 = 構築メソッド名(固定語彙の所有者::NodeRefs, 意味モデル);
    let edge_refs構築名 = 構築メソッド名(固定語彙の所有者::EdgeRefs, 意味モデル);
    let nodes = nodes変数名();
    let edges = edges変数名();

    quote! {
        #型doc
        pub struct #型名<'a> {
            #node_refsdoc
            pub #node_refsフィールド: #node_refs型<'a>,
            #edge_refsdoc
            pub #edge_refsフィールド: #edge_refs型<'a>,
        }
        impl<'a> #型名<'a> {
            #構築doc
            pub fn #構築名(#nodes: &'a Nodes, #edges: &'a Edges<'a>) -> Self {
                Self {
                    #node_refsフィールド: #node_refs型::#node_refs構築名(#nodes, #edges),
                    #edge_refsフィールド: #edge_refs型::#edge_refs構築名(#nodes, #edges),
                }
            }
        }
    }
}
