//! 値を持たない構造グラフの構築 — 図式グラフから汎用アルゴリズムへ射影する入口を所有する。

use std::hash::Hash;

use super::build_error::GraphError;
use super::Graph;

impl<K> Graph<(), (), K>
where
    K: Hash + Eq + Clone,
{
    /// ノードキー集合と辺 `(from, to)` の列から、値なしの構造グラフを作る。
    ///
    /// 図式グラフの `{label}_pairs()` から汎用アルゴリズム (`has_cycle` 等)
    /// へ射影する定型操作のためのヘルパー。キーは内部で `clone` して所有
    /// するので、呼び出し側で借用の生存期間を気にしなくてよい。
    /// 重複ノードキー・未知キーへの辺は [`Graph::build`] と同じ `GraphError`
    /// 規約でエラーを返す。
    ///
    /// # Examples
    ///
    /// ```
    /// use graphite::Graph;
    ///
    /// let g: Graph<(), (), &str> =
    ///     Graph::from_edges(vec!["a", "b", "c"], vec![("a", "b"), ("b", "c")]).unwrap();
    /// assert!(!g.has_cycle());
    /// ```
    ///
    /// 図式グラフの `{label}_pairs()` (`&K` を yield するイテレータ) から
    /// 射影したい場合は `.cloned()` を挟んで所有権を渡す:
    ///
    /// ```
    /// use graphite::Graph;
    ///
    /// let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    /// // 例えば `schema.produces_pairs()` のような `(&K, &K)` を yield する
    /// // イテレータを想定した図。
    /// let pairs: Vec<(&String, &String)> = vec![(&ids[0], &ids[1]), (&ids[1], &ids[2])];
    ///
    /// let g: Graph<(), (), String> = Graph::from_edges(
    ///     ids.iter().cloned(),
    ///     pairs.into_iter().map(|(a, b)| (a.clone(), b.clone())),
    /// )
    /// .unwrap();
    /// assert!(!g.has_cycle());
    /// ```
    pub fn from_edges(
        nodes: impl IntoIterator<Item = K>,
        edges: impl IntoIterator<Item = (K, K)>,
    ) -> Result<Self, GraphError<K>> {
        Self::build(
            nodes.into_iter().map(|k| (k, ())),
            edges.into_iter().map(|(from, to)| (from, to, ())),
        )
    }
}
