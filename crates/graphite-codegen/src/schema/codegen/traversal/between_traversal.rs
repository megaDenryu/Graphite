//! 2つのノード参照の間に張られた辺を探す端点対検索メソッドを生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::naming::{
    construction_stamp_field_ident, kind_api_method_ident, pair_index_field_ident,
};
use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::端点対のキーの形;

/// 端点対検索 (`{kind}_between` / `{kind}_try_between`) の生成で、有向辺と
/// 無向辺で異なる部分だけを束ねる。有向辺は端点対索引のキーが
/// `(位置, 位置)` のタプル、無向辺は `UnorderedPair::new(位置, 位置)` になる
/// (`gen_directed_edge_freeze_block`/`gen_undirected_edge_freeze_block` が
/// 積む索引のキー型に合わせる)。
pub(crate) struct EdgeQueryPairSpec {
    /// 相手側端点の参照型 (有向辺は終点側、無向辺は位置0側と同じ型)。
    other_reference: Ident,
    /// 端点対索引 (`{accessor}_by_pair`) を検索するキー式。
    pair_key: TokenStream,
    /// `try_between` の doc コメントに書く対の種類 (`順序付き`/`順序なし`)。
    pair_order_description: &'static str,
}

impl EdgeQueryPairSpec {
    fn from_edge(edge: &EdgeInfo<'_>) -> Self {
        // 端点対索引のキーの形は辺の向きから機械的に決まるが、判断の
        // 出どころを意味モデル (`schema::semantic::edge_definition`) の
        // 1系統へ寄せるため、ここでは向きを直接 match せず
        // `端点対のキーの形()` を経由する。
        match edge.定義.端点対のキーの形() {
            端点対のキーの形::順序付きの対 => EdgeQueryPairSpec {
                other_reference: edge.to_node.reference_ident(),
                pair_key: quote! { (self.internal_position, other.internal_position) },
                pair_order_description: "順序付き",
            },
            端点対のキーの形::順序なしの対 => EdgeQueryPairSpec {
                other_reference: edge.from_node.reference_ident(),
                pair_key: quote! {
                    graphite::UnorderedPair::new(self.internal_position, other.internal_position)
                },
                pair_order_description: "順序なし",
            },
        }
    }
}

/// 位置0側 `NodeRef` へ端点対検索 `{kind}_try_between` / `{kind}_between` を
/// 生成する。
///
/// `try_between` は2つの参照が同じ `Graph` から得られたかを構築印で照合し、
/// 異なれば [`graphite::GraphMismatch`] を返す。照合は受け手と相手の2者だけを
/// 突き合わせる (一方が有効なら他方も同じ `Graph` に属することが決まるため、
/// 3者目の照合は要らない)。
pub(crate) fn gen_between_traversal_methods(edge: &EdgeInfo<'_>) -> TokenStream {
    let EdgeQueryPairSpec {
        other_reference,
        pair_key,
        pair_order_description,
    } = EdgeQueryPairSpec::from_edge(edge);
    let accessor = &edge.accessor_ident;
    let try_between = kind_api_method_ident(accessor, "try_between");
    let between = kind_api_method_ident(accessor, "between");
    let node_reference = edge.from_node.reference_ident();
    let edge_reference = edge.reference_ident();
    let stamp = construction_stamp_field_ident(edge.kind.span());
    let pair_index = pair_index_field_ident(edge.kind);
    let between_result = if edge.unique_pair() {
        quote! { Option<#edge_reference<'graph>> }
    } else {
        quote! { impl Iterator<Item = #edge_reference<'graph>> + 'graph }
    };
    let between_body = if edge.unique_pair() {
        quote! {
            let found = self.graph.#pair_index.get(&#pair_key).copied();
            Ok(found.map(|internal_position| #edge_reference { graph: self.graph, internal_position }))
        }
    } else {
        quote! {
            let positions = self.graph.#pair_index.get(&#pair_key)
                .map(Vec::as_slice).unwrap_or(&[]);
            Ok(positions.iter().copied().map(move |internal_position| #edge_reference {
                graph: self.graph,
                internal_position,
            }))
        }
    };
    let try_between_doc =
        format!("{pair_order_description}端点対を平均 O(1)、追加確保なしで検索する。");
    let between_avoid_panic_doc =
        format!("パニックを避けたい場合は対の [`Self::{try_between}`] を使う。");
    quote! {
        #[doc = #try_between_doc]
        pub fn #try_between(self, other: #other_reference<'graph>)
            -> Result<#between_result, graphite::GraphMismatch>
        {
            if self.graph.#stamp != other.graph.#stamp { return Err(graphite::GraphMismatch); }
            #between_body
        }

        /// # Panics
        /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
        #[doc = #between_avoid_panic_doc]
        pub fn #between(self, other: #other_reference<'graph>) -> #between_result {
            self.#try_between(other).unwrap_or_else(|error| {
                panic!("{}::{}: {error}", stringify!(#node_reference), stringify!(#between))
            })
        }
    }
}
