//! 水準1相当: ジェネリックグラフ `Graph<N, E, K>`。
//!
//! `docs/graph_design_sketches.md` の決定1 (ノードの同一性はユーザーキー)・
//! 決定2 (可変性はクロージャスコープ builder → 凍結し、以後不変) をそのまま
//! Rust に輸入したもの。マクロは一切使わない、ふつうのジェネリック構造体。
//!
//! ## 内部表現
//!
//! `petgraph::graph::DiGraph<N, E>` (キーは内部の `NodeIndex`) を土台にし、
//! ユーザーキー `K` との相互変換のために `HashMap<K, NodeIndex>` (正引き) と
//! `HashMap<NodeIndex, K>` (逆引き) を持つ。`petgraph::graphmap::GraphMap` は
//! ノードキーに `Copy` を要求するため `String` のような非 `Copy` キーを直接
//! 扱えず不採用 (`.claude/skills/proc-macro-dev/SKILL.md` の注意通り)。
//!
//! ## 不変性
//!
//! `Graph` は構築後不変。可変な操作 (ノード追加・削除・辺追加) を一切公開
//! しない。構築は [`Graph::build`] (原子的な一括構築) と [`Graph::create`]
//! (builder をクロージャに貸し出し、戻ったら凍結) の 2 経路のみ。
//! `create` に渡すクロージャの型は `for<'b> FnOnce(&'b mut GraphBuilder<..>)`
//! であり、builder への参照をクロージャの外に持ち出すことを借用検査器が
//! 静的に拒否する (`std::thread::scope` と同じ仕組み)。

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error as StdError;
use std::fmt;
use std::hash::Hash;

/// ノード種別 `N`、エッジ種別 `E` (既定は属性なしを表す `()`)、
/// ノードキー種別 `K` (既定は `String`) を持つ有向グラフ。
///
/// 構築後は不変 — 可変 API は公開しない。`build`/`create` でのみ作れる。
#[derive(Debug)]
pub struct Graph<N, E = (), K = String> {
    inner: DiGraph<N, E>,
    index: HashMap<K, NodeIndex>,
    keys: HashMap<NodeIndex, K>,
}

/// [`Graph::build`] / [`Graph::create`] が返しうる構築エラー。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError<K> {
    /// ノード定義でキーが重複した。
    DuplicateKey(K),
    /// 辺が未知のキーを端点として参照している。
    /// `missing` は `from`/`to` のうちどちらが未定義だったかを示す。
    UnknownEndpoint { from: K, to: K, missing: K },
}

impl<K: fmt::Debug> fmt::Display for GraphError<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphError::DuplicateKey(k) => write!(f, "ノードキーが重複しています: {k:?}"),
            GraphError::UnknownEndpoint { from, to, missing } => write!(
                f,
                "辺 {from:?} -> {to:?} が未知のキー {missing:?} を参照しています"
            ),
        }
    }
}

impl<K: fmt::Debug> StdError for GraphError<K> {}

/// [`Graph::topological_sort`] / [`Graph::topological_levels`] /
/// [`Graph::critical_path_by`] が循環検出時に返すエラー。
///
/// `cycle` は循環を構成するノードキーの列。`cycle[0]` から `cycle[1]`、
/// ...、`cycle[last]` から `cycle[0]` へと辺を辿って戻ってこられる
/// (閉路になっている) ことを保証する。自己ループの場合は `cycle` は
/// 要素数 1 (`cycle[0]` 自身への辺)。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError<K> {
    pub cycle: Vec<K>,
}

impl<K: fmt::Debug> fmt::Display for CycleError<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "グラフに循環があります: ")?;
        for (i, k) in self.cycle.iter().enumerate() {
            if i > 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{k:?}")?;
        }
        if let Some(first) = self.cycle.first() {
            write!(f, " -> {first:?}")?;
        }
        Ok(())
    }
}

impl<K: fmt::Debug> StdError for CycleError<K> {}

/// [`Graph::create`] に貸し出される構築用 builder。
///
/// クロージャの外に参照を持ち出すことはできない (借用検査器が保証)。
/// 凍結 ([`Graph::create`] 内部で呼ばれる) までは多重度等の検査を一切
/// 行わない — 「構築中の型」と「構築後の型」を分ける、というのが
/// `docs/graph_design_sketches.md` 決定2/決定4 の要点。
pub struct GraphBuilder<N, E, K> {
    nodes: Vec<(K, N)>,
    edges: Vec<(K, K, E)>,
}

