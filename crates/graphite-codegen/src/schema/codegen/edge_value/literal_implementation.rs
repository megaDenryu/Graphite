//! `graph!` の辺リテラルから辺値型を作る trait 実装を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::辺の向き;

/// `graph!` の辺リテラルから辺値型を組み立てる trait 実装を生成する。
pub(crate) fn gen_edge_value_literal_impl(e: &EdgeInfo<'_>) -> TokenStream {
    let kind = e.kind;
    let p0_id = &e.from_node.id_ty;
    let p1_id = &e.to_node.id_ty;
    match (e.shape(), e.payload()) {
        (辺の向き::有向 { .. }, None) => quote! {
            impl graphite::DirectedEdgeLiteral<#p0_id, #p1_id, ()> for #kind {
                fn from_graph_literal(from: #p0_id, to: #p1_id, (): ()) -> Self {
                    Self::new(from, to)
                }
            }
        },
        (辺の向き::有向 { .. }, Some(payload)) => {
            let attrs = payload.型パス();
            quote! {
                impl graphite::DirectedEdgeLiteral<#p0_id, #p1_id, #attrs> for #kind {
                    fn from_graph_literal(
                        from: #p0_id,
                        to: #p1_id,
                        payload: #attrs,
                    ) -> Self {
                        Self::new(from, to, payload)
                    }
                }
            }
        }
        (辺の向き::無向 { .. }, None) => quote! {
            impl graphite::UndirectedEdgeLiteral<#p0_id, ()> for #kind {
                fn from_graph_literal(a: #p0_id, b: #p0_id, (): ()) -> Self {
                    Self::new(a, b)
                }
            }
        },
        (辺の向き::無向 { .. }, Some(payload)) => {
            let attrs = payload.型パス();
            quote! {
                impl graphite::UndirectedEdgeLiteral<#p0_id, #attrs> for #kind {
                    fn from_graph_literal(
                        a: #p0_id,
                        b: #p0_id,
                        payload: #attrs,
                    ) -> Self {
                        Self::new(a, b, payload)
                    }
                }
            }
        }
    }
}
