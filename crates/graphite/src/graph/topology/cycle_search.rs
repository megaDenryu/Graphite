//! 循環の有無判定と、閉路を含む強連結成分の選別を所有する。

use super::position::ノード位置;
use super::simple_cycle_extraction::単純閉路の切り出し;
use super::有向トポロジー;

/// 閉路を構成するノード位置の列。先頭から順に辺を辿って末尾まで進み、末尾から
/// 先頭へ戻れることを表す。要素数1のときは自己ループを表す。
pub(in crate::graph) struct 閉路の位置列(Vec<ノード位置>);

impl 閉路の位置列 {
    pub(in crate::graph::topology) fn 位置列から生成する(
        位置列: Vec<ノード位置>
    ) -> Self {
        Self(位置列)
    }

    pub(in crate::graph) fn 位置の並び(&self) -> &[ノード位置] {
        &self.0
    }
}

/// 循環の探索。トポロジーを借りて、循環の有無と閉路1本を答える。
pub(in crate::graph) struct 循環の探索<'t, N, E> {
    トポロジー: &'t 有向トポロジー<N, E>,
}

impl<'t, N, E> 循環の探索<'t, N, E> {
    pub(in crate::graph) fn トポロジーから始める(
        トポロジー: &'t 有向トポロジー<N, E>,
    ) -> Self {
        Self { トポロジー }
    }

    pub(in crate::graph) fn 循環があるか(&self) -> bool {
        petgraph::algo::is_cyclic_directed(self.トポロジー.内部グラフ())
    }

    /// 循環を1つ探して閉路の位置列で返す (循環がなければ `None`)。
    ///
    /// 強連結成分を求め、要素数が 2 以上の成分 (=循環を含む) か、要素数 1 かつ
    /// 自己ループを持つ成分を探す。要素数 2 以上の成分からは単純閉路を切り出す。
    pub(in crate::graph) fn 閉路を1本探す(&self) -> Option<閉路の位置列> {
        for 成分 in self.強連結成分の一覧() {
            if 成分.len() > 1 {
                return Some(
                    単純閉路の切り出し::強連結成分から始める(
                        self.トポロジー,
                        &成分,
                    )
                    .閉路を1本得る(),
                );
            }
            if 成分.len() == 1 {
                let 位置 = 成分[0];
                if self.トポロジー.辺があるか(位置, 位置) {
                    return Some(閉路の位置列::位置列から生成する(vec![位置]));
                }
            }
        }
        None
    }

    fn 強連結成分の一覧(&self) -> Vec<Vec<ノード位置>> {
        petgraph::algo::tarjan_scc(self.トポロジー.内部グラフ())
            .into_iter()
            .map(|成分| {
                成分
                    .into_iter()
                    .map(ノード位置::内部添字から生成する)
                    .collect()
            })
            .collect()
    }
}
