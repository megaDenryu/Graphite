// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/schema.rs:60
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    6305705011004229136u64, 214599680133105279u64, 11111355132224751474u64,
    7146795011124316022u64,
];
/// `OrderState` ノードの公開ID。
///
/// 宣言: `src/schema.rs` の `node OrderState`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderStateId(pub String);
/// `Submit` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubmitId(pub String);
/// `Pay` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PayId(pub String);
/// `Ship` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShipId(pub String);
/// `Deliver` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeliverId(pub String);
/// `Cancel` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CancelId(pub String);
/// `Refund` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefundId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __OrderStateInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __SubmitInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PayInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ShipInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __DeliverInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __CancelInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __RefundInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __OrderStateNamedPosition(__OrderStateInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __SubmitNamedPosition(__SubmitInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PayNamedPosition(__PayInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ShipNamedPosition(__ShipInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __DeliverNamedPosition(__DeliverInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __CancelNamedPosition(__CancelInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __RefundNamedPosition(__RefundInternalPosition, u64);
/// 構築時に組み立てる `Submit` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Clone, PartialEq)]
pub struct Submit {
    /// この辺の始点ノードの公開ID。
    pub before: OrderStateId,
    /// この辺の終点ノードの公開ID。
    pub after: OrderStateId,
}
impl Submit {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn new(from: OrderStateId, to: OrderStateId) -> Self {
        Self { before: from, after: to }
    }
}
impl graphite::DirectedEdgeLiteral<OrderStateId, OrderStateId, ()> for Submit {
    fn from_graph_literal(from: OrderStateId, to: OrderStateId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Submit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Submit)).field(&self.before).field(&self.after).finish()
    }
}
/// 構築時に組み立てる `Pay` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Clone, PartialEq)]
pub struct Pay {
    /// この辺の始点ノードの公開ID。
    pub before: OrderStateId,
    /// この辺の終点ノードの公開ID。
    pub after: OrderStateId,
}
impl Pay {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn new(from: OrderStateId, to: OrderStateId) -> Self {
        Self { before: from, after: to }
    }
}
impl graphite::DirectedEdgeLiteral<OrderStateId, OrderStateId, ()> for Pay {
    fn from_graph_literal(from: OrderStateId, to: OrderStateId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Pay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Pay)).field(&self.before).field(&self.after).finish()
    }
}
/// 構築時に組み立てる `Ship` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Clone, PartialEq)]
pub struct Ship {
    /// この辺の始点ノードの公開ID。
    pub before: OrderStateId,
    /// この辺の終点ノードの公開ID。
    pub after: OrderStateId,
}
impl Ship {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn new(from: OrderStateId, to: OrderStateId) -> Self {
        Self { before: from, after: to }
    }
}
impl graphite::DirectedEdgeLiteral<OrderStateId, OrderStateId, ()> for Ship {
    fn from_graph_literal(from: OrderStateId, to: OrderStateId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Ship)).field(&self.before).field(&self.after).finish()
    }
}
/// 構築時に組み立てる `Deliver` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Clone, PartialEq)]
pub struct Deliver {
    /// この辺の始点ノードの公開ID。
    pub before: OrderStateId,
    /// この辺の終点ノードの公開ID。
    pub after: OrderStateId,
}
impl Deliver {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn new(from: OrderStateId, to: OrderStateId) -> Self {
        Self { before: from, after: to }
    }
}
impl graphite::DirectedEdgeLiteral<OrderStateId, OrderStateId, ()> for Deliver {
    fn from_graph_literal(from: OrderStateId, to: OrderStateId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Deliver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Deliver))
            .field(&self.before)
            .field(&self.after)
            .finish()
    }
}
/// 構築時に組み立てる `Cancel` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
#[derive(Clone)]
pub struct Cancel {
    /// この辺の始点ノードの公開ID。
    pub before: OrderStateId,
    /// この辺の終点ノードの公開ID。
    pub after: OrderStateId,
    /// この辺が運ぶ積み荷。
    pub cancellation: CancelEdge,
}
impl Cancel {
    /// 始点と終点の公開IDと積み荷から構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn new(from: OrderStateId, to: OrderStateId, payload: CancelEdge) -> Self {
        Self {
            before: from,
            after: to,
            cancellation: payload,
        }
    }
    /// この辺値が運ぶ積み荷を借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn payload(&self) -> &CancelEdge {
        &self.cancellation
    }
}
impl graphite::DirectedEdgeLiteral<OrderStateId, OrderStateId, CancelEdge> for Cancel {
    fn from_graph_literal(
        from: OrderStateId,
        to: OrderStateId,
        payload: CancelEdge,
    ) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Cancel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Cancel))
    }
}
/// 構築時に組み立てる `Refund` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
#[derive(Clone)]
pub struct Refund {
    /// この辺の始点ノードの公開ID。
    pub before: OrderStateId,
    /// この辺の終点ノードの公開ID。
    pub after: OrderStateId,
    /// この辺が運ぶ積み荷。
    pub refund: RefundEdge,
}
impl Refund {
    /// 始点と終点の公開IDと積み荷から構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn new(from: OrderStateId, to: OrderStateId, payload: RefundEdge) -> Self {
        Self {
            before: from,
            after: to,
            refund: payload,
        }
    }
    /// この辺値が運ぶ積み荷を借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn payload(&self) -> &RefundEdge {
        &self.refund
    }
}
impl graphite::DirectedEdgeLiteral<OrderStateId, OrderStateId, RefundEdge> for Refund {
    fn from_graph_literal(
        from: OrderStateId,
        to: OrderStateId,
        payload: RefundEdge,
    ) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Refund {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Refund))
    }
}
#[allow(dead_code)]
struct __SubmitRecord {
    before: __OrderStateInternalPosition,
    after: __OrderStateInternalPosition,
}
#[allow(dead_code)]
struct __PayRecord {
    before: __OrderStateInternalPosition,
    after: __OrderStateInternalPosition,
}
#[allow(dead_code)]
struct __ShipRecord {
    before: __OrderStateInternalPosition,
    after: __OrderStateInternalPosition,
}
#[allow(dead_code)]
struct __DeliverRecord {
    before: __OrderStateInternalPosition,
    after: __OrderStateInternalPosition,
}
#[allow(dead_code)]
struct __CancelRecord {
    before: __OrderStateInternalPosition,
    after: __OrderStateInternalPosition,
    cancellation: CancelEdge,
}
#[allow(dead_code)]
struct __RefundRecord {
    before: __OrderStateInternalPosition,
    after: __OrderStateInternalPosition,
    refund: RefundEdge,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `src/schema.rs` の `schema OrderFsm`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    DuplicateOrderState(OrderStateId),
    /// このエッジ種別のキーが重複している。
    SubmitDuplicateKey(SubmitId),
    /// このエッジが未知の始点キーを参照している。
    SubmitUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: SubmitId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: OrderStateId,
    },
    /// このエッジが未知の終点キーを参照している。
    SubmitUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: SubmitId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: OrderStateId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    SubmitBeforeEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: OrderStateId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
    /// このエッジ種別のキーが重複している。
    PayDuplicateKey(PayId),
    /// このエッジが未知の始点キーを参照している。
    PayUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: PayId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: OrderStateId,
    },
    /// このエッジが未知の終点キーを参照している。
    PayUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: PayId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: OrderStateId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    PayBeforeEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: OrderStateId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
    /// このエッジ種別のキーが重複している。
    ShipDuplicateKey(ShipId),
    /// このエッジが未知の始点キーを参照している。
    ShipUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: ShipId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: OrderStateId,
    },
    /// このエッジが未知の終点キーを参照している。
    ShipUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: ShipId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: OrderStateId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    ShipBeforeEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: OrderStateId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
    /// このエッジ種別のキーが重複している。
    DeliverDuplicateKey(DeliverId),
    /// このエッジが未知の始点キーを参照している。
    DeliverUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: DeliverId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: OrderStateId,
    },
    /// このエッジが未知の終点キーを参照している。
    DeliverUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: DeliverId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: OrderStateId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    DeliverBeforeEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: OrderStateId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
    /// このエッジ種別のキーが重複している。
    CancelDuplicateKey(CancelId),
    /// このエッジが未知の始点キーを参照している。
    CancelUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: CancelId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: OrderStateId,
    },
    /// このエッジが未知の終点キーを参照している。
    CancelUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: CancelId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: OrderStateId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    CancelBeforeEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: OrderStateId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
    /// このエッジ種別のキーが重複している。
    RefundDuplicateKey(RefundId),
    /// このエッジが未知の始点キーを参照している。
    RefundUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: RefundId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: OrderStateId,
    },
    /// このエッジが未知の終点キーを参照している。
    RefundUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: RefundId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: OrderStateId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    RefundBeforeEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: OrderStateId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicateOrderState(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "OrderState", id)
            }
            Violation::SubmitDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Submit", id)
            }
            Violation::SubmitUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Submit", edge, "OrderState", source
                )
            }
            Violation::SubmitUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Submit", edge, "OrderState", target
                )
            }
            Violation::SubmitBeforeEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Submit", "OrderState", source, "0..1", count
                )
            }
            Violation::PayDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Pay", id)
            }
            Violation::PayUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Pay", edge, "OrderState", source
                )
            }
            Violation::PayUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Pay", edge, "OrderState", target
                )
            }
            Violation::PayBeforeEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Pay", "OrderState", source, "0..1", count
                )
            }
            Violation::ShipDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Ship", id)
            }
            Violation::ShipUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Ship", edge, "OrderState", source
                )
            }
            Violation::ShipUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Ship", edge, "OrderState", target
                )
            }
            Violation::ShipBeforeEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Ship", "OrderState", source, "0..1", count
                )
            }
            Violation::DeliverDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Deliver", id)
            }
            Violation::DeliverUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Deliver", edge, "OrderState", source
                )
            }
            Violation::DeliverUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Deliver", edge, "OrderState", target
                )
            }
            Violation::DeliverBeforeEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Deliver", "OrderState", source, "0..1", count
                )
            }
            Violation::CancelDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Cancel", id)
            }
            Violation::CancelUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Cancel", edge, "OrderState", source
                )
            }
            Violation::CancelUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Cancel", edge, "OrderState", target
                )
            }
            Violation::CancelBeforeEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Cancel", "OrderState", source, "0..1", count
                )
            }
            Violation::RefundDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Refund", id)
            }
            Violation::RefundUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Refund", edge, "OrderState", source
                )
            }
            Violation::RefundUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Refund", edge, "OrderState", target
                )
            }
            Violation::RefundBeforeEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Refund", "OrderState", source, "0..1", count
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
/// 宣言: `src/schema.rs` の `schema OrderFsm`
pub struct Graph {
    __graphite_node_order_state: graphite::KeyedTable<OrderStateId, super::OrderState>,
    submit: graphite::KeyedTable<SubmitId, __SubmitRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    submit_from_index: graphite::OptionalRoleIndex<__SubmitInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    submit_to_index: graphite::MultipleRoleIndex<__SubmitInternalPosition>,
    __graphite_submit_by_pair: std::collections::HashMap<
        (__OrderStateInternalPosition, __OrderStateInternalPosition),
        Vec<__SubmitInternalPosition>,
    >,
    pay: graphite::KeyedTable<PayId, __PayRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    pay_from_index: graphite::OptionalRoleIndex<__PayInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    pay_to_index: graphite::MultipleRoleIndex<__PayInternalPosition>,
    __graphite_pay_by_pair: std::collections::HashMap<
        (__OrderStateInternalPosition, __OrderStateInternalPosition),
        Vec<__PayInternalPosition>,
    >,
    ship: graphite::KeyedTable<ShipId, __ShipRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    ship_from_index: graphite::OptionalRoleIndex<__ShipInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    ship_to_index: graphite::MultipleRoleIndex<__ShipInternalPosition>,
    __graphite_ship_by_pair: std::collections::HashMap<
        (__OrderStateInternalPosition, __OrderStateInternalPosition),
        Vec<__ShipInternalPosition>,
    >,
    deliver: graphite::KeyedTable<DeliverId, __DeliverRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    deliver_from_index: graphite::OptionalRoleIndex<__DeliverInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    deliver_to_index: graphite::MultipleRoleIndex<__DeliverInternalPosition>,
    __graphite_deliver_by_pair: std::collections::HashMap<
        (__OrderStateInternalPosition, __OrderStateInternalPosition),
        Vec<__DeliverInternalPosition>,
    >,
    cancel: graphite::KeyedTable<CancelId, __CancelRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    cancel_from_index: graphite::OptionalRoleIndex<__CancelInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    cancel_to_index: graphite::MultipleRoleIndex<__CancelInternalPosition>,
    __graphite_cancel_by_pair: std::collections::HashMap<
        (__OrderStateInternalPosition, __OrderStateInternalPosition),
        Vec<__CancelInternalPosition>,
    >,
    refund: graphite::KeyedTable<RefundId, __RefundRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    refund_from_index: graphite::OptionalRoleIndex<__RefundInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    refund_to_index: graphite::MultipleRoleIndex<__RefundInternalPosition>,
    __graphite_refund_by_pair: std::collections::HashMap<
        (__OrderStateInternalPosition, __OrderStateInternalPosition),
        Vec<__RefundInternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `node OrderState`
    pub fn order_state_by_id<'graph>(
        &'graph self,
        id: &OrderStateId,
    ) -> Option<OrderStateRef<'graph>> {
        let internal_position = __OrderStateInternalPosition(
            self.__graphite_node_order_state.position(id)?,
        );
        Some(OrderStateRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `src/schema.rs` の `node OrderState`
    pub fn order_state_value_mut(
        &mut self,
        id: &OrderStateId,
    ) -> Option<&mut super::OrderState> {
        self.__graphite_node_order_state.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `node OrderState`
    pub fn order_state_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph OrderStateId> {
        self.__graphite_node_order_state.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `node OrderState`
    pub fn order_state_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = OrderStateRef<'graph>> + 'graph {
        self.__graphite_node_order_state
            .positions()
            .map(move |position| OrderStateRef {
                graph: self,
                internal_position: __OrderStateInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `node OrderState`
    pub fn order_state_len(&self) -> usize {
        self.__graphite_node_order_state.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn submit_by_id<'graph>(
        &'graph self,
        id: &SubmitId,
    ) -> Option<SubmitRef<'graph>> {
        Some(SubmitRef {
            graph: self,
            internal_position: __SubmitInternalPosition(self.submit.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn submit_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph SubmitId> {
        self.submit.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn submit_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = SubmitRef<'graph>> + 'graph {
        self.submit
            .positions()
            .map(move |position| SubmitRef {
                graph: self,
                internal_position: __SubmitInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn submit_len(&self) -> usize {
        self.submit.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn pay_by_id<'graph>(&'graph self, id: &PayId) -> Option<PayRef<'graph>> {
        Some(PayRef {
            graph: self,
            internal_position: __PayInternalPosition(self.pay.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn pay_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph PayId> {
        self.pay.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn pay_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = PayRef<'graph>> + 'graph {
        self.pay
            .positions()
            .map(move |position| PayRef {
                graph: self,
                internal_position: __PayInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn pay_len(&self) -> usize {
        self.pay.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn ship_by_id<'graph>(&'graph self, id: &ShipId) -> Option<ShipRef<'graph>> {
        Some(ShipRef {
            graph: self,
            internal_position: __ShipInternalPosition(self.ship.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn ship_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph ShipId> {
        self.ship.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn ship_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ShipRef<'graph>> + 'graph {
        self.ship
            .positions()
            .map(move |position| ShipRef {
                graph: self,
                internal_position: __ShipInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn ship_len(&self) -> usize {
        self.ship.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn deliver_by_id<'graph>(
        &'graph self,
        id: &DeliverId,
    ) -> Option<DeliverRef<'graph>> {
        Some(DeliverRef {
            graph: self,
            internal_position: __DeliverInternalPosition(self.deliver.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn deliver_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph DeliverId> {
        self.deliver.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn deliver_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = DeliverRef<'graph>> + 'graph {
        self.deliver
            .positions()
            .map(move |position| DeliverRef {
                graph: self,
                internal_position: __DeliverInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn deliver_len(&self) -> usize {
        self.deliver.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel_by_id<'graph>(
        &'graph self,
        id: &CancelId,
    ) -> Option<CancelRef<'graph>> {
        Some(CancelRef {
            graph: self,
            internal_position: __CancelInternalPosition(self.cancel.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel_payload_mut(&mut self, id: &CancelId) -> Option<&mut CancelEdge> {
        self.cancel
            .get_mut(id)
            .map(|record: &mut __CancelRecord| &mut record.cancellation)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph CancelId> {
        self.cancel.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = CancelRef<'graph>> + 'graph {
        self.cancel
            .positions()
            .map(move |position| CancelRef {
                graph: self,
                internal_position: __CancelInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel_len(&self) -> usize {
        self.cancel.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund_by_id<'graph>(
        &'graph self,
        id: &RefundId,
    ) -> Option<RefundRef<'graph>> {
        Some(RefundRef {
            graph: self,
            internal_position: __RefundInternalPosition(self.refund.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund_payload_mut(&mut self, id: &RefundId) -> Option<&mut RefundEdge> {
        self.refund.get_mut(id).map(|record: &mut __RefundRecord| &mut record.refund)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph RefundId> {
        self.refund.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = RefundRef<'graph>> + 'graph {
        self.refund
            .positions()
            .map(move |position| RefundRef {
                graph: self,
                internal_position: __RefundInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund_len(&self) -> usize {
        self.refund.len()
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
/// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Clone, Copy)]
pub struct SubmitRef<'graph> {
    graph: &'graph Graph,
    internal_position: __SubmitInternalPosition,
}
impl<'graph> SubmitRef<'graph> {
    fn record(self) -> &'graph __SubmitRecord {
        self.graph
            .submit
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn id(self) -> &'graph SubmitId {
        self.graph
            .submit
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn before(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().before.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn after(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().after.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn from(self) -> OrderStateRef<'graph> {
        self.before()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn to(self) -> OrderStateRef<'graph> {
        self.after()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn from_id(self) -> &'graph OrderStateId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn to_id(self) -> &'graph OrderStateId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for SubmitRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(SubmitRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Clone, Copy)]
pub struct PayRef<'graph> {
    graph: &'graph Graph,
    internal_position: __PayInternalPosition,
}
impl<'graph> PayRef<'graph> {
    fn record(self) -> &'graph __PayRecord {
        self.graph
            .pay
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn id(self) -> &'graph PayId {
        self.graph
            .pay
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn before(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().before.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn after(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().after.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn from(self) -> OrderStateRef<'graph> {
        self.before()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn to(self) -> OrderStateRef<'graph> {
        self.after()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn from_id(self) -> &'graph OrderStateId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn to_id(self) -> &'graph OrderStateId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for PayRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(PayRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Clone, Copy)]
pub struct ShipRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ShipInternalPosition,
}
impl<'graph> ShipRef<'graph> {
    fn record(self) -> &'graph __ShipRecord {
        self.graph
            .ship
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn id(self) -> &'graph ShipId {
        self.graph
            .ship
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn before(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().before.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn after(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().after.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn from(self) -> OrderStateRef<'graph> {
        self.before()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn to(self) -> OrderStateRef<'graph> {
        self.after()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn from_id(self) -> &'graph OrderStateId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn to_id(self) -> &'graph OrderStateId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for ShipRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ShipRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
#[derive(Clone, Copy)]
pub struct DeliverRef<'graph> {
    graph: &'graph Graph,
    internal_position: __DeliverInternalPosition,
}
impl<'graph> DeliverRef<'graph> {
    fn record(self) -> &'graph __DeliverRecord {
        self.graph
            .deliver
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn id(self) -> &'graph DeliverId {
        self.graph
            .deliver
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn before(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().before.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn after(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().after.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn from(self) -> OrderStateRef<'graph> {
        self.before()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn to(self) -> OrderStateRef<'graph> {
        self.after()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn from_id(self) -> &'graph OrderStateId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn to_id(self) -> &'graph OrderStateId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for DeliverRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(DeliverRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
#[derive(Clone, Copy)]
pub struct CancelRef<'graph> {
    graph: &'graph Graph,
    internal_position: __CancelInternalPosition,
}
impl<'graph> CancelRef<'graph> {
    fn record(self) -> &'graph __CancelRecord {
        self.graph
            .cancel
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn id(self) -> &'graph CancelId {
        self.graph
            .cancel
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn before(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().before.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn after(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().after.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn from(self) -> OrderStateRef<'graph> {
        self.before()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn to(self) -> OrderStateRef<'graph> {
        self.after()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn from_id(self) -> &'graph OrderStateId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn to_id(self) -> &'graph OrderStateId {
        self.to().id()
    }
    /// この辺個体が運ぶ積み荷を役割名で借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancellation(self) -> &'graph CancelEdge {
        &self.record().cancellation
    }
    /// この辺個体が運ぶ積み荷を、役割名によらない固定名で借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn payload(self) -> &'graph CancelEdge {
        &self.record().cancellation
    }
}
impl<'graph> std::fmt::Debug for CancelRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(CancelRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
#[derive(Clone, Copy)]
pub struct RefundRef<'graph> {
    graph: &'graph Graph,
    internal_position: __RefundInternalPosition,
}
impl<'graph> RefundRef<'graph> {
    fn record(self) -> &'graph __RefundRecord {
        self.graph
            .refund
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn id(self) -> &'graph RefundId {
        self.graph
            .refund
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn before(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().before.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn after(self) -> OrderStateRef<'graph> {
        OrderStateRef {
            graph: self.graph,
            internal_position: __OrderStateInternalPosition(self.record().after.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn from(self) -> OrderStateRef<'graph> {
        self.before()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn to(self) -> OrderStateRef<'graph> {
        self.after()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn from_id(self) -> &'graph OrderStateId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn to_id(self) -> &'graph OrderStateId {
        self.to().id()
    }
    /// この辺個体が運ぶ積み荷を役割名で借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund(self) -> &'graph RefundEdge {
        &self.record().refund
    }
    /// この辺個体が運ぶ積み荷を、役割名によらない固定名で借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn payload(self) -> &'graph RefundEdge {
        &self.record().refund
    }
}
impl<'graph> std::fmt::Debug for RefundRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(RefundRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
/// 制約検査を一切行わない。
///
/// 宣言: `src/schema.rs` の `schema OrderFsm`
pub struct Builder {
    __graphite_node_order_state: Vec<(OrderStateId, super::OrderState)>,
    submit: Vec<(SubmitId, Submit)>,
    pay: Vec<(PayId, Pay)>,
    ship: Vec<(ShipId, Ship)>,
    deliver: Vec<(DeliverId, Deliver)>,
    cancel: Vec<(CancelId, Cancel)>,
    refund: Vec<(RefundId, Refund)>,
    /// この構築を識別する構築印。`Builder::new()` が発行し、この
    /// `Builder` から挿入する全ての名前付き位置と、凍結成功後の
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
pub trait OrderFsmInsertable: Sized {
    /// この要素を挿入したときに受け取る公開ID型。
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
    /// 型付きの公開IDを指定して、この要素を `Builder` へ挿入する。
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id;
}
/// 束縛名の文字列からスキーマ内限定の既定IDを作れる要素だけが
/// 実装する。明示ID型には実装せず、文字列変換を要求しない。
pub trait OrderFsmDefaultId: OrderFsmInsertable {
    #[doc(hidden)]
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    /// 束縛名の文字列から既定IDを作り、この要素を `Builder` へ挿入する。
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id;
}
/// ノード挿入で使うトレイト境界。読み取りは `Graph` の種別メソッドと
/// `NodeRef` のメソッドが提供する。利用者がこのトレイトのメソッドを
/// 直接呼ぶことは想定しない。
pub trait OrderFsmNode: OrderFsmInsertable {}
impl OrderFsmInsertable for super::OrderState {
    type Id = OrderStateId;
    type NamedPosition = __OrderStateNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __OrderStateNamedPosition(
            __OrderStateInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_order_state.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.order_state(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.order_state(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __OrderStateNamedPosition {
    type Reference<'graph> = OrderStateRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        OrderStateRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrderFsmDefaultId for super::OrderState {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrderFsmInsertable::insert_named_with_id(self, b, OrderStateId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrderFsmInsertable::insert_with_id(self, b, OrderStateId(binding))
    }
}
impl OrderFsmNode for super::OrderState {}
/// 完成済みグラフ上の `OrderState` ノード個体。
///
/// 宣言: `src/schema.rs` の `node OrderState`
#[derive(Clone, Copy)]
pub struct OrderStateRef<'graph> {
    graph: &'graph Graph,
    internal_position: __OrderStateInternalPosition,
}
impl<'graph> OrderStateRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `node OrderState`
    pub fn id(self) -> &'graph OrderStateId {
        self.graph
            .__graphite_node_order_state
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `src/schema.rs` の `node OrderState`
    pub fn value(self) -> &'graph super::OrderState {
        self.graph
            .__graphite_node_order_state
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn submit_as_before(self) -> Option<SubmitRef<'graph>> {
        self.graph
            .submit_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| SubmitRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn submit_as_after(self) -> impl Iterator<Item = SubmitRef<'graph>> + 'graph {
        let positions = self.graph.submit_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| SubmitRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn submit_try_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> Result<
        impl Iterator<Item = SubmitRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_submit_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| SubmitRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::submit_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn submit_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> impl Iterator<Item = SubmitRef<'graph>> + 'graph {
        self.submit_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(OrderStateRef),
                    stringify!(submit_between)
                )
            })
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn pay_as_before(self) -> Option<PayRef<'graph>> {
        self.graph
            .pay_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| PayRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn pay_as_after(self) -> impl Iterator<Item = PayRef<'graph>> + 'graph {
        let positions = self.graph.pay_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| PayRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn pay_try_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> Result<impl Iterator<Item = PayRef<'graph>> + 'graph, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_pay_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| PayRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::pay_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn pay_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> impl Iterator<Item = PayRef<'graph>> + 'graph {
        self.pay_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(OrderStateRef), stringify!(pay_between)
                )
            })
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn ship_as_before(self) -> Option<ShipRef<'graph>> {
        self.graph
            .ship_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| ShipRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn ship_as_after(self) -> impl Iterator<Item = ShipRef<'graph>> + 'graph {
        let positions = self.graph.ship_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ShipRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn ship_try_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> Result<
        impl Iterator<Item = ShipRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_ship_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| ShipRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::ship_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn ship_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> impl Iterator<Item = ShipRef<'graph>> + 'graph {
        self.ship_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(OrderStateRef),
                    stringify!(ship_between)
                )
            })
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn deliver_as_before(self) -> Option<DeliverRef<'graph>> {
        self.graph
            .deliver_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| DeliverRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn deliver_as_after(self) -> impl Iterator<Item = DeliverRef<'graph>> + 'graph {
        let positions = self.graph.deliver_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| DeliverRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn deliver_try_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> Result<
        impl Iterator<Item = DeliverRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_deliver_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| DeliverRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::deliver_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn deliver_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> impl Iterator<Item = DeliverRef<'graph>> + 'graph {
        self.deliver_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(OrderStateRef),
                    stringify!(deliver_between)
                )
            })
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel_as_before(self) -> Option<CancelRef<'graph>> {
        self.graph
            .cancel_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| CancelRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel_as_after(self) -> impl Iterator<Item = CancelRef<'graph>> + 'graph {
        let positions = self.graph.cancel_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| CancelRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel_try_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> Result<
        impl Iterator<Item = CancelRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_cancel_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| CancelRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::cancel_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> impl Iterator<Item = CancelRef<'graph>> + 'graph {
        self.cancel_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(OrderStateRef),
                    stringify!(cancel_between)
                )
            })
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund_as_before(self) -> Option<RefundRef<'graph>> {
        self.graph
            .refund_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| RefundRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund_as_after(self) -> impl Iterator<Item = RefundRef<'graph>> + 'graph {
        let positions = self.graph.refund_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| RefundRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund_try_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> Result<
        impl Iterator<Item = RefundRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_refund_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| RefundRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::refund_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund_between(
        self,
        other: OrderStateRef<'graph>,
    ) -> impl Iterator<Item = RefundRef<'graph>> + 'graph {
        self.refund_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(OrderStateRef),
                    stringify!(refund_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for OrderStateRef<'graph> {
    type Target = super::OrderState;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_order_state
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for OrderStateRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(OrderStateRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait OrderFsmEdge: OrderFsmInsertable {}
impl OrderFsmInsertable for Submit {
    type Id = SubmitId;
    type NamedPosition = __SubmitNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __SubmitNamedPosition(
            __SubmitInternalPosition(
                graphite::TablePosition::from_index(b.submit.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.submit(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.submit(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __SubmitNamedPosition {
    type Reference<'graph> = SubmitRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        SubmitRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrderFsmDefaultId for Submit {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrderFsmInsertable::insert_named_with_id(self, b, SubmitId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrderFsmInsertable::insert_with_id(self, b, SubmitId(binding))
    }
}
impl OrderFsmEdge for Submit {}
impl OrderFsmInsertable for Pay {
    type Id = PayId;
    type NamedPosition = __PayNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __PayNamedPosition(
            __PayInternalPosition(graphite::TablePosition::from_index(b.pay.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.pay(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.pay(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __PayNamedPosition {
    type Reference<'graph> = PayRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        PayRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrderFsmDefaultId for Pay {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrderFsmInsertable::insert_named_with_id(self, b, PayId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrderFsmInsertable::insert_with_id(self, b, PayId(binding))
    }
}
impl OrderFsmEdge for Pay {}
impl OrderFsmInsertable for Ship {
    type Id = ShipId;
    type NamedPosition = __ShipNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ShipNamedPosition(
            __ShipInternalPosition(graphite::TablePosition::from_index(b.ship.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.ship(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.ship(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ShipNamedPosition {
    type Reference<'graph> = ShipRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ShipRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrderFsmDefaultId for Ship {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrderFsmInsertable::insert_named_with_id(self, b, ShipId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrderFsmInsertable::insert_with_id(self, b, ShipId(binding))
    }
}
impl OrderFsmEdge for Ship {}
impl OrderFsmInsertable for Deliver {
    type Id = DeliverId;
    type NamedPosition = __DeliverNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __DeliverNamedPosition(
            __DeliverInternalPosition(
                graphite::TablePosition::from_index(b.deliver.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.deliver(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.deliver(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __DeliverNamedPosition {
    type Reference<'graph> = DeliverRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        DeliverRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrderFsmDefaultId for Deliver {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrderFsmInsertable::insert_named_with_id(self, b, DeliverId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrderFsmInsertable::insert_with_id(self, b, DeliverId(binding))
    }
}
impl OrderFsmEdge for Deliver {}
impl OrderFsmInsertable for Cancel {
    type Id = CancelId;
    type NamedPosition = __CancelNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __CancelNamedPosition(
            __CancelInternalPosition(
                graphite::TablePosition::from_index(b.cancel.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.cancel(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.cancel(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __CancelNamedPosition {
    type Reference<'graph> = CancelRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        CancelRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrderFsmDefaultId for Cancel {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrderFsmInsertable::insert_named_with_id(self, b, CancelId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrderFsmInsertable::insert_with_id(self, b, CancelId(binding))
    }
}
impl OrderFsmEdge for Cancel {}
impl OrderFsmInsertable for Refund {
    type Id = RefundId;
    type NamedPosition = __RefundNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __RefundNamedPosition(
            __RefundInternalPosition(
                graphite::TablePosition::from_index(b.refund.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.refund(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.refund(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __RefundNamedPosition {
    type Reference<'graph> = RefundRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        RefundRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrderFsmDefaultId for Refund {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrderFsmInsertable::insert_named_with_id(self, b, RefundId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrderFsmInsertable::insert_with_id(self, b, RefundId(binding))
    }
}
impl OrderFsmEdge for Refund {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_order_state: Vec::new(),
            submit: Vec::new(),
            pay: Vec::new(),
            ship: Vec::new(),
            deliver: Vec::new(),
            cancel: Vec::new(),
            refund: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `node OrderState`
    pub fn order_state(
        &mut self,
        id: OrderStateId,
        value: super::OrderState,
    ) -> &mut Self {
        self.__graphite_node_order_state.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Submit = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn submit(&mut self, id: SubmitId, value: Submit) -> &mut Self {
        self.submit.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Pay = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn pay(&mut self, id: PayId, value: Pay) -> &mut Self {
        self.pay.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Ship = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn ship(&mut self, id: ShipId, value: Ship) -> &mut Self {
        self.ship.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Deliver = (before: OrderState) -> (after: OrderState) where each before: 0..1`
    pub fn deliver(&mut self, id: DeliverId, value: Deliver) -> &mut Self {
        self.deliver.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Cancel = (before: OrderState) -[cancellation: CancelEdge]-> (after: OrderState) where each before: 0..1`
    pub fn cancel(&mut self, id: CancelId, value: Cancel) -> &mut Self {
        self.cancel.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Refund = (before: OrderState) -[refund: RefundEdge]-> (after: OrderState) where each before: 0..1`
    pub fn refund(&mut self, id: RefundId, value: Refund) -> &mut Self {
        self.refund.push((id, value));
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
        N: OrderFsmNode + OrderFsmDefaultId,
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
        N: OrderFsmNode + OrderFsmDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: OrderFsmNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: OrderFsmNode>(
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
        E: OrderFsmEdge + OrderFsmDefaultId,
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
        E: OrderFsmEdge + OrderFsmDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: OrderFsmEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: OrderFsmEdge>(
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
        T: OrderFsmDefaultId,
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
        let mut __graphite_node_order_state: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_order_state {
            if !__graphite_node_order_state.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateOrderState(id));
            }
        }
        let mut __graphite_submit: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut submit_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut submit_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_submit_by_pair: std::collections::HashMap<
            (__OrderStateInternalPosition, __OrderStateInternalPosition),
            Vec<__SubmitInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.submit {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::SubmitDuplicateKey(id));
                continue;
            }
            let Submit { before: from, after: to } = value;
            let from_position = __graphite_node_order_state
                .position(&from)
                .map(__OrderStateInternalPosition);
            let to_position = __graphite_node_order_state
                .position(&to)
                .map(__OrderStateInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::SubmitUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::SubmitUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __SubmitInternalPosition(
                    graphite::TablePosition::from_index(__graphite_submit.len()),
                );
                __graphite_submit_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                submit_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                submit_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_submit
                    .insert(
                        id,
                        __SubmitRecord {
                            before: from_position,
                            after: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Submit) = |edge| {
            let _ = &edge.before;
        };
        for position in __graphite_node_order_state.positions() {
            let internal_position = __OrderStateInternalPosition(position);
            let key = __graphite_node_order_state
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = submit_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::SubmitBeforeEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_pay: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut pay_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut pay_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_pay_by_pair: std::collections::HashMap<
            (__OrderStateInternalPosition, __OrderStateInternalPosition),
            Vec<__PayInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.pay {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::PayDuplicateKey(id));
                continue;
            }
            let Pay { before: from, after: to } = value;
            let from_position = __graphite_node_order_state
                .position(&from)
                .map(__OrderStateInternalPosition);
            let to_position = __graphite_node_order_state
                .position(&to)
                .map(__OrderStateInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::PayUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::PayUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __PayInternalPosition(
                    graphite::TablePosition::from_index(__graphite_pay.len()),
                );
                __graphite_pay_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                pay_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                pay_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_pay
                    .insert(
                        id,
                        __PayRecord {
                            before: from_position,
                            after: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Pay) = |edge| {
            let _ = &edge.before;
        };
        for position in __graphite_node_order_state.positions() {
            let internal_position = __OrderStateInternalPosition(position);
            let key = __graphite_node_order_state
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = pay_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::PayBeforeEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_ship: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut ship_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut ship_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_ship_by_pair: std::collections::HashMap<
            (__OrderStateInternalPosition, __OrderStateInternalPosition),
            Vec<__ShipInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.ship {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ShipDuplicateKey(id));
                continue;
            }
            let Ship { before: from, after: to } = value;
            let from_position = __graphite_node_order_state
                .position(&from)
                .map(__OrderStateInternalPosition);
            let to_position = __graphite_node_order_state
                .position(&to)
                .map(__OrderStateInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::ShipUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::ShipUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __ShipInternalPosition(
                    graphite::TablePosition::from_index(__graphite_ship.len()),
                );
                __graphite_ship_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                ship_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                ship_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_ship
                    .insert(
                        id,
                        __ShipRecord {
                            before: from_position,
                            after: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Ship) = |edge| {
            let _ = &edge.before;
        };
        for position in __graphite_node_order_state.positions() {
            let internal_position = __OrderStateInternalPosition(position);
            let key = __graphite_node_order_state
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = ship_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::ShipBeforeEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_deliver: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut deliver_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut deliver_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_deliver_by_pair: std::collections::HashMap<
            (__OrderStateInternalPosition, __OrderStateInternalPosition),
            Vec<__DeliverInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.deliver {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::DeliverDuplicateKey(id));
                continue;
            }
            let Deliver { before: from, after: to } = value;
            let from_position = __graphite_node_order_state
                .position(&from)
                .map(__OrderStateInternalPosition);
            let to_position = __graphite_node_order_state
                .position(&to)
                .map(__OrderStateInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::DeliverUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::DeliverUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __DeliverInternalPosition(
                    graphite::TablePosition::from_index(__graphite_deliver.len()),
                );
                __graphite_deliver_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                deliver_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                deliver_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_deliver
                    .insert(
                        id,
                        __DeliverRecord {
                            before: from_position,
                            after: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Deliver) = |edge| {
            let _ = &edge.before;
        };
        for position in __graphite_node_order_state.positions() {
            let internal_position = __OrderStateInternalPosition(position);
            let key = __graphite_node_order_state
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = deliver_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::DeliverBeforeEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_cancel: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut cancel_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut cancel_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_cancel_by_pair: std::collections::HashMap<
            (__OrderStateInternalPosition, __OrderStateInternalPosition),
            Vec<__CancelInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.cancel {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::CancelDuplicateKey(id));
                continue;
            }
            let Cancel { before: from, after: to, cancellation } = value;
            let from_position = __graphite_node_order_state
                .position(&from)
                .map(__OrderStateInternalPosition);
            let to_position = __graphite_node_order_state
                .position(&to)
                .map(__OrderStateInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::CancelUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::CancelUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __CancelInternalPosition(
                    graphite::TablePosition::from_index(__graphite_cancel.len()),
                );
                __graphite_cancel_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                cancel_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                cancel_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_cancel
                    .insert(
                        id,
                        __CancelRecord {
                            before: from_position,
                            after: to_position,
                            cancellation,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Cancel) = |edge| {
            let _ = &edge.before;
        };
        for position in __graphite_node_order_state.positions() {
            let internal_position = __OrderStateInternalPosition(position);
            let key = __graphite_node_order_state
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = cancel_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::CancelBeforeEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_refund: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut refund_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut refund_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_refund_by_pair: std::collections::HashMap<
            (__OrderStateInternalPosition, __OrderStateInternalPosition),
            Vec<__RefundInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.refund {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::RefundDuplicateKey(id));
                continue;
            }
            let Refund { before: from, after: to, refund } = value;
            let from_position = __graphite_node_order_state
                .position(&from)
                .map(__OrderStateInternalPosition);
            let to_position = __graphite_node_order_state
                .position(&to)
                .map(__OrderStateInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::RefundUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::RefundUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __RefundInternalPosition(
                    graphite::TablePosition::from_index(__graphite_refund.len()),
                );
                __graphite_refund_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                refund_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                refund_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_refund
                    .insert(
                        id,
                        __RefundRecord {
                            before: from_position,
                            after: to_position,
                            refund,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Refund) = |edge| {
            let _ = &edge.before;
        };
        for position in __graphite_node_order_state.positions() {
            let internal_position = __OrderStateInternalPosition(position);
            let key = __graphite_node_order_state
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = refund_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::RefundBeforeEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let submit_from_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    submit_from_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let submit_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    submit_to_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let pay_from_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    pay_from_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let pay_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    pay_to_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let ship_from_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    ship_from_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let ship_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    ship_to_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let deliver_from_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    deliver_from_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let deliver_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    deliver_to_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let cancel_from_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    cancel_from_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let cancel_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    cancel_to_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let refund_from_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    refund_from_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let refund_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_order_state
                .positions()
                .map(|position| {
                    refund_to_index
                        .remove(&__OrderStateInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_order_state,
            submit: __graphite_submit,
            pay: __graphite_pay,
            ship: __graphite_ship,
            deliver: __graphite_deliver,
            cancel: __graphite_cancel,
            refund: __graphite_refund,
            submit_from_index,
            submit_to_index,
            __graphite_submit_by_pair,
            pay_from_index,
            pay_to_index,
            __graphite_pay_by_pair,
            ship_from_index,
            ship_to_index,
            __graphite_ship_by_pair,
            deliver_from_index,
            deliver_to_index,
            __graphite_deliver_by_pair,
            cancel_from_index,
            cancel_to_index,
            __graphite_cancel_by_pair,
            refund_from_index,
            refund_to_index,
            __graphite_refund_by_pair,
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
