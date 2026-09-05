//! 循環検出時に返す閉路の型と、その表示文言を所有する。

use std::error::Error as StdError;
use std::fmt;

/// [`crate::Graph::topological_sort`] / [`crate::Graph::topological_levels`] /
/// [`crate::Graph::critical_path_by`] が循環検出時に返すエラー。
///
/// `cycle` は循環を構成するノードキーの列。`cycle[0]` から `cycle[1]`、
/// ...、`cycle[last]` から `cycle[0]` へと辺を辿って戻ってこられる
/// (閉路になっている) ことを保証する。自己ループの場合は `cycle` は
/// 要素数 1 (`cycle[0]` 自身への辺)。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CycleError<K> {
    /// 循環を構成するノードキーの列。
    pub cycle: Vec<K>,
}

impl<K: fmt::Debug> fmt::Display for CycleError<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "グラフに循環があります: ")?;
        for (i, k) in self.cycle.iter().enumerate() {
            if i > 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{k:?}")?;
        }
        if let Some(first) = self.cycle.first() {
            write!(f, " -> {first:?}")?;
        }
        Ok(())
    }
}

impl<K: fmt::Debug> StdError for CycleError<K> {}
