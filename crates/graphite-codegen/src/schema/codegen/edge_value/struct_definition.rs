//! 辺値型の struct 定義を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::辺の向き;

// 辺値型の struct 定義を生成する。
pub(crate) fn gen_edge_value_struct_definition(e: &EdgeInfo<'_>) -> TokenStream {
    let kind = e.kind;
    let p0_id = &e.from_node.id_ty;
    let p1_id = &e.to_node.id_ty;
    match (e.shape(), e.payload()) {
        (辺の向き::有向 { 始点, 終点 }, None) => {
            let from_role = 始点.役割名();
            let to_role = 終点.役割名();
            quote! {
                pub struct #kind {
                    /// この辺の始点ノードの公開ID。
                    pub #from_role: #p0_id,
                    /// この辺の終点ノードの公開ID。
                    pub #to_role: #p1_id,
                }
            }
        }
        (辺の向き::有向 { 始点, 終点 }, Some(payload)) => {
            let from_role = 始点.役割名();
            let to_role = 終点.役割名();
            let payload_role = payload.役割名();
            let attrs = payload.型パス();
            quote! {
                pub struct #kind {
                    /// この辺の始点ノードの公開ID。
                    pub #from_role: #p0_id,
                    /// この辺の終点ノードの公開ID。
                    pub #to_role: #p1_id,
                    /// この辺が運ぶ積み荷。
                    pub #payload_role: #attrs,
                }
            }
        }
        (辺の向き::無向 { .. }, None) => {
            quote! { pub struct #kind { endpoints: graphite::UnorderedPair<#p0_id> } }
        }
        (辺の向き::無向 { .. }, Some(payload)) => {
            let payload_role = payload.役割名();
            let attrs = payload.型パス();
            quote! {
                pub struct #kind {
                    endpoints: graphite::UnorderedPair<#p0_id>,
                    /// この辺が運ぶ積み荷。
                    pub #payload_role: #attrs,
                }
            }
        }
    }
}
