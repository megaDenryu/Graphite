// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/schema_ids.rs:70
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    15273406592076785782u64, 4319742417705813317u64, 13840474139917085728u64,
    14537176934756227492u64,
];
/// `AutomaticNode` ノードの公開ID。
///
/// 宣言: `tests/schema_ids.rs` の `node AutomaticNode`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutomaticNodeId(pub String);
/// `AutomaticLink` 辺の公開ID。
///
/// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutomaticLinkId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ExternalNodeInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __AutomaticNodeInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __BooleanNodeInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ExternalLinkInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ExternalIncomingInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ExternalFriendInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __AutomaticLinkInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ExternalNodeNamedPosition(__ExternalNodeInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __AutomaticNodeNamedPosition(__AutomaticNodeInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __BooleanNodeNamedPosition(__BooleanNodeInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ExternalLinkNamedPosition(__ExternalLinkInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ExternalIncomingNamedPosition(__ExternalIncomingInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ExternalFriendNamedPosition(__ExternalFriendInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __AutomaticLinkNamedPosition(__AutomaticLinkInternalPosition, u64);
/// 構築時に組み立てる `ExternalLink` 辺の値。
///
/// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
#[derive(Clone, PartialEq)]
pub struct ExternalLink {
    pub source: ExternalNodeId,
    pub target: ExternalNodeId,
}
impl ExternalLink {
    pub fn new(from: ExternalNodeId, to: ExternalNodeId) -> Self {
        Self { source: from, target: to }
    }
}
impl graphite::DirectedEdgeLiteral<ExternalNodeId, ExternalNodeId, ()> for ExternalLink {
    fn from_graph_literal(from: ExternalNodeId, to: ExternalNodeId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for ExternalLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ExternalLink))
    }
}
/// 構築時に組み立てる `ExternalIncoming` 辺の値。
///
/// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
#[derive(Clone, PartialEq)]
pub struct ExternalIncoming {
    pub source: ExternalNodeId,
    pub target: ExternalNodeId,
}
impl ExternalIncoming {
    pub fn new(from: ExternalNodeId, to: ExternalNodeId) -> Self {
        Self { source: from, target: to }
    }
}
impl graphite::DirectedEdgeLiteral<ExternalNodeId, ExternalNodeId, ()>
for ExternalIncoming {
    fn from_graph_literal(from: ExternalNodeId, to: ExternalNodeId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for ExternalIncoming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ExternalIncoming))
    }
}
/// 構築時に組み立てる `ExternalFriend` 辺の値。
///
/// 宣言: `tests/schema_ids.rs` の `edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode`
#[derive(Clone, PartialEq)]
pub struct ExternalFriend {
    endpoints: graphite::UnorderedPair<ExternalNodeId>,
}
impl ExternalFriend {
    pub fn new(a: ExternalNodeId, b: ExternalNodeId) -> Self {
        Self {
            endpoints: graphite::UnorderedPair::new(a, b),
        }
    }
    pub fn endpoints(&self) -> (&ExternalNodeId, &ExternalNodeId) {
        self.endpoints.endpoints()
    }
}
impl graphite::UndirectedEdgeLiteral<ExternalNodeId, ()> for ExternalFriend {
    fn from_graph_literal(a: ExternalNodeId, b: ExternalNodeId, (): ()) -> Self {
        Self::new(a, b)
    }
}
impl std::fmt::Debug for ExternalFriend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ExternalFriend))
    }
}
/// 構築時に組み立てる `AutomaticLink` 辺の値。
///
/// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
#[derive(Clone, PartialEq)]
pub struct AutomaticLink {
    pub source: AutomaticNodeId,
    pub target: AutomaticNodeId,
}
impl AutomaticLink {
    pub fn new(from: AutomaticNodeId, to: AutomaticNodeId) -> Self {
        Self { source: from, target: to }
    }
}
impl graphite::DirectedEdgeLiteral<AutomaticNodeId, AutomaticNodeId, ()>
for AutomaticLink {
    fn from_graph_literal(from: AutomaticNodeId, to: AutomaticNodeId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for AutomaticLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(AutomaticLink))
            .field(&self.source)
            .field(&self.target)
            .finish()
    }
}
#[allow(dead_code)]
struct __ExternalLinkRecord {
    source: __ExternalNodeInternalPosition,
    target: __ExternalNodeInternalPosition,
}
#[allow(dead_code)]
struct __ExternalIncomingRecord {
    source: __ExternalNodeInternalPosition,
    target: __ExternalNodeInternalPosition,
}
#[allow(dead_code)]
struct __ExternalFriendRecord {
    endpoints: graphite::UnorderedPair<__ExternalNodeInternalPosition>,
}
#[allow(dead_code)]
struct __AutomaticLinkRecord {
    source: __AutomaticNodeInternalPosition,
    target: __AutomaticNodeInternalPosition,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `tests/schema_ids.rs` の `schema MixedIds`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    DuplicateExternalNode(ExternalNodeId),
    DuplicateAutomaticNode(AutomaticNodeId),
    DuplicateBooleanNode(bool),
    /// このエッジ種別のキーが重複している。
    ExternalLinkDuplicateKey(ExternalEdgeId),
    /// このエッジが未知の始点キーを参照している。
    ExternalLinkUnknownSource { edge: ExternalEdgeId, source: ExternalNodeId },
    /// このエッジが未知の終点キーを参照している。
    ExternalLinkUnknownTarget { edge: ExternalEdgeId, target: ExternalNodeId },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    ExternalLinkSourceEachViolation { source: ExternalNodeId, count: usize },
    /// このエッジ種別のキーが重複している。
    ExternalIncomingDuplicateKey(ExternalEdgeId),
    /// このエッジが未知の始点キーを参照している。
    ExternalIncomingUnknownSource { edge: ExternalEdgeId, source: ExternalNodeId },
    /// このエッジが未知の終点キーを参照している。
    ExternalIncomingUnknownTarget { edge: ExternalEdgeId, target: ExternalNodeId },
    /// このエッジ種別の `each` 制約違反 (入次数)。
    ExternalIncomingTargetEachViolation { target: ExternalNodeId, count: usize },
    /// このエッジ種別のキーが重複している。
    ExternalFriendDuplicateKey(ExternalEdgeId),
    /// このエッジが未知の端点キーを参照している (無向のため位置の
    /// 区別は無い)。
    ExternalFriendUnknownEndpoint { edge: ExternalEdgeId, endpoint: ExternalNodeId },
    /// このエッジ種別のキーが重複している。
    AutomaticLinkDuplicateKey(AutomaticLinkId),
    /// このエッジが未知の始点キーを参照している。
    AutomaticLinkUnknownSource { edge: AutomaticLinkId, source: AutomaticNodeId },
    /// このエッジが未知の終点キーを参照している。
    AutomaticLinkUnknownTarget { edge: AutomaticLinkId, target: AutomaticNodeId },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicateExternalNode(_) => {
                write!(f, "{}のキーが重複しています", "ExternalNode")
            }
            Violation::DuplicateAutomaticNode(id) => {
                write!(
                    f, "{}のキーが重複しています: {:?}", "AutomaticNode", id
                )
            }
            Violation::DuplicateBooleanNode(_) => {
                write!(f, "{}のキーが重複しています", "BooleanNode")
            }
            Violation::ExternalLinkDuplicateKey(_) => {
                write!(f, "{}のキーが重複しています", "ExternalLink")
            }
            Violation::ExternalLinkUnknownSource { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の始点, {})",
                    "ExternalLink", "ExternalNode"
                )
            }
            Violation::ExternalLinkUnknownTarget { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の終点, {})",
                    "ExternalLink", "ExternalNode"
                )
            }
            Violation::ExternalLinkSourceEachViolation { count, .. } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} の出次数 {} を期待しますが実際は {} 本です",
                    "ExternalLink", "ExternalNode", "ちょうど1", count
                )
            }
            Violation::ExternalIncomingDuplicateKey(_) => {
                write!(f, "{}のキーが重複しています", "ExternalIncoming")
            }
            Violation::ExternalIncomingUnknownSource { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の始点, {})",
                    "ExternalIncoming", "ExternalNode"
                )
            }
            Violation::ExternalIncomingUnknownTarget { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の終点, {})",
                    "ExternalIncoming", "ExternalNode"
                )
            }
            Violation::ExternalIncomingTargetEachViolation { count, .. } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} の入次数 {} を期待しますが実際は {} 本です",
                    "ExternalIncoming", "ExternalNode", "ちょうど1", count
                )
            }
            Violation::ExternalFriendDuplicateKey(_) => {
                write!(f, "{}のキーが重複しています", "ExternalFriend")
            }
            Violation::ExternalFriendUnknownEndpoint { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の端点, {})",
                    "ExternalFriend", "ExternalNode"
                )
            }
            Violation::AutomaticLinkDuplicateKey(id) => {
                write!(
                    f, "{}のキーが重複しています: {:?}", "AutomaticLink", id
                )
            }
            Violation::AutomaticLinkUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "AutomaticLink", edge, "AutomaticNode", source
                )
            }
            Violation::AutomaticLinkUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "AutomaticLink", edge, "AutomaticNode", target
                )
            }
        }
    }
}
impl std::fmt::Debug for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl std::error::Error for Violation {}
/// 凍結済み図式グラフ。構築後の構造は不変で、ノード値と辺の積み荷だけを
/// `&mut Graph` を要求する種別APIから更新できる。
///
/// 宣言: `tests/schema_ids.rs` の `schema MixedIds`
pub struct Graph {
    __graphite_node_external_node: graphite::KeyedTable<
        ExternalNodeId,
        super::ExternalNode,
    >,
    __graphite_node_automatic_node: graphite::KeyedTable<
        AutomaticNodeId,
        super::AutomaticNode,
    >,
    __graphite_node_boolean_node: graphite::KeyedTable<bool, super::BooleanNode>,
    external_link: graphite::KeyedTable<ExternalEdgeId, __ExternalLinkRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    external_link_from_index: graphite::ExactlyOneRoleIndex<
        __ExternalLinkInternalPosition,
    >,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    external_link_to_index: graphite::MultipleRoleIndex<__ExternalLinkInternalPosition>,
    __graphite_external_link_by_pair: std::collections::HashMap<
        (__ExternalNodeInternalPosition, __ExternalNodeInternalPosition),
        Vec<__ExternalLinkInternalPosition>,
    >,
    external_incoming: graphite::KeyedTable<ExternalEdgeId, __ExternalIncomingRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    external_incoming_from_index: graphite::MultipleRoleIndex<
        __ExternalIncomingInternalPosition,
    >,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    external_incoming_to_index: graphite::ExactlyOneRoleIndex<
        __ExternalIncomingInternalPosition,
    >,
    __graphite_external_incoming_by_pair: std::collections::HashMap<
        (__ExternalNodeInternalPosition, __ExternalNodeInternalPosition),
        Vec<__ExternalIncomingInternalPosition>,
    >,
    external_friend: graphite::KeyedTable<ExternalEdgeId, __ExternalFriendRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    external_friend_index: graphite::MultipleRoleIndex<__ExternalFriendInternalPosition>,
    __graphite_external_friend_by_pair: std::collections::HashMap<
        graphite::UnorderedPair<__ExternalNodeInternalPosition>,
        Vec<__ExternalFriendInternalPosition>,
    >,
    automatic_link: graphite::KeyedTable<AutomaticLinkId, __AutomaticLinkRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    automatic_link_from_index: graphite::MultipleRoleIndex<
        __AutomaticLinkInternalPosition,
    >,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    automatic_link_to_index: graphite::MultipleRoleIndex<
        __AutomaticLinkInternalPosition,
    >,
    __graphite_automatic_link_by_pair: std::collections::HashMap<
        (__AutomaticNodeInternalPosition, __AutomaticNodeInternalPosition),
        Vec<__AutomaticLinkInternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node ExternalNode(id: ExternalNodeId)`
    pub fn external_node_by_id<'graph>(
        &'graph self,
        id: &ExternalNodeId,
    ) -> Option<ExternalNodeRef<'graph>> {
        let internal_position = __ExternalNodeInternalPosition(
            self.__graphite_node_external_node.position(id)?,
        );
        Some(ExternalNodeRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node ExternalNode(id: ExternalNodeId)`
    pub fn external_node_value_mut(
        &mut self,
        id: &ExternalNodeId,
    ) -> Option<&mut super::ExternalNode> {
        self.__graphite_node_external_node.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node ExternalNode(id: ExternalNodeId)`
    pub fn external_node_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph ExternalNodeId> {
        self.__graphite_node_external_node.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node ExternalNode(id: ExternalNodeId)`
    pub fn external_node_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ExternalNodeRef<'graph>> + 'graph {
        self.__graphite_node_external_node
            .positions()
            .map(move |position| ExternalNodeRef {
                graph: self,
                internal_position: __ExternalNodeInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node ExternalNode(id: ExternalNodeId)`
    pub fn external_node_len(&self) -> usize {
        self.__graphite_node_external_node.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node AutomaticNode`
    pub fn automatic_node_by_id<'graph>(
        &'graph self,
        id: &AutomaticNodeId,
    ) -> Option<AutomaticNodeRef<'graph>> {
        let internal_position = __AutomaticNodeInternalPosition(
            self.__graphite_node_automatic_node.position(id)?,
        );
        Some(AutomaticNodeRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node AutomaticNode`
    pub fn automatic_node_value_mut(
        &mut self,
        id: &AutomaticNodeId,
    ) -> Option<&mut super::AutomaticNode> {
        self.__graphite_node_automatic_node.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node AutomaticNode`
    pub fn automatic_node_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph AutomaticNodeId> {
        self.__graphite_node_automatic_node.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node AutomaticNode`
    pub fn automatic_node_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = AutomaticNodeRef<'graph>> + 'graph {
        self.__graphite_node_automatic_node
            .positions()
            .map(move |position| AutomaticNodeRef {
                graph: self,
                internal_position: __AutomaticNodeInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node AutomaticNode`
    pub fn automatic_node_len(&self) -> usize {
        self.__graphite_node_automatic_node.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node BooleanNode(id: bool)`
    pub fn boolean_node_by_id<'graph>(
        &'graph self,
        id: &bool,
    ) -> Option<BooleanNodeRef<'graph>> {
        let internal_position = __BooleanNodeInternalPosition(
            self.__graphite_node_boolean_node.position(id)?,
        );
        Some(BooleanNodeRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node BooleanNode(id: bool)`
    pub fn boolean_node_value_mut(
        &mut self,
        id: &bool,
    ) -> Option<&mut super::BooleanNode> {
        self.__graphite_node_boolean_node.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node BooleanNode(id: bool)`
    pub fn boolean_node_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph bool> {
        self.__graphite_node_boolean_node.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node BooleanNode(id: bool)`
    pub fn boolean_node_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = BooleanNodeRef<'graph>> + 'graph {
        self.__graphite_node_boolean_node
            .positions()
            .map(move |position| BooleanNodeRef {
                graph: self,
                internal_position: __BooleanNodeInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `tests/schema_ids.rs` の `node BooleanNode(id: bool)`
    pub fn boolean_node_len(&self) -> usize {
        self.__graphite_node_boolean_node.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
    pub fn external_link_by_id<'graph>(
        &'graph self,
        id: &ExternalEdgeId,
    ) -> Option<ExternalLinkRef<'graph>> {
        Some(ExternalLinkRef {
            graph: self,
            internal_position: __ExternalLinkInternalPosition(
                self.external_link.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
    pub fn external_link_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph ExternalEdgeId> {
        self.external_link.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
    pub fn external_link_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ExternalLinkRef<'graph>> + 'graph {
        self.external_link
            .positions()
            .map(move |position| ExternalLinkRef {
                graph: self,
                internal_position: __ExternalLinkInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
    pub fn external_link_len(&self) -> usize {
        self.external_link.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
    pub fn external_incoming_by_id<'graph>(
        &'graph self,
        id: &ExternalEdgeId,
    ) -> Option<ExternalIncomingRef<'graph>> {
        Some(ExternalIncomingRef {
            graph: self,
            internal_position: __ExternalIncomingInternalPosition(
                self.external_incoming.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
    pub fn external_incoming_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph ExternalEdgeId> {
        self.external_incoming.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
    pub fn external_incoming_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ExternalIncomingRef<'graph>> + 'graph {
        self.external_incoming
            .positions()
            .map(move |position| ExternalIncomingRef {
                graph: self,
                internal_position: __ExternalIncomingInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
    pub fn external_incoming_len(&self) -> usize {
        self.external_incoming.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode`
    pub fn external_friend_by_id<'graph>(
        &'graph self,
        id: &ExternalEdgeId,
    ) -> Option<ExternalFriendRef<'graph>> {
        Some(ExternalFriendRef {
            graph: self,
            internal_position: __ExternalFriendInternalPosition(
                self.external_friend.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode`
    pub fn external_friend_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph ExternalEdgeId> {
        self.external_friend.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode`
    pub fn external_friend_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ExternalFriendRef<'graph>> + 'graph {
        self.external_friend
            .positions()
            .map(move |position| ExternalFriendRef {
                graph: self,
                internal_position: __ExternalFriendInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode`
    pub fn external_friend_len(&self) -> usize {
        self.external_friend.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
    pub fn automatic_link_by_id<'graph>(
        &'graph self,
        id: &AutomaticLinkId,
    ) -> Option<AutomaticLinkRef<'graph>> {
        Some(AutomaticLinkRef {
            graph: self,
            internal_position: __AutomaticLinkInternalPosition(
                self.automatic_link.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
    pub fn automatic_link_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph AutomaticLinkId> {
        self.automatic_link.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
    pub fn automatic_link_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = AutomaticLinkRef<'graph>> + 'graph {
        self.automatic_link
            .positions()
            .map(move |position| AutomaticLinkRef {
                graph: self,
                internal_position: __AutomaticLinkInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
    pub fn automatic_link_len(&self) -> usize {
        self.automatic_link.len()
    }
    /// builder をクロージャに貸し出し、戻ったら凍結して図式適合
    /// (端点種別・where 制約) を一括検査する。最初の1件の違反で
    /// `Err` になる (複数の違反を全件見たい場合は
    /// [`Self::create_collecting`] を使う)。
    pub fn create<F>(f: F) -> Result<Self, Violation>
    where
        F: for<'b> FnOnce(&'b mut Builder),
    {
        let mut builder = Builder::new();
        f(&mut builder);
        builder.freeze()
    }
    /// `graph!` が名前付き要素の名前付き位置を凍結境界の外へ運ぶための
    /// 内部構築経路。`Graph` の凍結に成功した場合だけ名前付き位置を返す。
    /// [`graphite::build_named_graph`] へ薄く委譲するだけで、
    /// [`graphite::NamedInsertPermit`] はそちらでしか作らない
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn create_named<F, N>(f: F) -> Result<(Self, N), Violation>
    where
        F: for<'b> FnOnce(&'b mut Builder, &'b graphite::NamedInsertPermit) -> N,
    {
        graphite::build_named_graph(Builder::new, f)
    }
    /// [`Self::create`] の複数違反収集版。builder をクロージャに
    /// 貸し出し、戻ったら凍結して図式適合を検査する点は `create` と
    /// 同じだが、最初の1件で打ち切らず全違反を `Vec` に集めて返す。
    pub fn create_collecting<F>(f: F) -> Result<Self, Vec<Violation>>
    where
        F: for<'b> FnOnce(&'b mut Builder),
    {
        let mut builder = Builder::new();
        f(&mut builder);
        builder.freeze_collecting()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
#[derive(Clone, Copy)]
pub struct ExternalLinkRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ExternalLinkInternalPosition,
}
impl<'graph> ExternalLinkRef<'graph> {
    fn record(self) -> &'graph __ExternalLinkRecord {
        self.graph
            .external_link
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph ExternalEdgeId {
        self.graph
            .external_link
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn source(self) -> ExternalNodeRef<'graph> {
        ExternalNodeRef {
            graph: self.graph,
            internal_position: __ExternalNodeInternalPosition(self.record().source.0),
        }
    }
    pub fn target(self) -> ExternalNodeRef<'graph> {
        ExternalNodeRef {
            graph: self.graph,
            internal_position: __ExternalNodeInternalPosition(self.record().target.0),
        }
    }
    pub fn from(self) -> ExternalNodeRef<'graph> {
        self.source()
    }
    pub fn to(self) -> ExternalNodeRef<'graph> {
        self.target()
    }
    pub fn from_id(self) -> &'graph ExternalNodeId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph ExternalNodeId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for ExternalLinkRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ExternalLinkRef))
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
#[derive(Clone, Copy)]
pub struct ExternalIncomingRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ExternalIncomingInternalPosition,
}
impl<'graph> ExternalIncomingRef<'graph> {
    fn record(self) -> &'graph __ExternalIncomingRecord {
        self.graph
            .external_incoming
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph ExternalEdgeId {
        self.graph
            .external_incoming
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn source(self) -> ExternalNodeRef<'graph> {
        ExternalNodeRef {
            graph: self.graph,
            internal_position: __ExternalNodeInternalPosition(self.record().source.0),
        }
    }
    pub fn target(self) -> ExternalNodeRef<'graph> {
        ExternalNodeRef {
            graph: self.graph,
            internal_position: __ExternalNodeInternalPosition(self.record().target.0),
        }
    }
    pub fn from(self) -> ExternalNodeRef<'graph> {
        self.source()
    }
    pub fn to(self) -> ExternalNodeRef<'graph> {
        self.target()
    }
    pub fn from_id(self) -> &'graph ExternalNodeId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph ExternalNodeId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for ExternalIncomingRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ExternalIncomingRef))
    }
}
/// 完成済みグラフ上の無向辺個体。
///
/// 宣言: `tests/schema_ids.rs` の `edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode`
#[derive(Clone, Copy)]
pub struct ExternalFriendRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ExternalFriendInternalPosition,
}
impl<'graph> ExternalFriendRef<'graph> {
    fn record(self) -> &'graph __ExternalFriendRecord {
        self.graph
            .external_friend
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph ExternalEdgeId {
        self.graph
            .external_friend
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn endpoints(self) -> (ExternalNodeRef<'graph>, ExternalNodeRef<'graph>) {
        let (first, second) = self.record().endpoints.endpoints();
        (
            ExternalNodeRef {
                graph: self.graph,
                internal_position: __ExternalNodeInternalPosition(first.0),
            },
            ExternalNodeRef {
                graph: self.graph,
                internal_position: __ExternalNodeInternalPosition(second.0),
            },
        )
    }
}
impl<'graph> std::fmt::Debug for ExternalFriendRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ExternalFriendRef))
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
#[derive(Clone, Copy)]
pub struct AutomaticLinkRef<'graph> {
    graph: &'graph Graph,
    internal_position: __AutomaticLinkInternalPosition,
}
impl<'graph> AutomaticLinkRef<'graph> {
    fn record(self) -> &'graph __AutomaticLinkRecord {
        self.graph
            .automatic_link
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph AutomaticLinkId {
        self.graph
            .automatic_link
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn source(self) -> AutomaticNodeRef<'graph> {
        AutomaticNodeRef {
            graph: self.graph,
            internal_position: __AutomaticNodeInternalPosition(self.record().source.0),
        }
    }
    pub fn target(self) -> AutomaticNodeRef<'graph> {
        AutomaticNodeRef {
            graph: self.graph,
            internal_position: __AutomaticNodeInternalPosition(self.record().target.0),
        }
    }
    pub fn from(self) -> AutomaticNodeRef<'graph> {
        self.source()
    }
    pub fn to(self) -> AutomaticNodeRef<'graph> {
        self.target()
    }
    pub fn from_id(self) -> &'graph AutomaticNodeId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph AutomaticNodeId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for AutomaticLinkRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(AutomaticLinkRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
///
/// 宣言: `tests/schema_ids.rs` の `schema MixedIds`
pub struct Builder {
    __graphite_node_external_node: Vec<(ExternalNodeId, super::ExternalNode)>,
    __graphite_node_automatic_node: Vec<(AutomaticNodeId, super::AutomaticNode)>,
    __graphite_node_boolean_node: Vec<(bool, super::BooleanNode)>,
    external_link: Vec<(ExternalEdgeId, ExternalLink)>,
    external_incoming: Vec<(ExternalEdgeId, ExternalIncoming)>,
    external_friend: Vec<(ExternalEdgeId, ExternalFriend)>,
    automatic_link: Vec<(AutomaticLinkId, AutomaticLink)>,
    /// この構築を識別する構築印。`Builder::new()` が発行し、この
    /// builder から挿入する全ての名前付き位置と、凍結成功後の
    /// `Graph` へ同じ値を刻む。
    __graphite_construction_stamp: u64,
}
/// 型付き ID を受け取るノード・エッジ共通の挿入トレイト。
///
/// 署名が `insert_with_id(self, b, id)` と、挿入される値を receiver に
/// して `Builder` を引数で受ける向きなのは、`graph!` がノード項の値の
/// 型を解析せず、正しい内部ストレージへの振り分けを値の型の trait
/// ディスパッチに頼るためである。利用者向けの公開入口は
/// `Builder::insert`/`Builder::add` の側にある。
///
/// `insert_named_with_id` は [`graphite::NamedInsertPermit`] を要求する
/// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
/// `insert_with_id` (許可証不要、名前付き位置を返さない) は独立した
/// 実装を持ち、`insert_named_with_id` を経由しない
/// (`create` のクロージャから許可証なしで呼べる必要があるため)。
pub trait MixedIdsInsertable: Sized {
    type Id;
    #[doc(hidden)]
    type NamedPosition;
    #[doc(hidden)]
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id;
}
/// 束縛名の文字列からスキーマ内限定の既定IDを作れる要素だけが
/// 実装する。明示ID型には実装せず、文字列変換を要求しない。
pub trait MixedIdsDefaultId: MixedIdsInsertable {
    #[doc(hidden)]
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id;
}
/// ノード挿入で使うトレイト境界。読み取りは `Graph` の種別メソッドと
/// `NodeRef` のメソッドが提供する。利用者がこのトレイトのメソッドを
/// 直接呼ぶことは想定しない。
pub trait MixedIdsNode: MixedIdsInsertable {}
impl MixedIdsInsertable for super::ExternalNode {
    type Id = ExternalNodeId;
    type NamedPosition = __ExternalNodeNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ExternalNodeNamedPosition(
            __ExternalNodeInternalPosition(
                graphite::TablePosition::from_index(
                    b.__graphite_node_external_node.len(),
                ),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.external_node(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.external_node(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ExternalNodeNamedPosition {
    type Reference<'graph> = ExternalNodeRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ExternalNodeRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl MixedIdsNode for super::ExternalNode {}
/// 完成済みグラフ上の `ExternalNode` ノード個体。
///
/// 宣言: `tests/schema_ids.rs` の `node ExternalNode(id: ExternalNodeId)`
#[derive(Clone, Copy)]
pub struct ExternalNodeRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ExternalNodeInternalPosition,
}
impl<'graph> ExternalNodeRef<'graph> {
    pub fn id(self) -> &'graph ExternalNodeId {
        self.graph
            .__graphite_node_external_node
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::ExternalNode {
        self.graph
            .__graphite_node_external_node
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する唯一の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
    pub fn external_link_as_source(self) -> ExternalLinkRef<'graph> {
        ExternalLinkRef {
            graph: self.graph,
            internal_position: *self
                .graph
                .external_link_from_index
                .get(self.internal_position.0),
        }
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
    pub fn external_link_as_target(
        self,
    ) -> impl Iterator<Item = ExternalLinkRef<'graph>> + 'graph {
        let positions = self.graph.external_link_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ExternalLinkRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
    pub fn external_link_try_between(
        self,
        other: ExternalNodeRef<'graph>,
    ) -> Result<
        impl Iterator<Item = ExternalLinkRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_external_link_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| ExternalLinkRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::external_link_try_between`] を使う。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1`
    pub fn external_link_between(
        self,
        other: ExternalNodeRef<'graph>,
    ) -> impl Iterator<Item = ExternalLinkRef<'graph>> + 'graph {
        self.external_link_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(ExternalNodeRef),
                    stringify!(external_link_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
    pub fn external_incoming_as_source(
        self,
    ) -> impl Iterator<Item = ExternalIncomingRef<'graph>> + 'graph {
        let positions = self
            .graph
            .external_incoming_from_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ExternalIncomingRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する唯一の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
    pub fn external_incoming_as_target(self) -> ExternalIncomingRef<'graph> {
        ExternalIncomingRef {
            graph: self.graph,
            internal_position: *self
                .graph
                .external_incoming_to_index
                .get(self.internal_position.0),
        }
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
    pub fn external_incoming_try_between(
        self,
        other: ExternalNodeRef<'graph>,
    ) -> Result<
        impl Iterator<Item = ExternalIncomingRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_external_incoming_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| ExternalIncomingRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::external_incoming_try_between`] を使う。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1`
    pub fn external_incoming_between(
        self,
        other: ExternalNodeRef<'graph>,
    ) -> impl Iterator<Item = ExternalIncomingRef<'graph>> + 'graph {
        self.external_incoming_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(ExternalNodeRef),
                    stringify!(external_incoming_between)
                )
            })
    }
    /// 接続辺を O(1) で参照し、追加確保なしで挿入順に走査する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode`
    pub fn external_friend_incident(
        self,
    ) -> impl Iterator<Item = ExternalFriendRef<'graph>> + 'graph {
        let positions = self.graph.external_friend_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ExternalFriendRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序なし端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode`
    pub fn external_friend_try_between(
        self,
        other: ExternalNodeRef<'graph>,
    ) -> Result<
        impl Iterator<Item = ExternalFriendRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_external_friend_by_pair
            .get(
                &graphite::UnorderedPair::new(
                    self.internal_position,
                    other.internal_position,
                ),
            )
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| ExternalFriendRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::external_friend_try_between`] を使う。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode`
    pub fn external_friend_between(
        self,
        other: ExternalNodeRef<'graph>,
    ) -> impl Iterator<Item = ExternalFriendRef<'graph>> + 'graph {
        self.external_friend_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(ExternalNodeRef),
                    stringify!(external_friend_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for ExternalNodeRef<'graph> {
    type Target = super::ExternalNode;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_external_node
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for ExternalNodeRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ExternalNodeRef))
    }
}
impl MixedIdsInsertable for super::AutomaticNode {
    type Id = AutomaticNodeId;
    type NamedPosition = __AutomaticNodeNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __AutomaticNodeNamedPosition(
            __AutomaticNodeInternalPosition(
                graphite::TablePosition::from_index(
                    b.__graphite_node_automatic_node.len(),
                ),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.automatic_node(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.automatic_node(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __AutomaticNodeNamedPosition {
    type Reference<'graph> = AutomaticNodeRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        AutomaticNodeRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl MixedIdsDefaultId for super::AutomaticNode {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        MixedIdsInsertable::insert_named_with_id(
            self,
            b,
            AutomaticNodeId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        MixedIdsInsertable::insert_with_id(self, b, AutomaticNodeId(binding))
    }
}
impl MixedIdsNode for super::AutomaticNode {}
/// 完成済みグラフ上の `AutomaticNode` ノード個体。
///
/// 宣言: `tests/schema_ids.rs` の `node AutomaticNode`
#[derive(Clone, Copy)]
pub struct AutomaticNodeRef<'graph> {
    graph: &'graph Graph,
    internal_position: __AutomaticNodeInternalPosition,
}
impl<'graph> AutomaticNodeRef<'graph> {
    pub fn id(self) -> &'graph AutomaticNodeId {
        self.graph
            .__graphite_node_automatic_node
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::AutomaticNode {
        self.graph
            .__graphite_node_automatic_node
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
    pub fn automatic_link_as_source(
        self,
    ) -> impl Iterator<Item = AutomaticLinkRef<'graph>> + 'graph {
        let positions = self
            .graph
            .automatic_link_from_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| AutomaticLinkRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
    pub fn automatic_link_as_target(
        self,
    ) -> impl Iterator<Item = AutomaticLinkRef<'graph>> + 'graph {
        let positions = self.graph.automatic_link_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| AutomaticLinkRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
    pub fn automatic_link_try_between(
        self,
        other: AutomaticNodeRef<'graph>,
    ) -> Result<
        impl Iterator<Item = AutomaticLinkRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_automatic_link_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| AutomaticLinkRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::automatic_link_try_between`] を使う。
    ///
    /// 宣言: `tests/schema_ids.rs` の `edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode)`
    pub fn automatic_link_between(
        self,
        other: AutomaticNodeRef<'graph>,
    ) -> impl Iterator<Item = AutomaticLinkRef<'graph>> + 'graph {
        self.automatic_link_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(AutomaticNodeRef),
                    stringify!(automatic_link_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for AutomaticNodeRef<'graph> {
    type Target = super::AutomaticNode;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_automatic_node
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for AutomaticNodeRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(AutomaticNodeRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
impl MixedIdsInsertable for super::BooleanNode {
    type Id = bool;
    type NamedPosition = __BooleanNodeNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __BooleanNodeNamedPosition(
            __BooleanNodeInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_boolean_node.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.boolean_node(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.boolean_node(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __BooleanNodeNamedPosition {
    type Reference<'graph> = BooleanNodeRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        BooleanNodeRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl MixedIdsNode for super::BooleanNode {}
/// 完成済みグラフ上の `BooleanNode` ノード個体。
///
/// 宣言: `tests/schema_ids.rs` の `node BooleanNode(id: bool)`
#[derive(Clone, Copy)]
pub struct BooleanNodeRef<'graph> {
    graph: &'graph Graph,
    internal_position: __BooleanNodeInternalPosition,
}
impl<'graph> BooleanNodeRef<'graph> {
    pub fn id(self) -> &'graph bool {
        self.graph
            .__graphite_node_boolean_node
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::BooleanNode {
        self.graph
            .__graphite_node_boolean_node
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::ops::Deref for BooleanNodeRef<'graph> {
    type Target = super::BooleanNode;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_boolean_node
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for BooleanNodeRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(BooleanNodeRef))
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait MixedIdsEdge: MixedIdsInsertable {}
impl MixedIdsInsertable for ExternalLink {
    type Id = ExternalEdgeId;
    type NamedPosition = __ExternalLinkNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ExternalLinkNamedPosition(
            __ExternalLinkInternalPosition(
                graphite::TablePosition::from_index(b.external_link.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.external_link(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.external_link(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ExternalLinkNamedPosition {
    type Reference<'graph> = ExternalLinkRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ExternalLinkRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl MixedIdsEdge for ExternalLink {}
impl MixedIdsInsertable for ExternalIncoming {
    type Id = ExternalEdgeId;
    type NamedPosition = __ExternalIncomingNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ExternalIncomingNamedPosition(
            __ExternalIncomingInternalPosition(
                graphite::TablePosition::from_index(b.external_incoming.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.external_incoming(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.external_incoming(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ExternalIncomingNamedPosition {
    type Reference<'graph> = ExternalIncomingRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ExternalIncomingRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl MixedIdsEdge for ExternalIncoming {}
impl MixedIdsInsertable for ExternalFriend {
    type Id = ExternalEdgeId;
    type NamedPosition = __ExternalFriendNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ExternalFriendNamedPosition(
            __ExternalFriendInternalPosition(
                graphite::TablePosition::from_index(b.external_friend.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.external_friend(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.external_friend(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ExternalFriendNamedPosition {
    type Reference<'graph> = ExternalFriendRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ExternalFriendRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl MixedIdsEdge for ExternalFriend {}
impl MixedIdsInsertable for AutomaticLink {
    type Id = AutomaticLinkId;
    type NamedPosition = __AutomaticLinkNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __AutomaticLinkNamedPosition(
            __AutomaticLinkInternalPosition(
                graphite::TablePosition::from_index(b.automatic_link.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.automatic_link(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.automatic_link(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __AutomaticLinkNamedPosition {
    type Reference<'graph> = AutomaticLinkRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        AutomaticLinkRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl MixedIdsDefaultId for AutomaticLink {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        MixedIdsInsertable::insert_named_with_id(
            self,
            b,
            AutomaticLinkId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        MixedIdsInsertable::insert_with_id(self, b, AutomaticLinkId(binding))
    }
}
impl MixedIdsEdge for AutomaticLink {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_external_node: Vec::new(),
            __graphite_node_automatic_node: Vec::new(),
            __graphite_node_boolean_node: Vec::new(),
            external_link: Vec::new(),
            external_incoming: Vec::new(),
            external_friend: Vec::new(),
            automatic_link: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    pub fn external_node(
        &mut self,
        id: ExternalNodeId,
        value: super::ExternalNode,
    ) -> &mut Self {
        self.__graphite_node_external_node.push((id, value));
        self
    }
    pub fn automatic_node(
        &mut self,
        id: AutomaticNodeId,
        value: super::AutomaticNode,
    ) -> &mut Self {
        self.__graphite_node_automatic_node.push((id, value));
        self
    }
    pub fn boolean_node(&mut self, id: bool, value: super::BooleanNode) -> &mut Self {
        self.__graphite_node_boolean_node.push((id, value));
        self
    }
    pub fn external_link(
        &mut self,
        id: ExternalEdgeId,
        value: ExternalLink,
    ) -> &mut Self {
        self.external_link.push((id, value));
        self
    }
    pub fn external_incoming(
        &mut self,
        id: ExternalEdgeId,
        value: ExternalIncoming,
    ) -> &mut Self {
        self.external_incoming.push((id, value));
        self
    }
    pub fn external_friend(
        &mut self,
        id: ExternalEdgeId,
        value: ExternalFriend,
    ) -> &mut Self {
        self.external_friend.push((id, value));
        self
    }
    pub fn automatic_link(
        &mut self,
        id: AutomaticLinkId,
        value: AutomaticLink,
    ) -> &mut Self {
        self.automatic_link.push((id, value));
        self
    }
    /// 型名付きメソッド (`b.#accessor(id, value)` 群、上記
    /// `#node_methods`) の総称版。`graph!` の左辺名付きノード項は
    /// 下記 `insert_named` (名前付き位置を返す許可証付き経路) へ
    /// 脱糖するため、このメソッド自体は `graph!` を経由しない。
    /// 値の型を手書きで組み立てる場合 (プログラム的構築など) に使う。
    /// `graph!` はノード項の値の型を一切パースしないため
    /// (`key = 式` の「式」でしかない)、値の型 (`N: #node_trait_ident`)
    /// から正しい内部ストレージへの振り分けを rustc の型推論任せに
    /// する点は `insert_named` と共通。命名判断・trait の形は
    /// `gen_node_trait_and_impls` のドキュメントコメント参照。
    pub fn insert<N>(&mut self, key: impl Into<String>, value: N) -> N::Id
    where
        N: MixedIdsNode + MixedIdsDefaultId,
    {
        value.insert_with_binding(self, key.into())
    }
    /// `graph!` が公開IDと名前付き要素の内部位置を同時に受け取る経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named<N>(
        &mut self,
        key: impl Into<String>,
        value: N,
        permit: &graphite::NamedInsertPermit,
    ) -> (N::Id, N::NamedPosition)
    where
        N: MixedIdsNode + MixedIdsDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: MixedIdsNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: MixedIdsNode>(
        &mut self,
        id: N::Id,
        value: N,
        permit: &graphite::NamedInsertPermit,
    ) -> (N::Id, N::NamedPosition) {
        value.insert_named_with_id(self, id, permit)
    }
    /// `insert` のエッジ版。`graph!` の辺行 `key = Kind(from -> to)`
    /// は名前付きフィールドの辺値型を関連コンストラクタで構築したあと、
    /// 下記 `add_named` へ脱糖する (`docs/schema_v4.md` §2/§3.2)。
    /// このメソッド自体は値の型から内部ストレージへ振り分ける総称
    /// ディスパッチを提供する手書き用APIで、`graph!` を直接経由しない。
    pub fn add<E>(&mut self, key: impl Into<String>, value: E) -> E::Id
    where
        E: MixedIdsEdge + MixedIdsDefaultId,
    {
        value.insert_with_binding(self, key.into())
    }
    /// `graph!` が公開IDと名前付き辺の内部位置を同時に受け取る経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named<E>(
        &mut self,
        key: impl Into<String>,
        value: E,
        permit: &graphite::NamedInsertPermit,
    ) -> (E::Id, E::NamedPosition)
    where
        E: MixedIdsEdge + MixedIdsDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: MixedIdsEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: MixedIdsEdge>(
        &mut self,
        id: E::Id,
        value: E,
        permit: &graphite::NamedInsertPermit,
    ) -> (E::Id, E::NamedPosition) {
        value.insert_named_with_id(self, id, permit)
    }
    /// `insert`/`add` のイテレータ版 (`docs/bulk_construction.md`、
    /// `docs/graph_splice.md` §2)。実行時データからの構築で for
    /// ループが構築コードに残るのを避けるため、要素単位 API の反復に
    /// 完全に一致する意味論 (挿入順保持・検証は凍結時) をまとめて
    /// 提供する。ノード用・エッジ用の呼び分けが要らない単一の総称
    /// メソッドに統一している (v4 破壊的変更、旧 `extend_nodes`/
    /// `extend_edges` は廃止): 値の型が既定IDを生成できれば
    /// ノードでもエッジでもよい (どちらになるかは rustc の
    /// 型推論任せ)。`graph!` のスプライス項 (`..式`) もこのメソッドへ
    /// 脱糖する。`insert`/`add` と同じ理由 (トレイトが schema ごとに
    /// 名前が異なる) で、graphite ランタイム側の共通機構ではなく
    /// ここに生成する。
    pub fn extend<K, T>(&mut self, items: impl IntoIterator<Item = (K, T)>) -> Vec<T::Id>
    where
        K: Into<String>,
        T: MixedIdsDefaultId,
    {
        items.into_iter().map(|(k, v)| v.insert_with_binding(self, k.into())).collect()
    }
    /// 検証ロジックの実体。最初の1件で打ち切らず全違反を `Vec` に
    /// 集めて返す。`freeze()` (単一エラー版) はこちらに委譲し先頭の1件を
    /// 取り出すだけの薄いラッパーにすることで、検証ロジックが二重実装に
    /// ならないようにしている。
    fn freeze_collecting(self) -> Result<Graph, Vec<Violation>> {
        let mut __violations: Vec<Violation> = Vec::new();
        let __graphite_construction_stamp = self.__graphite_construction_stamp;
        let mut __graphite_node_external_node: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_external_node {
            if !__graphite_node_external_node.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateExternalNode(id));
            }
        }
        let mut __graphite_node_automatic_node: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_automatic_node {
            if !__graphite_node_automatic_node.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateAutomaticNode(id));
            }
        }
        let mut __graphite_node_boolean_node: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_boolean_node {
            if !__graphite_node_boolean_node.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateBooleanNode(id));
            }
        }
        let mut __graphite_external_link: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut external_link_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut external_link_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_external_link_by_pair: std::collections::HashMap<
            (__ExternalNodeInternalPosition, __ExternalNodeInternalPosition),
            Vec<__ExternalLinkInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.external_link {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ExternalLinkDuplicateKey(id));
                continue;
            }
            let ExternalLink { source: from, target: to } = value;
            let from_position = __graphite_node_external_node
                .position(&from)
                .map(__ExternalNodeInternalPosition);
            let to_position = __graphite_node_external_node
                .position(&to)
                .map(__ExternalNodeInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::ExternalLinkUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::ExternalLinkUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __ExternalLinkInternalPosition(
                    graphite::TablePosition::from_index(__graphite_external_link.len()),
                );
                __graphite_external_link_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                external_link_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                external_link_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_external_link
                    .insert(
                        id,
                        __ExternalLinkRecord {
                            source: from_position,
                            target: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&ExternalLink) = |edge| {
            let _ = &edge.source;
        };
        for position in __graphite_node_external_node.positions() {
            let internal_position = __ExternalNodeInternalPosition(position);
            let key = __graphite_node_external_node
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = external_link_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if count != 1usize {
                __violations
                    .push(Violation::ExternalLinkSourceEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_external_incoming: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut external_incoming_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut external_incoming_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_external_incoming_by_pair: std::collections::HashMap<
            (__ExternalNodeInternalPosition, __ExternalNodeInternalPosition),
            Vec<__ExternalIncomingInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.external_incoming {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ExternalIncomingDuplicateKey(id));
                continue;
            }
            let ExternalIncoming { source: from, target: to } = value;
            let from_position = __graphite_node_external_node
                .position(&from)
                .map(__ExternalNodeInternalPosition);
            let to_position = __graphite_node_external_node
                .position(&to)
                .map(__ExternalNodeInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::ExternalIncomingUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::ExternalIncomingUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __ExternalIncomingInternalPosition(
                    graphite::TablePosition::from_index(
                        __graphite_external_incoming.len(),
                    ),
                );
                __graphite_external_incoming_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                external_incoming_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                external_incoming_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_external_incoming
                    .insert(
                        id,
                        __ExternalIncomingRecord {
                            source: from_position,
                            target: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&ExternalIncoming) = |edge| {
            let _ = &edge.target;
        };
        for position in __graphite_node_external_node.positions() {
            let internal_position = __ExternalNodeInternalPosition(position);
            let key = __graphite_node_external_node
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = external_incoming_to_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if count != 1usize {
                __violations
                    .push(Violation::ExternalIncomingTargetEachViolation {
                        target: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_external_friend: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut external_friend_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_external_friend_by_pair: std::collections::HashMap<
            graphite::UnorderedPair<__ExternalNodeInternalPosition>,
            Vec<__ExternalFriendInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.external_friend {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ExternalFriendDuplicateKey(id));
                continue;
            }
            let ExternalFriend { endpoints } = value;
            let (p0, p1) = endpoints.endpoints();
            let p0 = p0.clone();
            let p1 = p1.clone();
            let first_position = __graphite_node_external_node
                .position(&p0)
                .map(__ExternalNodeInternalPosition);
            let second_position = __graphite_node_external_node
                .position(&p1)
                .map(__ExternalNodeInternalPosition);
            if first_position.is_none() {
                __violations
                    .push(Violation::ExternalFriendUnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p0.clone(),
                    });
            }
            if p1 != p0 && second_position.is_none() {
                __violations
                    .push(Violation::ExternalFriendUnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p1.clone(),
                    });
            }
            if let (Some(first_position), Some(second_position)) = (
                first_position,
                second_position,
            ) {
                let internal_edge_position = __ExternalFriendInternalPosition(
                    graphite::TablePosition::from_index(__graphite_external_friend.len()),
                );
                __graphite_external_friend_by_pair
                    .entry(graphite::UnorderedPair::new(first_position, second_position))
                    .or_default()
                    .push(internal_edge_position);
                external_friend_index
                    .entry(first_position)
                    .or_default()
                    .push(internal_edge_position);
                if second_position != first_position {
                    external_friend_index
                        .entry(second_position)
                        .or_default()
                        .push(internal_edge_position);
                }
                let inserted = __graphite_external_friend
                    .insert(
                        id,
                        __ExternalFriendRecord {
                            endpoints: graphite::UnorderedPair::new(
                                first_position,
                                second_position,
                            ),
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_automatic_link: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut automatic_link_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut automatic_link_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_automatic_link_by_pair: std::collections::HashMap<
            (__AutomaticNodeInternalPosition, __AutomaticNodeInternalPosition),
            Vec<__AutomaticLinkInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.automatic_link {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::AutomaticLinkDuplicateKey(id));
                continue;
            }
            let AutomaticLink { source: from, target: to } = value;
            let from_position = __graphite_node_automatic_node
                .position(&from)
                .map(__AutomaticNodeInternalPosition);
            let to_position = __graphite_node_automatic_node
                .position(&to)
                .map(__AutomaticNodeInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::AutomaticLinkUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::AutomaticLinkUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __AutomaticLinkInternalPosition(
                    graphite::TablePosition::from_index(__graphite_automatic_link.len()),
                );
                __graphite_automatic_link_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                automatic_link_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                automatic_link_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_automatic_link
                    .insert(
                        id,
                        __AutomaticLinkRecord {
                            source: from_position,
                            target: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let external_link_from_index = graphite::ExactlyOneRoleIndex::from_buckets(
            __graphite_node_external_node
                .positions()
                .map(|position| {
                    external_link_from_index
                        .remove(&__ExternalNodeInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let external_link_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_external_node
                .positions()
                .map(|position| {
                    external_link_to_index
                        .remove(&__ExternalNodeInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let external_incoming_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_external_node
                .positions()
                .map(|position| {
                    external_incoming_from_index
                        .remove(&__ExternalNodeInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let external_incoming_to_index = graphite::ExactlyOneRoleIndex::from_buckets(
            __graphite_node_external_node
                .positions()
                .map(|position| {
                    external_incoming_to_index
                        .remove(&__ExternalNodeInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let external_friend_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_external_node
                .positions()
                .map(|position| {
                    external_friend_index
                        .remove(&__ExternalNodeInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let automatic_link_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_automatic_node
                .positions()
                .map(|position| {
                    automatic_link_from_index
                        .remove(&__AutomaticNodeInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let automatic_link_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_automatic_node
                .positions()
                .map(|position| {
                    automatic_link_to_index
                        .remove(&__AutomaticNodeInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_external_node,
            __graphite_node_automatic_node,
            __graphite_node_boolean_node,
            external_link: __graphite_external_link,
            external_incoming: __graphite_external_incoming,
            external_friend: __graphite_external_friend,
            automatic_link: __graphite_automatic_link,
            external_link_from_index,
            external_link_to_index,
            __graphite_external_link_by_pair,
            external_incoming_from_index,
            external_incoming_to_index,
            __graphite_external_incoming_by_pair,
            external_friend_index,
            __graphite_external_friend_by_pair,
            automatic_link_from_index,
            automatic_link_to_index,
            __graphite_automatic_link_by_pair,
            __graphite_construction_stamp,
        })
    }
    /// 最初の1件の違反で `Err` になる版。実装は
    /// `freeze_collecting` に委譲する。
    fn freeze(self) -> Result<Graph, Violation> {
        self.freeze_collecting().map_err(|mut violations| violations.remove(0))
    }
}
/// [`graphite::build_named_graph`] が `#schema_name`/`#violation_ident`
/// の具体型を知らずに凍結を呼べるようにするための橋渡し。
/// `freeze_into_graph` は既存の私有 `freeze()` (上記) へそのまま委譲する。
impl graphite::FreezableBuilder for Builder {
    type Graph = Graph;
    type Violation = Violation;
    fn freeze_into_graph(self) -> Result<Self::Graph, Self::Violation> {
        self.freeze()
    }
}
