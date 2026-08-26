// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: crates/graphite/tests/edge_roles.rs:27
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    10697115368782407328u64, 2608266299376936611u64, 9754844789010380434u64,
    3042117034582590502u64,
];
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PurchaseId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscriptionId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PersonInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ProductInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PurchaseInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __SubscriptionInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PersonNamedPosition(__PersonInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ProductNamedPosition(__ProductInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PurchaseNamedPosition(__PurchaseInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __SubscriptionNamedPosition(__SubscriptionInternalPosition, u64);
#[derive(Clone, PartialEq)]
pub struct Purchase {
    pub buyer: PersonId,
    pub product: ProductId,
    pub info: TransactionInfo,
}
impl Purchase {
    pub fn new(from: PersonId, to: ProductId, payload: TransactionInfo) -> Self {
        Self {
            buyer: from,
            product: to,
            info: payload,
        }
    }
    pub fn payload(&self) -> &TransactionInfo {
        &self.info
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, ProductId, TransactionInfo> for Purchase {
    fn from_graph_literal(
        from: PersonId,
        to: ProductId,
        payload: TransactionInfo,
    ) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Purchase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Purchase))
    }
}
#[derive(Clone, PartialEq)]
pub struct Subscription {
    pub member: PersonId,
    pub product: ProductId,
}
impl Subscription {
    pub fn new(from: PersonId, to: ProductId) -> Self {
        Self { member: from, product: to }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, ProductId, ()> for Subscription {
    fn from_graph_literal(from: PersonId, to: ProductId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Subscription))
            .field(&self.member)
            .field(&self.product)
            .finish()
    }
}
#[allow(dead_code)]
struct __PurchaseRecord {
    buyer: __PersonInternalPosition,
    product: __ProductInternalPosition,
    info: TransactionInfo,
}
#[allow(dead_code)]
struct __SubscriptionRecord {
    member: __PersonInternalPosition,
    product: __ProductInternalPosition,
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    DuplicatePerson(PersonId),
    DuplicateProduct(ProductId),
    /// このエッジ種別のキーが重複している。
    PurchaseDuplicateKey(PurchaseId),
    /// このエッジが未知の始点キーを参照している。
    PurchaseUnknownSource { edge: PurchaseId, source: PersonId },
    /// このエッジが未知の終点キーを参照している。
    PurchaseUnknownTarget { edge: PurchaseId, target: ProductId },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    PurchaseBuyerEachViolation { source: PersonId, count: usize },
    /// このエッジ種別の `each` 制約違反 (入次数)。
    PurchaseProductEachViolation { target: ProductId, count: usize },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    PurchaseUniquePairViolation { source: PersonId, target: ProductId },
    /// このエッジ種別のキーが重複している。
    SubscriptionDuplicateKey(SubscriptionId),
    /// このエッジが未知の始点キーを参照している。
    SubscriptionUnknownSource { edge: SubscriptionId, source: PersonId },
    /// このエッジが未知の終点キーを参照している。
    SubscriptionUnknownTarget { edge: SubscriptionId, target: ProductId },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    SubscriptionMemberEachViolation { source: PersonId, count: usize },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicatePerson(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Person", id)
            }
            Violation::DuplicateProduct(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Product", id)
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
                    "Purchase", edge, "Product", target
                )
            }
            Violation::PurchaseBuyerEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Purchase", "Person", source, "1..2", count
                )
            }
            Violation::PurchaseProductEachViolation { target, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について入次数 {} を期待しますが実際は {} 本です",
                    "Purchase", "Product", target, "0..1", count
                )
            }
            Violation::PurchaseUniquePairViolation { source, target } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    "Purchase", source, target
                )
            }
            Violation::SubscriptionDuplicateKey(id) => {
                write!(
                    f, "{}のキーが重複しています: {:?}", "Subscription", id
                )
            }
            Violation::SubscriptionUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Subscription", edge, "Person", source
                )
            }
            Violation::SubscriptionUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Subscription", edge, "Product", target
                )
            }
            Violation::SubscriptionMemberEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Subscription", "Person", source, "1..*", count
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
    __graphite_node_person: graphite::KeyedTable<PersonId, super::Person>,
    __graphite_node_product: graphite::KeyedTable<ProductId, super::Product>,
    purchase: graphite::KeyedTable<PurchaseId, __PurchaseRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    purchase_from_index: graphite::MultipleRoleIndex<__PurchaseInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    purchase_to_index: graphite::OptionalRoleIndex<__PurchaseInternalPosition>,
    __graphite_purchase_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __ProductInternalPosition),
        __PurchaseInternalPosition,
    >,
    subscription: graphite::KeyedTable<SubscriptionId, __SubscriptionRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    subscription_from_index: graphite::MultipleRoleIndex<__SubscriptionInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    subscription_to_index: graphite::MultipleRoleIndex<__SubscriptionInternalPosition>,
    __graphite_subscription_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __ProductInternalPosition),
        Vec<__SubscriptionInternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
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
    pub fn person_value_mut(&mut self, id: &PersonId) -> Option<&mut super::Person> {
        self.__graphite_node_person.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn person_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph PersonId> {
        self.__graphite_node_person.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
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
    pub fn person_len(&self) -> usize {
        self.__graphite_node_person.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    pub fn product_by_id<'graph>(
        &'graph self,
        id: &ProductId,
    ) -> Option<ProductRef<'graph>> {
        let internal_position = __ProductInternalPosition(
            self.__graphite_node_product.position(id)?,
        );
        Some(ProductRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    pub fn product_value_mut(&mut self, id: &ProductId) -> Option<&mut super::Product> {
        self.__graphite_node_product.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn product_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph ProductId> {
        self.__graphite_node_product.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    pub fn product_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ProductRef<'graph>> + 'graph {
        self.__graphite_node_product
            .positions()
            .map(move |position| ProductRef {
                graph: self,
                internal_position: __ProductInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    pub fn product_len(&self) -> usize {
        self.__graphite_node_product.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
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
    pub fn purchase_payload_mut(
        &mut self,
        id: &PurchaseId,
    ) -> Option<&mut TransactionInfo> {
        self.purchase.get_mut(id).map(|record: &mut __PurchaseRecord| &mut record.info)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn purchase_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph PurchaseId> {
        self.purchase.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
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
    pub fn purchase_len(&self) -> usize {
        self.purchase.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn subscription_by_id<'graph>(
        &'graph self,
        id: &SubscriptionId,
    ) -> Option<SubscriptionRef<'graph>> {
        Some(SubscriptionRef {
            graph: self,
            internal_position: __SubscriptionInternalPosition(
                self.subscription.position(id)?,
            ),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn subscription_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph SubscriptionId> {
        self.subscription.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn subscription_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = SubscriptionRef<'graph>> + 'graph {
        self.subscription
            .positions()
            .map(move |position| SubscriptionRef {
                graph: self,
                internal_position: __SubscriptionInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn subscription_len(&self) -> usize {
        self.subscription.len()
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
    pub fn id(self) -> &'graph PurchaseId {
        self.graph
            .purchase
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn buyer(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().buyer.0),
        }
    }
    pub fn product(self) -> ProductRef<'graph> {
        ProductRef {
            graph: self.graph,
            internal_position: __ProductInternalPosition(self.record().product.0),
        }
    }
    pub fn from(self) -> PersonRef<'graph> {
        self.buyer()
    }
    pub fn to(self) -> ProductRef<'graph> {
        self.product()
    }
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph ProductId {
        self.to().id()
    }
    pub fn info(self) -> &'graph TransactionInfo {
        &self.record().info
    }
    pub fn payload(self) -> &'graph TransactionInfo {
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
#[derive(Clone, Copy)]
pub struct SubscriptionRef<'graph> {
    graph: &'graph Graph,
    internal_position: __SubscriptionInternalPosition,
}
impl<'graph> SubscriptionRef<'graph> {
    fn record(self) -> &'graph __SubscriptionRecord {
        self.graph
            .subscription
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph SubscriptionId {
        self.graph
            .subscription
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn member(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().member.0),
        }
    }
    pub fn product(self) -> ProductRef<'graph> {
        ProductRef {
            graph: self.graph,
            internal_position: __ProductInternalPosition(self.record().product.0),
        }
    }
    pub fn from(self) -> PersonRef<'graph> {
        self.member()
    }
    pub fn to(self) -> ProductRef<'graph> {
        self.product()
    }
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph ProductId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for SubscriptionRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
pub struct Builder {
    __graphite_node_person: Vec<(PersonId, super::Person)>,
    __graphite_node_product: Vec<(ProductId, super::Product)>,
    purchase: Vec<(PurchaseId, Purchase)>,
    subscription: Vec<(SubscriptionId, Subscription)>,
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
pub trait CommerceInsertable: Sized {
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
pub trait CommerceDefaultId: CommerceInsertable {
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
pub trait CommerceNode: CommerceInsertable {}
impl CommerceInsertable for super::Person {
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
impl CommerceDefaultId for super::Person {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        CommerceInsertable::insert_named_with_id(self, b, PersonId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        CommerceInsertable::insert_with_id(self, b, PersonId(binding))
    }
}
impl CommerceNode for super::Person {}
///完成済みグラフ上の `Person` ノード個体。
#[derive(Clone, Copy)]
pub struct PersonRef<'graph> {
    graph: &'graph Graph,
    internal_position: __PersonInternalPosition,
}
impl<'graph> PersonRef<'graph> {
    pub fn id(self) -> &'graph PersonId {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
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
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn purchase_try_between(
        self,
        other: ProductRef<'graph>,
    ) -> Result<Option<PurchaseRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_purchase_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| PurchaseRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::purchase_try_between`] を使う。
    pub fn purchase_between(
        self,
        other: ProductRef<'graph>,
    ) -> Option<PurchaseRef<'graph>> {
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
    pub fn subscription_as_member(
        self,
    ) -> impl Iterator<Item = SubscriptionRef<'graph>> + 'graph {
        let positions = self.graph.subscription_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| SubscriptionRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn subscription_try_between(
        self,
        other: ProductRef<'graph>,
    ) -> Result<
        impl Iterator<Item = SubscriptionRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_subscription_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| SubscriptionRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::subscription_try_between`] を使う。
    pub fn subscription_between(
        self,
        other: ProductRef<'graph>,
    ) -> impl Iterator<Item = SubscriptionRef<'graph>> + 'graph {
        self.subscription_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef),
                    stringify!(subscription_between)
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
impl CommerceInsertable for super::Product {
    type Id = ProductId;
    type NamedPosition = __ProductNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ProductNamedPosition(
            __ProductInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_product.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.product(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.product(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ProductNamedPosition {
    type Reference<'graph> = ProductRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ProductRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl CommerceDefaultId for super::Product {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        CommerceInsertable::insert_named_with_id(self, b, ProductId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        CommerceInsertable::insert_with_id(self, b, ProductId(binding))
    }
}
impl CommerceNode for super::Product {}
///完成済みグラフ上の `Product` ノード個体。
#[derive(Clone, Copy)]
pub struct ProductRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ProductInternalPosition,
}
impl<'graph> ProductRef<'graph> {
    pub fn id(self) -> &'graph ProductId {
        self.graph
            .__graphite_node_product
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::Product {
        self.graph
            .__graphite_node_product
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    pub fn purchase_as_product(self) -> Option<PurchaseRef<'graph>> {
        self.graph
            .purchase_to_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| PurchaseRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn subscription_as_product(
        self,
    ) -> impl Iterator<Item = SubscriptionRef<'graph>> + 'graph {
        let positions = self.graph.subscription_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| SubscriptionRef {
                graph: self.graph,
                internal_position,
            })
    }
}
impl<'graph> std::ops::Deref for ProductRef<'graph> {
    type Target = super::Product;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_product
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for ProductRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ProductRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait CommerceEdge: CommerceInsertable {}
impl CommerceInsertable for Purchase {
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
impl CommerceDefaultId for Purchase {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        CommerceInsertable::insert_named_with_id(self, b, PurchaseId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        CommerceInsertable::insert_with_id(self, b, PurchaseId(binding))
    }
}
impl CommerceEdge for Purchase {}
impl CommerceInsertable for Subscription {
    type Id = SubscriptionId;
    type NamedPosition = __SubscriptionNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __SubscriptionNamedPosition(
            __SubscriptionInternalPosition(
                graphite::TablePosition::from_index(b.subscription.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.subscription(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.subscription(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __SubscriptionNamedPosition {
    type Reference<'graph> = SubscriptionRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        SubscriptionRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl CommerceDefaultId for Subscription {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        CommerceInsertable::insert_named_with_id(
            self,
            b,
            SubscriptionId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        CommerceInsertable::insert_with_id(self, b, SubscriptionId(binding))
    }
}
impl CommerceEdge for Subscription {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_person: Vec::new(),
            __graphite_node_product: Vec::new(),
            purchase: Vec::new(),
            subscription: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    pub fn person(&mut self, id: PersonId, value: super::Person) -> &mut Self {
        self.__graphite_node_person.push((id, value));
        self
    }
    pub fn product(&mut self, id: ProductId, value: super::Product) -> &mut Self {
        self.__graphite_node_product.push((id, value));
        self
    }
    pub fn purchase(&mut self, id: PurchaseId, value: Purchase) -> &mut Self {
        self.purchase.push((id, value));
        self
    }
    pub fn subscription(
        &mut self,
        id: SubscriptionId,
        value: Subscription,
    ) -> &mut Self {
        self.subscription.push((id, value));
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
        N: CommerceNode + CommerceDefaultId,
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
        N: CommerceNode + CommerceDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: CommerceNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: CommerceNode>(
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
        E: CommerceEdge + CommerceDefaultId,
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
        E: CommerceEdge + CommerceDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: CommerceEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: CommerceEdge>(
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
        T: CommerceDefaultId,
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
        let mut __graphite_node_product: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_product {
            if !__graphite_node_product.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateProduct(id));
            }
        }
        let mut __graphite_purchase: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut purchase_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut purchase_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_purchase_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __ProductInternalPosition),
            __PurchaseInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.purchase {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::PurchaseDuplicateKey(id));
                continue;
            }
            let Purchase { buyer: from, product: to, info } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_product
                .position(&to)
                .map(__ProductInternalPosition);
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
                if __graphite_purchase_by_pair
                    .contains_key(&(from_position, to_position))
                {
                    __violations
                        .push(Violation::PurchaseUniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __PurchaseInternalPosition(
                    graphite::TablePosition::from_index(__graphite_purchase.len()),
                );
                __graphite_purchase_by_pair
                    .insert((from_position, to_position), internal_edge_position);
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
                            product: to_position,
                            info,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Purchase) = |edge| {
            let _ = &edge.buyer;
        };
        let _: fn(&Purchase) = |edge| {
            let _ = &edge.product;
        };
        for position in __graphite_node_person.positions() {
            let internal_position = __PersonInternalPosition(position);
            let key = __graphite_node_person
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = purchase_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(1usize..=2usize).contains(&count) {
                __violations
                    .push(Violation::PurchaseBuyerEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        for position in __graphite_node_product.positions() {
            let internal_position = __ProductInternalPosition(position);
            let key = __graphite_node_product
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = purchase_to_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::PurchaseProductEachViolation {
                        target: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_subscription: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut subscription_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut subscription_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_subscription_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __ProductInternalPosition),
            Vec<__SubscriptionInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.subscription {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::SubscriptionDuplicateKey(id));
                continue;
            }
            let Subscription { member: from, product: to } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_product
                .position(&to)
                .map(__ProductInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::SubscriptionUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::SubscriptionUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __SubscriptionInternalPosition(
                    graphite::TablePosition::from_index(__graphite_subscription.len()),
                );
                __graphite_subscription_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                subscription_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                subscription_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_subscription
                    .insert(
                        id,
                        __SubscriptionRecord {
                            member: from_position,
                            product: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Subscription) = |edge| {
            let _ = &edge.member;
        };
        for position in __graphite_node_person.positions() {
            let internal_position = __PersonInternalPosition(position);
            let key = __graphite_node_person
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = subscription_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if count < 1usize {
                __violations
                    .push(Violation::SubscriptionMemberEachViolation {
                        source: key.clone(),
                        count,
                    });
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
        let purchase_to_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_product
                .positions()
                .map(|position| {
                    purchase_to_index
                        .remove(&__ProductInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let subscription_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    subscription_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let subscription_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_product
                .positions()
                .map(|position| {
                    subscription_to_index
                        .remove(&__ProductInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_person,
            __graphite_node_product,
            purchase: __graphite_purchase,
            subscription: __graphite_subscription,
            purchase_from_index,
            purchase_to_index,
            __graphite_purchase_by_pair,
            subscription_from_index,
            subscription_to_index,
            __graphite_subscription_by_pair,
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
