// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: crates/graphite/tests/role_query.rs:45
// 再生成: リポジトリルートで `cargo xtask generate` を実行する。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    12987367537200975726u64, 7845968990354384509u64, 336999901808702200u64,
    11540689986323353388u64,
];
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeAId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeBId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnconstrainedId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnconstrainedNoPayloadId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AtMostOneId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExactlyOneId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __NodeAInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __NodeBInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __UnconstrainedInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __UnconstrainedNoPayloadInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __AtMostOneInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ExactlyOneInternalPosition(usize);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __NodeANamedPosition(__NodeAInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __NodeBNamedPosition(__NodeBInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __UnconstrainedNamedPosition(__UnconstrainedInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __UnconstrainedNoPayloadNamedPosition(
    __UnconstrainedNoPayloadInternalPosition,
    u64,
);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __AtMostOneNamedPosition(__AtMostOneInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ExactlyOneNamedPosition(__ExactlyOneInternalPosition, u64);
#[derive(Clone, PartialEq)]
pub struct Unconstrained {
    pub source: NodeAId,
    pub target: NodeBId,
    pub weight: Weight,
}
impl Unconstrained {
    pub fn new(from: NodeAId, to: NodeBId, payload: Weight) -> Self {
        Self {
            source: from,
            target: to,
            weight: payload,
        }
    }
    pub fn payload(&self) -> &Weight {
        &self.weight
    }
}
impl graphite::DirectedEdgeLiteral<NodeAId, NodeBId, Weight> for Unconstrained {
    fn from_graph_literal(from: NodeAId, to: NodeBId, payload: Weight) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Unconstrained {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Unconstrained))
    }
}
#[derive(Clone, PartialEq)]
pub struct UnconstrainedNoPayload {
    pub source: NodeAId,
    pub target: NodeBId,
}
impl UnconstrainedNoPayload {
    pub fn new(from: NodeAId, to: NodeBId) -> Self {
        Self { source: from, target: to }
    }
}
impl graphite::DirectedEdgeLiteral<NodeAId, NodeBId, ()> for UnconstrainedNoPayload {
    fn from_graph_literal(from: NodeAId, to: NodeBId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for UnconstrainedNoPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(UnconstrainedNoPayload))
            .field(&self.source)
            .field(&self.target)
            .finish()
    }
}
#[derive(Clone, PartialEq)]
pub struct AtMostOne {
    pub src: NodeAId,
    pub dst: NodeBId,
}
impl AtMostOne {
    pub fn new(from: NodeAId, to: NodeBId) -> Self {
        Self { src: from, dst: to }
    }
}
impl graphite::DirectedEdgeLiteral<NodeAId, NodeBId, ()> for AtMostOne {
    fn from_graph_literal(from: NodeAId, to: NodeBId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for AtMostOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(AtMostOne)).field(&self.src).field(&self.dst).finish()
    }
}
#[derive(Clone, PartialEq)]
pub struct ExactlyOne {
    pub src: NodeAId,
    pub dst: NodeBId,
    pub weight: Weight,
}
impl ExactlyOne {
    pub fn new(from: NodeAId, to: NodeBId, payload: Weight) -> Self {
        Self {
            src: from,
            dst: to,
            weight: payload,
        }
    }
    pub fn payload(&self) -> &Weight {
        &self.weight
    }
}
impl graphite::DirectedEdgeLiteral<NodeAId, NodeBId, Weight> for ExactlyOne {
    fn from_graph_literal(from: NodeAId, to: NodeBId, payload: Weight) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for ExactlyOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ExactlyOne))
    }
}
#[allow(dead_code)]
struct __UnconstrainedRecord {
    source: __NodeAInternalPosition,
    target: __NodeBInternalPosition,
    weight: Weight,
}
#[allow(dead_code)]
struct __UnconstrainedNoPayloadRecord {
    source: __NodeAInternalPosition,
    target: __NodeBInternalPosition,
}
#[allow(dead_code)]
struct __AtMostOneRecord {
    src: __NodeAInternalPosition,
    dst: __NodeBInternalPosition,
}
#[allow(dead_code)]
struct __ExactlyOneRecord {
    src: __NodeAInternalPosition,
    dst: __NodeBInternalPosition,
    weight: Weight,
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    DuplicateNodeA(NodeAId),
    DuplicateNodeB(NodeBId),
    /// このエッジ種別のキーが重複している。
    UnconstrainedDuplicateKey(UnconstrainedId),
    /// このエッジが未知の始点キーを参照している。
    UnconstrainedUnknownSource { edge: UnconstrainedId, source: NodeAId },
    /// このエッジが未知の終点キーを参照している。
    UnconstrainedUnknownTarget { edge: UnconstrainedId, target: NodeBId },
    /// このエッジ種別のキーが重複している。
    UnconstrainedNoPayloadDuplicateKey(UnconstrainedNoPayloadId),
    /// このエッジが未知の始点キーを参照している。
    UnconstrainedNoPayloadUnknownSource {
        edge: UnconstrainedNoPayloadId,
        source: NodeAId,
    },
    /// このエッジが未知の終点キーを参照している。
    UnconstrainedNoPayloadUnknownTarget {
        edge: UnconstrainedNoPayloadId,
        target: NodeBId,
    },
    /// このエッジ種別のキーが重複している。
    AtMostOneDuplicateKey(AtMostOneId),
    /// このエッジが未知の始点キーを参照している。
    AtMostOneUnknownSource { edge: AtMostOneId, source: NodeAId },
    /// このエッジが未知の終点キーを参照している。
    AtMostOneUnknownTarget { edge: AtMostOneId, target: NodeBId },
    /// このエッジ種別の `each` 制約違反 (入次数)。
    AtMostOneDstEachViolation { target: NodeBId, count: usize },
    /// このエッジ種別のキーが重複している。
    ExactlyOneDuplicateKey(ExactlyOneId),
    /// このエッジが未知の始点キーを参照している。
    ExactlyOneUnknownSource { edge: ExactlyOneId, source: NodeAId },
    /// このエッジが未知の終点キーを参照している。
    ExactlyOneUnknownTarget { edge: ExactlyOneId, target: NodeBId },
    /// このエッジ種別の `each` 制約違反 (入次数)。
    ExactlyOneDstEachViolation { target: NodeBId, count: usize },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicateNodeA(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "NodeA", id)
            }
            Violation::DuplicateNodeB(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "NodeB", id)
            }
            Violation::UnconstrainedDuplicateKey(id) => {
                write!(
                    f, "{}のキーが重複しています: {:?}", "Unconstrained", id
                )
            }
            Violation::UnconstrainedUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Unconstrained", edge, "NodeA", source
                )
            }
            Violation::UnconstrainedUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Unconstrained", edge, "NodeB", target
                )
            }
            Violation::UnconstrainedNoPayloadDuplicateKey(id) => {
                write!(
                    f, "{}のキーが重複しています: {:?}",
                    "UnconstrainedNoPayload", id
                )
            }
            Violation::UnconstrainedNoPayloadUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "UnconstrainedNoPayload", edge, "NodeA", source
                )
            }
            Violation::UnconstrainedNoPayloadUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "UnconstrainedNoPayload", edge, "NodeB", target
                )
            }
            Violation::AtMostOneDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "AtMostOne", id)
            }
            Violation::AtMostOneUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "AtMostOne", edge, "NodeA", source
                )
            }
            Violation::AtMostOneUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "AtMostOne", edge, "NodeB", target
                )
            }
            Violation::AtMostOneDstEachViolation { target, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について入次数 {} を期待しますが実際は {} 本です",
                    "AtMostOne", "NodeB", target, "0..1", count
                )
            }
            Violation::ExactlyOneDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "ExactlyOne", id)
            }
            Violation::ExactlyOneUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "ExactlyOne", edge, "NodeA", source
                )
            }
            Violation::ExactlyOneUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "ExactlyOne", edge, "NodeB", target
                )
            }
            Violation::ExactlyOneDstEachViolation { target, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について入次数 {} を期待しますが実際は {} 本です",
                    "ExactlyOne", "NodeB", target, "ちょうど1", count
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
pub struct Graph {
    __graphite_node_node_a: graphite::KeyedTable<NodeAId, super::NodeA>,
    __graphite_node_node_b: graphite::KeyedTable<NodeBId, super::NodeB>,
    unconstrained: graphite::KeyedTable<UnconstrainedId, __UnconstrainedRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    unconstrained_from_index: graphite::MultipleRoleIndex<
        __UnconstrainedInternalPosition,
    >,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    unconstrained_to_index: graphite::MultipleRoleIndex<__UnconstrainedInternalPosition>,
    __graphite_unconstrained_by_pair: std::collections::HashMap<
        (__NodeAInternalPosition, __NodeBInternalPosition),
        Vec<__UnconstrainedInternalPosition>,
    >,
    unconstrained_no_payload: graphite::KeyedTable<
        UnconstrainedNoPayloadId,
        __UnconstrainedNoPayloadRecord,
    >,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    unconstrained_no_payload_from_index: graphite::MultipleRoleIndex<
        __UnconstrainedNoPayloadInternalPosition,
    >,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    unconstrained_no_payload_to_index: graphite::MultipleRoleIndex<
        __UnconstrainedNoPayloadInternalPosition,
    >,
    __graphite_unconstrained_no_payload_by_pair: std::collections::HashMap<
        (__NodeAInternalPosition, __NodeBInternalPosition),
        Vec<__UnconstrainedNoPayloadInternalPosition>,
    >,
    at_most_one: graphite::KeyedTable<AtMostOneId, __AtMostOneRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    at_most_one_from_index: graphite::MultipleRoleIndex<__AtMostOneInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    at_most_one_to_index: graphite::OptionalRoleIndex<__AtMostOneInternalPosition>,
    __graphite_at_most_one_by_pair: std::collections::HashMap<
        (__NodeAInternalPosition, __NodeBInternalPosition),
        Vec<__AtMostOneInternalPosition>,
    >,
    exactly_one: graphite::KeyedTable<ExactlyOneId, __ExactlyOneRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    exactly_one_from_index: graphite::MultipleRoleIndex<__ExactlyOneInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    exactly_one_to_index: graphite::ExactlyOneRoleIndex<__ExactlyOneInternalPosition>,
    __graphite_exactly_one_by_pair: std::collections::HashMap<
        (__NodeAInternalPosition, __NodeBInternalPosition),
        Vec<__ExactlyOneInternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    pub fn node_a_by_id<'graph>(&'graph self, id: &NodeAId) -> Option<NodeARef<'graph>> {
        let internal_position = __NodeAInternalPosition(
            self.__graphite_node_node_a.position(id)?,
        );
        Some(NodeARef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    pub fn node_a_value_mut(&mut self, id: &NodeAId) -> Option<&mut super::NodeA> {
        self.__graphite_node_node_a.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn node_a_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph NodeAId> {
        self.__graphite_node_node_a.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    pub fn node_a_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = NodeARef<'graph>> + 'graph {
        self.__graphite_node_node_a
            .positions()
            .map(move |position| NodeARef {
                graph: self,
                internal_position: __NodeAInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    pub fn node_a_len(&self) -> usize {
        self.__graphite_node_node_a.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    pub fn node_b_by_id<'graph>(&'graph self, id: &NodeBId) -> Option<NodeBRef<'graph>> {
        let internal_position = __NodeBInternalPosition(
            self.__graphite_node_node_b.position(id)?,
        );
        Some(NodeBRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    pub fn node_b_value_mut(&mut self, id: &NodeBId) -> Option<&mut super::NodeB> {
        self.__graphite_node_node_b.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn node_b_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph NodeBId> {
        self.__graphite_node_node_b.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    pub fn node_b_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = NodeBRef<'graph>> + 'graph {
        self.__graphite_node_node_b
            .positions()
            .map(move |position| NodeBRef {
                graph: self,
                internal_position: __NodeBInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    pub fn node_b_len(&self) -> usize {
        self.__graphite_node_node_b.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn unconstrained_by_id<'graph>(
        &'graph self,
        id: &UnconstrainedId,
    ) -> Option<UnconstrainedRef<'graph>> {
        Some(UnconstrainedRef {
            graph: self,
            internal_position: __UnconstrainedInternalPosition(
                self.unconstrained.position(id)?,
            ),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    pub fn unconstrained_payload_mut(
        &mut self,
        id: &UnconstrainedId,
    ) -> Option<&mut Weight> {
        self.unconstrained
            .get_mut(id)
            .map(|record: &mut __UnconstrainedRecord| &mut record.weight)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn unconstrained_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph UnconstrainedId> {
        self.unconstrained.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn unconstrained_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = UnconstrainedRef<'graph>> + 'graph {
        self.unconstrained
            .positions()
            .map(move |position| UnconstrainedRef {
                graph: self,
                internal_position: __UnconstrainedInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn unconstrained_len(&self) -> usize {
        self.unconstrained.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn unconstrained_no_payload_by_id<'graph>(
        &'graph self,
        id: &UnconstrainedNoPayloadId,
    ) -> Option<UnconstrainedNoPayloadRef<'graph>> {
        Some(UnconstrainedNoPayloadRef {
            graph: self,
            internal_position: __UnconstrainedNoPayloadInternalPosition(
                self.unconstrained_no_payload.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn unconstrained_no_payload_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph UnconstrainedNoPayloadId> {
        self.unconstrained_no_payload.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn unconstrained_no_payload_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = UnconstrainedNoPayloadRef<'graph>> + 'graph {
        self.unconstrained_no_payload
            .positions()
            .map(move |position| UnconstrainedNoPayloadRef {
                graph: self,
                internal_position: __UnconstrainedNoPayloadInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn unconstrained_no_payload_len(&self) -> usize {
        self.unconstrained_no_payload.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn at_most_one_by_id<'graph>(
        &'graph self,
        id: &AtMostOneId,
    ) -> Option<AtMostOneRef<'graph>> {
        Some(AtMostOneRef {
            graph: self,
            internal_position: __AtMostOneInternalPosition(
                self.at_most_one.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn at_most_one_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph AtMostOneId> {
        self.at_most_one.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn at_most_one_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = AtMostOneRef<'graph>> + 'graph {
        self.at_most_one
            .positions()
            .map(move |position| AtMostOneRef {
                graph: self,
                internal_position: __AtMostOneInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn at_most_one_len(&self) -> usize {
        self.at_most_one.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn exactly_one_by_id<'graph>(
        &'graph self,
        id: &ExactlyOneId,
    ) -> Option<ExactlyOneRef<'graph>> {
        Some(ExactlyOneRef {
            graph: self,
            internal_position: __ExactlyOneInternalPosition(
                self.exactly_one.position(id)?,
            ),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    pub fn exactly_one_payload_mut(&mut self, id: &ExactlyOneId) -> Option<&mut Weight> {
        self.exactly_one
            .get_mut(id)
            .map(|record: &mut __ExactlyOneRecord| &mut record.weight)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn exactly_one_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph ExactlyOneId> {
        self.exactly_one.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn exactly_one_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ExactlyOneRef<'graph>> + 'graph {
        self.exactly_one
            .positions()
            .map(move |position| ExactlyOneRef {
                graph: self,
                internal_position: __ExactlyOneInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn exactly_one_len(&self) -> usize {
        self.exactly_one.len()
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
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
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
#[derive(Clone, Copy)]
pub struct UnconstrainedRef<'graph> {
    graph: &'graph Graph,
    internal_position: __UnconstrainedInternalPosition,
}
impl<'graph> UnconstrainedRef<'graph> {
    fn record(self) -> &'graph __UnconstrainedRecord {
        self.graph
            .unconstrained
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph UnconstrainedId {
        self.graph
            .unconstrained
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn source(self) -> NodeARef<'graph> {
        NodeARef {
            graph: self.graph,
            internal_position: __NodeAInternalPosition(self.record().source.0),
        }
    }
    pub fn target(self) -> NodeBRef<'graph> {
        NodeBRef {
            graph: self.graph,
            internal_position: __NodeBInternalPosition(self.record().target.0),
        }
    }
    pub fn from(self) -> NodeARef<'graph> {
        self.source()
    }
    pub fn to(self) -> NodeBRef<'graph> {
        self.target()
    }
    pub fn from_id(self) -> &'graph NodeAId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph NodeBId {
        self.to().id()
    }
    pub fn weight(self) -> &'graph Weight {
        &self.record().weight
    }
    pub fn payload(self) -> &'graph Weight {
        &self.record().weight
    }
}
impl<'graph> std::fmt::Debug for UnconstrainedRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(UnconstrainedRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
#[derive(Clone, Copy)]
pub struct UnconstrainedNoPayloadRef<'graph> {
    graph: &'graph Graph,
    internal_position: __UnconstrainedNoPayloadInternalPosition,
}
impl<'graph> UnconstrainedNoPayloadRef<'graph> {
    fn record(self) -> &'graph __UnconstrainedNoPayloadRecord {
        self.graph
            .unconstrained_no_payload
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph UnconstrainedNoPayloadId {
        self.graph
            .unconstrained_no_payload
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn source(self) -> NodeARef<'graph> {
        NodeARef {
            graph: self.graph,
            internal_position: __NodeAInternalPosition(self.record().source.0),
        }
    }
    pub fn target(self) -> NodeBRef<'graph> {
        NodeBRef {
            graph: self.graph,
            internal_position: __NodeBInternalPosition(self.record().target.0),
        }
    }
    pub fn from(self) -> NodeARef<'graph> {
        self.source()
    }
    pub fn to(self) -> NodeBRef<'graph> {
        self.target()
    }
    pub fn from_id(self) -> &'graph NodeAId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph NodeBId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for UnconstrainedNoPayloadRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(UnconstrainedNoPayloadRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
#[derive(Clone, Copy)]
pub struct AtMostOneRef<'graph> {
    graph: &'graph Graph,
    internal_position: __AtMostOneInternalPosition,
}
impl<'graph> AtMostOneRef<'graph> {
    fn record(self) -> &'graph __AtMostOneRecord {
        self.graph
            .at_most_one
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph AtMostOneId {
        self.graph
            .at_most_one
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn src(self) -> NodeARef<'graph> {
        NodeARef {
            graph: self.graph,
            internal_position: __NodeAInternalPosition(self.record().src.0),
        }
    }
    pub fn dst(self) -> NodeBRef<'graph> {
        NodeBRef {
            graph: self.graph,
            internal_position: __NodeBInternalPosition(self.record().dst.0),
        }
    }
    pub fn from(self) -> NodeARef<'graph> {
        self.src()
    }
    pub fn to(self) -> NodeBRef<'graph> {
        self.dst()
    }
    pub fn from_id(self) -> &'graph NodeAId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph NodeBId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for AtMostOneRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(AtMostOneRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
#[derive(Clone, Copy)]
pub struct ExactlyOneRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ExactlyOneInternalPosition,
}
impl<'graph> ExactlyOneRef<'graph> {
    fn record(self) -> &'graph __ExactlyOneRecord {
        self.graph
            .exactly_one
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph ExactlyOneId {
        self.graph
            .exactly_one
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn src(self) -> NodeARef<'graph> {
        NodeARef {
            graph: self.graph,
            internal_position: __NodeAInternalPosition(self.record().src.0),
        }
    }
    pub fn dst(self) -> NodeBRef<'graph> {
        NodeBRef {
            graph: self.graph,
            internal_position: __NodeBInternalPosition(self.record().dst.0),
        }
    }
    pub fn from(self) -> NodeARef<'graph> {
        self.src()
    }
    pub fn to(self) -> NodeBRef<'graph> {
        self.dst()
    }
    pub fn from_id(self) -> &'graph NodeAId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph NodeBId {
        self.to().id()
    }
    pub fn weight(self) -> &'graph Weight {
        &self.record().weight
    }
    pub fn payload(self) -> &'graph Weight {
        &self.record().weight
    }
}
impl<'graph> std::fmt::Debug for ExactlyOneRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ExactlyOneRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
pub struct Builder {
    __graphite_node_node_a: Vec<(NodeAId, super::NodeA)>,
    __graphite_node_node_b: Vec<(NodeBId, super::NodeB)>,
    unconstrained: Vec<(UnconstrainedId, Unconstrained)>,
    unconstrained_no_payload: Vec<(UnconstrainedNoPayloadId, UnconstrainedNoPayload)>,
    at_most_one: Vec<(AtMostOneId, AtMostOne)>,
    exactly_one: Vec<(ExactlyOneId, ExactlyOne)>,
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
/// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
/// `insert_with_id` (許可証不要、名前付き位置を返さない) は独立した
/// 実装を持ち、`insert_named_with_id` を経由しない
/// (`create` のクロージャから許可証なしで呼べる必要があるため)。
pub trait RevQueryInsertable: Sized {
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
pub trait RevQueryDefaultId: RevQueryInsertable {
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
pub trait RevQueryNode: RevQueryInsertable {}
impl RevQueryInsertable for super::NodeA {
    type Id = NodeAId;
    type NamedPosition = __NodeANamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __NodeANamedPosition(
            __NodeAInternalPosition(b.__graphite_node_node_a.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.node_a(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.node_a(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __NodeANamedPosition {
    type Reference<'graph> = NodeARef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        NodeARef {
            graph,
            internal_position: self.0,
        }
    }
}
impl RevQueryDefaultId for super::NodeA {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        RevQueryInsertable::insert_named_with_id(self, b, NodeAId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        RevQueryInsertable::insert_with_id(self, b, NodeAId(binding))
    }
}
impl RevQueryNode for super::NodeA {}
/// 完成済みグラフ上の `#ty` ノード個体。
#[derive(Clone, Copy)]
pub struct NodeARef<'graph> {
    graph: &'graph Graph,
    internal_position: __NodeAInternalPosition,
}
impl<'graph> NodeARef<'graph> {
    pub fn id(self) -> &'graph NodeAId {
        self.graph
            .__graphite_node_node_a
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::NodeA {
        self.graph
            .__graphite_node_node_a
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn unconstrained_as_source(
        self,
    ) -> impl Iterator<Item = UnconstrainedRef<'graph>> + 'graph {
        let positions = self
            .graph
            .unconstrained_from_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| UnconstrainedRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn unconstrained_try_between(
        self,
        other: NodeBRef<'graph>,
    ) -> Result<
        impl Iterator<Item = UnconstrainedRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_unconstrained_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| UnconstrainedRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    pub fn unconstrained_between(
        self,
        other: NodeBRef<'graph>,
    ) -> impl Iterator<Item = UnconstrainedRef<'graph>> + 'graph {
        self.unconstrained_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(NodeARef),
                    stringify!(unconstrained_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn unconstrained_no_payload_as_source(
        self,
    ) -> impl Iterator<Item = UnconstrainedNoPayloadRef<'graph>> + 'graph {
        let positions = self
            .graph
            .unconstrained_no_payload_from_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| UnconstrainedNoPayloadRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn unconstrained_no_payload_try_between(
        self,
        other: NodeBRef<'graph>,
    ) -> Result<
        impl Iterator<Item = UnconstrainedNoPayloadRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_unconstrained_no_payload_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| UnconstrainedNoPayloadRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    pub fn unconstrained_no_payload_between(
        self,
        other: NodeBRef<'graph>,
    ) -> impl Iterator<Item = UnconstrainedNoPayloadRef<'graph>> + 'graph {
        self.unconstrained_no_payload_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(NodeARef),
                    stringify!(unconstrained_no_payload_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn at_most_one_as_src(
        self,
    ) -> impl Iterator<Item = AtMostOneRef<'graph>> + 'graph {
        let positions = self.graph.at_most_one_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| AtMostOneRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn at_most_one_try_between(
        self,
        other: NodeBRef<'graph>,
    ) -> Result<
        impl Iterator<Item = AtMostOneRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_at_most_one_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| AtMostOneRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    pub fn at_most_one_between(
        self,
        other: NodeBRef<'graph>,
    ) -> impl Iterator<Item = AtMostOneRef<'graph>> + 'graph {
        self.at_most_one_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(NodeARef),
                    stringify!(at_most_one_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn exactly_one_as_src(
        self,
    ) -> impl Iterator<Item = ExactlyOneRef<'graph>> + 'graph {
        let positions = self.graph.exactly_one_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ExactlyOneRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn exactly_one_try_between(
        self,
        other: NodeBRef<'graph>,
    ) -> Result<
        impl Iterator<Item = ExactlyOneRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_exactly_one_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| ExactlyOneRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    pub fn exactly_one_between(
        self,
        other: NodeBRef<'graph>,
    ) -> impl Iterator<Item = ExactlyOneRef<'graph>> + 'graph {
        self.exactly_one_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(NodeARef),
                    stringify!(exactly_one_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for NodeARef<'graph> {
    type Target = super::NodeA;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_node_a
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for NodeARef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(NodeARef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
impl RevQueryInsertable for super::NodeB {
    type Id = NodeBId;
    type NamedPosition = __NodeBNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __NodeBNamedPosition(
            __NodeBInternalPosition(b.__graphite_node_node_b.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.node_b(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.node_b(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __NodeBNamedPosition {
    type Reference<'graph> = NodeBRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        NodeBRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl RevQueryDefaultId for super::NodeB {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        RevQueryInsertable::insert_named_with_id(self, b, NodeBId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        RevQueryInsertable::insert_with_id(self, b, NodeBId(binding))
    }
}
impl RevQueryNode for super::NodeB {}
/// 完成済みグラフ上の `#ty` ノード個体。
#[derive(Clone, Copy)]
pub struct NodeBRef<'graph> {
    graph: &'graph Graph,
    internal_position: __NodeBInternalPosition,
}
impl<'graph> NodeBRef<'graph> {
    pub fn id(self) -> &'graph NodeBId {
        self.graph
            .__graphite_node_node_b
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::NodeB {
        self.graph
            .__graphite_node_node_b
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn unconstrained_as_target(
        self,
    ) -> impl Iterator<Item = UnconstrainedRef<'graph>> + 'graph {
        let positions = self.graph.unconstrained_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| UnconstrainedRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn unconstrained_no_payload_as_target(
        self,
    ) -> impl Iterator<Item = UnconstrainedNoPayloadRef<'graph>> + 'graph {
        let positions = self
            .graph
            .unconstrained_no_payload_to_index
            .get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| UnconstrainedNoPayloadRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    pub fn at_most_one_as_dst(self) -> Option<AtMostOneRef<'graph>> {
        self.graph
            .at_most_one_to_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| AtMostOneRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する唯一の辺を O(1)、追加確保なしで返す。
    pub fn exactly_one_as_dst(self) -> ExactlyOneRef<'graph> {
        ExactlyOneRef {
            graph: self.graph,
            internal_position: *self
                .graph
                .exactly_one_to_index
                .get(self.internal_position.0),
        }
    }
}
impl<'graph> std::ops::Deref for NodeBRef<'graph> {
    type Target = super::NodeB;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_node_b
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for NodeBRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(NodeBRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait RevQueryEdge: RevQueryInsertable {}
impl RevQueryInsertable for Unconstrained {
    type Id = UnconstrainedId;
    type NamedPosition = __UnconstrainedNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __UnconstrainedNamedPosition(
            __UnconstrainedInternalPosition(b.unconstrained.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.unconstrained(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.unconstrained(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __UnconstrainedNamedPosition {
    type Reference<'graph> = UnconstrainedRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        UnconstrainedRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl RevQueryDefaultId for Unconstrained {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        RevQueryInsertable::insert_named_with_id(
            self,
            b,
            UnconstrainedId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        RevQueryInsertable::insert_with_id(self, b, UnconstrainedId(binding))
    }
}
impl RevQueryEdge for Unconstrained {}
impl RevQueryInsertable for UnconstrainedNoPayload {
    type Id = UnconstrainedNoPayloadId;
    type NamedPosition = __UnconstrainedNoPayloadNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __UnconstrainedNoPayloadNamedPosition(
            __UnconstrainedNoPayloadInternalPosition(b.unconstrained_no_payload.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.unconstrained_no_payload(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.unconstrained_no_payload(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __UnconstrainedNoPayloadNamedPosition {
    type Reference<'graph> = UnconstrainedNoPayloadRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        UnconstrainedNoPayloadRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl RevQueryDefaultId for UnconstrainedNoPayload {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        RevQueryInsertable::insert_named_with_id(
            self,
            b,
            UnconstrainedNoPayloadId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        RevQueryInsertable::insert_with_id(self, b, UnconstrainedNoPayloadId(binding))
    }
}
impl RevQueryEdge for UnconstrainedNoPayload {}
impl RevQueryInsertable for AtMostOne {
    type Id = AtMostOneId;
    type NamedPosition = __AtMostOneNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __AtMostOneNamedPosition(
            __AtMostOneInternalPosition(b.at_most_one.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.at_most_one(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.at_most_one(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __AtMostOneNamedPosition {
    type Reference<'graph> = AtMostOneRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        AtMostOneRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl RevQueryDefaultId for AtMostOne {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        RevQueryInsertable::insert_named_with_id(self, b, AtMostOneId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        RevQueryInsertable::insert_with_id(self, b, AtMostOneId(binding))
    }
}
impl RevQueryEdge for AtMostOne {}
impl RevQueryInsertable for ExactlyOne {
    type Id = ExactlyOneId;
    type NamedPosition = __ExactlyOneNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ExactlyOneNamedPosition(
            __ExactlyOneInternalPosition(b.exactly_one.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.exactly_one(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.exactly_one(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ExactlyOneNamedPosition {
    type Reference<'graph> = ExactlyOneRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ExactlyOneRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl RevQueryDefaultId for ExactlyOne {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        RevQueryInsertable::insert_named_with_id(self, b, ExactlyOneId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        RevQueryInsertable::insert_with_id(self, b, ExactlyOneId(binding))
    }
}
impl RevQueryEdge for ExactlyOne {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_node_a: Vec::new(),
            __graphite_node_node_b: Vec::new(),
            unconstrained: Vec::new(),
            unconstrained_no_payload: Vec::new(),
            at_most_one: Vec::new(),
            exactly_one: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    pub fn node_a(&mut self, id: NodeAId, value: super::NodeA) -> &mut Self {
        self.__graphite_node_node_a.push((id, value));
        self
    }
    pub fn node_b(&mut self, id: NodeBId, value: super::NodeB) -> &mut Self {
        self.__graphite_node_node_b.push((id, value));
        self
    }
    pub fn unconstrained(
        &mut self,
        id: UnconstrainedId,
        value: Unconstrained,
    ) -> &mut Self {
        self.unconstrained.push((id, value));
        self
    }
    pub fn unconstrained_no_payload(
        &mut self,
        id: UnconstrainedNoPayloadId,
        value: UnconstrainedNoPayload,
    ) -> &mut Self {
        self.unconstrained_no_payload.push((id, value));
        self
    }
    pub fn at_most_one(&mut self, id: AtMostOneId, value: AtMostOne) -> &mut Self {
        self.at_most_one.push((id, value));
        self
    }
    pub fn exactly_one(&mut self, id: ExactlyOneId, value: ExactlyOne) -> &mut Self {
        self.exactly_one.push((id, value));
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
        N: RevQueryNode + RevQueryDefaultId,
    {
        value.insert_with_binding(self, key.into())
    }
    /// `graph!` が公開IDと名前付き要素の内部位置を同時に受け取る経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named<N>(
        &mut self,
        key: impl Into<String>,
        value: N,
        permit: &graphite::NamedInsertPermit,
    ) -> (N::Id, N::NamedPosition)
    where
        N: RevQueryNode + RevQueryDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: RevQueryNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: RevQueryNode>(
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
        E: RevQueryEdge + RevQueryDefaultId,
    {
        value.insert_with_binding(self, key.into())
    }
    /// `graph!` が公開IDと名前付き辺の内部位置を同時に受け取る経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named<E>(
        &mut self,
        key: impl Into<String>,
        value: E,
        permit: &graphite::NamedInsertPermit,
    ) -> (E::Id, E::NamedPosition)
    where
        E: RevQueryEdge + RevQueryDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: RevQueryEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: RevQueryEdge>(
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
        T: RevQueryDefaultId,
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
        let mut __graphite_node_node_a: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_node_a {
            if !__graphite_node_node_a.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateNodeA(id));
            }
        }
        let mut __graphite_node_node_b: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_node_b {
            if !__graphite_node_node_b.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateNodeB(id));
            }
        }
        let mut __graphite_unconstrained: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut unconstrained_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut unconstrained_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_unconstrained_by_pair: std::collections::HashMap<
            (__NodeAInternalPosition, __NodeBInternalPosition),
            Vec<__UnconstrainedInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.unconstrained {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::UnconstrainedDuplicateKey(id));
                continue;
            }
            let Unconstrained { source: from, target: to, weight } = value;
            let from_position = __graphite_node_node_a
                .position(&from)
                .map(__NodeAInternalPosition);
            let to_position = __graphite_node_node_b
                .position(&to)
                .map(__NodeBInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::UnconstrainedUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::UnconstrainedUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __UnconstrainedInternalPosition(
                    __graphite_unconstrained.len(),
                );
                __graphite_unconstrained_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                unconstrained_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                unconstrained_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_unconstrained
                    .insert(
                        id,
                        __UnconstrainedRecord {
                            source: from_position,
                            target: to_position,
                            weight,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_unconstrained_no_payload: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut unconstrained_no_payload_from_index: std::collections::HashMap<
            _,
            Vec<_>,
        > = std::collections::HashMap::new();
        let mut unconstrained_no_payload_to_index: std::collections::HashMap<
            _,
            Vec<_>,
        > = std::collections::HashMap::new();
        let mut __graphite_unconstrained_no_payload_by_pair: std::collections::HashMap<
            (__NodeAInternalPosition, __NodeBInternalPosition),
            Vec<__UnconstrainedNoPayloadInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.unconstrained_no_payload {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::UnconstrainedNoPayloadDuplicateKey(id));
                continue;
            }
            let UnconstrainedNoPayload { source: from, target: to } = value;
            let from_position = __graphite_node_node_a
                .position(&from)
                .map(__NodeAInternalPosition);
            let to_position = __graphite_node_node_b
                .position(&to)
                .map(__NodeBInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::UnconstrainedNoPayloadUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::UnconstrainedNoPayloadUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __UnconstrainedNoPayloadInternalPosition(
                    __graphite_unconstrained_no_payload.len(),
                );
                __graphite_unconstrained_no_payload_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                unconstrained_no_payload_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                unconstrained_no_payload_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_unconstrained_no_payload
                    .insert(
                        id,
                        __UnconstrainedNoPayloadRecord {
                            source: from_position,
                            target: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_at_most_one: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut at_most_one_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut at_most_one_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_at_most_one_by_pair: std::collections::HashMap<
            (__NodeAInternalPosition, __NodeBInternalPosition),
            Vec<__AtMostOneInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.at_most_one {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::AtMostOneDuplicateKey(id));
                continue;
            }
            let AtMostOne { src: from, dst: to } = value;
            let from_position = __graphite_node_node_a
                .position(&from)
                .map(__NodeAInternalPosition);
            let to_position = __graphite_node_node_b
                .position(&to)
                .map(__NodeBInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::AtMostOneUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::AtMostOneUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __AtMostOneInternalPosition(
                    __graphite_at_most_one.len(),
                );
                __graphite_at_most_one_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                at_most_one_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                at_most_one_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_at_most_one
                    .insert(
                        id,
                        __AtMostOneRecord {
                            src: from_position,
                            dst: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&AtMostOne) = |edge| {
            let _ = &edge.dst;
        };
        for position in __graphite_node_node_b.positions() {
            let internal_position = __NodeBInternalPosition(position);
            let key = __graphite_node_node_b
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = at_most_one_to_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::AtMostOneDstEachViolation {
                        target: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_exactly_one: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut exactly_one_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut exactly_one_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_exactly_one_by_pair: std::collections::HashMap<
            (__NodeAInternalPosition, __NodeBInternalPosition),
            Vec<__ExactlyOneInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.exactly_one {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ExactlyOneDuplicateKey(id));
                continue;
            }
            let ExactlyOne { src: from, dst: to, weight } = value;
            let from_position = __graphite_node_node_a
                .position(&from)
                .map(__NodeAInternalPosition);
            let to_position = __graphite_node_node_b
                .position(&to)
                .map(__NodeBInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::ExactlyOneUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::ExactlyOneUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __ExactlyOneInternalPosition(
                    __graphite_exactly_one.len(),
                );
                __graphite_exactly_one_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                exactly_one_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                exactly_one_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_exactly_one
                    .insert(
                        id,
                        __ExactlyOneRecord {
                            src: from_position,
                            dst: to_position,
                            weight,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&ExactlyOne) = |edge| {
            let _ = &edge.dst;
        };
        for position in __graphite_node_node_b.positions() {
            let internal_position = __NodeBInternalPosition(position);
            let key = __graphite_node_node_b
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = exactly_one_to_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if count != 1usize {
                __violations
                    .push(Violation::ExactlyOneDstEachViolation {
                        target: key.clone(),
                        count,
                    });
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let unconstrained_from_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_node_a.len())
                .map(|position| {
                    unconstrained_from_index
                        .remove(&__NodeAInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let unconstrained_to_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_node_b.len())
                .map(|position| {
                    unconstrained_to_index
                        .remove(&__NodeBInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let unconstrained_no_payload_from_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_node_a.len())
                .map(|position| {
                    unconstrained_no_payload_from_index
                        .remove(&__NodeAInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let unconstrained_no_payload_to_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_node_b.len())
                .map(|position| {
                    unconstrained_no_payload_to_index
                        .remove(&__NodeBInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let at_most_one_from_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_node_a.len())
                .map(|position| {
                    at_most_one_from_index
                        .remove(&__NodeAInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let at_most_one_to_index = graphite::OptionalRoleIndex::from_buckets(
            (0..__graphite_node_node_b.len())
                .map(|position| {
                    at_most_one_to_index
                        .remove(&__NodeBInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let exactly_one_from_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_node_a.len())
                .map(|position| {
                    exactly_one_from_index
                        .remove(&__NodeAInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let exactly_one_to_index = graphite::ExactlyOneRoleIndex::from_buckets(
            (0..__graphite_node_node_b.len())
                .map(|position| {
                    exactly_one_to_index
                        .remove(&__NodeBInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_node_a,
            __graphite_node_node_b,
            unconstrained: __graphite_unconstrained,
            unconstrained_no_payload: __graphite_unconstrained_no_payload,
            at_most_one: __graphite_at_most_one,
            exactly_one: __graphite_exactly_one,
            unconstrained_from_index,
            unconstrained_to_index,
            __graphite_unconstrained_by_pair,
            unconstrained_no_payload_from_index,
            unconstrained_no_payload_to_index,
            __graphite_unconstrained_no_payload_by_pair,
            at_most_one_from_index,
            at_most_one_to_index,
            __graphite_at_most_one_by_pair,
            exactly_one_from_index,
            exactly_one_to_index,
            __graphite_exactly_one_by_pair,
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
