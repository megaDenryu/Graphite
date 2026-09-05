// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/edge_roles.rs:50
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    12132313578864812107u64, 13719719992762152182u64, 7970136408359497809u64,
    6037311841120242053u64,
];
/// `Person` ノードの公開ID。
///
/// 宣言: `tests/edge_roles.rs` の `node Person`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);
/// `Item` ノードの公開ID。
///
/// 宣言: `tests/edge_roles.rs` の `node Item`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemId(pub String);
/// `Ownership` 辺の公開ID。
///
/// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnershipId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PersonInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ItemInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __OwnershipInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PersonNamedPosition(__PersonInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ItemNamedPosition(__ItemInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __OwnershipNamedPosition(__OwnershipInternalPosition, u64);
/// 構築時に組み立てる `Ownership` 辺の値。
///
/// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
#[derive(Clone, PartialEq)]
pub struct Ownership {
    /// この辺の始点ノードの公開ID。
    pub 所有者: PersonId,
    /// この辺の終点ノードの公開ID。
    pub 所有物: ItemId,
}
impl Ownership {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn new(from: PersonId, to: ItemId) -> Self {
        Self {
            所有者: from,
            所有物: to,
        }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, ItemId, ()> for Ownership {
    fn from_graph_literal(from: PersonId, to: ItemId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Ownership {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Ownership))
            .field(&self.所有者)
            .field(&self.所有物)
            .finish()
    }
}
#[allow(dead_code)]
struct __OwnershipRecord {
    所有者: __PersonInternalPosition,
    所有物: __ItemInternalPosition,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `tests/edge_roles.rs` の `schema JapaneseRoles`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    DuplicatePerson(PersonId),
    /// このノード種別のキーが重複している。
    DuplicateItem(ItemId),
    /// このエッジ種別のキーが重複している。
    OwnershipDuplicateKey(OwnershipId),
    /// このエッジが未知の始点キーを参照している。
    OwnershipUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: OwnershipId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: PersonId,
    },
    /// このエッジが未知の終点キーを参照している。
    OwnershipUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: OwnershipId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: ItemId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    Ownership所有者EachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: PersonId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
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
            Violation::OwnershipDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Ownership", id)
            }
            Violation::OwnershipUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として見つかりません (辺 `{}` {:?} の{})",
                    source, "Person", "Ownership", edge, "始点"
                )
            }
            Violation::OwnershipUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として見つかりません (辺 `{}` {:?} の{})",
                    target, "Item", "Ownership", edge, "終点"
                )
            }
            Violation::Ownership所有者EachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Ownership", "Person", source, "ちょうど1", count
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
/// 宣言: `tests/edge_roles.rs` の `schema JapaneseRoles`
pub struct Graph {
    __graphite_node_person: graphite::KeyedTable<PersonId, super::Person>,
    __graphite_node_item: graphite::KeyedTable<ItemId, super::Item>,
    ownership: graphite::KeyedTable<OwnershipId, __OwnershipRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    ownership_from_index: graphite::ExactlyOneRoleIndex<__OwnershipInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    ownership_to_index: graphite::MultipleRoleIndex<__OwnershipInternalPosition>,
    __graphite_ownership_by_pair: std::collections::HashMap<
        (__PersonInternalPosition, __ItemInternalPosition),
        Vec<__OwnershipInternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Person`
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
    /// 宣言: `tests/edge_roles.rs` の `node Person`
    pub fn person_value_mut(&mut self, id: &PersonId) -> Option<&mut super::Person> {
        self.__graphite_node_person.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Person`
    pub fn person_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph PersonId> {
        self.__graphite_node_person.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Person`
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
    /// 宣言: `tests/edge_roles.rs` の `node Person`
    pub fn person_len(&self) -> usize {
        self.__graphite_node_person.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Item`
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
    /// 宣言: `tests/edge_roles.rs` の `node Item`
    pub fn item_value_mut(&mut self, id: &ItemId) -> Option<&mut super::Item> {
        self.__graphite_node_item.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Item`
    pub fn item_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph ItemId> {
        self.__graphite_node_item.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Item`
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
    /// 宣言: `tests/edge_roles.rs` の `node Item`
    pub fn item_len(&self) -> usize {
        self.__graphite_node_item.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn ownership_by_id<'graph>(
        &'graph self,
        id: &OwnershipId,
    ) -> Option<OwnershipRef<'graph>> {
        Some(OwnershipRef {
            graph: self,
            internal_position: __OwnershipInternalPosition(self.ownership.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn ownership_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph OwnershipId> {
        self.ownership.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn ownership_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = OwnershipRef<'graph>> + 'graph {
        self.ownership
            .positions()
            .map(move |position| OwnershipRef {
                graph: self,
                internal_position: __OwnershipInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn ownership_len(&self) -> usize {
        self.ownership.len()
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
/// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
#[derive(Clone, Copy)]
pub struct OwnershipRef<'graph> {
    graph: &'graph Graph,
    internal_position: __OwnershipInternalPosition,
}
impl<'graph> OwnershipRef<'graph> {
    fn record(self) -> &'graph __OwnershipRecord {
        self.graph
            .ownership
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn id(self) -> &'graph OwnershipId {
        self.graph
            .ownership
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn 所有者(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().所有者.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn 所有物(self) -> ItemRef<'graph> {
        ItemRef {
            graph: self.graph,
            internal_position: __ItemInternalPosition(self.record().所有物.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn from(self) -> PersonRef<'graph> {
        self.所有者()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn to(self) -> ItemRef<'graph> {
        self.所有物()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn to_id(self) -> &'graph ItemId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for OwnershipRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(OwnershipRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
/// 制約検査を一切行わない。
///
/// 宣言: `tests/edge_roles.rs` の `schema JapaneseRoles`
pub struct Builder {
    __graphite_node_person: Vec<(PersonId, super::Person)>,
    __graphite_node_item: Vec<(ItemId, super::Item)>,
    ownership: Vec<(OwnershipId, Ownership)>,
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
pub trait JapaneseRolesInsertable: Sized {
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
pub trait JapaneseRolesDefaultId: JapaneseRolesInsertable {
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
pub trait JapaneseRolesNode: JapaneseRolesInsertable {}
impl JapaneseRolesInsertable for super::Person {
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
impl JapaneseRolesDefaultId for super::Person {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        JapaneseRolesInsertable::insert_named_with_id(self, b, PersonId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        JapaneseRolesInsertable::insert_with_id(self, b, PersonId(binding))
    }
}
impl JapaneseRolesNode for super::Person {}
/// 完成済みグラフ上の `Person` ノード個体。
///
/// 宣言: `tests/edge_roles.rs` の `node Person`
#[derive(Clone, Copy)]
pub struct PersonRef<'graph> {
    graph: &'graph Graph,
    internal_position: __PersonInternalPosition,
}
impl<'graph> PersonRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Person`
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
    /// 宣言: `tests/edge_roles.rs` の `node Person`
    pub fn value(self) -> &'graph super::Person {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する唯一の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn ownership_as_所有者(self) -> OwnershipRef<'graph> {
        OwnershipRef {
            graph: self.graph,
            internal_position: *self
                .graph
                .ownership_from_index
                .get(self.internal_position.0),
        }
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn ownership_try_between(
        self,
        other: ItemRef<'graph>,
    ) -> Result<
        impl Iterator<Item = OwnershipRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_ownership_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| OwnershipRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::ownership_try_between`] を使う。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn ownership_between(
        self,
        other: ItemRef<'graph>,
    ) -> impl Iterator<Item = OwnershipRef<'graph>> + 'graph {
        self.ownership_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef),
                    stringify!(ownership_between)
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
impl JapaneseRolesInsertable for super::Item {
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
impl JapaneseRolesDefaultId for super::Item {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        JapaneseRolesInsertable::insert_named_with_id(self, b, ItemId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        JapaneseRolesInsertable::insert_with_id(self, b, ItemId(binding))
    }
}
impl JapaneseRolesNode for super::Item {}
/// 完成済みグラフ上の `Item` ノード個体。
///
/// 宣言: `tests/edge_roles.rs` の `node Item`
#[derive(Clone, Copy)]
pub struct ItemRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ItemInternalPosition,
}
impl<'graph> ItemRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Item`
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
    /// 宣言: `tests/edge_roles.rs` の `node Item`
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
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn ownership_as_所有物(
        self,
    ) -> impl Iterator<Item = OwnershipRef<'graph>> + 'graph {
        let positions = self.graph.ownership_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| OwnershipRef {
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
pub trait JapaneseRolesEdge: JapaneseRolesInsertable {}
impl JapaneseRolesInsertable for Ownership {
    type Id = OwnershipId;
    type NamedPosition = __OwnershipNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __OwnershipNamedPosition(
            __OwnershipInternalPosition(
                graphite::TablePosition::from_index(b.ownership.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.ownership(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.ownership(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __OwnershipNamedPosition {
    type Reference<'graph> = OwnershipRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        OwnershipRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl JapaneseRolesDefaultId for Ownership {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        JapaneseRolesInsertable::insert_named_with_id(
            self,
            b,
            OwnershipId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        JapaneseRolesInsertable::insert_with_id(self, b, OwnershipId(binding))
    }
}
impl JapaneseRolesEdge for Ownership {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_person: Vec::new(),
            __graphite_node_item: Vec::new(),
            ownership: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Person`
    pub fn person(&mut self, id: PersonId, value: super::Person) -> &mut Self {
        self.__graphite_node_person.push((id, value));
        self
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/edge_roles.rs` の `node Item`
    pub fn item(&mut self, id: ItemId, value: super::Item) -> &mut Self {
        self.__graphite_node_item.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `tests/edge_roles.rs` の `edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1`
    pub fn ownership(&mut self, id: OwnershipId, value: Ownership) -> &mut Self {
        self.ownership.push((id, value));
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
        N: JapaneseRolesNode + JapaneseRolesDefaultId,
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
        N: JapaneseRolesNode + JapaneseRolesDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: JapaneseRolesNode>(
        &mut self,
        id: N::Id,
        value: N,
    ) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: JapaneseRolesNode>(
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
        E: JapaneseRolesEdge + JapaneseRolesDefaultId,
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
        E: JapaneseRolesEdge + JapaneseRolesDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: JapaneseRolesEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: JapaneseRolesEdge>(
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
        T: JapaneseRolesDefaultId,
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
        let mut __graphite_ownership: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut ownership_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut ownership_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_ownership_by_pair: std::collections::HashMap<
            (__PersonInternalPosition, __ItemInternalPosition),
            Vec<__OwnershipInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.ownership {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::OwnershipDuplicateKey(id));
                continue;
            }
            let Ownership { 所有者: from, 所有物: to } = value;
            let from_position = __graphite_node_person
                .position(&from)
                .map(__PersonInternalPosition);
            let to_position = __graphite_node_item
                .position(&to)
                .map(__ItemInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::OwnershipUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::OwnershipUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __OwnershipInternalPosition(
                    graphite::TablePosition::from_index(__graphite_ownership.len()),
                );
                __graphite_ownership_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                ownership_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                ownership_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_ownership
                    .insert(
                        id,
                        __OwnershipRecord {
                            所有者: from_position,
                            所有物: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Ownership) = |edge| {
            let _ = &edge.所有者;
        };
        for position in __graphite_node_person.positions() {
            let internal_position = __PersonInternalPosition(position);
            let key = __graphite_node_person
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = ownership_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if count != 1usize {
                __violations
                    .push(Violation::Ownership所有者EachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let ownership_from_index = graphite::ExactlyOneRoleIndex::from_buckets(
            __graphite_node_person
                .positions()
                .map(|position| {
                    ownership_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let ownership_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_item
                .positions()
                .map(|position| {
                    ownership_to_index
                        .remove(&__ItemInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_person,
            __graphite_node_item,
            ownership: __graphite_ownership,
            ownership_from_index,
            ownership_to_index,
            __graphite_ownership_by_pair,
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
