//! 凍結済みグラフ内の位置を表す非公開 newtype を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;

// 公開IDとは別に、凍結済みグラフ内の内部位置を表す非公開型を生成する。
// 種別ごとのnewtypeにすることで、別のノード表・辺表の位置を取り違えない。
// フィールドは `KeyedTable`/役割索引が共有する `graphite::TablePosition`
// (`graph_schema!` の生成コードが `graphite` クレートの外から扱う内部位置)
// であり、種別ごとのnewtypeはその上にもう1層、種別間の取り違えを防ぐ層を
// 重ねる。
pub(crate) fn gen_internal_position_types(
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
) -> Vec<TokenStream> {
    nodes
        .iter()
        .map(NodeInfo::internal_position_ident)
        .chain(edges.iter().map(EdgeInfo::internal_position_ident))
        .map(|position| {
            quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                struct #position(graphite::TablePosition);
            }
        })
        .collect()
}
