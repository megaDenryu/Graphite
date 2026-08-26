//! 具体型を知らずに凍結を呼べるようにする橋渡しの実装を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

/// 凍結を具体型を知らずに呼べるようにする橋渡しの実装を生成する。
pub(crate) fn gen_freezable_builder_impl(
    builder_ident: &Ident,
    schema_name: &Ident,
    violation_ident: &Ident,
) -> TokenStream {
    quote! {
        /// [`graphite::build_named_graph`] が `#schema_name`/`#violation_ident`
        /// の具体型を知らずに凍結を呼べるようにするための橋渡し。
        /// `freeze_into_graph` は既存の私有 `freeze()` (上記) へそのまま委譲する。
        impl graphite::FreezableBuilder for #builder_ident {
            type Graph = #schema_name;
            type Violation = #violation_ident;

            fn freeze_into_graph(self) -> Result<Self::Graph, Self::Violation> {
                self.freeze()
            }
        }
    }
}
