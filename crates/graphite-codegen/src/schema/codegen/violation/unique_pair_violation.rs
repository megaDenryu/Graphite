//! 端点対の重複を表す違反 variant とその表示を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;

/// `unique pair` 違反 (`{Kind}UniquePairViolation`)。有向辺は `source`/`target`、
/// 無向辺は順序の意味が無いため `a`/`b` を持つ。
pub(crate) fn gen_unique_pair_violation_case(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> (TokenStream, TokenStream) {
    let kind_str = edge.kind.to_string();
    let v = edge.unique_pair_violation_variant();
    if edge.is_directed() {
        let from_id = &edge.from_node.id_ty;
        let to_id = &edge.to_node.id_ty;
        let variant = quote! {
            /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
            /// 2本目の辺が張られた)。
            #v { source: #from_id, target: #to_id }
        };
        let display_arm = if edge.from_node.id_ty.is_debug_printable()
            && edge.to_node.id_ty.is_debug_printable()
        {
            quote! {
                #violation_ident::#v { source, target } => write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    #kind_str, source, target
                )
            }
        } else {
            quote! {
                #violation_ident::#v { .. } => write!(
                    f,
                    "unique pair違反: 辺 `{}` の同じ始点・終点の対に既に辺が存在します",
                    #kind_str
                )
            }
        };
        (variant, display_arm)
    } else {
        let node_id = &edge.from_node.id_ty;
        let variant = quote! {
            /// このエッジ種別の `unique pair` 違反 (無向のため
            /// 順序を無視した対で判定)。
            #v { a: #node_id, b: #node_id }
        };
        let display_arm = if edge.from_node.id_ty.is_debug_printable() {
            quote! {
                #violation_ident::#v { a, b } => write!(
                    f,
                    "unique pair違反: 辺 `{}` は {{{:?}, {:?}}} の対に既に辺が存在します",
                    #kind_str, a, b
                )
            }
        } else {
            quote! {
                #violation_ident::#v { .. } => write!(
                    f,
                    "unique pair違反: 辺 `{}` の同じ端点対に既に辺が存在します",
                    #kind_str
                )
            }
        };
        (variant, display_arm)
    }
}
