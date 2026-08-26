//! `graph!` の名前付き要素が凍結をまたいで運ぶ位置型を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;

/// `graph!` の名前付きラッパーへ凍結をまたいで内部位置を運ぶ型を生成する。
/// フィールドは非公開で、生成された挿入経路と `NamedGraphElement` 実装だけが
/// 構築・参照する。公開IDや `Graph` への参照は保持しない。
///
/// 第2要素は構築印 (`u64`)。挿入時にその場の `Builder` が持つ構築印を
/// そのまま埋め込み、`NamedGraphElement::bind` が `Graph` 側の構築印と
/// 照合する (`crates/graphite/src/schema_runtime/construction_stamp.rs` の
/// 構築印発行関数を参照)。
pub(crate) fn gen_named_position_types(
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
) -> Vec<TokenStream> {
    nodes
        .iter()
        .map(|node| {
            let named_position = node.named_position_ident();
            let internal_position = node.internal_position_ident();
            quote! {
                #[doc(hidden)]
                #[derive(Clone, Copy)]
                pub struct #named_position(#internal_position, u64);
            }
        })
        .chain(edges.iter().map(|edge| {
            let named_position = edge.named_position_ident();
            let internal_position = edge.internal_position_ident();
            quote! {
                #[doc(hidden)]
                #[derive(Clone, Copy)]
                pub struct #named_position(#internal_position, u64);
            }
        }))
        .collect()
}
