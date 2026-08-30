// 生成物7: グラフ本体。node_refs/edge_refs (旧 ノード参照達/辺参照達) の
// 2フィールドを持つだけ。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::literal::input::静的グラフ入力;

pub(super) fn グラフ本体を生成する(instance: &静的グラフ入力) -> TokenStream {
    let グラフ名 = &instance.グラフ名;
    quote! {
        struct #グラフ名<'a> {
            node_refs: NodeRefs<'a>,
            edge_refs: EdgeRefs<'a>,
        }
        impl<'a> #グラフ名<'a> {
            fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
                Self {
                    node_refs: NodeRefs::new(nodes, edges),
                    edge_refs: EdgeRefs::new(nodes, edges),
                }
            }
        }
    }
}
