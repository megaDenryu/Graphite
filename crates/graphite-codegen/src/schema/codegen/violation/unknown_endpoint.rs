//! 未知の端点キーを参照したことを表す違反 variant とその表示を生成する。
//!
//! 表示の組み立ては `unknown_endpoint_message` が持つ。このファイルは3ケース
//! (有向始点・有向終点・無向端点) の variant 定義だけを担う。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::violation::unknown_endpoint_message::未知端点の位置;

// 有向辺が未知の始点キーを参照した場合 (`{Kind}UnknownSource`)。
pub(crate) fn gen_unknown_source_case(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> (TokenStream, TokenStream) {
    let edge_id = &edge.id_ty;
    let from_id = &edge.from_node.id_ty;
    let unk_src = edge.unknown_source_variant();
    let variant = quote! {
        /// このエッジが未知の始点キーを参照している。
        #unk_src {
            /// 未知のキーを参照した辺の公開ID。
            edge: #edge_id,
            /// この辺が始点として参照した、対応するノードが存在しないキー。
            source: #from_id,
        }
    };
    let display_arm = 未知端点の位置::有向辺の始点.表示の腕(violation_ident, edge);
    (variant, display_arm)
}

// 有向辺が未知の終点キーを参照した場合 (`{Kind}UnknownTarget`)。
pub(crate) fn gen_unknown_target_case(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> (TokenStream, TokenStream) {
    let edge_id = &edge.id_ty;
    let to_id = &edge.to_node.id_ty;
    let unk_dst = edge.unknown_target_variant();
    let variant = quote! {
        /// このエッジが未知の終点キーを参照している。
        #unk_dst {
            /// 未知のキーを参照した辺の公開ID。
            edge: #edge_id,
            /// この辺が終点として参照した、対応するノードが存在しないキー。
            target: #to_id,
        }
    };
    let display_arm = 未知端点の位置::有向辺の終点.表示の腕(violation_ident, edge);
    (variant, display_arm)
}

// 無向辺が未知の端点キーを参照した場合 (`{Kind}UnknownEndpoint`)。
// 無向辺は位置の区別が無いため1種類で足りる。両端は同じノード型なので
// `from_node` で代表する。
pub(crate) fn gen_unknown_endpoint_case(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> (TokenStream, TokenStream) {
    let edge_id = &edge.id_ty;
    let node_id = &edge.from_node.id_ty;
    let unk = edge.unknown_endpoint_variant();
    let variant = quote! {
        /// このエッジが未知の端点キーを参照している (無向のため位置の
        /// 区別は無い)。
        #unk {
            /// 未知のキーを参照した辺の公開ID。
            edge: #edge_id,
            /// この辺が端点として参照した、対応するノードが存在しないキー。
            endpoint: #node_id,
        }
    };
    let display_arm = 未知端点の位置::無向辺の端点.表示の腕(violation_ident, edge);
    (variant, display_arm)
}
