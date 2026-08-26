//! 収集し終えた役割索引を、多重度に応じた公開表現へ確定する文を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::{EachSide, RoleCardinality};

pub(crate) fn finalize_role_index(
    edge: &EdgeInfo<'_>,
    side: EachSide,
    index: &Ident,
    node_field: &Ident,
    node_position: &Ident,
) -> TokenStream {
    let constructor = match edge.cardinality(side) {
        RoleCardinality::Exact => quote! { graphite::ExactlyOneRoleIndex::from_buckets },
        RoleCardinality::Optional => quote! { graphite::OptionalRoleIndex::from_buckets },
        RoleCardinality::Multiple => quote! { graphite::MultipleRoleIndex::from_buckets },
    };
    quote! {
        let #index = #constructor(
            (0..#node_field.len())
                .map(|position| {
                    #index
                        .remove(&#node_position(graphite::TablePosition(position)))
                        .unwrap_or_default()
                })
                .collect()
        );
    }
}
