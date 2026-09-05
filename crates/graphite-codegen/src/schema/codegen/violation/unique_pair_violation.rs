//! 端点対の重複を表す違反 variant とその表示を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;

// `unique pair` 違反 (`{Kind}UniquePairViolation`)。有向辺は `source`/`target`、
// 無向辺は順序の意味が無いため `a`/`b` を持つ。
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
            #v {
                /// 2本目の辺が張られた対の始点ノードの公開ID。
                source: #from_id,
                /// 2本目の辺が張られた対の終点ノードの公開ID。
                target: #to_id,
            }
        };
        // 生成コードは始点と終点の表示可否を別々に判定する。生成コードが利用者の宣言した
        // ID型へ `Debug` を要求しない契約のため、片方だけが生成ID型である構成が起こる。
        // 生成コードは、綴りを省いた側に「表示できない」と添える。読み手が「表示できないのか、
        // そもそも無いのか」を区別できないためである。
        let display_arm = match (
            edge.from_node.id_ty.is_debug_printable(),
            edge.to_node.id_ty.is_debug_printable(),
        ) {
            (true, true) => quote! {
                #violation_ident::#v { source, target } => write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    #kind_str, source, target
                )
            },
            (true, false) => quote! {
                #violation_ident::#v { source, .. } => write!(
                    f,
                    "unique pair違反: 辺 `{}` は始点 {:?} を含む対に既に辺が存在します (終点のキーは表示できない)",
                    #kind_str, source
                )
            },
            (false, true) => quote! {
                #violation_ident::#v { target, .. } => write!(
                    f,
                    "unique pair違反: 辺 `{}` は終点 {:?} を含む対に既に辺が存在します (始点のキーは表示できない)",
                    #kind_str, target
                )
            },
            (false, false) => quote! {
                #violation_ident::#v { .. } => write!(
                    f,
                    "unique pair違反: 辺 `{}` の同じ始点・終点の対に既に辺が存在します (始点と終点のキーは表示できない)",
                    #kind_str
                )
            },
        };
        (variant, display_arm)
    } else {
        let node_id = &edge.from_node.id_ty;
        let variant = quote! {
            /// このエッジ種別の `unique pair` 違反 (無向のため
            /// 順序を無視した対で判定)。
            #v {
                /// 2本目の辺が張られた対の一方の端点の公開ID。
                a: #node_id,
                /// 2本目の辺が張られた対のもう一方の端点の公開ID。
                b: #node_id,
            }
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
                    "unique pair違反: 辺 `{}` の同じ端点対に既に辺が存在します (端点のキーは表示できない)",
                    #kind_str
                )
            }
        };
        (variant, display_arm)
    }
}
