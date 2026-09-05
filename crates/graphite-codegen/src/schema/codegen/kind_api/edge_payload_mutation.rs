//! 辺の構造を保ったまま積み荷だけを可変借用するメソッドを生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::naming::kind_api_method_ident;
use crate::schema::codegen::edge_names::EdgeInfo;

// 辺の構造を保ったまま積み荷だけを可変借用する `{kind}_payload_mut` を
// `Graph` のメソッドとして生成する (積み荷が無ければ空)。
//
// 主語は `&mut Graph` である。`EdgeRef` は共有借用のハンドルなのでそこから
// 可変借用は作れず、引数も公開IDのままにする。
pub(crate) fn gen_edge_payload_mut_method(edge: &EdgeInfo<'_>) -> TokenStream {
    let Some(payload) = edge.payload() else {
        return quote! {};
    };
    let id_ty = &edge.id_ty;
    let accessor = &edge.accessor_ident;
    let record = edge.record_ident();
    let payload_role = payload.役割名();
    let payload_ty = payload.型パス();
    let payload_mut = kind_api_method_ident(accessor, "payload_mut");
    let 宣言元への参照 = &edge.宣言元への参照;
    quote! {
        /// 辺の構造を保ったまま積み荷だけを可変借用する。
        #宣言元への参照
        pub fn #payload_mut(&mut self, id: &#id_ty) -> Option<&mut #payload_ty> {
            self.#accessor.get_mut(id).map(|record: &mut #record| &mut record.#payload_role)
        }
    }
}
