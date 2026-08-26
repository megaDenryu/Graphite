//! [`Graph::create`] がクロージャへ貸し出す構築用 builder を所有する。

use std::hash::Hash;

use super::assembly::構築中のグラフ;
use super::build_error::GraphError;
use super::Graph;

/// [`Graph::create`] に貸し出される構築用 builder。
///
/// クロージャの外に参照を持ち出すことはできない (借用検査器が保証)。
/// 凍結 ([`Graph::create`] 内部で呼ばれる) までは多重度等の検査を一切
/// 行わない — 「構築中の型」と「構築後の型」を分ける、というのが
/// `../Bullet/docs/graph_design_sketches.md` 決定2/決定4 の要点。
pub struct GraphBuilder<N, E, K> {
    nodes: Vec<(K, N)>,
    edges: Vec<(K, K, E)>,
}

impl<N, E, K> GraphBuilder<N, E, K> {
    pub(in crate::graph) fn 空のbuilderから始める() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// ノードを 1 つ積む。
    pub fn node(&mut self, key: K, value: N) -> &mut Self {
        self.nodes.push((key, value));
        self
    }

    /// 辺を 1 つ積む。
    pub fn edge(&mut self, from: K, to: K, value: E) -> &mut Self {
        self.edges.push((from, to, value));
        self
    }
}

impl<N, E, K> GraphBuilder<N, E, K>
where
    K: Hash + Eq + Clone,
{
    /// 積んだノードと辺を検査しながら組み立て、不変のグラフへ凍結する。
    pub(in crate::graph) fn 凍結する(self) -> Result<Graph<N, E, K>, GraphError<K>> {
        構築中のグラフ::ノード列と辺列から組み立てる(self.nodes, self.edges)
    }
}
