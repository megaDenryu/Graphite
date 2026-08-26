//! 辺値型の関連コンストラクタと端点・積み荷の読み取りを生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::辺の向き;

/// 辺値型の関連コンストラクタと、積み荷・端点の読み取りを生成する。
pub(crate) fn gen_edge_value_constructor(e: &EdgeInfo<'_>) -> TokenStream {
    let p0_id = &e.from_node.id_ty;
    let p1_id = &e.to_node.id_ty;
    match (e.shape(), e.payload()) {
        (辺の向き::有向 { 始点, 終点 }, None) => {
            let from_role = 始点.役割名();
            let to_role = 終点.役割名();
            quote! {
                pub fn new(from: #p0_id, to: #p1_id) -> Self {
                    Self { #from_role: from, #to_role: to }
                }
            }
        }
        (辺の向き::有向 { 始点, 終点 }, Some(payload)) => {
            let from_role = 始点.役割名();
            let to_role = 終点.役割名();
            let payload_role = payload.役割名();
            let attrs = payload.型パス();
            quote! {
                pub fn new(from: #p0_id, to: #p1_id, payload: #attrs) -> Self {
                    Self {
                        #from_role: from,
                        #to_role: to,
                        #payload_role: payload,
                    }
                }
                pub fn payload(&self) -> &#attrs { &self.#payload_role }
            }
        }
        (辺の向き::無向 { .. }, None) => quote! {
            pub fn new(a: #p0_id, b: #p1_id) -> Self {
                Self { endpoints: graphite::UnorderedPair::new(a, b) }
            }
            pub fn endpoints(&self) -> (&#p0_id, &#p1_id) {
                self.endpoints.endpoints()
            }
        },
        (辺の向き::無向 { .. }, Some(payload)) => {
            let payload_role = payload.役割名();
            let attrs = payload.型パス();
            quote! {
                pub fn new(a: #p0_id, b: #p1_id, payload: #attrs) -> Self {
                    Self { endpoints: graphite::UnorderedPair::new(a, b), #payload_role: payload }
                }
                pub fn endpoints(&self) -> (&#p0_id, &#p1_id) {
                    self.endpoints.endpoints()
                }
                pub fn payload(&self) -> &#attrs { &self.#payload_role }
            }
        }
    }
}
