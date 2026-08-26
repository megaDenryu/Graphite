//! ノード・辺の各要素型への挿入トレイト実装と名前付き位置の束縛を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::public_id_type::PublicIdType;

/// ノードと辺に共通する名前付き挿入・名前付き位置の束縛実装を生成する。
pub(crate) struct InsertableNamedSpec<'a> {
    pub(crate) insertable_trait_ident: &'a Ident,
    pub(crate) builder_ident: &'a Ident,
    pub(crate) graph_ident: &'a Ident,
    pub(crate) value_type: TokenStream,
    pub(crate) id_ty: PublicIdType<'a>,
    pub(crate) named_position: &'a Ident,
    pub(crate) internal_position: &'a Ident,
    pub(crate) storage: &'a Ident,
    pub(crate) accessor: &'a Ident,
    pub(crate) reference: &'a Ident,
    pub(crate) stamp_field: &'a Ident,
    pub(crate) span: proc_macro2::Span,
}

pub(crate) fn gen_insertable_and_named_impl(spec: InsertableNamedSpec<'_>) -> TokenStream {
    let InsertableNamedSpec {
        insertable_trait_ident,
        builder_ident,
        graph_ident,
        value_type,
        id_ty,
        named_position,
        internal_position,
        storage,
        accessor,
        reference,
        stamp_field,
        span,
    } = spec;
    let insert_named_with_id = Ident::new("insert_named_with_id", span);
    let insert_with_id = Ident::new("insert_with_id", span);
    quote! {
        impl #insertable_trait_ident for #value_type {
            type Id = #id_ty;
            type NamedPosition = #named_position;

            fn #insert_named_with_id(
                self,
                b: &mut #builder_ident,
                id: Self::Id,
                _permit: &graphite::NamedInsertPermit,
            ) -> (Self::Id, Self::NamedPosition) {
                let named_position = #named_position(
                    #internal_position(graphite::TablePosition(b.#storage.len())),
                    b.#stamp_field,
                );
                let returned_id = id.clone();
                b.#accessor(id, self);
                (returned_id, named_position)
            }

            fn #insert_with_id(self, b: &mut #builder_ident, id: Self::Id) -> Self::Id {
                let returned_id = id.clone();
                b.#accessor(id, self);
                returned_id
            }
        }

        impl graphite::NamedGraphElement<#graph_ident> for #named_position {
            type Reference<'graph> = #reference<'graph>;

            fn bind<'graph>(&self, graph: &'graph #graph_ident) -> Self::Reference<'graph> {
                if graph.#stamp_field != self.1 {
                    panic!("名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です");
                }
                #reference { graph, internal_position: self.0 }
            }
        }
    }
}
