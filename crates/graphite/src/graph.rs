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

/// [`Graph::topological_sort`] が循環検出時に返すエラー。
/// `node` は循環の一部であることが判明したノードのキー。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError<K> {
    pub node: K,
}

impl<K: fmt::Debug> fmt::Display for CycleError<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "グラフに循環があります (ノード {:?} を経由)", self.node)
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
            Err(cycle) => Err(CycleError {
                node: self.keys[&cycle.node_id()].clone(),
            }),
        }
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
    /// グラフをファンクタとして見た map に相当する。
    pub fn map_nodes<M>(&self, mut f: impl FnMut(&N) -> M) -> Graph<M, E, K>
    where
        E: Clone,
    {
        let mut inner: DiGraph<M, E> = DiGraph::new();
        let mut index: HashMap<K, NodeIndex> = HashMap::new();
        let mut keys: HashMap<NodeIndex, K> = HashMap::new();
        let mut old_to_new: HashMap<NodeIndex, NodeIndex> = HashMap::new();

        for old_idx in self.inner.node_indices() {
            let key = self.keys[&old_idx].clone();
            let new_value = f(&self.inner[old_idx]);
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
    pub fn filter_nodes(&self, mut pred: impl FnMut(&N) -> bool) -> Graph<N, E, K>
    where
        N: Clone,
        E: Clone,
    {
        let mut inner: DiGraph<N, E> = DiGraph::new();
        let mut index: HashMap<K, NodeIndex> = HashMap::new();
        let mut keys: HashMap<NodeIndex, K> = HashMap::new();
        let mut old_to_new: HashMap<NodeIndex, NodeIndex> = HashMap::new();

        for old_idx in self.inner.node_indices() {
            if pred(&self.inner[old_idx]) {
                let key = self.keys[&old_idx].clone();
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
}
