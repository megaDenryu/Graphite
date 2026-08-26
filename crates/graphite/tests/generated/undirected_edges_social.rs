// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/undirected_edges.rs:30
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    594360863009716014u64, 1912780570926071469u64, 250009452544659368u64,
    5193048946222574060u64,
];
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FriendsId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WireId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PersonInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __FriendsInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __WireInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PersonNamedPosition(__PersonInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __FriendsNamedPosition(__FriendsInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __WireNamedPosition(__WireInternalPosition, u64);
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
#[derive(Clone, PartialEq)]
pub struct Wire {
    endpoints: graphite::UnorderedPair<PersonId>,
    pub cable: Cable,
}
impl Wire {
    pub fn new(a: PersonId, b: PersonId, payload: Cable) -> Self {
        Self {
            endpoints: graphite::UnorderedPair::new(a, b),
            cable: payload,
        }
    }
    pub fn endpoints(&self) -> (&PersonId, &PersonId) {
        self.endpoints.endpoints()
    }
    pub fn payload(&self) -> &Cable {
        &self.cable
    }
}
impl graphite::UndirectedEdgeLiteral<PersonId, Cable> for Wire {
    fn from_graph_literal(a: PersonId, b: PersonId, payload: Cable) -> Self {
        Self::new(a, b, payload)
    }
}
impl std::fmt::Debug for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Wire))
    }
}
#[allow(dead_code)]
struct __FriendsRecord {
    endpoints: graphite::UnorderedPair<__PersonInternalPosition>,
}
#[allow(dead_code)]
struct __WireRecord {
    endpoints: graphite::UnorderedPair<__PersonInternalPosition>,
    cable: Cable,
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    DuplicatePerson(PersonId),
    /// このエッジ種別のキーが重複している。
    FriendsDuplicateKey(FriendsId),
    /// このエッジが未知の端点キーを参照している (無向のため位置の
    /// 区別は無い)。
    FriendsUnknownEndpoint { edge: FriendsId, endpoint: PersonId },
    /// このエッジ種別の `unique pair` 違反 (無向のため
    /// 順序を無視した対で判定)。
    FriendsUniquePairViolation { a: PersonId, b: PersonId },
    /// このエッジ種別のキーが重複している。
    WireDuplicateKey(WireId),
    /// このエッジが未知の端点キーを参照している (無向のため位置の
    /// 区別は無い)。
    WireUnknownEndpoint { edge: WireId, endpoint: PersonId },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicatePerson(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Person", id)
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
            Violation::WireDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Wire", id)
            }
            Violation::WireUnknownEndpoint { edge, endpoint } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の端点, {}): {:?}",
                    "Wire", edge, "Person", endpoint
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
    friends: graphite::KeyedTable<FriendsId, __FriendsRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    friends_index: graphite::MultipleRoleIndex<__FriendsInternalPosition>,
    __graphite_friends_by_pair: std::collections::HashMap<
        graphite::UnorderedPair<__PersonInternalPosition>,
        __FriendsInternalPosition,
    >,
    wire: graphite::KeyedTable<WireId, __WireRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    wire_index: graphite::MultipleRoleIndex<__WireInternalPosition>,
    __graphite_wire_by_pair: std::collections::HashMap<
        graphite::UnorderedPair<__PersonInternalPosition>,
        Vec<__WireInternalPosition>,
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
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn wire_by_id<'graph>(&'graph self, id: &WireId) -> Option<WireRef<'graph>> {
        Some(WireRef {
            graph: self,
            internal_position: __WireInternalPosition(self.wire.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    pub fn wire_payload_mut(&mut self, id: &WireId) -> Option<&mut Cable> {
        self.wire.get_mut(id).map(|record: &mut __WireRecord| &mut record.cable)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn wire_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph WireId> {
        self.wire.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn wire_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = WireRef<'graph>> + 'graph {
        self.wire
            .positions()
            .map(move |position| WireRef {
                graph: self,
                internal_position: __WireInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn wire_len(&self) -> usize {
        self.wire.len()
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
/// 完成済みグラフ上の無向辺個体。
#[derive(Clone, Copy)]
pub struct WireRef<'graph> {
    graph: &'graph Graph,
    internal_position: __WireInternalPosition,
}
impl<'graph> WireRef<'graph> {
    fn record(self) -> &'graph __WireRecord {
        self.graph
            .wire
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph WireId {
        self.graph
            .wire
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
    pub fn cable(self) -> &'graph Cable {
        &self.record().cable
    }
    pub fn payload(self) -> &'graph Cable {
        &self.record().cable
    }
}
impl<'graph> std::fmt::Debug for WireRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(WireRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
pub struct Builder {
    __graphite_node_person: Vec<(PersonId, super::Person)>,
    friends: Vec<(FriendsId, Friends)>,
    wire: Vec<(WireId, Wire)>,
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
pub trait SocialInsertable: Sized {
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
pub trait SocialDefaultId: SocialInsertable {
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
pub trait SocialNode: SocialInsertable {}
impl SocialInsertable for super::Person {
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
impl SocialDefaultId for super::Person {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        SocialInsertable::insert_named_with_id(self, b, PersonId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        SocialInsertable::insert_with_id(self, b, PersonId(binding))
    }
}
impl SocialNode for super::Person {}
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
    /// 接続辺を O(1) で参照し、追加確保なしで挿入順に走査する。
    pub fn wire_incident(self) -> impl Iterator<Item = WireRef<'graph>> + 'graph {
        let positions = self.graph.wire_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| WireRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序なし端点対を平均 O(1)、追加確保なしで検索する。
    pub fn wire_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<
        impl Iterator<Item = WireRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_wire_by_pair
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
                .map(move |internal_position| WireRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::wire_try_between`] を使う。
    pub fn wire_between(
        self,
        other: PersonRef<'graph>,
    ) -> impl Iterator<Item = WireRef<'graph>> + 'graph {
        self.wire_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef), stringify!(wire_between)
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
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait SocialEdge: SocialInsertable {}
impl SocialInsertable for Friends {
    type Id = FriendsId;
    type NamedPosition = __FriendsNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __FriendsNamedPosition(
            __FriendsInternalPosition(
                graphite::TablePosition::from_index(b.friends.len()),
            ),
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
impl SocialDefaultId for Friends {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        SocialInsertable::insert_named_with_id(self, b, FriendsId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        SocialInsertable::insert_with_id(self, b, FriendsId(binding))
    }
}
impl SocialEdge for Friends {}
impl SocialInsertable for Wire {
    type Id = WireId;
    type NamedPosition = __WireNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __WireNamedPosition(
            __WireInternalPosition(graphite::TablePosition::from_index(b.wire.len())),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.wire(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.wire(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __WireNamedPosition {
    type Reference<'graph> = WireRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        WireRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl SocialDefaultId for Wire {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        SocialInsertable::insert_named_with_id(self, b, WireId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        SocialInsertable::insert_with_id(self, b, WireId(binding))
    }
}
impl SocialEdge for Wire {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_person: Vec::new(),
            friends: Vec::new(),
            wire: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    pub fn person(&mut self, id: PersonId, value: super::Person) -> &mut Self {
        self.__graphite_node_person.push((id, value));
        self
    }
    pub fn friends(&mut self, id: FriendsId, value: Friends) -> &mut Self {
        self.friends.push((id, value));
        self
    }
    pub fn wire(&mut self, id: WireId, value: Wire) -> &mut Self {
        self.wire.push((id, value));
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
        N: SocialNode + SocialDefaultId,
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
        N: SocialNode + SocialDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: SocialNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: SocialNode>(
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
        E: SocialEdge + SocialDefaultId,
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
        E: SocialEdge + SocialDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: SocialEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: SocialEdge>(
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
        T: SocialDefaultId,
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
                    graphite::TablePosition::from_index(__graphite_friends.len()),
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
        let mut __graphite_wire: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut wire_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_wire_by_pair: std::collections::HashMap<
            graphite::UnorderedPair<__PersonInternalPosition>,
            Vec<__WireInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.wire {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::WireDuplicateKey(id));
                continue;
            }
            let Wire { endpoints, cable } = value;
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
                    .push(Violation::WireUnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p0.clone(),
                    });
            }
            if p1 != p0 && second_position.is_none() {
                __violations
                    .push(Violation::WireUnknownEndpoint {
                        edge: id.clone(),
                        endpoint: p1.clone(),
                    });
            }
            if let (Some(first_position), Some(second_position)) = (
                first_position,
                second_position,
            ) {
                let internal_edge_position = __WireInternalPosition(
                    graphite::TablePosition::from_index(__graphite_wire.len()),
                );
                __graphite_wire_by_pair
                    .entry(graphite::UnorderedPair::new(first_position, second_position))
                    .or_default()
                    .push(internal_edge_position);
                wire_index
                    .entry(first_position)
                    .or_default()
                    .push(internal_edge_position);
                if second_position != first_position {
                    wire_index
                        .entry(second_position)
                        .or_default()
                        .push(internal_edge_position);
                }
                let inserted = __graphite_wire
                    .insert(
                        id,
                        __WireRecord {
                            endpoints: graphite::UnorderedPair::new(
                                first_position,
                                second_position,
                            ),
                            cable,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let friends_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    friends_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let wire_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    wire_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_person,
            friends: __graphite_friends,
            wire: __graphite_wire,
            friends_index,
            __graphite_friends_by_pair,
            wire_index,
            __graphite_wire_by_pair,
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
