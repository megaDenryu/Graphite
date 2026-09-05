//! 束縛名の文字列から既定IDを作るトレイト実装を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::public_id_type::PublicIdType;

// 束縛名の文字列から既定IDを作る `{Schema}DefaultId` の実装を生成する。
// 明示ID型の要素には実装しないため、その場合は空を返す。
pub(crate) fn gen_default_id_impl(
    default_id_trait_ident: &Ident,
    insertable_trait_ident: &Ident,
    builder_ident: &Ident,
    value_type: &TokenStream,
    id_ty: PublicIdType<'_>,
) -> TokenStream {
    let Some(generated_id) = id_ty.generated_ident() else {
        return quote! {};
    };
    quote! {
        impl #default_id_trait_ident for #value_type {
            fn insert_named_with_binding(
                self,
                b: &mut #builder_ident,
                binding: String,
                permit: &graphite::NamedInsertPermit,
            ) -> (Self::Id, Self::NamedPosition) {
                #insertable_trait_ident::insert_named_with_id(
                    self,
                    b,
                    #generated_id(binding),
                    permit,
                )
            }

            fn insert_with_binding(self, b: &mut #builder_ident, binding: String) -> Self::Id {
                #insertable_trait_ident::insert_with_id(self, b, #generated_id(binding))
            }
        }
    }
}
