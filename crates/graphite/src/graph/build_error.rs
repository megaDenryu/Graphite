//! 構築時に検出する失敗 (キー重複・未知端点) の型と、その表示文言を所有する。

use std::error::Error as StdError;
use std::fmt;

/// [`crate::Graph::build`] / [`crate::Graph::create`] が返しうる構築エラー。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError<K> {
    /// ノード定義でキーが重複した。
    DuplicateKey(K),
    /// 辺が未知のキーを端点として参照している。
    /// `missing` は `from`/`to` のうちどちらが未定義だったかを示す。
    UnknownEndpoint {
        /// 辺の始点として書かれたキー。
        from: K,
        /// 辺の終点として書かれたキー。
        to: K,
        /// `from`/`to` のうち、ノード定義に無かった側のキー。
        missing: K,
    },
}

impl<K: fmt::Debug> fmt::Display for GraphError<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GraphError::DuplicateKey(k) => write!(f, "ノードキーが重複しています: {k:?}"),
            GraphError::UnknownEndpoint { from, to, missing } => write!(
                f,
                "辺 {from:?} -> {to:?} が未知のキー {missing:?} を参照しています"
            ),
        }
    }
}

impl<K: fmt::Debug> StdError for GraphError<K> {}
