//! トポロジー内でノードを指す位置を所有する。
//!
//! 位置は `petgraph` の `NodeIndex` をそのまま包むだけの値であり、追加費用は無い。
//! この newtype があることで、キーの世界 (`crate::graph` 直下) は `petgraph` の型を
//! 名指しせずに位置を運べる。

use petgraph::graph::NodeIndex;

/// トポロジーの中でノードを指す位置。`petgraph` の `NodeIndex` を包む。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub(in crate::graph) struct ノード位置(NodeIndex);

impl ノード位置 {
    pub(in crate::graph::topology) fn 内部添字から生成する(添字: NodeIndex) -> Self {
        Self(添字)
    }

    pub(in crate::graph::topology) fn 内部添字(self) -> NodeIndex {
        self.0
    }
}
