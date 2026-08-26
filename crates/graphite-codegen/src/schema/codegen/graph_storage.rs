//! 凍結済みグラフが所有するノード表・辺表・各種索引のフィールドを生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::naming::{construction_stamp_field_ident, pair_index_field_ident};
use crate::schema::codegen::declaration_doc::宣言元への参照;
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;
use crate::schema::codegen::pair_index::gen_pair_index_map_type;
use crate::schema::semantic::{EachSide, RoleCardinality};

pub(crate) fn gen_schema_struct(
    schema_name: &Ident,
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
    schema宣言元への参照: &宣言元への参照,
) -> TokenStream {
    let stamp_field = construction_stamp_field_ident(schema_name.span());
    let node_fields = nodes.iter().map(|n| {
        let field = &n.field_ident;
        let id = &n.id_ty;
        let ty = &n.type_ident;
        quote! { #field: graphite::KeyedTable<#id, super::#ty> }
    });
    let edge_fields = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        let index_field = &e.index_field_ident;
        let id_ty = &e.id_ty;
        let record = e.record_ident();
        let edge_position = e.internal_position_ident();
        // 有向辺のみ終点索引を永続化する (`docs/reverse_query.md`)。
        // 終点役割クエリの索引であり、v4.1 で入次数 each 検証のためだけに
        // 一時構築していた索引をこれに統合した (無向辺は `index_field` が
        // 既に対称に両端を積むので不要)。
        let pair_index = pair_index_field_ident(e.kind);
        let pair_index_type = gen_pair_index_map_type(e);
        let to_index_decl = if e.is_directed() {
            let to_index_field = &e.to_index_field_ident;
            let to_index_ty = role_index_type(e, EachSide::Target, &edge_position);
            quote! {
                ,
                /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
                /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
                #to_index_field: #to_index_ty,
                #pair_index: #pair_index_type
            }
        } else {
            quote! {
                ,
                #pair_index: #pair_index_type
            }
        };
        let index_ty = if e.is_directed() {
            role_index_type(e, EachSide::Source, &edge_position)
        } else {
            quote! { graphite::MultipleRoleIndex<#edge_position> }
        };
        quote! {
            #accessor: graphite::KeyedTable<#id_ty, #record>,
            /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
            /// キーの一覧 (凍結時に構築)。
            #index_field: #index_ty
            #to_index_decl
        }
    });

    quote! {
        /// 凍結済み図式グラフ。構築後の構造は不変で、ノード値と辺の積み荷だけを
        /// `&mut Graph` を要求する種別APIから更新できる。
        #schema宣言元への参照
        pub struct #schema_name {
            #(#node_fields,)*
            #(#edge_fields,)*
            /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
            /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
            /// するかを `NamedGraphElement::bind` が照合するのに使う。
            #stamp_field: u64,
        }
    }
}

pub(crate) fn role_index_type(
    edge: &EdgeInfo<'_>,
    side: EachSide,
    edge_position: &Ident,
) -> TokenStream {
    match edge.cardinality(side) {
        RoleCardinality::Exact => quote! { graphite::ExactlyOneRoleIndex<#edge_position> },
        RoleCardinality::Optional => quote! { graphite::OptionalRoleIndex<#edge_position> },
        RoleCardinality::Multiple => quote! { graphite::MultipleRoleIndex<#edge_position> },
    }
}
