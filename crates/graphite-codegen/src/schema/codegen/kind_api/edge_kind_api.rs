//! 辺種別1つ分の種別API (`Graph` のメソッド) を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::naming::kind_api_method_ident;
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::kind_api::edge_payload_mutation::gen_edge_payload_mut_method;

/// 辺種別1つ分の種別API (`Graph` のメソッド) を生成する。
/// `{kind}_payload_mut` は積み荷を持つ辺種別にだけ生やす。
pub(crate) fn gen_edge_kind_api_methods(edge: &EdgeInfo<'_>) -> TokenStream {
    let id_ty = &edge.id_ty;
    let accessor = &edge.accessor_ident;
    let reference = edge.reference_ident();
    let internal_position = edge.internal_position_ident();
    let by_id = kind_api_method_ident(accessor, "by_id");
    let ids = kind_api_method_ident(accessor, "ids");
    let iter = kind_api_method_ident(accessor, "iter");
    let len = kind_api_method_ident(accessor, "len");
    let payload_mut = gen_edge_payload_mut_method(edge);
    let 宣言元への参照 = &edge.宣言元への参照;
    quote! {
        /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
        #宣言元への参照
        pub fn #by_id<'graph>(&'graph self, id: &#id_ty) -> Option<#reference<'graph>> {
            Some(#reference {
                graph: self,
                internal_position: #internal_position(self.#accessor.position(id)?),
            })
        }

        #payload_mut

        /// この種別の辺の公開IDを挿入順に走査する。
        #宣言元への参照
        pub fn #ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph #id_ty> {
            self.#accessor.ids()
        }

        /// この種別の辺個体を挿入順に走査する。追加確保はしない。
        #宣言元への参照
        pub fn #iter<'graph>(&'graph self) -> impl Iterator<Item = #reference<'graph>> + 'graph {
            self.#accessor.positions().map(move |position| #reference {
                graph: self,
                internal_position: #internal_position(position),
            })
        }

        /// この種別の辺の件数を返す。
        #宣言元への参照
        pub fn #len(&self) -> usize {
            self.#accessor.len()
        }
    }
}
