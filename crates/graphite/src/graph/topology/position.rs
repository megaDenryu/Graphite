//! トポロジー内でノードを指す位置を所有する。
//!
//! 位置は `petgraph` の `NodeIndex` をそのまま包むだけの値であり、追加費用は無い。
//! この newtype があることで、キーの世界 (`crate::graph` 直下) は `petgraph` の型を
//! 名指しせずに位置を運べる。

use std::collections::HashMap;

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

/// トポロジーを作り直すときに、変換前の位置と変換後の位置を対応づける表。
/// 写されなかったノードは載らないため、辺の両端が残ったかの判定も兼ねる。
pub(in crate::graph::topology) struct 位置対応表 {
    対応: HashMap<ノード位置, ノード位置>,
}

impl 位置対応表 {
    pub(in crate::graph::topology) fn 空の対応表を生成する() -> Self {
        Self {
            対応: HashMap::new(),
        }
    }

    pub(in crate::graph::topology) fn 対応を記録する(
        &mut self,
        変換前: ノード位置,
        変換後: ノード位置,
    ) {
        self.対応.insert(変換前, 変換後);
    }

    pub(in crate::graph::topology) fn 変換後の位置(
        &self,
        変換前: ノード位置,
    ) -> Option<ノード位置> {
        self.対応.get(&変換前).copied()
    }
}
