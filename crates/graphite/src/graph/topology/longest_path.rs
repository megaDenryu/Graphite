//! ノード重み付き最長経路の動的計画法 — 距離と先行ノードの表を所有する。

use std::collections::HashMap;

use super::position::ノード位置;
use super::有向トポロジー;

/// 最長経路の算出。トポロジカル順序に沿って
/// `距離[v] = max(距離[v], 距離[u] + 重み[v])` (`u -> v` の辺ごと) と緩和していく
/// DAG 上の動的計画法であり、途中の距離表と先行ノード表を所有する。
pub(in crate::graph) struct 最長経路の算出<'t, N, E, W> {
    トポロジー: &'t 有向トポロジー<N, E>,
    順序: Vec<ノード位置>,
    重み: HashMap<ノード位置, W>,
    距離: HashMap<ノード位置, W>,
    先行: HashMap<ノード位置, ノード位置>,
}

impl<'t, N, E, W> 最長経路の算出<'t, N, E, W>
where
    W: Ord + Copy + Default + std::ops::Add<Output = W>,
{
    pub(in crate::graph) fn トポロジカル順序と重みから始める(
        トポロジー: &'t 有向トポロジー<N, E>,
        順序: Vec<ノード位置>,
        重み: HashMap<ノード位置, W>,
    ) -> Self {
        let 距離 = 順序.iter().map(|&位置| (位置, 重み[&位置])).collect();
        Self {
            トポロジー,
            順序,
            重み,
            距離,
            先行: HashMap::new(),
        }
    }

    /// 最長経路の位置列と、その経路上のノード重みの総和。ノードが1つも無ければ
    /// 空の経路と `W::default()` を返す。
    pub(in crate::graph) fn 経路と総和を求める(mut self) -> (Vec<ノード位置>, W) {
        if self.順序.is_empty() {
            return (Vec::new(), W::default());
        }
        self.トポロジカル順に緩和する();
        let 終点 = self.最長の終点();
        (self.終点から経路を遡る(終点), self.距離[&終点])
    }

    fn トポロジカル順に緩和する(&mut self) {
        let トポロジー = self.トポロジー;
        for 添字 in 0..self.順序.len() {
            let 位置 = self.順序[添字];
            let 現在の距離 = self.距離[&位置];
            for 次 in トポロジー.出ていく先(位置) {
                let 候補 = 現在の距離 + self.重み[&次];
                if 候補 > self.距離[&次] {
                    self.距離.insert(次, 候補);
                    self.先行.insert(次, 位置);
                }
            }
        }
    }

    fn 最長の終点(&self) -> ノード位置 {
        *self
            .順序
            .iter()
            .max_by_key(|&&位置| self.距離[&位置])
            .expect("順序が空でないことを呼び出し側で確認済み")
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
