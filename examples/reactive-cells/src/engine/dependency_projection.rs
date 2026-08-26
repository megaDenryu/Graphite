//! `Feeds`/`Lhs`/`Rhs` の3種の依存エッジを、1つの汎用グラフへ射影する。
//!
//! 3種はいずれも「依存元→依存先」という同じ向きの意味を持つ
//! (`src/schema.rs` 参照) ので、単純に合流させてよい。射影先の汎用グラフが
//! `reachable_from`/`topological_sort` を既に持つため、エンジン側は水準1の
//! アルゴリズムを再実装しない。

use graphite::Graph;

use crate::schema::{CellId, Sheet};

pub(super) fn project_dependency_graph(graph: &Sheet::Graph) -> Graph<(), (), CellId> {
    Graph::from_edges(
        graph.cell_ids().cloned(),
        graph.feeds_iter()
            .map(|edge| {
                (
                    edge.dependency().id().clone(),
                    edge.dependent().id().clone(),
                )
            })
            .chain(
                graph.lhs_iter()
                    .map(|edge| (edge.operand().id().clone(), edge.operation().id().clone())),
            )
            .chain(
                graph.rhs_iter()
                    .map(|edge| (edge.operand().id().clone(), edge.operation().id().clone())),
            ),
    )
    .expect(
        "cell_ids()とfeeds_iter/lhs_iter/rhs_iter()の端点整合はSheet::create/create_collectingの検証で\
         既に保証されているはず (未知キー・重複キーはここでは起こらない)",
    )
}
