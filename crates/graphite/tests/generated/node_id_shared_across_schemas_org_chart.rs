// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: crates/graphite/tests/node_id_shared_across_schemas.rs:41
// 再生成: リポジトリルートで `cargo xtask generate` を実行する。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    3943974244354000377u64, 2237153179102795534u64, 16642672125601081459u64,
    1196869239246215543u64,
];
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BelongsToId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PersonInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __DepartmentInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __BelongsToInternalPosition(usize);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PersonNamedPosition(__PersonInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __DepartmentNamedPosition(__DepartmentInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __BelongsToNamedPosition(__BelongsToInternalPosition, u64);
#[derive(Clone, PartialEq)]
pub struct BelongsTo {
    pub person: PersonId,
    pub department: DepartmentId,
}
impl BelongsTo {
    pub fn new(from: PersonId, to: DepartmentId) -> Self {
        Self {
            person: from,
            department: to,
        }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, DepartmentId, ()> for BelongsTo {
    fn from_graph_literal(from: PersonId, to: DepartmentId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for BelongsTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(BelongsTo))
    }
}
#[allow(dead_code)]
struct __BelongsToRecord {
    person: __PersonInternalPosition,
    department: __DepartmentInternalPosition,
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    DuplicatePerson(PersonId),
    DuplicateDepartment(DepartmentId),
    /// このエッジ種別のキーが重複している。
    BelongsToDuplicateKey(BelongsToId),
    /// このエッジが未知の始点キーを参照している。
    BelongsToUnknownSource { edge: BelongsToId, source: PersonId },
    /// このエッジが未知の終点キーを参照している。
    BelongsToUnknownTarget { edge: BelongsToId, target: DepartmentId },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    BelongsToPersonEachViolation { source: PersonId, count: usize },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicatePerson(_) => {
                write!(f, "{}のキーが重複しています", "Person")
            }
            Violation::DuplicateDepartment(_) => {
                write!(f, "{}のキーが重複しています", "Department")
            }
            Violation::BelongsToDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "BelongsTo", id)
            }
            Violation::BelongsToUnknownSource { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の始点, {})",
                    "BelongsTo", "Person"
                )
            }
            Violation::BelongsToUnknownTarget { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の終点, {})",
                    "BelongsTo", "Department"
                )
            }
            Violation::BelongsToPersonEachViolation { count, .. } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} の出次数 {} を期待しますが実際は {} 本です",
                    "BelongsTo", "Person", "0..1", count
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
    __graphite_node_department: graphite::KeyedTable<DepartmentId, super::Department>,
    belongs_to: graphite::KeyedTable<BelongsToId, __BelongsToRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    belongs_to_from_index: graphite::OptionalRoleIndex<__BelongsToInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    belongs_to_to_index: graphite::MultipleRoleIndex<__BelongsToInternalPosition>,
    __graphite_belongs_to_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __DepartmentInternalPosition),
        Vec<__BelongsToInternalPosition>,
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
    pub fn department_by_id<'graph>(
        &'graph self,
        id: &DepartmentId,
    ) -> Option<DepartmentRef<'graph>> {
        let internal_position = __DepartmentInternalPosition(
            self.__graphite_node_department.position(id)?,
        );
        Some(DepartmentRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    pub fn department_value_mut(
        &mut self,
        id: &DepartmentId,
    ) -> Option<&mut super::Department> {
        self.__graphite_node_department.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn department_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph DepartmentId> {
        self.__graphite_node_department.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    pub fn department_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = DepartmentRef<'graph>> + 'graph {
        self.__graphite_node_department
            .positions()
            .map(move |position| DepartmentRef {
                graph: self,
                internal_position: __DepartmentInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    pub fn department_len(&self) -> usize {
        self.__graphite_node_department.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn belongs_to_by_id<'graph>(
        &'graph self,
        id: &BelongsToId,
    ) -> Option<BelongsToRef<'graph>> {
        Some(BelongsToRef {
            graph: self,
            internal_position: __BelongsToInternalPosition(self.belongs_to.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn belongs_to_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph BelongsToId> {
        self.belongs_to.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn belongs_to_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = BelongsToRef<'graph>> + 'graph {
        self.belongs_to
            .positions()
            .map(move |position| BelongsToRef {
                graph: self,
                internal_position: __BelongsToInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn belongs_to_len(&self) -> usize {
        self.belongs_to.len()
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
pub struct BelongsToRef<'graph> {
    graph: &'graph Graph,
    internal_position: __BelongsToInternalPosition,
}
impl<'graph> BelongsToRef<'graph> {
    fn record(self) -> &'graph __BelongsToRecord {
        self.graph
            .belongs_to
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph BelongsToId {
        self.graph
            .belongs_to
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn person(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().person.0),
        }
    }
    pub fn department(self) -> DepartmentRef<'graph> {
        DepartmentRef {
            graph: self.graph,
            internal_position: __DepartmentInternalPosition(self.record().department.0),
        }
    }
    pub fn from(self) -> PersonRef<'graph> {
        self.person()
    }
    pub fn to(self) -> DepartmentRef<'graph> {
        self.department()
    }
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph DepartmentId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for BelongsToRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(BelongsToRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
pub struct Builder {
    __graphite_node_person: Vec<(PersonId, super::Person)>,
    __graphite_node_department: Vec<(DepartmentId, super::Department)>,
    belongs_to: Vec<(BelongsToId, BelongsTo)>,
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
pub trait OrgChartInsertable: Sized {
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
pub trait OrgChartDefaultId: OrgChartInsertable {
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
pub trait OrgChartNode: OrgChartInsertable {}
impl OrgChartInsertable for super::Person {
    type Id = PersonId;
    type NamedPosition = __PersonNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __PersonNamedPosition(
            __PersonInternalPosition(b.__graphite_node_person.len()),
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
impl OrgChartNode for super::Person {}
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
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    pub fn belongs_to_as_person(self) -> Option<BelongsToRef<'graph>> {
        self.graph
            .belongs_to_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| BelongsToRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn belongs_to_try_between(
        self,
        other: DepartmentRef<'graph>,
    ) -> Result<
        impl Iterator<Item = BelongsToRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_belongs_to_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| BelongsToRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::belongs_to_try_between`] を使う。
    pub fn belongs_to_between(
        self,
        other: DepartmentRef<'graph>,
    ) -> impl Iterator<Item = BelongsToRef<'graph>> + 'graph {
        self.belongs_to_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef),
                    stringify!(belongs_to_between)
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
        f.write_str(stringify!(PersonRef))
    }
}
impl OrgChartInsertable for super::Department {
    type Id = DepartmentId;
    type NamedPosition = __DepartmentNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __DepartmentNamedPosition(
            __DepartmentInternalPosition(b.__graphite_node_department.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.department(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.department(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __DepartmentNamedPosition {
    type Reference<'graph> = DepartmentRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        DepartmentRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrgChartNode for super::Department {}
///完成済みグラフ上の `Department` ノード個体。
#[derive(Clone, Copy)]
pub struct DepartmentRef<'graph> {
    graph: &'graph Graph,
    internal_position: __DepartmentInternalPosition,
}
impl<'graph> DepartmentRef<'graph> {
    pub fn id(self) -> &'graph DepartmentId {
        self.graph
            .__graphite_node_department
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::Department {
        self.graph
            .__graphite_node_department
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn belongs_to_as_department(
        self,
    ) -> impl Iterator<Item = BelongsToRef<'graph>> + 'graph {
        let positions = self.graph.belongs_to_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| BelongsToRef {
                graph: self.graph,
                internal_position,
            })
    }
}
impl<'graph> std::ops::Deref for DepartmentRef<'graph> {
    type Target = super::Department;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_department
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for DepartmentRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(DepartmentRef))
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait OrgChartEdge: OrgChartInsertable {}
impl OrgChartInsertable for BelongsTo {
    type Id = BelongsToId;
    type NamedPosition = __BelongsToNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __BelongsToNamedPosition(
            __BelongsToInternalPosition(b.belongs_to.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.belongs_to(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.belongs_to(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __BelongsToNamedPosition {
    type Reference<'graph> = BelongsToRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        BelongsToRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrgChartDefaultId for BelongsTo {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrgChartInsertable::insert_named_with_id(self, b, BelongsToId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrgChartInsertable::insert_with_id(self, b, BelongsToId(binding))
    }
}
impl OrgChartEdge for BelongsTo {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_person: Vec::new(),
            __graphite_node_department: Vec::new(),
            belongs_to: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    pub fn person(&mut self, id: PersonId, value: super::Person) -> &mut Self {
        self.__graphite_node_person.push((id, value));
        self
    }
    pub fn department(
        &mut self,
        id: DepartmentId,
        value: super::Department,
    ) -> &mut Self {
        self.__graphite_node_department.push((id, value));
        self
    }
    pub fn belongs_to(&mut self, id: BelongsToId, value: BelongsTo) -> &mut Self {
        self.belongs_to.push((id, value));
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
        N: OrgChartNode + OrgChartDefaultId,
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
        N: OrgChartNode + OrgChartDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: OrgChartNode>(&mut self, id: N::Id, value: N) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: OrgChartNode>(
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
        E: OrgChartEdge + OrgChartDefaultId,
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
        E: OrgChartEdge + OrgChartDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: OrgChartEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: OrgChartEdge>(
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
        T: OrgChartDefaultId,
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
        let mut __graphite_node_department: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_department {
            if !__graphite_node_department.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateDepartment(id));
            }
        }
        let mut __graphite_belongs_to: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut belongs_to_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut belongs_to_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_belongs_to_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __DepartmentInternalPosition),
            Vec<__BelongsToInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.belongs_to {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::BelongsToDuplicateKey(id));
                continue;
            }
            let BelongsTo { person: from, department: to } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_department
                .position(&to)
                .map(__DepartmentInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::BelongsToUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::BelongsToUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __BelongsToInternalPosition(
                    __graphite_belongs_to.len(),
                );
                __graphite_belongs_to_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                belongs_to_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                belongs_to_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_belongs_to
                    .insert(
                        id,
                        __BelongsToRecord {
                            person: from_position,
                            department: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&BelongsTo) = |edge| {
            let _ = &edge.person;
        };
        for position in __graphite_node_person.positions() {
            let internal_position = __PersonInternalPosition(position);
            let key = __graphite_node_person
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = belongs_to_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::BelongsToPersonEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let belongs_to_from_index = graphite::OptionalRoleIndex::from_buckets(
            (0..__graphite_node_person.len())
                .map(|position| {
                    belongs_to_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let belongs_to_to_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_department.len())
                .map(|position| {
                    belongs_to_to_index
                        .remove(&__DepartmentInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_person,
            __graphite_node_department,
            belongs_to: __graphite_belongs_to,
            belongs_to_from_index,
            belongs_to_to_index,
            __graphite_belongs_to_by_pair,
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
