//! 辺値型の `Debug` 実装を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::辺の向き;

/// 辺値型の `Debug` 実装を生成する。
///
/// 利用者定義IDと積み荷へ `Debug` を要求しない契約を守るため、端点を表示できるのは
/// 両端が自動生成IDで積み荷がない場合に限る。
pub(crate) fn gen_edge_value_debug_impl(e: &EdgeInfo<'_>) -> TokenStream {
    let kind = e.kind;
    if !(e.payload().is_none()
        && e.from_node.id_ty.is_debug_printable()
        && e.to_node.id_ty.is_debug_printable())
    {
        return quote! {
            impl std::fmt::Debug for #kind {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_str(stringify!(#kind))
                }
            }
        };
    }
    let (first, second) = match e.shape() {
        辺の向き::有向 { 始点, 終点 } => {
            let from_role = 始点.役割名();
            let to_role = 終点.役割名();
            (quote! { self.#from_role }, quote! { self.#to_role })
        }
        辺の向き::無向 { .. } => {
            (quote! { self.endpoints().0 }, quote! { self.endpoints().1 })
        }
    };
    quote! {
        impl std::fmt::Debug for #kind {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!(#kind))
                    .field(&#first)
                    .field(&#second)
                    .finish()
            }
        }
    }
}
