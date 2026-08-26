//! 構築中のグラフ — 検査しながらトポロジーとキー対応表を同時に組み立てる途中状態を所有する。
//!
//! 完成した [`Graph`] が持ってはならない「まだ検査を通っていない」状態をこの型が
//! 引き受け、検査を全部通った値だけを [`構築中のグラフ::完成させる`] から返す。

use std::hash::Hash;

use super::build_error::GraphError;
use super::key_correspondence::キー対応表;
use super::topology::有向トポロジー;
use super::Graph;

/// 検査の途中にあるグラフ。トポロジーとキー対応表を対で育てる。
pub(in crate::graph) struct 構築中のグラフ<N, E, K> {
    トポロジー: 有向トポロジー<N, E>,
    キー対応: キー対応表<K>,
}

impl<N, E, K> 構築中のグラフ<N, E, K>
where
    K: Hash + Eq + Clone,
{
    pub(in crate::graph) fn 空のグラフから始める() -> Self {
        Self {
            トポロジー: 有向トポロジー::空のトポロジーを生成する(),
            キー対応: キー対応表::空の対応表を生成する(),
        }
    }

    /// `(キー, ノード値)` の列と `(始点キー, 終点キー, 辺値)` の列を、渡された
    /// 順に検査しながら積んで完成させる。列は中間の `Vec` へ写さずそのまま消費する。
    pub(in crate::graph) fn ノード列と辺列から組み立てる(
        ノード列: impl IntoIterator<Item = (K, N)>,
        辺列: impl IntoIterator<Item = (K, K, E)>,
    ) -> Result<Graph<N, E, K>, GraphError<K>> {
        let mut 組み立て = Self::空のグラフから始める();
        for (キー, 値) in ノード列 {
            組み立て.ノードを検査して積む(キー, 値)?;
        }
        for (始点キー, 終点キー, 値) in 辺列 {
            組み立て.辺を検査して積む(始点キー, 終点キー, 値)?;
        }
        Ok(組み立て.完成させる())
    }

    /// ノードを1つ積む。キーが既に登録済みなら重複として拒む。
    fn ノードを検査して積む(&mut self, キー: K, 値: N) -> Result<(), GraphError<K>> {
        if self.キー対応.キーが登録済みか(&キー) {
            return Err(GraphError::DuplicateKey(キー));
        }
        let 位置 = self.トポロジー.ノードを追加する(値);
        self.キー対応.対応を登録する(キー, 位置);
        Ok(())
    }

    /// 辺を1つ積む。端点キーが未登録なら未知端点として拒む。始点を先に検査する
    /// ため、両端とも未登録なら始点が `missing` として報告される。
    fn 辺を検査して積む(
        &mut self,
        始点キー: K,
        終点キー: K,
        値: E,
    ) -> Result<(), GraphError<K>> {
        let Some(始点) = self.キー対応.位置(&始点キー) else {
            return Err(GraphError::UnknownEndpoint {
                missing: 始点キー.clone(),
                from: 始点キー,
                to: 終点キー,
            });
        };
        let Some(終点) = self.キー対応.位置(&終点キー) else {
            return Err(GraphError::UnknownEndpoint {
                missing: 終点キー.clone(),
                from: 始点キー,
                to: 終点キー,
            });
        };
        self.トポロジー.辺を追加する(始点, 終点, 値);
        Ok(())
    }

    /// 検査を全部通った部品から、完成した不変のグラフを作る。
    pub(in crate::graph) fn 完成させる(self) -> Graph<N, E, K> {
        Graph::部品から組み立てる(self.トポロジー, self.キー対応)
    }
}
