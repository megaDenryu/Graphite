//! 強連結成分から単純閉路を1本切り出す反復DFSと、その作業状態を所有する。

use std::collections::{HashMap, HashSet};

use super::cycle_search::閉路の位置列;
use super::position::ノード位置;
use super::有向トポロジー;

/// 強連結成分の中で反復 DFS を進める途中状態。経路・経路上の順番・訪問済み・
/// 未処理の枝という4つの作業用集合をこの型がまとめて所有する。
pub(in crate::graph::topology) struct 単純閉路の切り出し<'t, N, E> {
    トポロジー: &'t 有向トポロジー<N, E>,
    成分の集合: HashSet<ノード位置>,
    経路: Vec<ノード位置>,
    経路上の順番: HashMap<ノード位置, usize>,
    訪問済み: HashSet<ノード位置>,
    未処理の枝: Vec<(ノード位置, std::vec::IntoIter<ノード位置>)>,
}

impl<'t, N, E> 単純閉路の切り出し<'t, N, E> {
    /// 成分の先頭を始点にして探索を初期化する。成分は要素数2以上で、循環を
    /// 含むことが呼び出し側で保証されている。
    pub(in crate::graph::topology) fn 強連結成分から始める(
        トポロジー: &'t 有向トポロジー<N, E>,
        成分: &[ノード位置],
    ) -> Self {
        let 始点 = 成分[0];
        let mut 切り出し = Self {
            トポロジー,
            成分の集合: 成分.iter().copied().collect(),
            経路: vec![始点],
            経路上の順番: HashMap::new(),
            訪問済み: HashSet::new(),
            未処理の枝: Vec::new(),
        };
        切り出し.経路上の順番.insert(始点, 0);
        切り出し.訪問済み.insert(始点);
        let 行き先 = 切り出し.成分内の行き先(始点);
        切り出し.未処理の枝.push((始点, 行き先.into_iter()));
        切り出し
    }

    /// 逆辺 (経路上のノードへ戻る辺) を見つけた時点で、その宛先から現在の
    /// ノードまでの経路を切り出す。これがそのまま単純閉路になる。
    pub(in crate::graph::topology) fn 閉路を1本得る(mut self) -> 閉路の位置列 {
        while let Some((_, 残り)) = self.未処理の枝.last_mut() {
            let 次の行き先 = 残り.next();
            match 次の行き先 {
                Some(次) => {
                    if let Some(&順番) = self.経路上の順番.get(&次) {
                        return 閉路の位置列::位置列から生成する(
                            self.経路[順番..].to_vec(),
                        );
                    }
                    if self.訪問済み.insert(次) {
                        self.経路.push(次);
                        self.経路上の順番.insert(次, self.経路.len() - 1);
                        let 行き先 = self.成分内の行き先(次);
                        self.未処理の枝.push((次, 行き先.into_iter()));
                    }
                }
                None => {
                    if let Some((位置, _)) = self.未処理の枝.pop() {
                        self.経路.pop();
                        self.経路上の順番.remove(&位置);
                    }
                }
            }
        }
        unreachable!("要素数2以上の強連結成分は必ず閉路を含む")
    }

    fn 成分内の行き先(&self, 位置: ノード位置) -> Vec<ノード位置> {
        self.トポロジー
            .出ていく先(位置)
            .filter(|隣| self.成分の集合.contains(隣))
            .collect()
    }
}
