//! 完成済みグラフ上の個体を指す参照型を、向きごとの生成へ振り分ける。

pub(crate) mod core_methods;
pub(crate) mod debug_implementation;
pub(crate) mod directed_edge_reference;
pub(crate) mod node_reference;
pub(crate) mod payload_methods;
pub(crate) mod undirected_edge_reference;

use proc_macro2::TokenStream;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::辺の向き;
use directed_edge_reference::gen_directed_edge_reference_type;
use undirected_edge_reference::gen_undirected_edge_reference_type;

/// 完成済みグラフ上の辺個体を表す薄い参照値を生成する。端点を返すメソッドは、
/// 保存レコード内の内部位置から NodeRef を直接作り、公開IDの索引を検索しない。
pub(crate) fn gen_edge_reference_types(
    graph_ident: &Ident,
    edges: &[EdgeInfo<'_>],
) -> Vec<TokenStream> {
    edges
        .iter()
        .map(|edge| match edge.shape() {
            辺の向き::有向 { 始点, 終点 } => {
                gen_directed_edge_reference_type(graph_ident, edge, 始点.役割名(), 終点.役割名())
            }
            辺の向き::無向 { .. } => gen_undirected_edge_reference_type(graph_ident, edge),
        })
        .collect()
}
