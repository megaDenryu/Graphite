//! 未知の端点キーを参照したことを表す違反 variant とその表示を生成する。
//!
//! このファイルは1ファイル100行の原則の例外である。有向始点・有向終点・無向
//! 端点の3ケースは variant 定義と Debug 可否による表示分岐が同じ形であり、
//! 分割すると3ケースの対応関係が読み比べられなくなるため、まとめて置いている。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;

/// 有向辺が未知の始点キーを参照した場合 (`{Kind}UnknownSource`)。
pub(crate) fn gen_unknown_source_case(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> (TokenStream, TokenStream) {
    let kind_str = edge.kind.to_string();
    let edge_id = &edge.id_ty;
    let from_id = &edge.from_node.id_ty;
    let from_type_str = edge.from_node.type_ident.to_string();
    let unk_src = edge.unknown_source_variant();
    let variant = quote! {
        /// このエッジが未知の始点キーを参照している。
        #unk_src { edge: #edge_id, source: #from_id }
    };
    let display_arm =
        if edge.id_ty.is_debug_printable() && edge.from_node.id_ty.is_debug_printable() {
            quote! {
                #violation_ident::#unk_src { edge, source } => write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    #kind_str, edge, #from_type_str, source
                )
            }
        } else {
            quote! {
                #violation_ident::#unk_src { .. } => write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の始点, {})",
                    #kind_str, #from_type_str
                )
            }
        };
    (variant, display_arm)
}

/// 有向辺が未知の終点キーを参照した場合 (`{Kind}UnknownTarget`)。
pub(crate) fn gen_unknown_target_case(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> (TokenStream, TokenStream) {
    let kind_str = edge.kind.to_string();
    let edge_id = &edge.id_ty;
    let to_id = &edge.to_node.id_ty;
    let to_type_str = edge.to_node.type_ident.to_string();
    let unk_dst = edge.unknown_target_variant();
    let variant = quote! {
        /// このエッジが未知の終点キーを参照している。
        #unk_dst { edge: #edge_id, target: #to_id }
    };
    let display_arm = if edge.id_ty.is_debug_printable() && edge.to_node.id_ty.is_debug_printable()
    {
        quote! {
            #violation_ident::#unk_dst { edge, target } => write!(
                f,
                "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                #kind_str, edge, #to_type_str, target
            )
        }
    } else {
        quote! {
            #violation_ident::#unk_dst { .. } => write!(
                f,
                "未知のキーが参照されています (辺 `{}` の終点, {})",
                #kind_str, #to_type_str
            )
        }
    };
    (variant, display_arm)
}

/// 無向辺が未知の端点キーを参照した場合 (`{Kind}UnknownEndpoint`)。
/// 無向辺は位置の区別が無いため1種類で足りる。両端は同じノード型なので
/// `from_node` で代表する。
pub(crate) fn gen_unknown_endpoint_case(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> (TokenStream, TokenStream) {
    let kind_str = edge.kind.to_string();
    let edge_id = &edge.id_ty;
    let node_id = &edge.from_node.id_ty;
    let node_type_str = edge.from_node.type_ident.to_string();
    let unk = edge.unknown_endpoint_variant();
    let variant = quote! {
        /// このエッジが未知の端点キーを参照している (無向のため位置の
        /// 区別は無い)。
        #unk { edge: #edge_id, endpoint: #node_id }
    };
    let display_arm =
        if edge.id_ty.is_debug_printable() && edge.from_node.id_ty.is_debug_printable() {
            quote! {
                #violation_ident::#unk { edge, endpoint } => write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の端点, {}): {:?}",
                    #kind_str, edge, #node_type_str, endpoint
                )
            }
        } else {
            quote! {
                #violation_ident::#unk { .. } => write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の端点, {})",
                    #kind_str, #node_type_str
                )
            }
        };
    (variant, display_arm)
}
