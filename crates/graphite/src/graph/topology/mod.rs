//! 有向トポロジー — `petgraph` を包み、ノード位置だけで完結する基本操作を所有する。
//!
//! この配下だけが `petgraph` を名指しする。キーの世界 (`crate::graph` 直下) は
//! [`ノード位置`] を通してのみトポロジーへ触れる。

pub(in crate::graph) mod position;

use petgraph::graph::DiGraph;
use petgraph::visit::EdgeRef;
use petgraph::Direction;

pub(in crate::graph) use position::ノード位置;

/// ノード値 `N`・辺値 `E` を持つ有向グラフの形そのもの。ユーザーキーは持たない。
#[derive(Debug)]
pub(in crate::graph) struct 有向トポロジー<N, E> {
    内部グラフ: DiGraph<N, E>,
}

impl<N, E> 有向トポロジー<N, E> {
    pub(in crate::graph) fn 空のトポロジーを生成する() -> Self {
        Self {
            内部グラフ: DiGraph::new(),
        }
    }

    pub(in crate::graph) fn ノードを追加する(&mut self, 値: N) -> ノード位置 {
        ノード位置::内部添字から生成する(self.内部グラフ.add_node(値))
    }

    pub(in crate::graph) fn 辺を追加する(
        &mut self,
        始点: ノード位置,
        終点: ノード位置,
        値: E,
    ) {
        self.内部グラフ
            .add_edge(始点.内部添字(), 終点.内部添字(), 値);
    }

    pub(in crate::graph) fn ノード数(&self) -> usize {
        self.内部グラフ.node_count()
    }

    pub(in crate::graph) fn 辺数(&self) -> usize {
        self.内部グラフ.edge_count()
    }

    pub(in crate::graph) fn ノード値(&self, 位置: ノード位置) -> &N {
        &self.内部グラフ[位置.内部添字()]
    }

    /// ノードを挿入順に並べた位置列。`add_node` の呼び出し順に単調増加する
    /// 内部添字をそのまま辿るため、この列がそのまま挿入順になる。
    pub(in crate::graph) fn 挿入順の位置列(&self) -> Vec<ノード位置> {
        self.内部グラフ
            .node_indices()
            .map(ノード位置::内部添字から生成する)
            .collect()
    }

    pub(in crate::graph) fn 出ていく先(
        &self,
        位置: ノード位置,
    ) -> impl Iterator<Item = ノード位置> + '_ {
        self.内部グラフ
            .neighbors_directed(位置.内部添字(), Direction::Outgoing)
            .map(ノード位置::内部添字から生成する)
    }

    pub(in crate::graph) fn 入ってくる元(
        &self,
        位置: ノード位置,
    ) -> impl Iterator<Item = ノード位置> + '_ {
        self.内部グラフ
            .neighbors_directed(位置.内部添字(), Direction::Incoming)
            .map(ノード位置::内部添字から生成する)
    }

    pub(in crate::graph) fn 辺値(
        &self, 始点: ノード位置, 終点: ノード位置
    ) -> Option<&E> {
        let 辺添字 = self
            .内部グラフ
            .find_edge(始点.内部添字(), 終点.内部添字())?;
        self.内部グラフ.edge_weight(辺添字)
    }

    pub(in crate::graph) fn 辺があるか(
        &self, 始点: ノード位置, 終点: ノード位置
    ) -> bool {
        self.内部グラフ
            .find_edge(始点.内部添字(), 終点.内部添字())
            .is_some()
    }

    pub(in crate::graph) fn 辺の一覧(
        &self,
    ) -> impl Iterator<Item = (ノード位置, ノード位置, &E)> + '_ {
        self.内部グラフ.edge_references().map(|辺| {
            (
                ノード位置::内部添字から生成する(辺.source()),
                ノード位置::内部添字から生成する(辺.target()),
                辺.weight(),
            )
        })
    }

    pub(in crate::graph) fn 循環があるか(&self) -> bool {
        petgraph::algo::is_cyclic_directed(&self.内部グラフ)
    }

    pub(in crate::graph) fn 強連結成分の一覧(&self) -> Vec<Vec<ノード位置>> {
        petgraph::algo::tarjan_scc(&self.内部グラフ)
            .into_iter()
            .map(|成分| {
                成分
                    .into_iter()
                    .map(ノード位置::内部添字から生成する)
                    .collect()
            })
            .collect()
    }

    /// 全体のトポロジカル順序。循環があるときは `None`。
    pub(in crate::graph) fn トポロジカル順序(&self) -> Option<Vec<ノード位置>> {
        petgraph::algo::toposort(&self.内部グラフ, None)
            .ok()
            .map(|順序| {
                順序
                    .into_iter()
                    .map(ノード位置::内部添字から生成する)
                    .collect()
            })
    }

    /// 始点から深さ優先で到達できる位置列 (始点自身を含む)。
    pub(in crate::graph) fn 深さ優先で到達できる位置列(
        &self,
        始点: ノード位置,
    ) -> Vec<ノード位置> {
        let mut 探索 = petgraph::visit::Dfs::new(&self.内部グラフ, 始点.内部添字());
        let mut 結果 = Vec::new();
        while let Some(添字) = 探索.next(&self.内部グラフ) {
            結果.push(ノード位置::内部添字から生成する(添字));
        }
        結果
    }
}
