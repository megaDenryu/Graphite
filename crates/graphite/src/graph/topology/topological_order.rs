//! 全体のトポロジカル順序を求め、循環していたら閉路の探索へ切り替える。

use super::cycle_search::{循環の探索, 閉路の位置列};
use super::position::ノード位置;
use super::有向トポロジー;

/// トポロジカル順序の算出。トポロジーを借りて、依存の順に並べた位置列を求める。
pub(in crate::graph) struct トポロジカル順序の算出<'t, N, E> {
    トポロジー: &'t 有向トポロジー<N, E>,
}

impl<'t, N, E> トポロジカル順序の算出<'t, N, E> {
    pub(in crate::graph) fn トポロジーから始める(
        トポロジー: &'t 有向トポロジー<N, E>,
    ) -> Self {
        Self { トポロジー }
    }

    /// 依存の順に並べた位置列。循環しているときは閉路を1本探して返す。
    pub(in crate::graph) fn 順序を求める(
        &self,
    ) -> Result<Vec<ノード位置>, 閉路の位置列> {
        match petgraph::algo::toposort(self.トポロジー.内部グラフ(), None) {
            Ok(順序) => Ok(順序
                .into_iter()
                .map(ノード位置::内部添字から生成する)
                .collect()),
            Err(_) => Err(循環の探索::トポロジーから始める(self.トポロジー)
                .閉路を1本探す()
                .expect("toposortが失敗したので循環が存在するはず")),
        }
    }
}
