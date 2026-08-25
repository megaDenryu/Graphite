// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: examples/async-dag/src/schema.rs:40
// 再生成: リポジトリルートで `cargo xtask generate` を実行する。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    12963486527544898402u64, 14105697897529115437u64, 10476055547082950308u64,
    17392680166970779096u64,
];
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DependsOnId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ServiceInternalPosition(usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __DependsOnInternalPosition(usize);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ServiceNamedPosition(__ServiceInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __DependsOnNamedPosition(__DependsOnInternalPosition, u64);
#[derive(Clone, PartialEq)]
pub struct DependsOn {
    pub dependent: ServiceId,
    pub dependency: ServiceId,
}
impl DependsOn {
    pub fn new(from: ServiceId, to: ServiceId) -> Self {
        Self {
            dependent: from,
            dependency: to,
        }
    }
}
impl graphite::DirectedEdgeLiteral<ServiceId, ServiceId, ()> for DependsOn {
    fn from_graph_literal(from: ServiceId, to: ServiceId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for DependsOn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(DependsOn))
            .field(&self.dependent)
            .field(&self.dependency)
            .finish()
    }
}
#[allow(dead_code)]
struct __DependsOnRecord {
    dependent: __ServiceInternalPosition,
    dependency: __ServiceInternalPosition,
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    DuplicateService(ServiceId),
    /// このエッジ種別のキーが重複している。
    DependsOnDuplicateKey(DependsOnId),
    /// このエッジが未知の始点キーを参照している。
    DependsOnUnknownSource { edge: DependsOnId, source: ServiceId },
    /// このエッジが未知の終点キーを参照している。
    DependsOnUnknownTarget { edge: DependsOnId, target: ServiceId },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    DependsOnUniquePairViolation { source: ServiceId, target: ServiceId },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicateService(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Service", id)
            }
            Violation::DependsOnDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "DependsOn", id)
            }
            Violation::DependsOnUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の始点, {}): {:?}",
                    "DependsOn", edge, "Service", source
                )
            }
            Violation::DependsOnUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` {:?} の終点, {}): {:?}",
                    "DependsOn", edge, "Service", target
                )
            }
            Violation::DependsOnUniquePairViolation { source, target } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` は {:?} -> {:?} の対に既に辺が存在します",
                    "DependsOn", source, target
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
    __graphite_node_service: graphite::KeyedTable<ServiceId, super::Service>,
    depends_on: graphite::KeyedTable<DependsOnId, __DependsOnRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    depends_on_from_index: graphite::MultipleRoleIndex<__DependsOnInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    depends_on_to_index: graphite::MultipleRoleIndex<__DependsOnInternalPosition>,
    __graphite_depends_on_by_pair: std::collections::HashMap<
        (__ServiceInternalPosition, __ServiceInternalPosition),
        __DependsOnInternalPosition,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    pub fn service_by_id<'graph>(
        &'graph self,
        id: &ServiceId,
    ) -> Option<ServiceRef<'graph>> {
        let internal_position = __ServiceInternalPosition(
            self.__graphite_node_service.position(id)?,
        );
        Some(ServiceRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    pub fn service_value_mut(&mut self, id: &ServiceId) -> Option<&mut super::Service> {
        self.__graphite_node_service.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn service_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph ServiceId> {
        self.__graphite_node_service.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    pub fn service_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ServiceRef<'graph>> + 'graph {
        self.__graphite_node_service
            .positions()
            .map(move |position| ServiceRef {
                graph: self,
                internal_position: __ServiceInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    pub fn service_len(&self) -> usize {
        self.__graphite_node_service.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn depends_on_by_id<'graph>(
        &'graph self,
        id: &DependsOnId,
    ) -> Option<DependsOnRef<'graph>> {
        Some(DependsOnRef {
            graph: self,
            internal_position: __DependsOnInternalPosition(self.depends_on.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn depends_on_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph DependsOnId> {
        self.depends_on.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn depends_on_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = DependsOnRef<'graph>> + 'graph {
        self.depends_on
            .positions()
            .map(move |position| DependsOnRef {
                graph: self,
                internal_position: __DependsOnInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn depends_on_len(&self) -> usize {
        self.depends_on.len()
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
pub struct DependsOnRef<'graph> {
    graph: &'graph Graph,
    internal_position: __DependsOnInternalPosition,
}
impl<'graph> DependsOnRef<'graph> {
    fn record(self) -> &'graph __DependsOnRecord {
        self.graph
            .depends_on
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph DependsOnId {
        self.graph
            .depends_on
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn dependent(self) -> ServiceRef<'graph> {
        ServiceRef {
            graph: self.graph,
            internal_position: __ServiceInternalPosition(self.record().dependent.0),
        }
    }
    pub fn dependency(self) -> ServiceRef<'graph> {
        ServiceRef {
            graph: self.graph,
            internal_position: __ServiceInternalPosition(self.record().dependency.0),
        }
    }
    pub fn from(self) -> ServiceRef<'graph> {
        self.dependent()
    }
    pub fn to(self) -> ServiceRef<'graph> {
        self.dependency()
    }
    pub fn from_id(self) -> &'graph ServiceId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph ServiceId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for DependsOnRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(DependsOnRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
pub struct Builder {
    __graphite_node_service: Vec<(ServiceId, super::Service)>,
    depends_on: Vec<(DependsOnId, DependsOn)>,
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
pub trait OrchestrationInsertable: Sized {
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
pub trait OrchestrationDefaultId: OrchestrationInsertable {
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
pub trait OrchestrationNode: OrchestrationInsertable {}
impl OrchestrationInsertable for super::Service {
    type Id = ServiceId;
    type NamedPosition = __ServiceNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ServiceNamedPosition(
            __ServiceInternalPosition(b.__graphite_node_service.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.service(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.service(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ServiceNamedPosition {
    type Reference<'graph> = ServiceRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ServiceRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrchestrationDefaultId for super::Service {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrchestrationInsertable::insert_named_with_id(
            self,
            b,
            ServiceId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrchestrationInsertable::insert_with_id(self, b, ServiceId(binding))
    }
}
impl OrchestrationNode for super::Service {}
///完成済みグラフ上の `Service` ノード個体。
#[derive(Clone, Copy)]
pub struct ServiceRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ServiceInternalPosition,
}
impl<'graph> ServiceRef<'graph> {
    pub fn id(self) -> &'graph ServiceId {
        self.graph
            .__graphite_node_service
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::Service {
        self.graph
            .__graphite_node_service
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn depends_on_as_dependent(
        self,
    ) -> impl Iterator<Item = DependsOnRef<'graph>> + 'graph {
        let positions = self.graph.depends_on_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| DependsOnRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn depends_on_as_dependency(
        self,
    ) -> impl Iterator<Item = DependsOnRef<'graph>> + 'graph {
        let positions = self.graph.depends_on_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| DependsOnRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn depends_on_try_between(
        self,
        other: ServiceRef<'graph>,
    ) -> Result<Option<DependsOnRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_depends_on_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| DependsOnRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::depends_on_try_between`] を使う。
    pub fn depends_on_between(
        self,
        other: ServiceRef<'graph>,
    ) -> Option<DependsOnRef<'graph>> {
        self.depends_on_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(ServiceRef),
                    stringify!(depends_on_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for ServiceRef<'graph> {
    type Target = super::Service;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_service
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for ServiceRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ServiceRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait OrchestrationEdge: OrchestrationInsertable {}
impl OrchestrationInsertable for DependsOn {
    type Id = DependsOnId;
    type NamedPosition = __DependsOnNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __DependsOnNamedPosition(
            __DependsOnInternalPosition(b.depends_on.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.depends_on(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.depends_on(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __DependsOnNamedPosition {
    type Reference<'graph> = DependsOnRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        DependsOnRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl OrchestrationDefaultId for DependsOn {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        OrchestrationInsertable::insert_named_with_id(
            self,
            b,
            DependsOnId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        OrchestrationInsertable::insert_with_id(self, b, DependsOnId(binding))
    }
}
impl OrchestrationEdge for DependsOn {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_service: Vec::new(),
            depends_on: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    pub fn service(&mut self, id: ServiceId, value: super::Service) -> &mut Self {
        self.__graphite_node_service.push((id, value));
        self
    }
    pub fn depends_on(&mut self, id: DependsOnId, value: DependsOn) -> &mut Self {
        self.depends_on.push((id, value));
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
        N: OrchestrationNode + OrchestrationDefaultId,
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
        N: OrchestrationNode + OrchestrationDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: OrchestrationNode>(
        &mut self,
        id: N::Id,
        value: N,
    ) -> N::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付きノードを名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
    #[doc(hidden)]
    pub fn insert_named_with_id<N: OrchestrationNode>(
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
        E: OrchestrationEdge + OrchestrationDefaultId,
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
        E: OrchestrationEdge + OrchestrationDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: OrchestrationEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/lib.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: OrchestrationEdge>(
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
        T: OrchestrationDefaultId,
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
        let mut __graphite_node_service: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_service {
            if !__graphite_node_service.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateService(id));
            }
        }
        let mut __graphite_depends_on: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut depends_on_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut depends_on_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_depends_on_by_pair: std::collections::HashMap<
            (__ServiceInternalPosition, __ServiceInternalPosition),
            __DependsOnInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.depends_on {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::DependsOnDuplicateKey(id));
                continue;
            }
            let DependsOn { dependent: from, dependency: to } = value;
            let from_position = __graphite_node_service
                .position(&from)
                .map(__ServiceInternalPosition);
            let to_position = __graphite_node_service
                .position(&to)
                .map(__ServiceInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::DependsOnUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::DependsOnUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                if __graphite_depends_on_by_pair
                    .contains_key(&(from_position, to_position))
                {
                    __violations
                        .push(Violation::DependsOnUniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __DependsOnInternalPosition(
                    __graphite_depends_on.len(),
                );
                __graphite_depends_on_by_pair
                    .insert((from_position, to_position), internal_edge_position);
                depends_on_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                depends_on_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_depends_on
                    .insert(
                        id,
                        __DependsOnRecord {
                            dependent: from_position,
                            dependency: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let depends_on_from_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_service.len())
                .map(|position| {
                    depends_on_from_index
                        .remove(&__ServiceInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let depends_on_to_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_service.len())
                .map(|position| {
                    depends_on_to_index
                        .remove(&__ServiceInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_service,
            depends_on: __graphite_depends_on,
            depends_on_from_index,
            depends_on_to_index,
            __graphite_depends_on_by_pair,
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
