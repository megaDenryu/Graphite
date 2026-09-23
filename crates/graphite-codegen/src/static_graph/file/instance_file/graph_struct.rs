// このファイルは `Graph` (具体グラフ本体、Graph型名を固定名にする確定判断
// による) の本体を組み立てる。`Graph` は `node_refs`/`edge_refs` の2
// フィールドだけを持つ (固定語彙、B分類)。`Graph::new` は `&'a Edges<'a>`
// だけを引数に取り (PR #45レビューC)、`Edges` が構築時に保持した
// `&'a Nodes` (`edge_entities.rs` の `__graphite_nodes` フィールド) から
// 由来の `Nodes` を導く。これにより、別の `Nodes` から作った `Edges` を
// 組み合わせて不整合な `Graph` を作ることができない (`Graph::new`が受け取る
// 値は常に1組の `(Nodes, Edges)` に由来する)。`NodeRefs::new`/
// `EdgeRefs::new` (`ref_collections.rs`) はこの関数だけが呼ぶ内部専用の
// 素の構築子であり (C分類、非公開)、フィールドの型・構築子名は
// `naming::fixed_vocabulary` から読み、この生成器の他の箇所と綴りを1箇所で
// 揃える。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::naming::{
    edges変数名, nodes変数名, 個体参照フィールド名, 個体参照集合型名, 構築メソッド名, グラフ型名, 辺参照フィールド名,
    辺参照集合型名,
};
use crate::static_graph::semantic::意味モデル;

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn graph本体を組み立てる(意味モデル: &意味モデル) -> TokenStream {
    let 型名 = グラフ型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let 構築名 = 構築メソッド名(意味モデル);
    let 構築doc = doc属性を組み立てる(構築名.追跡());
    let node_refsフィールド = 個体参照フィールド名(意味モデル);
    let node_refsdoc = doc属性を組み立てる(node_refsフィールド.追跡());
    let edge_refsフィールド = 辺参照フィールド名(意味モデル);
    let edge_refsdoc = doc属性を組み立てる(edge_refsフィールド.追跡());
    let node_refs型 = 個体参照集合型名(意味モデル);
    let edge_refs型 = 辺参照集合型名(意味モデル);
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
            pub fn #構築名(#edges: &'a Edges<'a>) -> Self {
                let #nodes = #edges.__graphite_nodes;
                Self {
                    #node_refsフィールド: #node_refs型::new(#nodes, #edges),
                    #edge_refsフィールド: #edge_refs型::new(#nodes, #edges),
                }
            }
        }
    }
}
