// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: crates/graphite/tests/traversal_api.rs:23
// 再生成: リポジトリルートで `cargo xtask generate` を実行する。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    2390679431758203756u64, 9631175040940765815u64, 15761920802152593070u64,
    9127356853236581378u64,
];
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PurchaseId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MentorId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 関係Id(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FriendsId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PersonInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ProductInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PurchaseInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __MentorInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __関係InternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __FriendsInternalPosition(graphite::TablePosition);
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
pub struct __MentorNamedPosition(__MentorInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __関係NamedPosition(__関係InternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __FriendsNamedPosition(__FriendsInternalPosition, u64);
#[derive(Clone, PartialEq)]
pub struct Purchase {
    pub buyer: PersonId,
    pub product: ProductId,
}
impl Purchase {
    pub fn new(from: PersonId, to: ProductId) -> Self {
        Self { buyer: from, product: to }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, ProductId, ()> for Purchase {
    fn from_graph_literal(from: PersonId, to: ProductId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Purchase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Purchase))
            .field(&self.buyer)
            .field(&self.product)
            .finish()
    }
}
#[derive(Clone, PartialEq)]
pub struct Mentor {
    pub subordinate: PersonId,
    pub superior: PersonId,
}
impl Mentor {
    pub fn new(from: PersonId, to: PersonId) -> Self {
        Self {
            subordinate: from,
            superior: to,
        }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, PersonId, ()> for Mentor {
    fn from_graph_literal(from: PersonId, to: PersonId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Mentor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Mentor))
            .field(&self.subordinate)
            .field(&self.superior)
            .finish()
    }
}
#[derive(Clone, PartialEq)]
pub struct 関係 {
    pub 始点: PersonId,
    pub 終点: PersonId,
}
impl 関係 {
    pub fn new(from: PersonId, to: PersonId) -> Self {
        Self { 始点: from, 終点: to }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, PersonId, ()> for 関係 {
    fn from_graph_literal(from: PersonId, to: PersonId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for 関係 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(関係))
            .field(&self.始点)
            .field(&self.終点)
            .finish()
    }
}
#[derive(Clone, PartialEq)]
pub struct Friends {
    endpoints: graphite::UnorderedPair<PersonId>,
}
impl Friends {
    pub fn new(a: PersonId, b: PersonId) -> Self {
        Self {
            endpoints: graphite::UnorderedPair::new(a, b),
        }
    }
    pub fn endpoints(&self) -> (&PersonId, &PersonId) {
        self.endpoints.endpoints()
    }
}
impl graphite::UndirectedEdgeLiteral<PersonId, ()> for Friends {
    fn from_graph_literal(a: PersonId, b: PersonId, (): ()) -> Self {
        Self::new(a, b)
    }
}
impl std::fmt::Debug for Friends {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Friends))
            .field(&self.endpoints().0)
            .field(&self.endpoints().1)
            .finish()
    }
}
#[allow(dead_code)]
struct __PurchaseRecord {
    buyer: __PersonInternalPosition,
    product: __ProductInternalPosition,
}
#[allow(dead_code)]
struct __MentorRecord {
    subordinate: __PersonInternalPosition,
    superior: __PersonInternalPosition,
}
#[allow(dead_code)]
struct __関係Record {
    始点: __PersonInternalPosition,
    終点: __PersonInternalPosition,
}
#[allow(dead_code)]
struct __FriendsRecord {
    endpoints: graphite::UnorderedPair<__PersonInternalPosition>,
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
    /// このエッジ種別のキーが重複している。
    MentorDuplicateKey(MentorId),
    /// このエッジが未知の始点キーを参照している。
    MentorUnknownSource { edge: MentorId, source: PersonId },
    /// このエッジが未知の終点キーを参照している。
    MentorUnknownTarget { edge: MentorId, target: PersonId },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    MentorSubordinateEachViolation { source: PersonId, count: usize },
    /// このエッジ種別のキーが重複している。
    関係DuplicateKey(関係Id),
    /// このエッジが未知の始点キーを参照している。
    関係UnknownSource { edge: 関係Id, source: PersonId },
    /// このエッジが未知の終点キーを参照している。
    関係UnknownTarget { edge: 関係Id, target: PersonId },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    関係UniquePairViolation { source: PersonId, target: PersonId },
    /// このエッジ種別のキーが重複している。
    FriendsDuplicateKey(FriendsId),
    /// このエッジが未知の端点キーを参照している (無向のため位置の
    /// 区別は無い)。
    FriendsUnknownEndpoint { edge: FriendsId, endpoint: PersonId },
    /// このエッジ種別の `unique pair` 違反 (無向のため
    /// 順序を無視した対で判定)。
    FriendsUniquePairViolation { a: PersonId, b: PersonId },
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
            Violation::MentorDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Mentor", id)
            }
            Violation::MentorUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "Mentor", edge, "Person", source
                )
            }
            Violation::MentorUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "Mentor", edge, "Person", target
                )
            }
            Violation::MentorSubordinateEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Mentor", "Person", source, "0..1", count
                )
            }
            Violation::関係DuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "関係", id)
            }
            Violation::関係UnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "関係", edge, "Person", source
                )
            }
            Violation::関係UnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "関係", edge, "Person", target
                )
            }
            Violation::関係UniquePairViolation { source, target } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    "関係", source, target
                )
            }
            Violation::FriendsDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Friends", id)
            }
            Violation::FriendsUnknownEndpoint { edge, endpoint } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の端点, {}): {:?}",
                    "Friends", edge, "Person", endpoint
                )
            }
            Violation::FriendsUniquePairViolation { a, b } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {{{:?}, {:?}}} の対に既に辺が存在します",
                    "Friends", a, b
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
    purchase_to_index: graphite::MultipleRoleIndex<__PurchaseInternalPosition>,
    __graphite_purchase_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __ProductInternalPosition),
        Vec<__PurchaseInternalPosition>,
    >,
    mentor: graphite::KeyedTable<MentorId, __MentorRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    mentor_from_index: graphite::OptionalRoleIndex<__MentorInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    mentor_to_index: graphite::MultipleRoleIndex<__MentorInternalPosition>,
    __graphite_mentor_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __PersonInternalPosition),
        Vec<__MentorInternalPosition>,
    >,
    関係: graphite::KeyedTable<関係Id, __関係Record>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    関係_from_index: graphite::MultipleRoleIndex<__関係InternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    関係_to_index: graphite::MultipleRoleIndex<__関係InternalPosition>,
    __graphite_関係_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __PersonInternalPosition),
        __関係InternalPosition,
    >,
    friends: graphite::KeyedTable<FriendsId, __FriendsRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    friends_index: graphite::MultipleRoleIndex<__FriendsInternalPosition>,
    __graphite_friends_by_pair: std::collections::HashMap<
        graphite::UnorderedPair<__PersonInternalPosition>,
        __FriendsInternalPosition,
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
    pub fn mentor_by_id<'graph>(
        &'graph self,
        id: &MentorId,
    ) -> Option<MentorRef<'graph>> {
        Some(MentorRef {
            graph: self,
            internal_position: __MentorInternalPosition(self.mentor.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn mentor_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph MentorId> {
        self.mentor.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn mentor_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = MentorRef<'graph>> + 'graph {
        self.mentor
            .positions()
            .map(move |position| MentorRef {
                graph: self,
                internal_position: __MentorInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn mentor_len(&self) -> usize {
        self.mentor.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn 関係_by_id<'graph>(
        &'graph self,
        id: &関係Id,
    ) -> Option<関係Ref<'graph>> {
        Some(関係Ref {
            graph: self,
            internal_position: __関係InternalPosition(self.関係.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn 関係_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph 関係Id> {
        self.関係.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn 関係_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = 関係Ref<'graph>> + 'graph {
        self.関係
            .positions()
            .map(move |position| 関係Ref {
                graph: self,
                internal_position: __関係InternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn 関係_len(&self) -> usize {
        self.関係.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn friends_by_id<'graph>(
        &'graph self,
        id: &FriendsId,
    ) -> Option<FriendsRef<'graph>> {
        Some(FriendsRef {
            graph: self,
            internal_position: __FriendsInternalPosition(self.friends.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn friends_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph FriendsId> {
        self.friends.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn friends_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = FriendsRef<'graph>> + 'graph {
        self.friends
            .positions()
            .map(move |position| FriendsRef {
                graph: self,
                internal_position: __FriendsInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn friends_len(&self) -> usize {
        self.friends.len()
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
pub struct MentorRef<'graph> {
    graph: &'graph Graph,
    internal_position: __MentorInternalPosition,
}
impl<'graph> MentorRef<'graph> {
    fn record(self) -> &'graph __MentorRecord {
        self.graph
            .mentor
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph MentorId {
        self.graph
            .mentor
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn subordinate(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().subordinate.0),
        }
    }
    pub fn superior(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().superior.0),
        }
    }
    pub fn from(self) -> PersonRef<'graph> {
        self.subordinate()
    }
    pub fn to(self) -> PersonRef<'graph> {
        self.superior()
    }
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph PersonId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for MentorRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(MentorRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
#[derive(Clone, Copy)]
pub struct 関係Ref<'graph> {
    graph: &'graph Graph,
    internal_position: __関係InternalPosition,
}
impl<'graph> 関係Ref<'graph> {
    fn record(self) -> &'graph __関係Record {
        self.graph
            .関係
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph 関係Id {
        self.graph
            .関係
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn 始点(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().始点.0),
        }
    }
    pub fn 終点(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().終点.0),
        }
    }
    pub fn from(self) -> PersonRef<'graph> {
        self.始点()
    }
    pub fn to(self) -> PersonRef<'graph> {
        self.終点()
    }
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph PersonId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for 関係Ref<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(関係Ref))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の無向辺個体。
#[derive(Clone, Copy)]
pub struct FriendsRef<'graph> {
    graph: &'graph Graph,
    internal_position: __FriendsInternalPosition,
}
impl<'graph> FriendsRef<'graph> {
    fn record(self) -> &'graph __FriendsRecord {
        self.graph
            .friends
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph FriendsId {
        self.graph
            .friends
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn endpoints(self) -> (PersonRef<'graph>, PersonRef<'graph>) {
        let (first, second) = self.record().endpoints.endpoints();
        (
            PersonRef {
                graph: self.graph,
                internal_position: __PersonInternalPosition(first.0),
            },
            PersonRef {
                graph: self.graph,
                internal_position: __PersonInternalPosition(second.0),
            },
        )
    }
}
impl<'graph> std::fmt::Debug for FriendsRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(FriendsRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
pub struct Builder {
    __graphite_node_person: Vec<(PersonId, super::Person)>,
    __graphite_node_product: Vec<(ProductId, super::Product)>,
    purchase: Vec<(PurchaseId, Purchase)>,
    mentor: Vec<(MentorId, Mentor)>,
    関係: Vec<(関係Id, 関係)>,
    friends: Vec<(FriendsId, Friends)>,
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
pub trait TraversalInsertable: Sized {
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
pub trait TraversalDefaultId: TraversalInsertable {
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
pub trait TraversalNode: TraversalInsertable {}
impl TraversalInsertable for super::Person {
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
                graphite::TablePosition(b.__graphite_node_person.len()),
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
impl TraversalDefaultId for super::Person {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        TraversalInsertable::insert_named_with_id(self, b, PersonId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        TraversalInsertable::insert_with_id(self, b, PersonId(binding))
    }
}
impl TraversalNode for super::Person {}
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
    ///パニックを避けたい場合は対の [`Self::purchase_try_between`] を使う。
    pub fn purchase_between(
        self,
        other: ProductRef<'graph>,
    ) -> impl Iterator<Item = PurchaseRef<'graph>> + 'graph {
        self.purchase_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef),
                    stringify!(purchase_between)
                )
            })
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    pub fn mentor_as_subordinate(self) -> Option<MentorRef<'graph>> {
        self.graph
            .mentor_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| MentorRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn mentor_as_superior(self) -> impl Iterator<Item = MentorRef<'graph>> + 'graph {
        let positions = self.graph.mentor_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| MentorRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn mentor_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<
        impl Iterator<Item = MentorRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_mentor_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| MentorRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::mentor_try_between`] を使う。
    pub fn mentor_between(
        self,
        other: PersonRef<'graph>,
    ) -> impl Iterator<Item = MentorRef<'graph>> + 'graph {
        self.mentor_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef), stringify!(mentor_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn 関係_as_始点(self) -> impl Iterator<Item = 関係Ref<'graph>> + 'graph {
        let positions = self.graph.関係_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 関係Ref {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn 関係_as_終点(self) -> impl Iterator<Item = 関係Ref<'graph>> + 'graph {
        let positions = self.graph.関係_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| 関係Ref {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn 関係_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<Option<関係Ref<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_関係_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| 関係Ref {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::関係_try_between`] を使う。
    pub fn 関係_between(self, other: PersonRef<'graph>) -> Option<関係Ref<'graph>> {
        self.関係_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef), stringify!(関係_between)
                )
            })
    }
    /// 接続辺を O(1) で参照し、追加確保なしで挿入順に走査する。
    pub fn friends_incident(self) -> impl Iterator<Item = FriendsRef<'graph>> + 'graph {
        let positions = self.graph.friends_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| FriendsRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序なし端点対を平均 O(1)、追加確保なしで検索する。
    pub fn friends_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<Option<FriendsRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_friends_by_pair
            .get(
                &graphite::UnorderedPair::new(
                    self.internal_position,
                    other.internal_position,
                ),
            )
            .copied();
        Ok(
            found
                .map(|internal_position| FriendsRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::friends_try_between`] を使う。
    pub fn friends_between(
        self,
        other: PersonRef<'graph>,
    ) -> Option<FriendsRef<'graph>> {
        self.friends_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef), stringify!(friends_between)
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
impl TraversalInsertable for super::Product {
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
                graphite::TablePosition(b.__graphite_node_product.len()),
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
impl TraversalDefaultId for super::Product {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        TraversalInsertable::insert_named_with_id(self, b, ProductId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        TraversalInsertable::insert_with_id(self, b, ProductId(binding))
    }
}
impl TraversalNode for super::Product {}
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
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn purchase_as_product(
        self,
    ) -> impl Iterator<Item = PurchaseRef<'graph>> + 'graph {
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
pub trait TraversalEdge: TraversalInsertable {}
impl TraversalInsertable for Purchase {
    type Id = PurchaseId;
    type NamedPosition = __PurchaseNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __PurchaseNamedPosition(
            __PurchaseInternalPosition(graphite::TablePosition(b.purchase.len())),
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
impl TraversalDefaultId for Purchase {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        TraversalInsertable::insert_named_with_id(self, b, PurchaseId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        TraversalInsertable::insert_with_id(self, b, PurchaseId(binding))
    }
}
impl TraversalEdge for Purchase {}
impl TraversalInsertable for Mentor {
    type Id = MentorId;
    type NamedPosition = __MentorNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __MentorNamedPosition(
            __MentorInternalPosition(graphite::TablePosition(b.mentor.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.mentor(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.mentor(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __MentorNamedPosition {
    type Reference<'graph> = MentorRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        MentorRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl TraversalDefaultId for Mentor {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        TraversalInsertable::insert_named_with_id(self, b, MentorId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        TraversalInsertable::insert_with_id(self, b, MentorId(binding))
    }
}
impl TraversalEdge for Mentor {}
impl TraversalInsertable for 関係 {
    type Id = 関係Id;
    type NamedPosition = __関係NamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __関係NamedPosition(
            __関係InternalPosition(graphite::TablePosition(b.関係.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.関係(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.関係(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __関係NamedPosition {
    type Reference<'graph> = 関係Ref<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        関係Ref {
            graph,
            internal_position: self.0,
        }
    }
}
impl TraversalDefaultId for 関係 {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        TraversalInsertable::insert_named_with_id(self, b, 関係Id(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        TraversalInsertable::insert_with_id(self, b, 関係Id(binding))
    }
}
impl TraversalEdge for 関係 {}
impl TraversalInsertable for Friends {
    type Id = FriendsId;
    type NamedPosition = __FriendsNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __FriendsNamedPosition(
            __FriendsInternalPosition(graphite::TablePosition(b.friends.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.friends(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.friends(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __FriendsNamedPosition {
    type Reference<'graph> = FriendsRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        FriendsRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl TraversalDefaultId for Friends {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        TraversalInsertable::insert_named_with_id(self, b, FriendsId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        TraversalInsertable::insert_with_id(self, b, FriendsId(binding))
    }
}
impl TraversalEdge for Friends {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_person: Vec::new(),
            __graphite_node_product: Vec::new(),
            purchase: Vec::new(),
            mentor: Vec::new(),
            関係: Vec::new(),
            friends: Vec::new(),
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
    pub fn mentor(&mut self, id: MentorId, value: Mentor) -> &mut Self {
        self.mentor.push((id, value));
        self
    }
    pub fn 関係(&mut self, id: 関係Id, value: 関係) -> &mut Self {
        self.関係.push((id, value));
        self
    }
    pub fn friends(&mut self, id: FriendsId, value: Friends) -> &mut Self {
        self.friends.push((id, value));
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
        N: TraversalNode + TraversalDefaultId,
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
        N: TraversalNode + TraversalDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: TraversalNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: TraversalNode>(
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
        E: TraversalEdge + TraversalDefaultId,
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
        E: TraversalEdge + TraversalDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: TraversalEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: TraversalEdge>(
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
        T: TraversalDefaultId,
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
            Vec<__PurchaseInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.purchase {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::PurchaseDuplicateKey(id));
                continue;
            }
            let Purchase { buyer: from, product: to } = value;
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
                let internal_edge_position = __PurchaseInternalPosition(
                    graphite::TablePosition(__graphite_purchase.len()),
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
                            product: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_mentor: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut mentor_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut mentor_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_mentor_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __PersonInternalPosition),
            Vec<__MentorInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.mentor {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::MentorDuplicateKey(id));
                continue;
            }
            let Mentor { subordinate: from, superior: to } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_person
                .position(&to)
                .map(__PersonInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::MentorUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::MentorUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __MentorInternalPosition(
                    graphite::TablePosition(__graphite_mentor.len()),
                );
                __graphite_mentor_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                mentor_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                mentor_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_mentor
                    .insert(
                        id,
                        __MentorRecord {
                            subordinate: from_position,
                            superior: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Mentor) = |edge| {
            let _ = &edge.subordinate;
        };
        for position in __graphite_node_person.positions() {
            let internal_position = __PersonInternalPosition(position);
            let key = __graphite_node_person
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = mentor_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::MentorSubordinateEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        let mut __graphite_関係: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut 関係_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut 関係_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_関係_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __PersonInternalPosition),
            __関係InternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.関係 {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::関係DuplicateKey(id));
                continue;
            }
            let 関係 { 始点: from, 終点: to } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_person
                .position(&to)
                .map(__PersonInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::関係UnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::関係UnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                if __graphite_関係_by_pair.contains_key(&(from_position, to_position))
                {
                    __violations
                        .push(Violation::関係UniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __関係InternalPosition(
                    graphite::TablePosition(__graphite_関係.len()),
                );
                __graphite_関係_by_pair
                    .insert((from_position, to_position), internal_edge_position);
                関係_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                関係_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_関係
                    .insert(
                        id,
                        __関係Record {
                            始点: from_position,
                            終点: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_friends: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut friends_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_friends_by_pair: std::collections::HashMap<
            graphite::UnorderedPair<__PersonInternalPosition>,
            __FriendsInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.friends {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::FriendsDuplicateKey(id));
                continue;
            }
            let Friends { endpoints } = value;
            let (p0, p1) = endpoints.endpoints();
            let p0 = p0.clone();
            let p1 = p1.clone();
            let first_position = __graphite_node_person
                .position(&p0)
                .map(__PersonInternalPosition);
            let second_position = __graphite_node_person
                .position(&p1)
                .map(__PersonInternalPosition);
            if first_position.is_none() {
                __violations
                    .push(Violation::FriendsUnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p0.clone(),
                    });
            }
            if p1 != p0 && second_position.is_none() {
                __violations
                    .push(Violation::FriendsUnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p1.clone(),
                    });
            }
            if let (Some(first_position), Some(second_position)) = (
                first_position,
                second_position,
            ) {
                if __graphite_friends_by_pair
                    .contains_key(
                        &graphite::UnorderedPair::new(first_position, second_position),
                    )
                {
                    __violations
                        .push(Violation::FriendsUniquePairViolation {
                            a: p0.clone(),
                            b: p1.clone(),
                        });
                }
                let internal_edge_position = __FriendsInternalPosition(
                    graphite::TablePosition(__graphite_friends.len()),
                );
                __graphite_friends_by_pair
                    .insert(
                        graphite::UnorderedPair::new(first_position, second_position),
                        internal_edge_position,
                    );
                friends_index
                    .entry(first_position)
                    .or_default()
                    .push(internal_edge_position);
                if second_position != first_position {
                    friends_index
                        .entry(second_position)
                        .or_default()
                        .push(internal_edge_position);
                }
                let inserted = __graphite_friends
                    .insert(
                        id,
                        __FriendsRecord {
                            endpoints: graphite::UnorderedPair::new(
                                first_position,
                                second_position,
                            ),
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let purchase_from_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_person.len())
                .map(|position| {
                    purchase_from_index
                        .remove(
                            &__PersonInternalPosition(graphite::TablePosition(position)),
                        )
                        .unwrap_or_default()
                })
                .collect(),
        );
        let purchase_to_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_product.len())
                .map(|position| {
                    purchase_to_index
                        .remove(
                            &__ProductInternalPosition(graphite::TablePosition(position)),
                        )
                        .unwrap_or_default()
                })
                .collect(),
        );
        let mentor_from_index = graphite::OptionalRoleIndex::from_buckets(
            (0..__graphite_node_person.len())
                .map(|position| {
                    mentor_from_index
                        .remove(
                            &__PersonInternalPosition(graphite::TablePosition(position)),
                        )
                        .unwrap_or_default()
                })
                .collect(),
        );
        let mentor_to_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_person.len())
                .map(|position| {
                    mentor_to_index
                        .remove(
                            &__PersonInternalPosition(graphite::TablePosition(position)),
                        )
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 関係_from_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_person.len())
                .map(|position| {
                    関係_from_index
                        .remove(
                            &__PersonInternalPosition(graphite::TablePosition(position)),
                        )
                        .unwrap_or_default()
                })
                .collect(),
        );
        let 関係_to_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_person.len())
                .map(|position| {
                    関係_to_index
                        .remove(
                            &__PersonInternalPosition(graphite::TablePosition(position)),
                        )
                        .unwrap_or_default()
                })
                .collect(),
        );
        let friends_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_person.len())
                .map(|position| {
                    friends_index
                        .remove(
                            &__PersonInternalPosition(graphite::TablePosition(position)),
                        )
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_person,
            __graphite_node_product,
            purchase: __graphite_purchase,
            mentor: __graphite_mentor,
            関係: __graphite_関係,
            friends: __graphite_friends,
            purchase_from_index,
            purchase_to_index,
            __graphite_purchase_by_pair,
            mentor_from_index,
            mentor_to_index,
            __graphite_mentor_by_pair,
            関係_from_index,
            関係_to_index,
            __graphite_関係_by_pair,
            friends_index,
            __graphite_friends_by_pair,
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
