//! 評価状態 — 現在値と未再計算の集合の整合を所有する。
//!
//! 「値の書き込みと未再計算印の更新は常に対である」という不変条件を守るため、
//! 現在値と未再計算集合を別々に触れる口を外へ出さず、対で更新するメソッドだけを持つ。

use std::collections::{HashMap, HashSet};

/// 計算グラフの可変な部分。依存構造が構築後不変なのに対し、こちらだけが動く
/// (`reactive-cells` の `Engine` が不変な依存グラフ + 可変な値ストアを分けるのと
/// 同じ整理、`docs/graph_design_sketches.md` 決定2)。
pub(in crate::compute) struct 評価状態<V> {
    /// 現在の値。入力ノードは常に存在する。計算ノードは一度でも評価されると
    /// エントリができ、以後未再計算になっても値は残ったまま (古い値として
    /// 参照可能だが、外へは [`crate::ComputeGraph::get`] 経由でしか読めないので
    /// 古い値が漏れることはない)。
    現在値: HashMap<String, V>,
    /// 計算ノードのうち、依存元の変更以降まだ再評価していないもの (未評価の
    /// 初期状態を含む)。入力ノードは決してこの集合に入らない (値は直接
    /// 書き込まれるので「古い」という概念がない)。
    未再計算: HashSet<String>,
}

impl<V> 評価状態<V> {
    /// 入力値と、未再計算で始まる計算ノードのキー列から作る。
    pub(in crate::compute) fn 入力値と未再計算のキー列から生成する(
        入力値: HashMap<String, V>,
        未再計算のキー列: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            現在値: 入力値,
            未再計算: 未再計算のキー列.into_iter().collect(),
        }
    }

    pub(in crate::compute) fn 未再計算か(&self, キー: &str) -> bool {
        self.未再計算.contains(キー)
    }

    pub(in crate::compute) fn 現在値(&self, キー: &str) -> &V {
        &self.現在値[キー]
    }

    /// 依存キー列の並びどおりに現在値を集める (位置引数の並びそのもの)。
    pub(in crate::compute) fn 依存値を集める(&self, 依存キー列: &[String]) -> Vec<&V> {
        依存キー列.iter().map(|依存| &self.現在値[依存]).collect()
    }

    /// 入力ノードへ値を書き込み、影響先 (書き込んだキー自身を除く) を未再計算に
    /// する。書き込んだキーの新しい値は直接与えられたので再計算は要らない。
    pub(in crate::compute) fn 入力値を書き込んで影響先を未再計算にする(
        &mut self,
        キー: &str,
        値: V,
        影響先: Vec<String>,
    ) {
        self.現在値.insert(キー.to_string(), 値);
        for 影響先のキー in 影響先 {
            if 影響先のキー != キー {
                self.未再計算.insert(影響先のキー);
            }
        }
    }

    /// 計算ノードの評価結果を書き込み、同時に未再計算の印を落とす。
    pub(in crate::compute) fn 計算結果を書き込む(&mut self, キー: &str, 値: V) {
        self.現在値.insert(キー.to_string(), 値);
        self.未再計算.remove(キー);
    }
}