impl<N, E, K> GraphBuilder<N, E, K> {
    fn new() -> Self {
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
    fn freeze(self) -> Result<Graph<N, E, K>, GraphError<K>> {
        Graph::build(self.nodes, self.edges)
    }
}

impl<N, E, K> Graph<N, E, K>
where
    K: Hash + Eq + Clone,
{
    /// `(キー, ノード値)` の列と `(始点キー, 終点キー, 辺値)` の列から
    /// 一括構築する。キー重複・未知キーへの辺は `Err` で報告する。
    pub fn build(
        nodes: impl IntoIterator<Item = (K, N)>,
        edges: impl IntoIterator<Item = (K, K, E)>,
    ) -> Result<Self, GraphError<K>> {
        let mut inner = DiGraph::new();
        let mut index: HashMap<K, NodeIndex> = HashMap::new();
        let mut keys: HashMap<NodeIndex, K> = HashMap::new();

        for (key, value) in nodes {
            if index.contains_key(&key) {
                return Err(GraphError::DuplicateKey(key));
            }
            let idx = inner.add_node(value);
            keys.insert(idx, key.clone());
            index.insert(key, idx);
        }

        for (from, to, value) in edges {
            let from_idx = *index.get(&from).ok_or_else(|| GraphError::UnknownEndpoint {
                from: from.clone(),
                to: to.clone(),
                missing: from.clone(),
            })?;
            let to_idx = *index.get(&to).ok_or_else(|| GraphError::UnknownEndpoint {
                from: from.clone(),
                to: to.clone(),
                missing: to.clone(),
            })?;
            inner.add_edge(from_idx, to_idx, value);
        }

        Ok(Self { inner, index, keys })
    }

    /// builder をクロージャに貸し出し、戻ったら凍結して一括検証する。
    ///
    /// `F: for<'b> FnOnce(&'b mut GraphBuilder<N, E, K>)` という高階トレイト
    /// 境界により、builder への参照をクロージャの外の変数に取っておくことは
    /// コンパイルエラーになる (`std::thread::scope` と同じ仕組み)。
    pub fn create<F>(f: F) -> Result<Self, GraphError<K>>
    where
        F: for<'b> FnOnce(&'b mut GraphBuilder<N, E, K>),
    {
        let mut builder = GraphBuilder::new();
        f(&mut builder);
        builder.freeze()
    }

    /// キーからノード値を引く。
    pub fn node(&self, key: &K) -> Option<&N> {
        self.index.get(key).map(|&idx| &self.inner[idx])
    }

