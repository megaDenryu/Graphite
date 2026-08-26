//! 無向辺の接続探索メソッドを生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::naming::incident_method_ident;
use crate::schema::codegen::edge_names::EdgeInfo;

/// 無向辺の接続探索メソッド `{kind}_incident()` を生成する。
pub(crate) fn gen_incident_traversal_method(edge: &EdgeInfo<'_>) -> TokenStream {
    let method = incident_method_ident(edge.kind);
    let edge_reference = edge.reference_ident();
    let index = &edge.index_field_ident;
    quote! {
        /// 接続辺を O(1) で参照し、追加確保なしで挿入順に走査する。
        pub fn #method(self) -> impl Iterator<Item = #edge_reference<'graph>> + 'graph {
            let positions = self.graph.#index.get(self.internal_position.0);
            positions.iter().copied().map(move |internal_position| #edge_reference {
                graph: self.graph,
                internal_position,
            })
        }
    }
}
