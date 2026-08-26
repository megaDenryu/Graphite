//! 始点から辿る走査 — 深さ優先の到達集合と、幅優先での辺数最短の経路を所有する。

use std::collections::{HashMap, HashSet, VecDeque};

use super::position::ノード位置;
use super::有向トポロジー;

/// 到達可能な位置の収集。始点自身を含む反射的な到達集合を深さ優先で集める。
pub(in crate::graph) struct 到達可能な位置の収集<'t, N, E> {
    トポロジー: &'t 有向トポロジー<N, E>,
}

impl<'t, N, E> 到達可能な位置の収集<'t, N, E> {
    pub(in crate::graph) fn トポロジーから始める(
        トポロジー: &'t 有向トポロジー<N, E>,
    ) -> Self {
        Self { トポロジー }
    }

    pub(in crate::graph) fn 始点から到達できる位置列(
        &self,
        始点: ノード位置,
    ) -> Vec<ノード位置> {
        let mut 探索 = petgraph::visit::Dfs::new(self.トポロジー.内部グラフ(), 始点.内部添字());
        let mut 結果 = Vec::new();
        while let Some(添字) = 探索.next(self.トポロジー.内部グラフ()) {
            結果.push(ノード位置::内部添字から生成する(添字));
        }
        結果
    }
}

/// 辺数最短の経路探索。幅優先探索の訪問済み集合・待ち行列・先行ノード表を所有する。
pub(in crate::graph) struct 辺数最短の経路探索<'t, N, E> {
    トポロジー: &'t 有向トポロジー<N, E>,
    訪問済み: HashSet<ノード位置>,
    待ち行列: VecDeque<ノード位置>,
    先行: HashMap<ノード位置, ノード位置>,
}

impl<'t, N, E> 辺数最短の経路探索<'t, N, E> {
    pub(in crate::graph) fn トポロジーから始める(
        トポロジー: &'t 有向トポロジー<N, E>,
    ) -> Self {
        Self {
            トポロジー,
            訪問済み: HashSet::new(),
            待ち行列: VecDeque::new(),
            先行: HashMap::new(),
        }
    }

    /// 始点から終点への辺数最短の経路。到達できなければ `None`。始点と終点が
    /// 同じときは始点1つだけの経路を返す。
    pub(in crate::graph) fn 経路を求める(
        mut self,
        始点: ノード位置,
        終点: ノード位置,
    ) -> Option<Vec<ノード位置>> {
        if 始点 == 終点 {
            return Some(vec![始点]);
        }

        let トポロジー = self.トポロジー;
        self.訪問済み.insert(始点);
        self.待ち行列.push_back(始点);

        while let Some(現在) = self.待ち行列.pop_front() {
            for 次 in トポロジー.出ていく先(現在) {
                if self.訪問済み.insert(次) {
                    self.先行.insert(次, 現在);
                    if 次 == 終点 {
                        return Some(self.終点から経路を遡る(終点));
                    }
                    self.待ち行列.push_back(次);
                }
            }
        }
        None
    }

    fn 終点から経路を遡る(&self, 終点: ノード位置) -> Vec<ノード位置> {
        let mut 経路 = vec![終点];
        let mut 遡り = 終点;
        while let Some(&手前) = self.先行.get(&遡り) {
            経路.push(手前);
            遡り = 手前;
        }
        経路.reverse();
        経路
    }
}
