//! `Graph<N, E, K>` — 水準1相当のジェネリックグラフを提供する。
//!
//! `../Bullet/docs/graph_design_sketches.md` の決定1 (ノードの同一性はユーザーキー)・
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
//!
//! ## 100行原則の例外
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 再設計待ち)。この
//! ファイルは150行を超える。このファイルは公開契約の窓口としてメソッドを1画
//! 面へ集める。上限を超えたため、判定は issue #28 のやること4 が行う。超過
//! を許す根拠の台帳は `docs/development/line_count_ledger.md` にある。
//!
//! このファイルは、キーの有無の判定と位置列⇄キー列の写し取り以外のロジックを
//! 書かない。アルゴリズムの本体は [`topology`] 配下の型が所有する。

mod assembly;
mod build_error;
mod builder;
mod cycle_error;
mod key_correspondence;
mod structure_graph;
mod topology;

use std::collections::HashMap;
use std::hash::Hash;

pub use build_error::GraphError;
pub use builder::GraphBuilder;
pub use cycle_error::CycleError;

use assembly::構築中のグラフ;
use key_correspondence::キー対応表;
use topology::{
    トポロジカル順序の算出, ノード位置, 依存レベルの分割, 到達可能な位置の収集, 循環の探索,
    最長経路の算出, 有向トポロジー, 辺数最短の経路探索, 閉路の位置列,
};

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

    /// キーからノード値を読み出す。
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
        self.キー対応
            .位置(key)
            .map(|位置| {
                self.トポロジー
                    .出ていく先(位置)
                    .map(|隣| self.キー対応.キー(隣))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// `key` へ入ってくる辺の始点キー一覧 (`out_neighbors` と対称)。
    /// `key` が存在しなければ空。
    pub fn in_neighbors(&self, key: &K) -> Vec<&K> {
        self.キー対応
            .位置(key)
            .map(|位置| {
                self.トポロジー
                    .入ってくる元(位置)
                    .map(|隣| self.キー対応.キー(隣))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// `from -> to` の辺属性を読み出す。辺が存在しない・端点キーが未知なら `None`。
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
        let 順序 = self.トポロジカル順序を求める()?;
        Ok(self.位置列をキー列へ翻訳する(順序))
    }

    /// 依存のないノードから順にレベル (波) 分割したトポロジカルソート。
    /// 各レベルは「まだ処理されていない先行ノードを持たないノード」の
    /// 集合であり、レベル内の順序はノードの挿入順 (`build`/`create` に
    /// 渡した順) で決定的。循環がある場合は `CycleError` を返す。
    pub fn topological_levels(&self) -> Result<Vec<Vec<&K>>, CycleError<K>> {
        self.循環が無いことを確認する()?;
        Ok(依存レベルの分割::トポロジーから始める(&self.トポロジー)
            .レベル列を求める()
            .into_iter()
            .map(|レベル| self.位置列をキー列へ翻訳する(レベル))
            .collect())
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
        let 順序 = self.トポロジカル順序を求める()?;
        let 重み = self.ノード重みの表を作る(順序.as_slice(), node_weight);
        let (経路, 総和) = 最長経路の算出::トポロジカル順序と重みから始める(
            &self.トポロジー,
            順序,
            重み,
        )
        .経路と総和を求める();
        Ok((self.位置列をキー列へ翻訳する(経路), 総和))
    }

    /// 位置の列を、利用者が読むキーの列へ翻訳する。
    fn 位置列をキー列へ翻訳する(&self, 位置列: Vec<ノード位置>) -> Vec<&K> {
        位置列
            .into_iter()
            .map(|位置| self.キー対応.キー(位置))
            .collect()
    }

    /// トポロジカル順序を位置列で求める。循環していたらキー列のエラーへ翻訳する。
    fn トポロジカル順序を求める(&self) -> Result<Vec<ノード位置>, CycleError<K>> {
        トポロジカル順序の算出::トポロジーから始める(&self.トポロジー)
            .順序を求める()
            .map_err(|閉路| self.閉路をエラーへ翻訳する(閉路))
    }

    /// 位置ごとのノード重みを、利用者が渡した重み付けから引いて表にする。
    fn ノード重みの表を作る<W>(
        &self,
        位置列: &[ノード位置],
        node_weight: impl Fn(&K, &N) -> W,
    ) -> HashMap<ノード位置, W> {
        位置列
            .iter()
            .map(|&位置| {
                let 重み = node_weight(self.キー対応.キー(位置), self.トポロジー.ノード値(位置));
                (位置, 重み)
            })
            .collect()
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

    /// グラフ中に循環が無いことを確認する。循環があればキー列のエラーを返す。
    fn 循環が無いことを確認する(&self) -> Result<(), CycleError<K>> {
        match self.閉路を1本探す() {
            None => Ok(()),
            Some(閉路) => Err(self.閉路をエラーへ翻訳する(閉路)),
        }
    }

    /// `key` から到達可能な全ノードキー (`key` 自身も含む反射的な到達可能性)。
    /// `key` が存在しなければ空。
    pub fn reachable_from(&self, key: &K) -> Vec<&K> {
        self.キー対応
            .位置(key)
            .map(|始点| {
                self.位置列をキー列へ翻訳する(
                    到達可能な位置の収集::トポロジーから始める(
                        &self.トポロジー,
                    )
                    .始点から到達できる位置列(始点),
                )
            })
            .unwrap_or_default()
    }

    /// `from` から `to` への (辺数最短の) 経路をキー列で返す。
    /// 到達不能・端点キーが未知なら `None`。`from == to` なら `[from]` を返す。
    pub fn path(&self, from: &K, to: &K) -> Option<Vec<&K>> {
        let 始点 = self.キー対応.位置(from)?;
        let 終点 = self.キー対応.位置(to)?;
        let 経路 = 辺数最短の経路探索::トポロジーから始める(&self.トポロジー)
            .経路を求める(始点, 終点)?;
        Some(self.位置列をキー列へ翻訳する(経路))
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
        構築中のグラフ::元のグラフから選んで写して始める(
            self,
            |キー, 値| Some(f(キー, 値)),
        )
        .完成させる()
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
        構築中のグラフ::元のグラフから選んで写して始める(self, |キー, 値| {
            pred(キー, 値).then(|| 値.clone())
        })
        .完成させる()
    }
}
