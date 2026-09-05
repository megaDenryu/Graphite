// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/named_graph.rs:28
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    12042915227829816677u64, 11487058971239738810u64, 2329270621513830063u64,
    10198531821772805379u64,
];
/// `Person` ノードの公開ID。
///
/// 宣言: `tests/named_graph.rs` の `node Person`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);
/// `Item` ノードの公開ID。
///
/// 宣言: `tests/named_graph.rs` の `node Item`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemId(pub String);
/// `Purchase` 辺の公開ID。
///
/// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PurchaseId(pub String);
/// `Knows` 辺の公開ID。
///
/// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KnowsId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PersonInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ItemInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PurchaseInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __KnowsInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PersonNamedPosition(__PersonInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ItemNamedPosition(__ItemInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PurchaseNamedPosition(__PurchaseInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __KnowsNamedPosition(__KnowsInternalPosition, u64);
/// 構築時に組み立てる `Purchase` 辺の値。
///
/// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
#[derive(Clone, PartialEq)]
pub struct Purchase {
    /// この辺の始点ノードの公開ID。
    pub buyer: PersonId,
    /// この辺の終点ノードの公開ID。
    pub item: ItemId,
    /// この辺が運ぶ積み荷。
    pub info: PurchaseInfo,
}
impl Purchase {
    /// 始点と終点の公開IDと積み荷から構築用の辺値を作る。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn new(from: PersonId, to: ItemId, payload: PurchaseInfo) -> Self {
        Self {
            buyer: from,
            item: to,
            info: payload,
        }
    }
    /// この辺値が運ぶ積み荷を借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn payload(&self) -> &PurchaseInfo {
        &self.info
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, ItemId, PurchaseInfo> for Purchase {
    fn from_graph_literal(from: PersonId, to: ItemId, payload: PurchaseInfo) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Purchase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Purchase))
    }
}
/// 構築時に組み立てる `Knows` 辺の値。
///
/// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
#[derive(Clone, PartialEq)]
pub struct Knows {
    /// この辺の始点ノードの公開ID。
    pub knower: PersonId,
    /// この辺の終点ノードの公開ID。
    pub known: PersonId,
}
impl Knows {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn new(from: PersonId, to: PersonId) -> Self {
        Self { knower: from, known: to }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, PersonId, ()> for Knows {
    fn from_graph_literal(from: PersonId, to: PersonId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Knows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Knows)).field(&self.knower).field(&self.known).finish()
    }
}
#[allow(dead_code)]
struct __PurchaseRecord {
    buyer: __PersonInternalPosition,
    item: __ItemInternalPosition,
    info: PurchaseInfo,
}
#[allow(dead_code)]
struct __KnowsRecord {
    knower: __PersonInternalPosition,
    known: __PersonInternalPosition,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `tests/named_graph.rs` の `schema NamedWorld`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    DuplicatePerson(PersonId),
    /// このノード種別のキーが重複している。
    DuplicateItem(ItemId),
    /// このエッジ種別のキーが重複している。
    PurchaseDuplicateKey(PurchaseId),
    /// このエッジが未知の始点キーを参照している。
    PurchaseUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: PurchaseId,
        /// 参照先が見つからなかった始点ノードの公開ID。
        source: PersonId,
    },
    /// このエッジが未知の終点キーを参照している。
    PurchaseUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: PurchaseId,
        /// 参照先が見つからなかった終点ノードの公開ID。
        target: ItemId,
    },
    /// このエッジ種別のキーが重複している。
    KnowsDuplicateKey(KnowsId),
    /// このエッジが未知の始点キーを参照している。
    KnowsUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: KnowsId,
        /// 参照先が見つからなかった始点ノードの公開ID。
        source: PersonId,
    },
    /// このエッジが未知の終点キーを参照している。
    KnowsUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: KnowsId,
        /// 参照先が見つからなかった終点ノードの公開ID。
        target: PersonId,
    },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicatePerson(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Person", id)
            }
            Violation::DuplicateItem(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Item", id)
            }
            Violation::PurchaseDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Purchase", id)
            }
            Violation::PurchaseUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Purchase", edge, "Person", source
                )
            }
            Violation::PurchaseUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Purchase", edge, "Item", target
                )
            }
            Violation::KnowsDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Knows", id)
            }
            Violation::KnowsUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Knows", edge, "Person", source
                )
            }
            Violation::KnowsUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Knows", edge, "Person", target
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
/// 宣言: `tests/named_graph.rs` の `schema NamedWorld`
pub struct Graph {
    __graphite_node_person: graphite::KeyedTable<PersonId, super::Person>,
    __graphite_node_item: graphite::KeyedTable<ItemId, super::Item>,
    purchase: graphite::KeyedTable<PurchaseId, __PurchaseRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    purchase_from_index: graphite::MultipleRoleIndex<__PurchaseInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    purchase_to_index: graphite::MultipleRoleIndex<__PurchaseInternalPosition>,
    __graphite_purchase_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __ItemInternalPosition),
        Vec<__PurchaseInternalPosition>,
    >,
    knows: graphite::KeyedTable<KnowsId, __KnowsRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    knows_from_index: graphite::MultipleRoleIndex<__KnowsInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    knows_to_index: graphite::MultipleRoleIndex<__KnowsInternalPosition>,
    __graphite_knows_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __PersonInternalPosition),
        Vec<__KnowsInternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Person`
    pub fn person_by_id<'graph>(
        &'graph self,
        id: &PersonId,
    ) -> Option<PersonRef<'graph>> {
        let internal_position = __PersonInternalPosition(
            self.__graphite_node_person.position(id)?,
        );
        Some(PersonRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Person`
    pub fn person_value_mut(&mut self, id: &PersonId) -> Option<&mut super::Person> {
        self.__graphite_node_person.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Person`
    pub fn person_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph PersonId> {
        self.__graphite_node_person.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Person`
    pub fn person_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = PersonRef<'graph>> + 'graph {
        self.__graphite_node_person
            .positions()
            .map(move |position| PersonRef {
                graph: self,
                internal_position: __PersonInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Person`
    pub fn person_len(&self) -> usize {
        self.__graphite_node_person.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Item`
    pub fn item_by_id<'graph>(&'graph self, id: &ItemId) -> Option<ItemRef<'graph>> {
        let internal_position = __ItemInternalPosition(
            self.__graphite_node_item.position(id)?,
        );
        Some(ItemRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Item`
    pub fn item_value_mut(&mut self, id: &ItemId) -> Option<&mut super::Item> {
        self.__graphite_node_item.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Item`
    pub fn item_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph ItemId> {
        self.__graphite_node_item.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Item`
    pub fn item_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ItemRef<'graph>> + 'graph {
        self.__graphite_node_item
            .positions()
            .map(move |position| ItemRef {
                graph: self,
                internal_position: __ItemInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Item`
    pub fn item_len(&self) -> usize {
        self.__graphite_node_item.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase_by_id<'graph>(
        &'graph self,
        id: &PurchaseId,
    ) -> Option<PurchaseRef<'graph>> {
        Some(PurchaseRef {
            graph: self,
            internal_position: __PurchaseInternalPosition(self.purchase.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase_payload_mut(
        &mut self,
        id: &PurchaseId,
    ) -> Option<&mut PurchaseInfo> {
        self.purchase.get_mut(id).map(|record: &mut __PurchaseRecord| &mut record.info)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph PurchaseId> {
        self.purchase.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = PurchaseRef<'graph>> + 'graph {
        self.purchase
            .positions()
            .map(move |position| PurchaseRef {
                graph: self,
                internal_position: __PurchaseInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase_len(&self) -> usize {
        self.purchase.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knows_by_id<'graph>(&'graph self, id: &KnowsId) -> Option<KnowsRef<'graph>> {
        Some(KnowsRef {
            graph: self,
            internal_position: __KnowsInternalPosition(self.knows.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knows_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph KnowsId> {
        self.knows.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knows_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = KnowsRef<'graph>> + 'graph {
        self.knows
            .positions()
            .map(move |position| KnowsRef {
                graph: self,
                internal_position: __KnowsInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knows_len(&self) -> usize {
        self.knows.len()
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
/// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
#[derive(Clone, Copy)]
pub struct PurchaseRef<'graph> {
    graph: &'graph Graph,
    internal_position: __PurchaseInternalPosition,
}
impl<'graph> PurchaseRef<'graph> {
    fn record(self) -> &'graph __PurchaseRecord {
        self.graph
            .purchase
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn id(self) -> &'graph PurchaseId {
        self.graph
            .purchase
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn buyer(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().buyer.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn item(self) -> ItemRef<'graph> {
        ItemRef {
            graph: self.graph,
            internal_position: __ItemInternalPosition(self.record().item.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn from(self) -> PersonRef<'graph> {
        self.buyer()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn to(self) -> ItemRef<'graph> {
        self.item()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn to_id(self) -> &'graph ItemId {
        self.to().id()
    }
    /// この辺個体が運ぶ積み荷を役割名で借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn info(self) -> &'graph PurchaseInfo {
        &self.record().info
    }
    /// この辺個体が運ぶ積み荷を、役割名によらない固定名で借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn payload(self) -> &'graph PurchaseInfo {
        &self.record().info
    }
}
impl<'graph> std::fmt::Debug for PurchaseRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(PurchaseRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
#[derive(Clone, Copy)]
pub struct KnowsRef<'graph> {
    graph: &'graph Graph,
    internal_position: __KnowsInternalPosition,
}
impl<'graph> KnowsRef<'graph> {
    fn record(self) -> &'graph __KnowsRecord {
        self.graph
            .knows
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn id(self) -> &'graph KnowsId {
        self.graph
            .knows
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knower(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().knower.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn known(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().known.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn from(self) -> PersonRef<'graph> {
        self.knower()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn to(self) -> PersonRef<'graph> {
        self.known()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn to_id(self) -> &'graph PersonId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for KnowsRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(KnowsRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
///
/// 宣言: `tests/named_graph.rs` の `schema NamedWorld`
pub struct Builder {
    __graphite_node_person: Vec<(PersonId, super::Person)>,
    __graphite_node_item: Vec<(ItemId, super::Item)>,
    purchase: Vec<(PurchaseId, Purchase)>,
    knows: Vec<(KnowsId, Knows)>,
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
pub trait NamedWorldInsertable: Sized {
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
    /// 型付きの公開IDを指定して、この要素を構築器へ挿入する。
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id;
}
/// 束縛名の文字列からスキーマ内限定の既定IDを作れる要素だけが
/// 実装する。明示ID型には実装せず、文字列変換を要求しない。
pub trait NamedWorldDefaultId: NamedWorldInsertable {
    #[doc(hidden)]
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    /// 束縛名の文字列から既定IDを作り、この要素を構築器へ挿入する。
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id;
}
/// ノード挿入で使うトレイト境界。読み取りは `Graph` の種別メソッドと
/// `NodeRef` のメソッドが提供する。利用者がこのトレイトのメソッドを
/// 直接呼ぶことは想定しない。
pub trait NamedWorldNode: NamedWorldInsertable {}
impl NamedWorldInsertable for super::Person {
    type Id = PersonId;
    type NamedPosition = __PersonNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __PersonNamedPosition(
            __PersonInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_person.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.person(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.person(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __PersonNamedPosition {
    type Reference<'graph> = PersonRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        PersonRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl NamedWorldDefaultId for super::Person {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        NamedWorldInsertable::insert_named_with_id(self, b, PersonId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        NamedWorldInsertable::insert_with_id(self, b, PersonId(binding))
    }
}
impl NamedWorldNode for super::Person {}
/// 完成済みグラフ上の `Person` ノード個体。
///
/// 宣言: `tests/named_graph.rs` の `node Person`
#[derive(Clone, Copy)]
pub struct PersonRef<'graph> {
    graph: &'graph Graph,
    internal_position: __PersonInternalPosition,
}
impl<'graph> PersonRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Person`
    pub fn id(self) -> &'graph PersonId {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Person`
    pub fn value(self) -> &'graph super::Person {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase_as_buyer(
        self,
    ) -> impl Iterator<Item = PurchaseRef<'graph>> + 'graph {
        let positions = self.graph.purchase_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| PurchaseRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase_try_between(
        self,
        other: ItemRef<'graph>,
    ) -> Result<
        impl Iterator<Item = PurchaseRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_purchase_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| PurchaseRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::purchase_try_between`] を使う。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase_between(
        self,
        other: ItemRef<'graph>,
    ) -> impl Iterator<Item = PurchaseRef<'graph>> + 'graph {
        self.purchase_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef),
                    stringify!(purchase_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knows_as_knower(self) -> impl Iterator<Item = KnowsRef<'graph>> + 'graph {
        let positions = self.graph.knows_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| KnowsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knows_as_known(self) -> impl Iterator<Item = KnowsRef<'graph>> + 'graph {
        let positions = self.graph.knows_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| KnowsRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knows_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<
        impl Iterator<Item = KnowsRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_knows_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| KnowsRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::knows_try_between`] を使う。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knows_between(
        self,
        other: PersonRef<'graph>,
    ) -> impl Iterator<Item = KnowsRef<'graph>> + 'graph {
        self.knows_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef), stringify!(knows_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for PersonRef<'graph> {
    type Target = super::Person;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for PersonRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(PersonRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
impl NamedWorldInsertable for super::Item {
    type Id = ItemId;
    type NamedPosition = __ItemNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ItemNamedPosition(
            __ItemInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_item.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.item(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.item(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ItemNamedPosition {
    type Reference<'graph> = ItemRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ItemRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl NamedWorldDefaultId for super::Item {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        NamedWorldInsertable::insert_named_with_id(self, b, ItemId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        NamedWorldInsertable::insert_with_id(self, b, ItemId(binding))
    }
}
impl NamedWorldNode for super::Item {}
/// 完成済みグラフ上の `Item` ノード個体。
///
/// 宣言: `tests/named_graph.rs` の `node Item`
#[derive(Clone, Copy)]
pub struct ItemRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ItemInternalPosition,
}
impl<'graph> ItemRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Item`
    pub fn id(self) -> &'graph ItemId {
        self.graph
            .__graphite_node_item
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Item`
    pub fn value(self) -> &'graph super::Item {
        self.graph
            .__graphite_node_item
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase_as_item(self) -> impl Iterator<Item = PurchaseRef<'graph>> + 'graph {
        let positions = self.graph.purchase_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| PurchaseRef {
                graph: self.graph,
                internal_position,
            })
    }
}
impl<'graph> std::ops::Deref for ItemRef<'graph> {
    type Target = super::Item;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_item
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for ItemRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ItemRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait NamedWorldEdge: NamedWorldInsertable {}
impl NamedWorldInsertable for Purchase {
    type Id = PurchaseId;
    type NamedPosition = __PurchaseNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __PurchaseNamedPosition(
            __PurchaseInternalPosition(
                graphite::TablePosition::from_index(b.purchase.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.purchase(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.purchase(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __PurchaseNamedPosition {
    type Reference<'graph> = PurchaseRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        PurchaseRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl NamedWorldDefaultId for Purchase {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        NamedWorldInsertable::insert_named_with_id(self, b, PurchaseId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        NamedWorldInsertable::insert_with_id(self, b, PurchaseId(binding))
    }
}
impl NamedWorldEdge for Purchase {}
impl NamedWorldInsertable for Knows {
    type Id = KnowsId;
    type NamedPosition = __KnowsNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __KnowsNamedPosition(
            __KnowsInternalPosition(graphite::TablePosition::from_index(b.knows.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.knows(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.knows(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __KnowsNamedPosition {
    type Reference<'graph> = KnowsRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        KnowsRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl NamedWorldDefaultId for Knows {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        NamedWorldInsertable::insert_named_with_id(self, b, KnowsId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        NamedWorldInsertable::insert_with_id(self, b, KnowsId(binding))
    }
}
impl NamedWorldEdge for Knows {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_person: Vec::new(),
            __graphite_node_item: Vec::new(),
            purchase: Vec::new(),
            knows: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Person`
    pub fn person(&mut self, id: PersonId, value: super::Person) -> &mut Self {
        self.__graphite_node_person.push((id, value));
        self
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/named_graph.rs` の `node Item`
    pub fn item(&mut self, id: ItemId, value: super::Item) -> &mut Self {
        self.__graphite_node_item.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item)`
    pub fn purchase(&mut self, id: PurchaseId, value: Purchase) -> &mut Self {
        self.purchase.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/named_graph.rs` の `edge Knows = (knower: Person) -> (known: Person)`
    pub fn knows(&mut self, id: KnowsId, value: Knows) -> &mut Self {
        self.knows.push((id, value));
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
        N: NamedWorldNode + NamedWorldDefaultId,
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
        N: NamedWorldNode + NamedWorldDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: NamedWorldNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: NamedWorldNode>(
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
        E: NamedWorldEdge + NamedWorldDefaultId,
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
        E: NamedWorldEdge + NamedWorldDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: NamedWorldEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: NamedWorldEdge>(
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
        T: NamedWorldDefaultId,
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
        let mut __graphite_node_person: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_person {
            if !__graphite_node_person.insert(id.clone(), value) {
                __violations.push(Violation::DuplicatePerson(id));
            }
        }
        let mut __graphite_node_item: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_item {
            if !__graphite_node_item.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateItem(id));
            }
        }
        let mut __graphite_purchase: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut purchase_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut purchase_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_purchase_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __ItemInternalPosition),
            Vec<__PurchaseInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.purchase {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::PurchaseDuplicateKey(id));
                continue;
            }
            let Purchase { buyer: from, item: to, info } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_item
                .position(&to)
                .map(__ItemInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::PurchaseUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::PurchaseUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __PurchaseInternalPosition(
                    graphite::TablePosition::from_index(__graphite_purchase.len()),
                );
                __graphite_purchase_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                purchase_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                purchase_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_purchase
                    .insert(
                        id,
                        __PurchaseRecord {
                            buyer: from_position,
                            item: to_position,
                            info,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_knows: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut knows_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut knows_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_knows_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __PersonInternalPosition),
            Vec<__KnowsInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.knows {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::KnowsDuplicateKey(id));
                continue;
            }
            let Knows { knower: from, known: to } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_person
                .position(&to)
                .map(__PersonInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::KnowsUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::KnowsUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __KnowsInternalPosition(
                    graphite::TablePosition::from_index(__graphite_knows.len()),
                );
                __graphite_knows_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                knows_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                knows_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_knows
                    .insert(
                        id,
                        __KnowsRecord {
                            knower: from_position,
                            known: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let purchase_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    purchase_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let purchase_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_item
                .positions()
                .map(|position| {
                    purchase_to_index
                        .remove(&__ItemInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let knows_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    knows_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let knows_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    knows_to_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_person,
            __graphite_node_item,
            purchase: __graphite_purchase,
            knows: __graphite_knows,
            purchase_from_index,
            purchase_to_index,
            __graphite_purchase_by_pair,
            knows_from_index,
            knows_to_index,
            __graphite_knows_by_pair,
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
