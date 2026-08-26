//! 依存レベル (波) への分割 — レベル内の順序が挿入順であることを保証する。

use std::collections::{HashMap, HashSet};

use super::position::ノード位置;
use super::有向トポロジー;

/// 依存レベルの分割。挿入順の位置列・残りの入次数・まだ確定していない位置の
/// 集合という3つの作業状態をこの型が所有する。
pub(in crate::graph) struct 依存レベルの分割<'t, N, E> {
    トポロジー: &'t 有向トポロジー<N, E>,
    挿入順: Vec<ノード位置>,
    入次数: HashMap<ノード位置, usize>,
    未確定: HashSet<ノード位置>,
}

impl<'t, N, E> 依存レベルの分割<'t, N, E> {
    /// 前提: 循環がないことを呼び出し側が先に確かめている。
    pub(in crate::graph) fn トポロジーから始める(
        トポロジー: &'t 有向トポロジー<N, E>,
    ) -> Self {
        let 挿入順 = トポロジー.挿入順の位置列();
        let 入次数 = 挿入順
            .iter()
            .map(|&位置| (位置, トポロジー.入ってくる元(位置).count()))
            .collect();
        let 未確定 = 挿入順.iter().copied().collect();
        Self {
            トポロジー,
            挿入順,
            入次数,
            未確定,
        }
    }

    /// まだ処理していない先行ノードを持たないノードの集合を、順に切り出す。
    /// レベルの中の順序はノードの挿入順で決定的になる。
    pub(in crate::graph) fn レベル列を求める(mut self) -> Vec<Vec<ノード位置>> {
        let mut レベル列: Vec<Vec<ノード位置>> = Vec::new();
        while !self.未確定.is_empty() {
            let 現在のレベル = self.先行を持たない未確定の位置列();
            debug_assert!(
                !現在のレベル.is_empty(),
                "循環なしを確認済みなのでフロンティアが空になることはない"
            );
            self.現在のレベルを確定する(&現在のレベル);
            レベル列.push(現在のレベル);
        }
        レベル列
    }

    fn 先行を持たない未確定の位置列(&self) -> Vec<ノード位置> {
        self.挿入順
            .iter()
            .copied()
            .filter(|位置| self.未確定.contains(位置) && self.入次数[位置] == 0)
            .collect()
    }

    fn 現在のレベルを確定する(&mut self, 現在のレベル: &[ノード位置]) {
        let トポロジー = self.トポロジー;
        for &位置 in 現在のレベル {
            self.未確定.remove(&位置);
        }
        for &位置 in 現在のレベル {
            for 次 in トポロジー.出ていく先(位置) {
                if let Some(残り) = self.入次数.get_mut(&次) {
                    *残り = 残り.saturating_sub(1);
                }
            }
        }
    }
}
