//! 水準1相当: ジェネリックグラフ `Graph<N, E, K>`。
//!
//! `docs/graph_design_sketches.md` の決定1 (ノードの同一性はユーザーキー)・
//! 決定2 (可変性はクロージャスコープ builder → 凍結し、以後不変) をそのまま
//! Rust に輸入したもの。マクロは一切使わない、ふつうのジェネリック構造体。
//!
//! ## 内部表現
//!
//! 「キーの世界」と「位置の世界」を分けている。位置の世界は
//! [`topology::有向トポロジー`] が持ち、`petgraph::graph::DiGraph<N, E>` を包む。
//! キーの世界は [`key_correspondence::キー対応表`] が持ち、ユーザーキー `K` と
//! 内部位置の相互対応 (正引きと逆引き) を所有する。
//! `petgraph::graphmap::GraphMap` はノードキーに `Copy` を要求するため
//! `String` のような非 `Copy` キーを直接扱えず不採用
//! (`.claude/skills/proc-macro-dev/SKILL.md` の注意通り)。
//!
//! ## 不変性
//!
//! `Graph` は構築後不変。可変な操作 (ノード追加・削除・辺追加) を一切公開
//! しない。構築は [`Graph::build`] (原子的な一括構築) と [`Graph::create`]
//! (builder をクロージャに貸し出し、戻ったら凍結) の 2 経路のみ。
//! `create` に渡すクロージャの型は `for<'b> FnOnce(&'b mut GraphBuilder<..>)`
//! であり、builder への参照をクロージャの外に持ち出すことを借用検査器が
//! 静的に拒否する (`std::thread::scope` と同じ仕組み)。

mod assembly;
mod build_error;
mod builder;
mod cycle_error;
mod key_correspondence;
mod structure_graph;
mod topology;

use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

pub use build_error::GraphError;
pub use builder::GraphBuilder;
pub use cycle_error::CycleError;

use assembly::構築中のグラフ;
use key_correspondence::キー対応表;
use topology::{ノード位置, 循環の探索, 有向トポロジー, 閉路の位置列};

/// ノード種別 `N`、エッジ種別 `E` (既定は属性なしを表す `()`)、
/// ノードキー種別 `K` (既定は `String`) を持つ有向グラフ。
///
/// 構築後は不変 — 可変 API は公開しない。`build`/`create` でのみ作れる。
#[derive(Debug)]
pub struct Graph<N, E = (), K = String> {
    トポロジー: 有向トポロジー<N, E>,
    キー対応: キー対応表<K>,
}

impl<N, E, K> Graph<N, E, K> {
    /// 部品からグラフを組み立てる、この型の唯一の私有コンストラクタ。
    fn 部品から組み立てる(
        トポロジー: 有向トポロジー<N, E>,
        キー対応: キー対応表<K>,
    ) -> Self {
        Self {
            トポロジー,
            キー対応,
        }
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
        構築中のグラフ::ノード列と辺列から組み立てる(nodes, edges)
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
        let mut builder = GraphBuilder::空のbuilderから始める();
        f(&mut builder);
        builder.凍結する()
    }

    /// キーからノード値を引く。
    pub fn node(&self, key: &K) -> Option<&N> {
        self.キー対応
            .位置(key)
            .map(|位置| self.トポロジー.ノード値(位置))
    }

