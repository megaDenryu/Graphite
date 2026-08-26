//! 有向辺の役割探索メソッドを、その役割の多重度に合わせた戻り型で生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::naming::traversal_method_ident;
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::{EachSide, RoleCardinality};

/// 有向辺の役割探索メソッド `{kind}_as_{役割}()` を1つ生成する。
///
/// 凍結時に構築済みの役割索引を内部位置で引くだけなので O(1)、追加確保なし。
/// 戻り型は問い合わせた役割そのものの `each` 制約で決まる
/// (`docs/schema_v4.md` §3.2)。
pub(crate) fn gen_role_traversal_method(
    edge: &EdgeInfo<'_>,
    role: &Ident,
    side: EachSide,
    cardinality: RoleCardinality,
) -> TokenStream {
    let method = traversal_method_ident(edge.kind, role);
    let edge_reference = edge.reference_ident();
    let index = match side {
        EachSide::Source => &edge.index_field_ident,
        EachSide::Target => &edge.to_index_field_ident,
    };
    match cardinality {
        RoleCardinality::Exact => quote! {
            /// この役割に接続する唯一の辺を O(1)、追加確保なしで返す。
            pub fn #method(self) -> #edge_reference<'graph> {
                #edge_reference {
                    graph: self.graph,
                    internal_position: *self.graph.#index.get(self.internal_position.0),
                }
            }
        },
        RoleCardinality::Optional => quote! {
            /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
            pub fn #method(self) -> Option<#edge_reference<'graph>> {
                self.graph.#index.get(self.internal_position.0).copied()
                    .map(|internal_position| #edge_reference { graph: self.graph, internal_position })
            }
        },
        RoleCardinality::Multiple => quote! {
            /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
            /// 問い合わせ時に結果 `Vec` を確保しない。
            pub fn #method(self) -> impl Iterator<Item = #edge_reference<'graph>> + 'graph {
                let positions = self.graph.#index.get(self.internal_position.0);
                positions.iter().copied().map(move |internal_position| #edge_reference {
                    graph: self.graph,
                    internal_position,
                })
            }
        },
    }
}
