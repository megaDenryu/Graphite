//! 構築器の impl 全体を、部品ごとの生成を並べて組み立てる。

pub(crate) mod edge_insert_api;
pub(crate) mod extend_api;
pub(crate) mod freezable_implementation;
pub(crate) mod kind_methods;
pub(crate) mod node_insert_api;
pub(crate) mod struct_definition;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::naming::construction_stamp_field_ident;
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::freeze::gen_freeze_body;
use crate::schema::codegen::node_names::NodeInfo;
use edge_insert_api::gen_builder_edge_insert_api;
use extend_api::gen_builder_extend_api;
use freezable_implementation::gen_freezable_builder_impl;
use kind_methods::gen_builder_kind_methods;
use node_insert_api::gen_builder_node_insert_api;

// 生成する構築器の型名群とスキーマ情報を一か所で受け取るため引数が多い。
#[allow(clippy::too_many_arguments)]
pub(crate) fn gen_builder_impl(
    builder_ident: &Ident,
    violation_ident: &Ident,
    node_trait_ident: &Ident,
    edge_trait_ident: &Ident,
    default_id_trait_ident: &Ident,
    schema_name: &Ident,
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_field_inits = nodes.iter().map(|n| {
        let field = &n.field_ident;
        quote! { #field: Vec::new() }
    });
    let edge_field_inits = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        quote! { #accessor: Vec::new() }
    });
    let kind_methods = gen_builder_kind_methods(nodes, edges);
    let node_insert_api = gen_builder_node_insert_api(node_trait_ident, default_id_trait_ident);
    let edge_insert_api = gen_builder_edge_insert_api(edge_trait_ident, default_id_trait_ident);
    let extend_api = gen_builder_extend_api(default_id_trait_ident);
    let freeze_body = gen_freeze_body(schema_name, violation_ident, nodes, edges);
    let freezable_impl = gen_freezable_builder_impl(builder_ident, schema_name, violation_ident);
    let stamp_field = construction_stamp_field_ident(builder_ident.span());

    quote! {
        impl #builder_ident {
            fn new() -> Self {
                Self {
                    #(#node_field_inits,)*
                    #(#edge_field_inits,)*
                    #stamp_field: graphite::次の構築印を発行する(),
                }
            }

            #kind_methods
            #node_insert_api
            #edge_insert_api
            #extend_api

            #freeze_body
        }

        #freezable_impl
    }
}
