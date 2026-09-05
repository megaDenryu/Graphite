//! 凍結後の辺が端点の内部位置と積み荷を持つ非公開レコード型を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::{積み荷, 辺の向き};

// 辺レコード構造体・辺参照値の積み荷フィールド `role: 型` を生成する
// (積み荷が無ければ空)。有向/無向で生成コードが同一なため
// `gen_edge_record_structs` から共有する純粋関数。
pub(crate) fn edge_record_payload_fields(payload: Option<&積み荷>) -> Vec<TokenStream> {
    payload
        .into_iter()
        .map(|payload| {
            let role = payload.役割名();
            let ty = payload.型パス();
            quote! { #role: #ty }
        })
        .collect()
}

// 辺値は構築時の公開IDを保持するが、完成後のレコードは端点を内部位置で
// 保持する。積み荷だけを辺値から移して保持し、探索時のID検索を不要にする。
pub(crate) fn gen_edge_record_structs(edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    edges
        .iter()
        .map(|edge| {
            let record = edge.record_ident();
            let from_position = edge.from_node.internal_position_ident();
            let to_position = edge.to_node.internal_position_ident();
            let payload_field = edge_record_payload_fields(edge.payload());
            match edge.shape() {
                辺の向き::有向 { 始点, 終点 } => {
                    let from_role = 始点.役割名();
                    let to_role = 終点.役割名();
                    quote! {
                        #[allow(dead_code)]
                        struct #record {
                            #from_role: #from_position,
                            #to_role: #to_position,
                            #(#payload_field,)*
                        }
                    }
                }
                辺の向き::無向 { .. } => {
                    quote! {
                        #[allow(dead_code)]
                        struct #record {
                            endpoints: graphite::UnorderedPair<#from_position>,
                            #(#payload_field,)*
                        }
                    }
                }
            }
        })
        .collect()
}
