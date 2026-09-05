//! 構築器そのものの struct 定義を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::naming::construction_stamp_field_ident;
use crate::schema::codegen::declaration_doc::宣言元への参照;
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;

pub(crate) fn gen_builder_struct(
    builder_ident: &Ident,
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
    スキーマ宣言元への参照: &宣言元への参照,
) -> TokenStream {
    let stamp_field = construction_stamp_field_ident(builder_ident.span());
    let node_fields = nodes.iter().map(|n| {
        let field = &n.field_ident;
        let id = &n.id_ty;
        let ty = &n.type_ident;
        quote! { #field: Vec<(#id, super::#ty)> }
    });
    let edge_fields = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        let id_ty = &e.id_ty;
        let kind = e.kind;
        quote! { #accessor: Vec<(#id_ty, #kind)> }
    });

    quote! {
        /// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
        /// 制約検査を一切行わない。
        #スキーマ宣言元への参照
        pub struct #builder_ident {
            #(#node_fields,)*
            #(#edge_fields,)*
            /// この構築を識別する構築印。`Builder::new()` が発行し、この
            /// `Builder` から挿入する全ての名前付き位置と、凍結成功後の
            /// `Graph` へ同じ値を刻む。
            #stamp_field: u64,
        }
    }
}