    /// 全ノードキーを走査するイテレータ (順序は未規定)。
    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.キー対応.キーの一覧()
    }

    /// 全ノードを `(キー, 値)` で走査するイテレータ (順序は未規定)。
    pub fn nodes(&self) -> impl Iterator<Item = (&K, &N)> {
        self.キー対応
            .キーと位置の一覧()
            .map(move |(キー, 位置)| (キー, self.トポロジー.ノード値(位置)))
    }

    /// ノード数。
    pub fn node_count(&self) -> usize {
        self.トポロジー.ノード数()
    }

    /// 辺数。
    pub fn edge_count(&self) -> usize {
        self.トポロジー.辺数()
    }

    /// `key` から出て行く辺の終点キー一覧。`key` が存在しなければ空。
    pub fn out_neighbors(&self, key: &K) -> Vec<&K> {
        match self.キー対応.位置(key) {
            Some(位置) => self
                .トポロジー
                .出ていく先(位置)
                .map(|隣| self.キー対応.キー(隣))
                .collect(),
            None => Vec::new(),
        }
    }

    /// `key` へ入ってくる辺の始点キー一覧 (`out_neighbors` と対称)。
    /// `key` が存在しなければ空。
    pub fn in_neighbors(&self, key: &K) -> Vec<&K> {
        match self.キー対応.位置(key) {
            Some(位置) => self
                .トポロジー
                .入ってくる元(位置)
                .map(|隣| self.キー対応.キー(隣))
                .collect(),
            None => Vec::new(),
        }
    }

    /// `from -> to` の辺属性を引く。辺が存在しない・端点キーが未知なら `None`。
    pub fn edge_weight(&self, from: &K, to: &K) -> Option<&E> {
        let 始点 = self.キー対応.位置(from)?;
        let 終点 = self.キー対応.位置(to)?;
        self.トポロジー.辺値(始点, 終点)
    }

    /// グラフに循環があるか。
    pub fn has_cycle(&self) -> bool {
        循環の探索::トポロジーから始める(&self.トポロジー).循環があるか()
    }

    /// トポロジカルソート。循環がある場合は `CycleError` を返す。
    pub fn topological_sort(&self) -> Result<Vec<&K>, CycleError<K>> {
        match self.トポロジー.トポロジカル順序() {
            Some(順序) => Ok(順序
                .into_iter()
                .map(|位置| self.キー対応.キー(位置))
                .collect()),
            None => Err(self.閉路をエラーへ翻訳する(
                self.閉路を1本探す()
                    .expect("toposortが失敗したので循環が存在するはず"),
            )),
        }
    }

    /// 依存のないノードから順にレベル (波) 分割したトポロジカルソート。
    /// 各レベルは「まだ処理されていない先行ノードを持たないノード」の
    /// 集合であり、レベル内の順序はノードの挿入順 (`build`/`create` に
    /// 渡した順) で決定的。循環がある場合は `CycleError` を返す。
    pub fn topological_levels(&self) -> Result<Vec<Vec<&K>>, CycleError<K>> {
        if let Some(閉路) = self.閉路を1本探す() {
            return Err(self.閉路をエラーへ翻訳する(閉路));
        }

        let 挿入順 = self.トポロジー.挿入順の位置列();

        let mut 入次数: HashMap<ノード位置, usize> = 挿入順
            .iter()
            .map(|&位置| (位置, self.トポロジー.入ってくる元(位置).count()))
            .collect();
        let mut 未確定: HashSet<ノード位置> = 挿入順.iter().copied().collect();

        let mut レベル列: Vec<Vec<&K>> = Vec::new();

        while !未確定.is_empty() {
            let 現在のレベル: Vec<ノード位置> = 挿入順
                .iter()
                .copied()
                .filter(|位置| 未確定.contains(位置) && 入次数[位置] == 0)
                .collect();

            debug_assert!(
                !現在のレベル.is_empty(),
                "循環なしを確認済みなのでフロンティアが空になることはない"
            );

            for &位置 in &現在のレベル {
                未確定.remove(&位置);
            }
            for &位置 in &現在のレベル {
                for 次 in self.トポロジー.出ていく先(位置) {
                    if let Some(残り) = 入次数.get_mut(&次) {
                        *残り = 残り.saturating_sub(1);
                    }
                }
            }

            レベル列.push(
                現在のレベル
                    .iter()
                    .map(|&位置| self.キー対応.キー(位置))
                    .collect(),
            );
        }

        Ok(レベル列)
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
                let value = self
                    .node(key)
                    .expect("topological_sortが返すキーは必ず存在する");
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

    /// 閉路を構成する位置列を、利用者が読むキー列のエラーへ翻訳する。
    fn 閉路をエラーへ翻訳する(&self, 閉路: 閉路の位置列) -> CycleError<K> {
        CycleError {
            cycle: 閉路
                .位置の並び()
                .iter()
                .map(|&位置| self.キー対応.キー(位置).clone())
                .collect(),
        }
    }

    /// グラフ中の循環を 1 つ探して閉路の位置列で返す (循環がなければ `None`)。
    fn 閉路を1本探す(&self) -> Option<閉路の位置列> {
        循環の探索::トポロジーから始める(&self.トポロジー).閉路を1本探す()
    }

    /// `key` から到達可能な全ノードキー (`key` 自身も含む反射的な到達可能性)。
    /// `key` が存在しなければ空。
    pub fn reachable_from(&self, key: &K) -> Vec<&K> {
        match self.キー対応.位置(key) {
            Some(始点) => self
                .トポロジー
                .深さ優先で到達できる位置列(始点)
                .into_iter()
                .map(|位置| self.キー対応.キー(位置))
                .collect(),
            None => Vec::new(),
        }
    }

    /// `from` から `to` への (辺数最短の) 経路をキー列で返す。
    /// 到達不能・端点キーが未知なら `None`。`from == to` なら `[from]` を返す。
    pub fn path(&self, from: &K, to: &K) -> Option<Vec<&K>> {
        let 始点 = self.キー対応.位置(from)?;
        let 終点 = self.キー対応.位置(to)?;

        if 始点 == 終点 {
            return Some(vec![self.キー対応.キー(始点)]);
        }

        let mut 訪問済み: HashSet<ノード位置> = HashSet::new();
        let mut 待ち行列: VecDeque<ノード位置> = VecDeque::new();
        let mut 先行: HashMap<ノード位置, ノード位置> = HashMap::new();

        訪問済み.insert(始点);
        待ち行列.push_back(始点);

        while let Some(現在) = 待ち行列.pop_front() {
            for 次 in self.トポロジー.出ていく先(現在) {
                if 訪問済み.insert(次) {
                    先行.insert(次, 現在);
                    if 次 == 終点 {
                        let mut 経路 = vec![次];
                        let mut 遡り = 次;
                        while let Some(&手前) = 先行.get(&遡り) {
                            経路.push(手前);
                            遡り = 手前;
                        }
                        経路.reverse();
                        return Some(
                            経路
                                .into_iter()
                                .map(|位置| self.キー対応.キー(位置))
                                .collect(),
                        );
                    }
                    待ち行列.push_back(次);
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
        let mut トポロジー: 有向トポロジー<M, E> =
            有向トポロジー::空のトポロジーを生成する();
        let mut キー対応 = キー対応表::空の対応表を生成する();
        let mut 位置の対応: HashMap<ノード位置, ノード位置> = HashMap::new();

        for 変換前 in self.トポロジー.挿入順の位置列() {
            let キー = self.キー対応.キー(変換前).clone();
            let 新しい値 = f(&キー, self.トポロジー.ノード値(変換前));
            let 変換後 = トポロジー.ノードを追加する(新しい値);
            位置の対応.insert(変換前, 変換後);
            キー対応.対応を登録する(キー, 変換後);
        }

        for (始点, 終点, 値) in self.トポロジー.辺の一覧() {
            トポロジー.辺を追加する(位置の対応[&始点], 位置の対応[&終点], 値.clone());
        }

        Graph::部品から組み立てる(トポロジー, キー対応)
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
        let mut トポロジー: 有向トポロジー<N, E> =
            有向トポロジー::空のトポロジーを生成する();
        let mut キー対応 = キー対応表::空の対応表を生成する();
        let mut 位置の対応: HashMap<ノード位置, ノード位置> = HashMap::new();

        for 変換前 in self.トポロジー.挿入順の位置列() {
            let キー = self.キー対応.キー(変換前).clone();
            if pred(&キー, self.トポロジー.ノード値(変換前)) {
                let 変換後 =
                    トポロジー.ノードを追加する(self.トポロジー.ノード値(変換前).clone());
                位置の対応.insert(変換前, 変換後);
                キー対応.対応を登録する(キー, 変換後);
            }
        }

        for (始点, 終点, 値) in self.トポロジー.辺の一覧() {
            if let (Some(&新しい始点), Some(&新しい終点)) =
                (位置の対応.get(&始点), 位置の対応.get(&終点))
            {
                トポロジー.辺を追加する(新しい始点, 新しい終点, 値.clone());
            }
        }

        Graph::部品から組み立てる(トポロジー, キー対応)
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
        assert_eq!(
            g.out_neighbors(&"田中".to_string()),
            vec![&"佐藤".to_string()]
        );
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
        assert_eq!(in_neighbors, vec!["佐藤".to_string(), "田中".to_string()]);

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
        let allowed: HashSet<String> = ["田中".to_string(), "鈴木".to_string()]
            .into_iter()
            .collect();
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

        let labeled: Graph<String> =
            g.map_nodes_with_key(|key, person| format!("{key}:{}", person.age));

        assert_eq!(labeled.node(&"田中".to_string()).unwrap(), "田中:30");
        assert_eq!(labeled.node(&"佐藤".to_string()).unwrap(), "佐藤:25");
    }
}
