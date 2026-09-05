//! 有向辺の `EdgeRef` 型と、役割名による端点の取り出しを生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::reference::core_methods::edge_reference_core_methods;
use crate::schema::codegen::reference::debug_implementation::gen_reference_debug_impl;
use crate::schema::codegen::reference::payload_methods::edge_reference_payload_methods;

// 有向辺1種別分の `EdgeRef` を生成する。両端は役割名のメソッドで返し、
// `from`/`to`/`from_id`/`to_id` は役割名によらない固定名の別名にする。
pub(crate) fn gen_directed_edge_reference_type(
    graph_ident: &Ident,
    edge: &EdgeInfo<'_>,
    from_role: &Ident,
    to_role: &Ident,
) -> TokenStream {
    let id_ty = &edge.id_ty;
    let accessor = &edge.accessor_ident;
    let reference = edge.reference_ident();
    let internal_position = edge.internal_position_ident();
    let record = edge.record_ident();
    let kind_span = edge.kind.span();
    let 宣言元への参照 = &edge.宣言元への参照;
    let core_methods =
        edge_reference_core_methods(accessor, &record, id_ty, kind_span, 宣言元への参照);
    let payload_methods =
        edge_reference_payload_methods(edge.kind, edge.payload(), 宣言元への参照);
    let debug_impl = gen_reference_debug_impl(&reference, edge.id_ty.is_debug_printable());
    let from_reference = edge.from_node.reference_ident();
    let to_reference = edge.to_node.reference_ident();
    let from_position = edge.from_node.internal_position_ident();
    let to_position = edge.to_node.internal_position_ident();
    let from_id = &edge.from_node.id_ty;
    let to_id = &edge.to_node.id_ty;
    let from_ident = Ident::new("from", kind_span);
    let to_ident = Ident::new("to", kind_span);
    let from_id_ident = Ident::new("from_id", kind_span);
    let to_id_ident = Ident::new("to_id", kind_span);
    quote! {
        /// 完成済みグラフ上の有向辺個体。
        #宣言元への参照
        #[derive(Clone, Copy)]
        pub struct #reference<'graph> {
            graph: &'graph #graph_ident,
            internal_position: #internal_position,
        }

        impl<'graph> #reference<'graph> {
            #core_methods

            /// この辺個体の始点側の端点を役割名で返す。
            #宣言元への参照
            pub fn #from_role(self) -> #from_reference<'graph> {
                #from_reference {
                    graph: self.graph,
                    internal_position: #from_position(self.record().#from_role.0),
                }
            }

            /// この辺個体の終点側の端点を役割名で返す。
            #宣言元への参照
            pub fn #to_role(self) -> #to_reference<'graph> {
                #to_reference {
                    graph: self.graph,
                    internal_position: #to_position(self.record().#to_role.0),
                }
            }

            /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
            #宣言元への参照
            pub fn #from_ident(self) -> #from_reference<'graph> {
                self.#from_role()
            }

            /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
            #宣言元への参照
            pub fn #to_ident(self) -> #to_reference<'graph> {
                self.#to_role()
            }

            /// この辺個体の始点側の端点の公開IDを借用する。
            #宣言元への参照
            pub fn #from_id_ident(self) -> &'graph #from_id {
                self.from().id()
            }

            /// この辺個体の終点側の端点の公開IDを借用する。
            #宣言元への参照
            pub fn #to_id_ident(self) -> &'graph #to_id {
                self.to().id()
            }

            #payload_methods
        }

        #debug_impl
    }
}