    /// 全ノードキーを走査するイテレータ (順序は未規定)。
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.index.keys()
    }

    /// 全ノードを `(キー, 値)` で走査するイテレータ (順序は未規定)。
    pub fn nodes(&self) -> impl Iterator<Item = (&K, &N)> {
        self.index.iter().map(move |(k, &idx)| (k, &self.inner[idx]))
    }

    /// ノード数。
    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    /// 辺数。
    pub fn edge_count(&self) -> usize {
        self.inner.edge_count()
    }

    /// `key` から出て行く辺の終点キー一覧。`key` が存在しなければ空。
    pub fn out_neighbors(&self, key: &K) -> Vec<&K> {
        match self.index.get(key) {
            Some(&idx) => self
                .inner
                .neighbors_directed(idx, Direction::Outgoing)
                .map(|n| &self.keys[&n])
                .collect(),
            None => Vec::new(),
        }
    }

    /// `key` へ入ってくる辺の始点キー一覧 (`out_neighbors` と対称)。
    /// `key` が存在しなければ空。
    pub fn in_neighbors(&self, key: &K) -> Vec<&K> {
        match self.index.get(key) {
            Some(&idx) => self
                .inner
                .neighbors_directed(idx, Direction::Incoming)
                .map(|n| &self.keys[&n])
                .collect(),
            None => Vec::new(),
        }
    }

    /// `from -> to` の辺属性を引く。辺が存在しない・端点キーが未知なら `None`。
    pub fn edge_weight(&self, from: &K, to: &K) -> Option<&E> {
        let from_idx = *self.index.get(from)?;
        let to_idx = *self.index.get(to)?;
        let edge_idx = self.inner.find_edge(from_idx, to_idx)?;
        self.inner.edge_weight(edge_idx)
    }

    /// グラフに循環があるか。
    pub fn has_cycle(&self) -> bool {
        petgraph::algo::is_cyclic_directed(&self.inner)
    }

    /// トポロジカルソート。循環がある場合は `CycleError` を返す。
    pub fn topological_sort(&self) -> Result<Vec<&K>, CycleError<K>> {
        match petgraph::algo::toposort(&self.inner, None) {
            Ok(order) => Ok(order.into_iter().map(|idx| &self.keys[&idx]).collect()),
            Err(_) => Err(self.cycle_error(
                self.find_cycle_indices()
                    .expect("toposortが失敗したので循環が存在するはず"),
            )),
        }
    }

    /// 依存のないノードから順にレベル (波) 分割したトポロジカルソート。
    /// 各レベルは「まだ処理されていない先行ノードを持たないノード」の
    /// 集合であり、レベル内の順序はノードの挿入順 (`build`/`create` に
    /// 渡した順) で決定的。循環がある場合は `CycleError` を返す。
    pub fn topological_levels(&self) -> Result<Vec<Vec<&K>>, CycleError<K>> {
        if let Some(cycle_indices) = self.find_cycle_indices() {
            return Err(self.cycle_error(cycle_indices));
        }

        // ノード挿入順 (NodeIndex は `add_node` 呼び出し順に単調増加するため
        // `node_indices()` の列がそのまま挿入順になる)。
        let insertion_order: Vec<NodeIndex> = self.inner.node_indices().collect();

        let mut in_degree: HashMap<NodeIndex, usize> = insertion_order
            .iter()
            .map(|&idx| (idx, self.inner.neighbors_directed(idx, Direction::Incoming).count()))
            .collect();
        let mut remaining: HashSet<NodeIndex> = insertion_order.iter().copied().collect();

        let mut levels: Vec<Vec<&K>> = Vec::new();

        while !remaining.is_empty() {
            let frontier: Vec<NodeIndex> = insertion_order
                .iter()
                .copied()
                .filter(|idx| remaining.contains(idx) && in_degree[idx] == 0)
                .collect();

            debug_assert!(
                !frontier.is_empty(),
                "循環なしを確認済みなのでフロンティアが空になることはない"
            );

            for &idx in &frontier {
                remaining.remove(&idx);
            }
            for &idx in &frontier {
                for succ in self.inner.neighbors_directed(idx, Direction::Outgoing) {
                    if let Some(d) = in_degree.get_mut(&succ) {
                        *d = d.saturating_sub(1);
                    }
                }
            }

            levels.push(frontier.iter().map(|idx| &self.keys[idx]).collect());
        }

        Ok(levels)
    }

    /// ノード重み付き最長経路 (クリティカルパス)。
    ///
    /// トポロジカル順序に沿って `dist[v] = max(dist[v], dist[u] + weight(v))`
    /// (`u -> v` の辺ごと) と緩和していく DAG 上の最長経路 DP。
    /// 空グラフは `(vec![], W::default())` を返す。循環がある場合は
    /// `CycleError` を返す。
    pub fn critical_path_by<W>(
        &self,
        node_weight: impl Fn(&K, &N) -> W,
    ) -> Result<(Vec<&K>, W), CycleError<K>>
    where
        W: Ord + Copy + Default + std::ops::Add<Output = W>,
    {
        let order = self.topological_sort()?;
        if order.is_empty() {
            return Ok((Vec::new(), W::default()));
        }

        let weight_of: HashMap<&K, W> = order
            .iter()
            .map(|&key| {
                let value = self.node(key).expect("topological_sortが返すキーは必ず存在する");
                (key, node_weight(key, value))
            })
            .collect();

        let mut dist: HashMap<&K, W> = order.iter().map(|&key| (key, weight_of[key])).collect();
        let mut pred: HashMap<&K, &K> = HashMap::new();

        for &key in &order {
            let cur = dist[key];
            for succ in self.out_neighbors(key) {
                let candidate = cur + weight_of[succ];
                if candidate > dist[succ] {
                    dist.insert(succ, candidate);
                    pred.insert(succ, key);
                }
            }
        }

        let end = *order
            .iter()
            .max_by_key(|&&key| dist[key])
            .expect("orderは空でないことを上で確認済み");

        let total = dist[end];
        let mut path = vec![end];
        let mut cur = end;
        while let Some(&p) = pred.get(cur) {
            path.push(p);
            cur = p;
        }
        path.reverse();

        Ok((path, total))
    }

    /// `indices` (循環を構成する `NodeIndex` 列) を `CycleError<K>` に変換する。
    fn cycle_error(&self, indices: Vec<NodeIndex>) -> CycleError<K> {
        CycleError {
            cycle: indices.into_iter().map(|idx| self.keys[&idx].clone()).collect(),
        }
    }

    /// グラフ中の循環を 1 つ探して構成ノードの `NodeIndex` 列で返す
    /// (循環がなければ `None`)。`cycle[0] -> cycle[1] -> .. -> cycle[last] ->
    /// cycle[0]` の閉路になっている。
    ///
    /// `petgraph::algo::tarjan_scc` で強連結成分を求め、要素数が 2 以上の
    /// 成分 (=循環を含む) か、要素数 1 かつ自己ループを持つ成分を探す。
    /// 見つかった成分の中で DFS を行い、逆辺 (訪問中のノードへ戻る辺) を
    /// 検出した時点で「その逆辺の宛先」から「現在のノード」までの DFS
    /// パスを切り出せば、それがそのまま単純閉路になる。
    fn find_cycle_indices(&self) -> Option<Vec<NodeIndex>> {
        for scc in petgraph::algo::tarjan_scc(&self.inner) {
            if scc.len() > 1 {
                return Some(self.extract_cycle_from_scc(&scc));
            }
            if scc.len() == 1 {
                let idx = scc[0];
                if self.inner.find_edge(idx, idx).is_some() {
                    return Some(vec![idx]);
                }
            }
        }
        None
    }

    /// 強連結成分 `scc` (要素数 2 以上、循環を含むことが保証されている)
    /// の中から単純閉路を 1 つ復元する。反復 DFS + パススタックによる
    /// 逆辺検出。
    fn extract_cycle_from_scc(&self, scc: &[NodeIndex]) -> Vec<NodeIndex> {
        let scc_set: HashSet<NodeIndex> = scc.iter().copied().collect();
        let start = scc[0];

        let mut path: Vec<NodeIndex> = vec![start];
        let mut path_pos: HashMap<NodeIndex, usize> = HashMap::new();
        path_pos.insert(start, 0);
        let mut visited: HashSet<NodeIndex> = HashSet::new();
        visited.insert(start);

        let neighbors_of = |idx: NodeIndex| -> Vec<NodeIndex> {
            self.inner
                .neighbors_directed(idx, Direction::Outgoing)
                .filter(|n| scc_set.contains(n))
                .collect()
        };

        let mut frames: Vec<(NodeIndex, std::vec::IntoIter<NodeIndex>)> =
            vec![(start, neighbors_of(start).into_iter())];

        while let Some((_, iter)) = frames.last_mut() {
            match iter.next() {
                Some(next) => {
                    if let Some(&pos) = path_pos.get(&next) {
                        // 逆辺を発見: pos..end が単純閉路。
                        return path[pos..].to_vec();
                    }
                    if visited.insert(next) {
                        path.push(next);
                        path_pos.insert(next, path.len() - 1);
                        frames.push((next, neighbors_of(next).into_iter()));
                    }
                }
                None => {
                    if let Some((node, _)) = frames.pop() {
                        path.pop();
                        path_pos.remove(&node);
                    }
                }
            }
        }

        unreachable!("要素数2以上の強連結成分は必ず閉路を含む")
    }

    /// `key` から到達可能な全ノードキー (`key` 自身も含む反射的な到達可能性)。
    /// `key` が存在しなければ空。
    pub fn reachable_from(&self, key: &K) -> Vec<&K> {
        match self.index.get(key) {
            Some(&start) => {
                let mut dfs = petgraph::visit::Dfs::new(&self.inner, start);
                let mut result = Vec::new();
                while let Some(idx) = dfs.next(&self.inner) {
                    result.push(&self.keys[&idx]);
                }
                result
            }
            None => Vec::new(),
        }
    }

    /// `from` から `to` への (辺数最短の) 経路をキー列で返す。
    /// 到達不能・端点キーが未知なら `None`。`from == to` なら `[from]` を返す。
    pub fn path(&self, from: &K, to: &K) -> Option<Vec<&K>> {
        let from_idx = *self.index.get(from)?;
        let to_idx = *self.index.get(to)?;

        if from_idx == to_idx {
            return Some(vec![&self.keys[&from_idx]]);
        }

        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut queue: VecDeque<NodeIndex> = VecDeque::new();
        let mut pred: HashMap<NodeIndex, NodeIndex> = HashMap::new();

        visited.insert(from_idx);
        queue.push_back(from_idx);

        while let Some(cur) = queue.pop_front() {
            for next in self.inner.neighbors_directed(cur, Direction::Outgoing) {
                if visited.insert(next) {
                    pred.insert(next, cur);
                    if next == to_idx {
                        let mut path = vec![next];
                        let mut c = next;
                        while let Some(&p) = pred.get(&c) {
                            path.push(p);
                            c = p;
                        }
                        path.reverse();
                        return Some(path.into_iter().map(|idx| &self.keys[&idx]).collect());
                    }
                    queue.push_back(next);
                }
            }
        }
        None
    }

    /// 構造 (キー・トポロジー) を保ったまま、ノード値だけを `f` で変換する。
    /// グラフをファンクタとして見た map に相当する。キーが要らないなら
    /// こちらを使う (キーも見たい場合は [`Graph::map_nodes_with_key`])。
    pub fn map_nodes<M>(&self, mut f: impl FnMut(&N) -> M) -> Graph<M, E, K>
    where
        E: Clone,
    {
        self.map_nodes_with_key(|_, v| f(v))
    }

    /// [`Graph::map_nodes`] のキー付き版。`f` にはノード値だけでなくキーも
    /// 渡される (キーに応じて変換内容を変えたい場合に使う)。
    pub fn map_nodes_with_key<M>(&self, mut f: impl FnMut(&K, &N) -> M) -> Graph<M, E, K>
    where
        E: Clone,
    {
        let mut inner: DiGraph<M, E> = DiGraph::new();
        let mut index: HashMap<K, NodeIndex> = HashMap::new();
        let mut keys: HashMap<NodeIndex, K> = HashMap::new();
        let mut old_to_new: HashMap<NodeIndex, NodeIndex> = HashMap::new();

        for old_idx in self.inner.node_indices() {
            let key = self.keys[&old_idx].clone();
            let new_value = f(&key, &self.inner[old_idx]);
            let new_idx = inner.add_node(new_value);
            old_to_new.insert(old_idx, new_idx);
            keys.insert(new_idx, key.clone());
            index.insert(key, new_idx);
        }

        for edge in self.inner.edge_references() {
            let new_from = old_to_new[&edge.source()];
            let new_to = old_to_new[&edge.target()];
            inner.add_edge(new_from, new_to, edge.weight().clone());
        }

        Graph { inner, index, keys }
    }

    /// 述語 `pred` を満たすノードだけを残す。辺は両端が生き残ったものだけ残る。
    /// キーが要らないならこちらを使う (キーで絞り込みたい場合は
    /// [`Graph::filter_nodes_with_key`])。
    pub fn filter_nodes(&self, mut pred: impl FnMut(&N) -> bool) -> Graph<N, E, K>
    where
        N: Clone,
        E: Clone,
    {
        self.filter_nodes_with_key(|_, v| pred(v))
    }

    /// [`Graph::filter_nodes`] のキー付き版。`pred` にはノード値だけでなく
    /// キーも渡される (例: 特定の ID 集合に含まれるノードだけ抽出する)。
    pub fn filter_nodes_with_key(&self, mut pred: impl FnMut(&K, &N) -> bool) -> Graph<N, E, K>
    where
        N: Clone,
        E: Clone,
    {
        let mut inner: DiGraph<N, E> = DiGraph::new();
        let mut index: HashMap<K, NodeIndex> = HashMap::new();
        let mut keys: HashMap<NodeIndex, K> = HashMap::new();
        let mut old_to_new: HashMap<NodeIndex, NodeIndex> = HashMap::new();

        for old_idx in self.inner.node_indices() {
            let key = self.keys[&old_idx].clone();
            if pred(&key, &self.inner[old_idx]) {
                let new_idx = inner.add_node(self.inner[old_idx].clone());
                old_to_new.insert(old_idx, new_idx);
                keys.insert(new_idx, key.clone());
                index.insert(key, new_idx);
            }
        }

        for edge in self.inner.edge_references() {
            if let (Some(&new_from), Some(&new_to)) = (
                old_to_new.get(&edge.source()),
                old_to_new.get(&edge.target()),
            ) {
                inner.add_edge(new_from, new_to, edge.weight().clone());
            }
        }

        Graph { inner, index, keys }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    fn sample_people() -> Vec<(String, Person)> {
        vec![
            (
                "田中".to_string(),
                Person {
                    name: "田中".to_string(),
                    age: 30,
                },
            ),
            (
                "佐藤".to_string(),
                Person {
                    name: "佐藤".to_string(),
                    age: 25,
                },
            ),
            (
                "鈴木".to_string(),
                Person {
                    name: "鈴木".to_string(),
                    age: 40,
                },
            ),
        ]
    }

    #[test]
    fn build_正常系() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
            ],
        )
        .expect("構築に成功するはず");

        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
        assert_eq!(g.node(&"田中".to_string()).unwrap().age, 30);
        assert!(g.node(&"存在しない".to_string()).is_none());
    }

    #[test]
    fn build_重複キーはエラー() {
        let err = Graph::<Person>::build(
            vec![
                (
                    "田中".to_string(),
                    Person {
                        name: "田中".to_string(),
                        age: 30,
                    },
                ),
                (
                    "田中".to_string(),
                    Person {
                        name: "田中2".to_string(),
                        age: 31,
                    },
                ),
            ],
            vec![],
        )
        .unwrap_err();

        assert_eq!(err, GraphError::DuplicateKey("田中".to_string()));
    }

    #[test]
    fn build_未知キーへの辺はエラー() {
        let err = Graph::<Person>::build(
            sample_people(),
            vec![("田中".to_string(), "存在しない".to_string(), ())],
        )
        .unwrap_err();

        assert_eq!(
            err,
            GraphError::UnknownEndpoint {
                from: "田中".to_string(),
                to: "存在しない".to_string(),
                missing: "存在しない".to_string(),
            }
        );
    }

    #[test]
    fn create_builderパターンで構築できる() {
        let g: Graph<Person> = Graph::create(|b| {
            for (k, v) in sample_people() {
                b.node(k, v);
            }
            b.edge("田中".to_string(), "佐藤".to_string(), ());
        })
        .expect("構築に成功するはず");

        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 1);
        assert_eq!(g.out_neighbors(&"田中".to_string()), vec![&"佐藤".to_string()]);
    }

    #[test]
    fn create_builder内のエラーも_resultで返る() {
        let result: Result<Graph<Person>, _> = Graph::create(|b| {
            b.node(
                "田中".to_string(),
                Person {
                    name: "田中".to_string(),
                    age: 30,
                },
            );
            b.edge("田中".to_string(), "存在しない".to_string(), ());
        });

        assert!(result.is_err());
    }

    #[test]
    fn has_cycle_循環なし() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
            ],
        )
        .unwrap();
        assert!(!g.has_cycle());
    }

    #[test]
    fn has_cycle_循環あり() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
                ("鈴木".to_string(), "田中".to_string(), ()),
            ],
        )
        .unwrap();
        assert!(g.has_cycle());
    }

    #[test]
    fn topological_sort_循環なしなら順序を返す() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
            ],
        )
        .unwrap();

        let order = g.topological_sort().expect("循環がないので成功するはず");
        let pos = |k: &str| order.iter().position(|&x| x == k).unwrap();
        assert!(pos("田中") < pos("佐藤"));
        assert!(pos("佐藤") < pos("鈴木"));
    }

    #[test]
    fn topological_sort_循環ありならエラー() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
                ("鈴木".to_string(), "田中".to_string(), ()),
            ],
        )
        .unwrap();

        assert!(g.topological_sort().is_err());
    }

    #[test]
    fn reachable_from_到達可能なノードを返す() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![("田中".to_string(), "佐藤".to_string(), ())],
        )
        .unwrap();

        let mut reachable: Vec<String> = g
            .reachable_from(&"田中".to_string())
            .into_iter()
            .cloned()
            .collect();
        reachable.sort();
        assert_eq!(reachable, vec!["佐藤".to_string(), "田中".to_string()]);

        // 辺の無い鈴木からは自分自身のみ到達可能
        assert_eq!(
            g.reachable_from(&"鈴木".to_string()),
            vec![&"鈴木".to_string()]
        );

        // 存在しないキーは空
        assert!(g.reachable_from(&"存在しない".to_string()).is_empty());
    }

    #[test]
    fn path_経路を返す() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
            ],
        )
        .unwrap();

        let path = g
            .path(&"田中".to_string(), &"鈴木".to_string())
            .expect("経路があるはず");
        assert_eq!(
            path,
            vec![
                &"田中".to_string(),
                &"佐藤".to_string(),
                &"鈴木".to_string()
            ]
        );

        // 到達不能
        assert!(g.path(&"鈴木".to_string(), &"田中".to_string()).is_none());

        // 自分自身への経路
        assert_eq!(
            g.path(&"田中".to_string(), &"田中".to_string()),
            Some(vec![&"田中".to_string()])
        );
    }

    #[test]
    fn map_nodes_構造を保ったまま値を変換する() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![("田中".to_string(), "佐藤".to_string(), ())],
        )
        .unwrap();

        let ages: Graph<u32> = g.map_nodes(|p| p.age);

        assert_eq!(ages.node_count(), 3);
        assert_eq!(ages.edge_count(), 1);
        assert_eq!(*ages.node(&"田中".to_string()).unwrap(), 30);
        assert_eq!(
            ages.out_neighbors(&"田中".to_string()),
            vec![&"佐藤".to_string()]
        );
    }

    #[test]
    fn filter_nodes_述語を満たすノードと両端が生き残った辺だけ残す() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
            ],
        )
        .unwrap();

        // 30 歳以上: 田中(30), 鈴木(40) が残り、佐藤(25) は落ちる
        // → 田中-佐藤, 佐藤-鈴木 の辺は両方とも片方の端点を失うので消える
        let adults = g.filter_nodes(|p| p.age >= 30);

        assert_eq!(adults.node_count(), 2);
        assert_eq!(adults.edge_count(), 0);
        assert!(adults.node(&"田中".to_string()).is_some());
        assert!(adults.node(&"鈴木".to_string()).is_some());
        assert!(adults.node(&"佐藤".to_string()).is_none());
    }

    #[test]
    fn edge_weight_辺属性にアクセスできる() {
        #[derive(Debug, Clone, PartialEq)]
        struct Friendship {
            since: u32,
        }

        let g: Graph<Person, Friendship> = Graph::build(
            sample_people(),
            vec![(
                "田中".to_string(),
                "佐藤".to_string(),
                Friendship { since: 2015 },
            )],
        )
        .unwrap();

        assert_eq!(
            g.edge_weight(&"田中".to_string(), &"佐藤".to_string()),
            Some(&Friendship { since: 2015 })
        );
        assert_eq!(
            g.edge_weight(&"佐藤".to_string(), &"田中".to_string()),
            None
        );
    }

    #[test]
    fn keys_と_nodes_で全件走査できる() {
        let g: Graph<Person> = Graph::build(sample_people(), vec![]).unwrap();

        let mut keys: Vec<&String> = g.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec![
                &"佐藤".to_string(),
                &"田中".to_string(),
                &"鈴木".to_string()
            ]
        );

        assert_eq!(g.nodes().count(), 3);
    }

    #[test]
    fn in_neighbors_out_neighborsと対称() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "鈴木".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
            ],
        )
        .unwrap();

        let mut in_neighbors: Vec<String> = g
            .in_neighbors(&"鈴木".to_string())
            .into_iter()
            .cloned()
            .collect();
        in_neighbors.sort();
        assert_eq!(
            in_neighbors,
            vec!["佐藤".to_string(), "田中".to_string()]
        );

        // 入る辺の無いノードは空。
        assert!(g.in_neighbors(&"田中".to_string()).is_empty());
        // 存在しないキーも空。
        assert!(g.in_neighbors(&"存在しない".to_string()).is_empty());
    }

    #[test]
    fn from_edges_pairsイテレータから射影してhas_cycleが動く() {
        let ids = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        // `{label}_pairs()` のような `(&K, &K)` を yield するイテレータを模す。
        let pairs: Vec<(&String, &String)> = vec![(&ids[0], &ids[1]), (&ids[1], &ids[2])];

        let g: Graph<(), (), String> = Graph::from_edges(
            ids.iter().cloned(),
            pairs.into_iter().map(|(a, b)| (a.clone(), b.clone())),
        )
        .unwrap();

        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
        assert!(!g.has_cycle());

        // 循環にもなる。
        let cyclic: Graph<(), (), &str> =
            Graph::from_edges(vec!["a", "b"], vec![("a", "b"), ("b", "a")]).unwrap();
        assert!(cyclic.has_cycle());
    }

    #[test]
    fn from_edges_未知キーへの辺はエラー() {
        let err = Graph::<(), (), &str>::from_edges(vec!["a", "b"], vec![("a", "c")]).unwrap_err();
        assert_eq!(
            err,
            GraphError::UnknownEndpoint {
                from: "a",
                to: "c",
                missing: "c",
            }
        );
    }

    #[test]
    fn topological_levels_依存のないノードから順にレベル分割する() {
        let g: Graph<()> = Graph::build(
            vec![
                ("fetch".to_string(), ()),
                ("build_a".to_string(), ()),
                ("build_b".to_string(), ()),
                ("link".to_string(), ()),
            ],
            vec![
                ("fetch".to_string(), "build_a".to_string(), ()),
                ("fetch".to_string(), "build_b".to_string(), ()),
                ("build_a".to_string(), "link".to_string(), ()),
                ("build_b".to_string(), "link".to_string(), ()),
            ],
        )
        .unwrap();

        let levels = g.topological_levels().expect("循環がないので成功するはず");
        assert_eq!(levels.len(), 3);
        assert_eq!(levels[0], vec![&"fetch".to_string()]);
        // レベル内の順序は挿入順 (build_a が build_b より先に宣言されている)。
        assert_eq!(
            levels[1],
            vec![&"build_a".to_string(), &"build_b".to_string()]
        );
        assert_eq!(levels[2], vec![&"link".to_string()]);
    }

    #[test]
    fn topological_levels_循環ありならエラー() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
                ("鈴木".to_string(), "田中".to_string(), ()),
            ],
        )
        .unwrap();

        assert!(g.topological_levels().is_err());
    }

    #[test]
    fn critical_path_by_ノード重み付き最長経路を返す() {
        // 田中(30) -> 佐藤(25) -> 鈴木(40)。年齢をノード重みとして使う。
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
            ],
        )
        .unwrap();

        let (path, total) = g
            .critical_path_by(|_key, person| person.age)
            .expect("循環がないので成功するはず");

        assert_eq!(
            path,
            vec![
                &"田中".to_string(),
                &"佐藤".to_string(),
                &"鈴木".to_string()
            ]
        );
        assert_eq!(total, 30 + 25 + 40);
    }

    #[test]
    fn critical_path_by_空グラフはvecと初期値を返す() {
        let g: Graph<Person> = Graph::build(vec![], vec![]).unwrap();
        let (path, total): (Vec<&String>, u32) = g
            .critical_path_by(|_key, person| person.age)
            .expect("空グラフは循環なしとして成功するはず");
        assert!(path.is_empty());
        assert_eq!(total, 0);
    }

    #[test]
    fn critical_path_by_循環ありならエラー() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
                ("鈴木".to_string(), "田中".to_string(), ()),
            ],
        )
        .unwrap();

        assert!(g.critical_path_by(|_key, person| person.age).is_err());
    }

    #[test]
    fn cycle_error_循環を構成するノード列全体を返す() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
                ("鈴木".to_string(), "田中".to_string(), ()),
            ],
        )
        .unwrap();

        let err = g.topological_sort().unwrap_err();
        assert_eq!(err.cycle.len(), 3);

        let mut sorted = err.cycle.clone();
        sorted.sort();
        assert_eq!(
            sorted,
            vec!["佐藤".to_string(), "田中".to_string(), "鈴木".to_string()]
        );

        // cycle[0] から辿って cycle[0] に戻る閉路になっていることを検証する。
        for i in 0..err.cycle.len() {
            let from = &err.cycle[i];
            let to = &err.cycle[(i + 1) % err.cycle.len()];
            assert!(
                g.edge_weight(from, to).is_some(),
                "{from:?} -> {to:?} の辺が無い"
            );
        }
    }

    #[test]
    fn cycle_error_自己ループも循環として検出する() {
        let g: Graph<(), (), &str> = Graph::from_edges(vec!["a"], vec![("a", "a")]).unwrap();
        let err = g.topological_sort().unwrap_err();
        assert_eq!(err.cycle, vec!["a"]);
    }

    #[test]
    fn filter_nodes_with_key_キーに依存するフィルタができる() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![
                ("田中".to_string(), "佐藤".to_string(), ()),
                ("佐藤".to_string(), "鈴木".to_string(), ()),
            ],
        )
        .unwrap();

        // 特定のID集合に含まれるノードだけ抽出する (値ではなくキーで判定)。
        let allowed: HashSet<String> = ["田中".to_string(), "鈴木".to_string()].into_iter().collect();
        let filtered = g.filter_nodes_with_key(|key, _person| allowed.contains(key));

        assert_eq!(filtered.node_count(), 2);
        assert!(filtered.node(&"田中".to_string()).is_some());
        assert!(filtered.node(&"鈴木".to_string()).is_some());
        assert!(filtered.node(&"佐藤".to_string()).is_none());
        // 両端が生き残っていない辺は消える。
        assert_eq!(filtered.edge_count(), 0);
    }

    #[test]
    fn map_nodes_with_key_キーも見て変換できる() {
        let g: Graph<Person> = Graph::build(
            sample_people(),
            vec![("田中".to_string(), "佐藤".to_string(), ())],
        )
        .unwrap();

        let labeled: Graph<String> = g.map_nodes_with_key(|key, person| format!("{key}:{}", person.age));

        assert_eq!(
            labeled.node(&"田中".to_string()).unwrap(),
            "田中:30"
        );
        assert_eq!(
            labeled.node(&"佐藤".to_string()).unwrap(),
            "佐藤:25"
        );
    }
}
