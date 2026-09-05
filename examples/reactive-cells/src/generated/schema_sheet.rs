// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/schema.rs:81
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    3772461139475860626u64, 18124488242770300103u64, 8414127093800046084u64,
    14208174019845936776u64,
];
/// `Cell` ノードの公開ID。
///
/// 宣言: `src/schema.rs` の `node Cell`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CellId(pub String);
/// `Feeds` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeedsId(pub String);
/// `Lhs` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LhsId(pub String);
/// `Rhs` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RhsId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __CellInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __FeedsInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __LhsInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __RhsInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __CellNamedPosition(__CellInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __FeedsNamedPosition(__FeedsInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __LhsNamedPosition(__LhsInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __RhsNamedPosition(__RhsInternalPosition, u64);
/// 構築時に組み立てる `Feeds` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
#[derive(Clone, PartialEq)]
pub struct Feeds {
    /// この辺の始点ノードの公開ID。
    pub dependency: CellId,
    /// この辺の終点ノードの公開ID。
    pub dependent: CellId,
}
impl Feeds {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn new(from: CellId, to: CellId) -> Self {
        Self {
            dependency: from,
            dependent: to,
        }
    }
}
impl graphite::DirectedEdgeLiteral<CellId, CellId, ()> for Feeds {
    fn from_graph_literal(from: CellId, to: CellId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Feeds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Feeds))
            .field(&self.dependency)
            .field(&self.dependent)
            .finish()
    }
}
/// 構築時に組み立てる `Lhs` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
#[derive(Clone, PartialEq)]
pub struct Lhs {
    /// この辺の始点ノードの公開ID。
    pub operand: CellId,
    /// この辺の終点ノードの公開ID。
    pub operation: CellId,
}
impl Lhs {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn new(from: CellId, to: CellId) -> Self {
        Self {
            operand: from,
            operation: to,
        }
    }
}
impl graphite::DirectedEdgeLiteral<CellId, CellId, ()> for Lhs {
    fn from_graph_literal(from: CellId, to: CellId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Lhs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Lhs))
            .field(&self.operand)
            .field(&self.operation)
            .finish()
    }
}
/// 構築時に組み立てる `Rhs` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
#[derive(Clone, PartialEq)]
pub struct Rhs {
    /// この辺の始点ノードの公開ID。
    pub operand: CellId,
    /// この辺の終点ノードの公開ID。
    pub operation: CellId,
}
impl Rhs {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn new(from: CellId, to: CellId) -> Self {
        Self {
            operand: from,
            operation: to,
        }
    }
}
impl graphite::DirectedEdgeLiteral<CellId, CellId, ()> for Rhs {
    fn from_graph_literal(from: CellId, to: CellId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Rhs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Rhs))
            .field(&self.operand)
            .field(&self.operation)
            .finish()
    }
}
#[allow(dead_code)]
struct __FeedsRecord {
    dependency: __CellInternalPosition,
    dependent: __CellInternalPosition,
}
#[allow(dead_code)]
struct __LhsRecord {
    operand: __CellInternalPosition,
    operation: __CellInternalPosition,
}
#[allow(dead_code)]
struct __RhsRecord {
    operand: __CellInternalPosition,
    operation: __CellInternalPosition,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `src/schema.rs` の `schema Sheet`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    DuplicateCell(CellId),
    /// このエッジ種別のキーが重複している。
    FeedsDuplicateKey(FeedsId),
    /// このエッジが未知の始点キーを参照している。
    FeedsUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: FeedsId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: CellId,
    },
    /// このエッジが未知の終点キーを参照している。
    FeedsUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: FeedsId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: CellId,
    },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    FeedsUniquePairViolation {
        /// 2本目の辺が張られた対の始点ノードの公開ID。
        source: CellId,
        /// 2本目の辺が張られた対の終点ノードの公開ID。
        target: CellId,
    },
    /// このエッジ種別のキーが重複している。
    LhsDuplicateKey(LhsId),
    /// このエッジが未知の始点キーを参照している。
    LhsUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: LhsId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: CellId,
    },
    /// このエッジが未知の終点キーを参照している。
    LhsUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: LhsId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: CellId,
    },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    LhsUniquePairViolation {
        /// 2本目の辺が張られた対の始点ノードの公開ID。
        source: CellId,
        /// 2本目の辺が張られた対の終点ノードの公開ID。
        target: CellId,
    },
    /// このエッジ種別のキーが重複している。
    RhsDuplicateKey(RhsId),
    /// このエッジが未知の始点キーを参照している。
    RhsUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: RhsId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: CellId,
    },
    /// このエッジが未知の終点キーを参照している。
    RhsUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: RhsId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: CellId,
    },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    RhsUniquePairViolation {
        /// 2本目の辺が張られた対の始点ノードの公開ID。
        source: CellId,
        /// 2本目の辺が張られた対の終点ノードの公開ID。
        target: CellId,
    },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicateCell(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Cell", id)
            }
            Violation::FeedsDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Feeds", id)
            }
            Violation::FeedsUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として見つかりません (辺 `{}` {:?} の{})",
                    source, "Cell", "Feeds", edge, "始点"
                )
            }
            Violation::FeedsUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として見つかりません (辺 `{}` {:?} の{})",
                    target, "Cell", "Feeds", edge, "終点"
                )
            }
            Violation::FeedsUniquePairViolation { source, target } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    "Feeds", source, target
                )
            }
            Violation::LhsDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Lhs", id)
            }
            Violation::LhsUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として見つかりません (辺 `{}` {:?} の{})",
                    source, "Cell", "Lhs", edge, "始点"
                )
            }
            Violation::LhsUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として見つかりません (辺 `{}` {:?} の{})",
                    target, "Cell", "Lhs", edge, "終点"
                )
            }
            Violation::LhsUniquePairViolation { source, target } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    "Lhs", source, target
                )
            }
            Violation::RhsDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Rhs", id)
            }
            Violation::RhsUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として見つかりません (辺 `{}` {:?} の{})",
                    source, "Cell", "Rhs", edge, "始点"
                )
            }
            Violation::RhsUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として見つかりません (辺 `{}` {:?} の{})",
                    target, "Cell", "Rhs", edge, "終点"
                )
            }
            Violation::RhsUniquePairViolation { source, target } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    "Rhs", source, target
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
/// 宣言: `src/schema.rs` の `schema Sheet`
pub struct Graph {
    __graphite_node_cell: graphite::KeyedTable<CellId, super::Cell>,
    feeds: graphite::KeyedTable<FeedsId, __FeedsRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    feeds_from_index: graphite::MultipleRoleIndex<__FeedsInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    feeds_to_index: graphite::MultipleRoleIndex<__FeedsInternalPosition>,
    __graphite_feeds_by_pair: std::collections::HashMap<
        (__CellInternalPosition, __CellInternalPosition),
        __FeedsInternalPosition,
    >,
    lhs: graphite::KeyedTable<LhsId, __LhsRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    lhs_from_index: graphite::MultipleRoleIndex<__LhsInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    lhs_to_index: graphite::MultipleRoleIndex<__LhsInternalPosition>,
    __graphite_lhs_by_pair: std::collections::HashMap<
        (__CellInternalPosition, __CellInternalPosition),
        __LhsInternalPosition,
    >,
    rhs: graphite::KeyedTable<RhsId, __RhsRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    rhs_from_index: graphite::MultipleRoleIndex<__RhsInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    rhs_to_index: graphite::MultipleRoleIndex<__RhsInternalPosition>,
    __graphite_rhs_by_pair: std::collections::HashMap<
        (__CellInternalPosition, __CellInternalPosition),
        __RhsInternalPosition,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `node Cell`
    pub fn cell_by_id<'graph>(&'graph self, id: &CellId) -> Option<CellRef<'graph>> {
        let internal_position = __CellInternalPosition(
            self.__graphite_node_cell.position(id)?,
        );
        Some(CellRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `src/schema.rs` の `node Cell`
    pub fn cell_value_mut(&mut self, id: &CellId) -> Option<&mut super::Cell> {
        self.__graphite_node_cell.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `node Cell`
    pub fn cell_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph CellId> {
        self.__graphite_node_cell.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `node Cell`
    pub fn cell_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = CellRef<'graph>> + 'graph {
        self.__graphite_node_cell
            .positions()
            .map(move |position| CellRef {
                graph: self,
                internal_position: __CellInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `node Cell`
    pub fn cell_len(&self) -> usize {
        self.__graphite_node_cell.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn feeds_by_id<'graph>(&'graph self, id: &FeedsId) -> Option<FeedsRef<'graph>> {
        Some(FeedsRef {
            graph: self,
            internal_position: __FeedsInternalPosition(self.feeds.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn feeds_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph FeedsId> {
        self.feeds.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn feeds_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = FeedsRef<'graph>> + 'graph {
        self.feeds
            .positions()
            .map(move |position| FeedsRef {
                graph: self,
                internal_position: __FeedsInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn feeds_len(&self) -> usize {
        self.feeds.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn lhs_by_id<'graph>(&'graph self, id: &LhsId) -> Option<LhsRef<'graph>> {
        Some(LhsRef {
            graph: self,
            internal_position: __LhsInternalPosition(self.lhs.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn lhs_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph LhsId> {
        self.lhs.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn lhs_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = LhsRef<'graph>> + 'graph {
        self.lhs
            .positions()
            .map(move |position| LhsRef {
                graph: self,
                internal_position: __LhsInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn lhs_len(&self) -> usize {
        self.lhs.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn rhs_by_id<'graph>(&'graph self, id: &RhsId) -> Option<RhsRef<'graph>> {
        Some(RhsRef {
            graph: self,
            internal_position: __RhsInternalPosition(self.rhs.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn rhs_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph RhsId> {
        self.rhs.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn rhs_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = RhsRef<'graph>> + 'graph {
        self.rhs
            .positions()
            .map(move |position| RhsRef {
                graph: self,
                internal_position: __RhsInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn rhs_len(&self) -> usize {
        self.rhs.len()
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
/// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
#[derive(Clone, Copy)]
pub struct FeedsRef<'graph> {
    graph: &'graph Graph,
    internal_position: __FeedsInternalPosition,
}
impl<'graph> FeedsRef<'graph> {
    fn record(self) -> &'graph __FeedsRecord {
        self.graph
            .feeds
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn id(self) -> &'graph FeedsId {
        self.graph
            .feeds
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn dependency(self) -> CellRef<'graph> {
        CellRef {
            graph: self.graph,
            internal_position: __CellInternalPosition(self.record().dependency.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn dependent(self) -> CellRef<'graph> {
        CellRef {
            graph: self.graph,
            internal_position: __CellInternalPosition(self.record().dependent.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn from(self) -> CellRef<'graph> {
        self.dependency()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn to(self) -> CellRef<'graph> {
        self.dependent()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn from_id(self) -> &'graph CellId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn to_id(self) -> &'graph CellId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for FeedsRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(FeedsRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
#[derive(Clone, Copy)]
pub struct LhsRef<'graph> {
    graph: &'graph Graph,
    internal_position: __LhsInternalPosition,
}
impl<'graph> LhsRef<'graph> {
    fn record(self) -> &'graph __LhsRecord {
        self.graph
            .lhs
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn id(self) -> &'graph LhsId {
        self.graph
            .lhs
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn operand(self) -> CellRef<'graph> {
        CellRef {
            graph: self.graph,
            internal_position: __CellInternalPosition(self.record().operand.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn operation(self) -> CellRef<'graph> {
        CellRef {
            graph: self.graph,
            internal_position: __CellInternalPosition(self.record().operation.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn from(self) -> CellRef<'graph> {
        self.operand()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn to(self) -> CellRef<'graph> {
        self.operation()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn from_id(self) -> &'graph CellId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn to_id(self) -> &'graph CellId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for LhsRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(LhsRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
#[derive(Clone, Copy)]
pub struct RhsRef<'graph> {
    graph: &'graph Graph,
    internal_position: __RhsInternalPosition,
}
impl<'graph> RhsRef<'graph> {
    fn record(self) -> &'graph __RhsRecord {
        self.graph
            .rhs
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn id(self) -> &'graph RhsId {
        self.graph
            .rhs
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn operand(self) -> CellRef<'graph> {
        CellRef {
            graph: self.graph,
            internal_position: __CellInternalPosition(self.record().operand.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn operation(self) -> CellRef<'graph> {
        CellRef {
            graph: self.graph,
            internal_position: __CellInternalPosition(self.record().operation.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn from(self) -> CellRef<'graph> {
        self.operand()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn to(self) -> CellRef<'graph> {
        self.operation()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn from_id(self) -> &'graph CellId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn to_id(self) -> &'graph CellId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for RhsRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(RhsRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
/// 制約検査を一切行わない。
///
/// 宣言: `src/schema.rs` の `schema Sheet`
pub struct Builder {
    __graphite_node_cell: Vec<(CellId, super::Cell)>,
    feeds: Vec<(FeedsId, Feeds)>,
    lhs: Vec<(LhsId, Lhs)>,
    rhs: Vec<(RhsId, Rhs)>,
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
pub trait SheetInsertable: Sized {
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
pub trait SheetDefaultId: SheetInsertable {
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
pub trait SheetNode: SheetInsertable {}
impl SheetInsertable for super::Cell {
    type Id = CellId;
    type NamedPosition = __CellNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __CellNamedPosition(
            __CellInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_cell.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.cell(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.cell(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __CellNamedPosition {
    type Reference<'graph> = CellRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        CellRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl SheetDefaultId for super::Cell {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        SheetInsertable::insert_named_with_id(self, b, CellId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        SheetInsertable::insert_with_id(self, b, CellId(binding))
    }
}
impl SheetNode for super::Cell {}
/// 完成済みグラフ上の `Cell` ノード個体。
///
/// 宣言: `src/schema.rs` の `node Cell`
#[derive(Clone, Copy)]
pub struct CellRef<'graph> {
    graph: &'graph Graph,
    internal_position: __CellInternalPosition,
}
impl<'graph> CellRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `node Cell`
    pub fn id(self) -> &'graph CellId {
        self.graph
            .__graphite_node_cell
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `src/schema.rs` の `node Cell`
    pub fn value(self) -> &'graph super::Cell {
        self.graph
            .__graphite_node_cell
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn feeds_as_dependency(self) -> impl Iterator<Item = FeedsRef<'graph>> + 'graph {
        let positions = self.graph.feeds_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| FeedsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn feeds_as_dependent(self) -> impl Iterator<Item = FeedsRef<'graph>> + 'graph {
        let positions = self.graph.feeds_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| FeedsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn feeds_try_between(
        self,
        other: CellRef<'graph>,
    ) -> Result<Option<FeedsRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_feeds_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| FeedsRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::feeds_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn feeds_between(self, other: CellRef<'graph>) -> Option<FeedsRef<'graph>> {
        self.feeds_try_between(other)
            .unwrap_or_else(|error| {
                panic!("{}::{}: {error}", stringify!(CellRef), stringify!(feeds_between))
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn lhs_as_operand(self) -> impl Iterator<Item = LhsRef<'graph>> + 'graph {
        let positions = self.graph.lhs_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| LhsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn lhs_as_operation(self) -> impl Iterator<Item = LhsRef<'graph>> + 'graph {
        let positions = self.graph.lhs_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| LhsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn lhs_try_between(
        self,
        other: CellRef<'graph>,
    ) -> Result<Option<LhsRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_lhs_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| LhsRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::lhs_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn lhs_between(self, other: CellRef<'graph>) -> Option<LhsRef<'graph>> {
        self.lhs_try_between(other)
            .unwrap_or_else(|error| {
                panic!("{}::{}: {error}", stringify!(CellRef), stringify!(lhs_between))
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn rhs_as_operand(self) -> impl Iterator<Item = RhsRef<'graph>> + 'graph {
        let positions = self.graph.rhs_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| RhsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn rhs_as_operation(self) -> impl Iterator<Item = RhsRef<'graph>> + 'graph {
        let positions = self.graph.rhs_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| RhsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn rhs_try_between(
        self,
        other: CellRef<'graph>,
    ) -> Result<Option<RhsRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_rhs_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| RhsRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::rhs_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn rhs_between(self, other: CellRef<'graph>) -> Option<RhsRef<'graph>> {
        self.rhs_try_between(other)
            .unwrap_or_else(|error| {
                panic!("{}::{}: {error}", stringify!(CellRef), stringify!(rhs_between))
            })
    }
}
impl<'graph> std::ops::Deref for CellRef<'graph> {
    type Target = super::Cell;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_cell
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for CellRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(CellRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait SheetEdge: SheetInsertable {}
impl SheetInsertable for Feeds {
    type Id = FeedsId;
    type NamedPosition = __FeedsNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __FeedsNamedPosition(
            __FeedsInternalPosition(graphite::TablePosition::from_index(b.feeds.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.feeds(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.feeds(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __FeedsNamedPosition {
    type Reference<'graph> = FeedsRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        FeedsRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl SheetDefaultId for Feeds {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        SheetInsertable::insert_named_with_id(self, b, FeedsId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        SheetInsertable::insert_with_id(self, b, FeedsId(binding))
    }
}
impl SheetEdge for Feeds {}
impl SheetInsertable for Lhs {
    type Id = LhsId;
    type NamedPosition = __LhsNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __LhsNamedPosition(
            __LhsInternalPosition(graphite::TablePosition::from_index(b.lhs.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.lhs(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.lhs(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __LhsNamedPosition {
    type Reference<'graph> = LhsRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        LhsRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl SheetDefaultId for Lhs {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        SheetInsertable::insert_named_with_id(self, b, LhsId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        SheetInsertable::insert_with_id(self, b, LhsId(binding))
    }
}
impl SheetEdge for Lhs {}
impl SheetInsertable for Rhs {
    type Id = RhsId;
    type NamedPosition = __RhsNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __RhsNamedPosition(
            __RhsInternalPosition(graphite::TablePosition::from_index(b.rhs.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.rhs(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.rhs(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __RhsNamedPosition {
    type Reference<'graph> = RhsRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        RhsRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl SheetDefaultId for Rhs {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        SheetInsertable::insert_named_with_id(self, b, RhsId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        SheetInsertable::insert_with_id(self, b, RhsId(binding))
    }
}
impl SheetEdge for Rhs {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_cell: Vec::new(),
            feeds: Vec::new(),
            lhs: Vec::new(),
            rhs: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `node Cell`
    pub fn cell(&mut self, id: CellId, value: super::Cell) -> &mut Self {
        self.__graphite_node_cell.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Feeds = (dependency: Cell) -> (dependent: Cell) where unique pair`
    pub fn feeds(&mut self, id: FeedsId, value: Feeds) -> &mut Self {
        self.feeds.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Lhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn lhs(&mut self, id: LhsId, value: Lhs) -> &mut Self {
        self.lhs.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Rhs = (operand: Cell) -> (operation: Cell) where unique pair`
    pub fn rhs(&mut self, id: RhsId, value: Rhs) -> &mut Self {
        self.rhs.push((id, value));
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
        N: SheetNode + SheetDefaultId,
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
        N: SheetNode + SheetDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: SheetNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: SheetNode>(
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
        E: SheetEdge + SheetDefaultId,
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
        E: SheetEdge + SheetDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: SheetEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: SheetEdge>(
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
        T: SheetDefaultId,
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
        let mut __graphite_node_cell: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_cell {
            if !__graphite_node_cell.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateCell(id));
            }
        }
        let mut __graphite_feeds: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut feeds_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut feeds_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_feeds_by_pair: std::collections::HashMap<
            (__CellInternalPosition, __CellInternalPosition),
            __FeedsInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.feeds {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::FeedsDuplicateKey(id));
                continue;
            }
            let Feeds { dependency: from, dependent: to } = value;
            let from_position = __graphite_node_cell
                .position(&from)
                .map(__CellInternalPosition);
            let to_position = __graphite_node_cell
                .position(&to)
                .map(__CellInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::FeedsUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::FeedsUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                if __graphite_feeds_by_pair.contains_key(&(from_position, to_position)) {
                    __violations
                        .push(Violation::FeedsUniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __FeedsInternalPosition(
                    graphite::TablePosition::from_index(__graphite_feeds.len()),
                );
                __graphite_feeds_by_pair
                    .insert((from_position, to_position), internal_edge_position);
                feeds_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                feeds_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_feeds
                    .insert(
                        id,
                        __FeedsRecord {
                            dependency: from_position,
                            dependent: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_lhs: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut lhs_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut lhs_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_lhs_by_pair: std::collections::HashMap<
            (__CellInternalPosition, __CellInternalPosition),
            __LhsInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.lhs {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::LhsDuplicateKey(id));
                continue;
            }
            let Lhs { operand: from, operation: to } = value;
            let from_position = __graphite_node_cell
                .position(&from)
                .map(__CellInternalPosition);
            let to_position = __graphite_node_cell
                .position(&to)
                .map(__CellInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::LhsUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::LhsUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                if __graphite_lhs_by_pair.contains_key(&(from_position, to_position)) {
                    __violations
                        .push(Violation::LhsUniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __LhsInternalPosition(
                    graphite::TablePosition::from_index(__graphite_lhs.len()),
                );
                __graphite_lhs_by_pair
                    .insert((from_position, to_position), internal_edge_position);
                lhs_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                lhs_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_lhs
                    .insert(
                        id,
                        __LhsRecord {
                            operand: from_position,
                            operation: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_rhs: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut rhs_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut rhs_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_rhs_by_pair: std::collections::HashMap<
            (__CellInternalPosition, __CellInternalPosition),
            __RhsInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.rhs {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::RhsDuplicateKey(id));
                continue;
            }
            let Rhs { operand: from, operation: to } = value;
            let from_position = __graphite_node_cell
                .position(&from)
                .map(__CellInternalPosition);
            let to_position = __graphite_node_cell
                .position(&to)
                .map(__CellInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::RhsUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::RhsUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                if __graphite_rhs_by_pair.contains_key(&(from_position, to_position)) {
                    __violations
                        .push(Violation::RhsUniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __RhsInternalPosition(
                    graphite::TablePosition::from_index(__graphite_rhs.len()),
                );
                __graphite_rhs_by_pair
                    .insert((from_position, to_position), internal_edge_position);
                rhs_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                rhs_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_rhs
                    .insert(
                        id,
                        __RhsRecord {
                            operand: from_position,
                            operation: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let feeds_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_cell
                .positions()
                .map(|position| {
                    feeds_from_index
                        .remove(&__CellInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let feeds_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_cell
                .positions()
                .map(|position| {
                    feeds_to_index
                        .remove(&__CellInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let lhs_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_cell
                .positions()
                .map(|position| {
                    lhs_from_index
                        .remove(&__CellInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let lhs_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_cell
                .positions()
                .map(|position| {
                    lhs_to_index
                        .remove(&__CellInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let rhs_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_cell
                .positions()
                .map(|position| {
                    rhs_from_index
                        .remove(&__CellInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let rhs_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_cell
                .positions()
                .map(|position| {
                    rhs_to_index
                        .remove(&__CellInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_cell,
            feeds: __graphite_feeds,
            lhs: __graphite_lhs,
            rhs: __graphite_rhs,
            feeds_from_index,
            feeds_to_index,
            __graphite_feeds_by_pair,
            lhs_from_index,
            lhs_to_index,
            __graphite_lhs_by_pair,
            rhs_from_index,
            rhs_to_index,
            __graphite_rhs_by_pair,
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
