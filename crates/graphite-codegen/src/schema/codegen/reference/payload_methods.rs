//! 辺参照が積み荷を読み出すメソッドを生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::semantic::積み荷;

/// 辺参照値の積み荷アクセサ (役割名メソッドと `payload()` エイリアス) を
/// 生成する (積み荷が無ければ空)。有向/無向で生成コードが同一なため
/// `gen_edge_reference_types` から共有する純粋関数。`payload()` のスパンは
/// 辺種別トークンを継承する (`docs/development/ide_support_spec.md` §1.9)。
pub(crate) fn edge_reference_payload_methods(
    kind: &Ident,
    payload: Option<&積み荷>,
) -> TokenStream {
    let payload_ident = Ident::new("payload", kind.span());
    let methods = payload.into_iter().map(|payload| {
        let role = payload.役割名();
        let ty = payload.型パス();
        quote! {
            pub fn #role(self) -> &'graph #ty {
                &self.record().#role
            }

            pub fn #payload_ident(self) -> &'graph #ty {
                &self.record().#role
            }
        }
    });
    quote! { #(#methods)* }
}
