//! 異なる `Graph` 由来の参照を混ぜたという契約違反そのものを表す型を所有する。

use std::fmt;

/// 異なる `Graph` から得た参照を1つの検索へ渡したことを表す。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraphMismatch;

impl fmt::Display for GraphMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("異なる Graph の値から得た参照は組み合わせられません。同じ graph! または同じ Graph の値から得た参照だけを渡してください")
    }
}

impl std::error::Error for GraphMismatch {}
