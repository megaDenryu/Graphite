//! 無向辺の `EdgeRef` 型と、順序なし対としての端点の取り出しを生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::reference::core_methods::edge_reference_core_methods;
use crate::schema::codegen::reference::debug_implementation::gen_reference_debug_impl;
use crate::schema::codegen::reference::payload_methods::edge_reference_payload_methods;

/// 無向辺1種別分の `EdgeRef` を生成する。位置の区別が無いため、両端は
/// `endpoints()` が順序なし対として返す。
pub(crate) fn gen_undirected_edge_reference_type(
    graph_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> TokenStream {
    let id_ty = &edge.id_ty;
    let accessor = &edge.accessor_ident;
    let reference = edge.reference_ident();
    let internal_position = edge.internal_position_ident();
    let record = edge.record_ident();
    let kind_span = edge.kind.span();
    let core_methods = edge_reference_core_methods(accessor, &record, id_ty, kind_span);
    let payload_methods = edge_reference_payload_methods(edge.kind, edge.payload());
    let debug_impl = gen_reference_debug_impl(&reference, edge.id_ty.is_debug_printable());
    let node_reference = edge.from_node.reference_ident();
    let node_position = edge.from_node.internal_position_ident();
    let endpoints_ident = Ident::new("endpoints", kind_span);
    quote! {
        /// 完成済みグラフ上の無向辺個体。
        #[derive(Clone, Copy)]
        pub struct #reference<'graph> {
            graph: &'graph #graph_ident,
            internal_position: #internal_position,
        }

        impl<'graph> #reference<'graph> {
            #core_methods

            pub fn #endpoints_ident(self) -> (#node_reference<'graph>, #node_reference<'graph>) {
                let (first, second) = self.record().endpoints.endpoints();
                (
                    #node_reference {
                        graph: self.graph,
                        internal_position: #node_position(first.0),
                    },
                    #node_reference {
                        graph: self.graph,
                        internal_position: #node_position(second.0),
                    },
                )
            }

            #payload_methods
        }

        #debug_impl
    }
}
