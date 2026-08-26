// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: examples/build-pipeline/src/schema.rs:55
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    17042905339929178511u64, 8905530070458688262u64, 6474964121283827073u64,
    17604770973997348229u64,
];
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConsumesId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __TaskInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ArtifactInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ProducesInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ConsumesInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __TaskNamedPosition(__TaskInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ArtifactNamedPosition(__ArtifactInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ProducesNamedPosition(__ProducesInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ConsumesNamedPosition(__ConsumesInternalPosition, u64);
#[derive(Clone, PartialEq)]
pub struct Produces {
    pub task: TaskId,
    pub artifact: ArtifactId,
}
impl Produces {
    pub fn new(from: TaskId, to: ArtifactId) -> Self {
        Self { task: from, artifact: to }
    }
}
impl graphite::DirectedEdgeLiteral<TaskId, ArtifactId, ()> for Produces {
    fn from_graph_literal(from: TaskId, to: ArtifactId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Produces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Produces))
    }
}
#[derive(Clone, PartialEq)]
pub struct Consumes {
    pub task: TaskId,
    pub artifact: ArtifactId,
}
impl Consumes {
    pub fn new(from: TaskId, to: ArtifactId) -> Self {
        Self { task: from, artifact: to }
    }
}
impl graphite::DirectedEdgeLiteral<TaskId, ArtifactId, ()> for Consumes {
    fn from_graph_literal(from: TaskId, to: ArtifactId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Consumes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Consumes))
    }
}
#[allow(dead_code)]
struct __ProducesRecord {
    task: __TaskInternalPosition,
    artifact: __ArtifactInternalPosition,
}
#[allow(dead_code)]
struct __ConsumesRecord {
    task: __TaskInternalPosition,
    artifact: __ArtifactInternalPosition,
}
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    DuplicateTask(TaskId),
    DuplicateArtifact(ArtifactId),
    /// このエッジ種別のキーが重複している。
    ProducesDuplicateKey(ProducesId),
    /// このエッジが未知の始点キーを参照している。
    ProducesUnknownSource { edge: ProducesId, source: TaskId },
    /// このエッジが未知の終点キーを参照している。
    ProducesUnknownTarget { edge: ProducesId, target: ArtifactId },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    ProducesUniquePairViolation { source: TaskId, target: ArtifactId },
    /// このエッジ種別のキーが重複している。
    ConsumesDuplicateKey(ConsumesId),
    /// このエッジが未知の始点キーを参照している。
    ConsumesUnknownSource { edge: ConsumesId, source: TaskId },
    /// このエッジが未知の終点キーを参照している。
    ConsumesUnknownTarget { edge: ConsumesId, target: ArtifactId },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    ConsumesUniquePairViolation { source: TaskId, target: ArtifactId },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicateTask(_) => {
                write!(f, "{}のキーが重複しています", "Task")
            }
            Violation::DuplicateArtifact(_) => {
                write!(f, "{}のキーが重複しています", "Artifact")
            }
            Violation::ProducesDuplicateKey(_) => {
                write!(f, "{}のキーが重複しています", "Produces")
            }
            Violation::ProducesUnknownSource { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の始点, {})",
                    "Produces", "Task"
                )
            }
            Violation::ProducesUnknownTarget { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の終点, {})",
                    "Produces", "Artifact"
                )
            }
            Violation::ProducesUniquePairViolation { .. } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` の同じ始点・終点の対に既に辺が存在します",
                    "Produces"
                )
            }
            Violation::ConsumesDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Consumes", id)
            }
            Violation::ConsumesUnknownSource { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の始点, {})",
                    "Consumes", "Task"
                )
            }
            Violation::ConsumesUnknownTarget { .. } => {
                write!(
                    f,
                    "未知のキーが参照されています (辺 `{}` の終点, {})",
                    "Consumes", "Artifact"
                )
            }
            Violation::ConsumesUniquePairViolation { .. } => {
                write!(
                    f,
                    "unique pair違反: 辺 `{}` の同じ始点・終点の対に既に辺が存在します",
                    "Consumes"
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
    __graphite_node_task: graphite::KeyedTable<TaskId, super::Task>,
    __graphite_node_artifact: graphite::KeyedTable<ArtifactId, super::Artifact>,
    produces: graphite::KeyedTable<ProducesId, __ProducesRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    produces_from_index: graphite::MultipleRoleIndex<__ProducesInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    produces_to_index: graphite::MultipleRoleIndex<__ProducesInternalPosition>,
    __graphite_produces_by_pair: std::collections::HashMap<
        (__TaskInternalPosition, __ArtifactInternalPosition),
        __ProducesInternalPosition,
    >,
    consumes: graphite::KeyedTable<ConsumesId, __ConsumesRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    consumes_from_index: graphite::MultipleRoleIndex<__ConsumesInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    consumes_to_index: graphite::MultipleRoleIndex<__ConsumesInternalPosition>,
    __graphite_consumes_by_pair: std::collections::HashMap<
        (__TaskInternalPosition, __ArtifactInternalPosition),
        __ConsumesInternalPosition,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    pub fn task_by_id<'graph>(&'graph self, id: &TaskId) -> Option<TaskRef<'graph>> {
        let internal_position = __TaskInternalPosition(
            self.__graphite_node_task.position(id)?,
        );
        Some(TaskRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    pub fn task_value_mut(&mut self, id: &TaskId) -> Option<&mut super::Task> {
        self.__graphite_node_task.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn task_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph TaskId> {
        self.__graphite_node_task.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    pub fn task_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = TaskRef<'graph>> + 'graph {
        self.__graphite_node_task
            .positions()
            .map(move |position| TaskRef {
                graph: self,
                internal_position: __TaskInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    pub fn task_len(&self) -> usize {
        self.__graphite_node_task.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    pub fn artifact_by_id<'graph>(
        &'graph self,
        id: &ArtifactId,
    ) -> Option<ArtifactRef<'graph>> {
        let internal_position = __ArtifactInternalPosition(
            self.__graphite_node_artifact.position(id)?,
        );
        Some(ArtifactRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    pub fn artifact_value_mut(
        &mut self,
        id: &ArtifactId,
    ) -> Option<&mut super::Artifact> {
        self.__graphite_node_artifact.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    pub fn artifact_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph ArtifactId> {
        self.__graphite_node_artifact.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    pub fn artifact_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ArtifactRef<'graph>> + 'graph {
        self.__graphite_node_artifact
            .positions()
            .map(move |position| ArtifactRef {
                graph: self,
                internal_position: __ArtifactInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    pub fn artifact_len(&self) -> usize {
        self.__graphite_node_artifact.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn produces_by_id<'graph>(
        &'graph self,
        id: &ProducesId,
    ) -> Option<ProducesRef<'graph>> {
        Some(ProducesRef {
            graph: self,
            internal_position: __ProducesInternalPosition(self.produces.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn produces_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph ProducesId> {
        self.produces.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn produces_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ProducesRef<'graph>> + 'graph {
        self.produces
            .positions()
            .map(move |position| ProducesRef {
                graph: self,
                internal_position: __ProducesInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn produces_len(&self) -> usize {
        self.produces.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    pub fn consumes_by_id<'graph>(
        &'graph self,
        id: &ConsumesId,
    ) -> Option<ConsumesRef<'graph>> {
        Some(ConsumesRef {
            graph: self,
            internal_position: __ConsumesInternalPosition(self.consumes.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    pub fn consumes_ids<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = &'graph ConsumesId> {
        self.consumes.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    pub fn consumes_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ConsumesRef<'graph>> + 'graph {
        self.consumes
            .positions()
            .map(move |position| ConsumesRef {
                graph: self,
                internal_position: __ConsumesInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    pub fn consumes_len(&self) -> usize {
        self.consumes.len()
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
pub struct ProducesRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ProducesInternalPosition,
}
impl<'graph> ProducesRef<'graph> {
    fn record(self) -> &'graph __ProducesRecord {
        self.graph
            .produces
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph ProducesId {
        self.graph
            .produces
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn task(self) -> TaskRef<'graph> {
        TaskRef {
            graph: self.graph,
            internal_position: __TaskInternalPosition(self.record().task.0),
        }
    }
    pub fn artifact(self) -> ArtifactRef<'graph> {
        ArtifactRef {
            graph: self.graph,
            internal_position: __ArtifactInternalPosition(self.record().artifact.0),
        }
    }
    pub fn from(self) -> TaskRef<'graph> {
        self.task()
    }
    pub fn to(self) -> ArtifactRef<'graph> {
        self.artifact()
    }
    pub fn from_id(self) -> &'graph TaskId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph ArtifactId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for ProducesRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ProducesRef))
    }
}
/// 完成済みグラフ上の有向辺個体。
#[derive(Clone, Copy)]
pub struct ConsumesRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ConsumesInternalPosition,
}
impl<'graph> ConsumesRef<'graph> {
    fn record(self) -> &'graph __ConsumesRecord {
        self.graph
            .consumes
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph ConsumesId {
        self.graph
            .consumes
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn task(self) -> TaskRef<'graph> {
        TaskRef {
            graph: self.graph,
            internal_position: __TaskInternalPosition(self.record().task.0),
        }
    }
    pub fn artifact(self) -> ArtifactRef<'graph> {
        ArtifactRef {
            graph: self.graph,
            internal_position: __ArtifactInternalPosition(self.record().artifact.0),
        }
    }
    pub fn from(self) -> TaskRef<'graph> {
        self.task()
    }
    pub fn to(self) -> ArtifactRef<'graph> {
        self.artifact()
    }
    pub fn from_id(self) -> &'graph TaskId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph ArtifactId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for ConsumesRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ConsumesRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 構築用 builder。凍結 (`freeze()`) までは where 制約検査を一切行わない。
pub struct Builder {
    __graphite_node_task: Vec<(TaskId, super::Task)>,
    __graphite_node_artifact: Vec<(ArtifactId, super::Artifact)>,
    produces: Vec<(ProducesId, Produces)>,
    consumes: Vec<(ConsumesId, Consumes)>,
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
pub trait BuildPipelineInsertable: Sized {
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
pub trait BuildPipelineDefaultId: BuildPipelineInsertable {
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
pub trait BuildPipelineNode: BuildPipelineInsertable {}
impl BuildPipelineInsertable for super::Task {
    type Id = TaskId;
    type NamedPosition = __TaskNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __TaskNamedPosition(
            __TaskInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_task.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.task(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.task(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __TaskNamedPosition {
    type Reference<'graph> = TaskRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        TaskRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl BuildPipelineNode for super::Task {}
///完成済みグラフ上の `Task` ノード個体。
#[derive(Clone, Copy)]
pub struct TaskRef<'graph> {
    graph: &'graph Graph,
    internal_position: __TaskInternalPosition,
}
impl<'graph> TaskRef<'graph> {
    pub fn id(self) -> &'graph TaskId {
        self.graph
            .__graphite_node_task
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::Task {
        self.graph
            .__graphite_node_task
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn produces_as_task(self) -> impl Iterator<Item = ProducesRef<'graph>> + 'graph {
        let positions = self.graph.produces_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ProducesRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn produces_try_between(
        self,
        other: ArtifactRef<'graph>,
    ) -> Result<Option<ProducesRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_produces_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| ProducesRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::produces_try_between`] を使う。
    pub fn produces_between(
        self,
        other: ArtifactRef<'graph>,
    ) -> Option<ProducesRef<'graph>> {
        self.produces_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(TaskRef), stringify!(produces_between)
                )
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn consumes_as_task(self) -> impl Iterator<Item = ConsumesRef<'graph>> + 'graph {
        let positions = self.graph.consumes_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ConsumesRef {
                graph: self.graph,
                internal_position,
            })
    }
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn consumes_try_between(
        self,
        other: ArtifactRef<'graph>,
    ) -> Result<Option<ConsumesRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_consumes_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| ConsumesRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::consumes_try_between`] を使う。
    pub fn consumes_between(
        self,
        other: ArtifactRef<'graph>,
    ) -> Option<ConsumesRef<'graph>> {
        self.consumes_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(TaskRef), stringify!(consumes_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for TaskRef<'graph> {
    type Target = super::Task;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_task
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for TaskRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(TaskRef))
    }
}
impl BuildPipelineInsertable for super::Artifact {
    type Id = ArtifactId;
    type NamedPosition = __ArtifactNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ArtifactNamedPosition(
            __ArtifactInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_artifact.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.artifact(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.artifact(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ArtifactNamedPosition {
    type Reference<'graph> = ArtifactRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ArtifactRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl BuildPipelineNode for super::Artifact {}
///完成済みグラフ上の `Artifact` ノード個体。
#[derive(Clone, Copy)]
pub struct ArtifactRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ArtifactInternalPosition,
}
impl<'graph> ArtifactRef<'graph> {
    pub fn id(self) -> &'graph ArtifactId {
        self.graph
            .__graphite_node_artifact
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::Artifact {
        self.graph
            .__graphite_node_artifact
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn produces_as_artifact(
        self,
    ) -> impl Iterator<Item = ProducesRef<'graph>> + 'graph {
        let positions = self.graph.produces_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ProducesRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn consumes_as_artifact(
        self,
    ) -> impl Iterator<Item = ConsumesRef<'graph>> + 'graph {
        let positions = self.graph.consumes_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ConsumesRef {
                graph: self.graph,
                internal_position,
            })
    }
}
impl<'graph> std::ops::Deref for ArtifactRef<'graph> {
    type Target = super::Artifact;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_artifact
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for ArtifactRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ArtifactRef))
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait BuildPipelineEdge: BuildPipelineInsertable {}
impl BuildPipelineInsertable for Produces {
    type Id = ProducesId;
    type NamedPosition = __ProducesNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ProducesNamedPosition(
            __ProducesInternalPosition(
                graphite::TablePosition::from_index(b.produces.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.produces(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.produces(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ProducesNamedPosition {
    type Reference<'graph> = ProducesRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ProducesRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl BuildPipelineEdge for Produces {}
impl BuildPipelineInsertable for Consumes {
    type Id = ConsumesId;
    type NamedPosition = __ConsumesNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ConsumesNamedPosition(
            __ConsumesInternalPosition(
                graphite::TablePosition::from_index(b.consumes.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.consumes(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.consumes(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ConsumesNamedPosition {
    type Reference<'graph> = ConsumesRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ConsumesRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl BuildPipelineDefaultId for Consumes {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        BuildPipelineInsertable::insert_named_with_id(
            self,
            b,
            ConsumesId(binding),
            permit,
        )
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        BuildPipelineInsertable::insert_with_id(self, b, ConsumesId(binding))
    }
}
impl BuildPipelineEdge for Consumes {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_task: Vec::new(),
            __graphite_node_artifact: Vec::new(),
            produces: Vec::new(),
            consumes: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    pub fn task(&mut self, id: TaskId, value: super::Task) -> &mut Self {
        self.__graphite_node_task.push((id, value));
        self
    }
    pub fn artifact(&mut self, id: ArtifactId, value: super::Artifact) -> &mut Self {
        self.__graphite_node_artifact.push((id, value));
        self
    }
    pub fn produces(&mut self, id: ProducesId, value: Produces) -> &mut Self {
        self.produces.push((id, value));
        self
    }
    pub fn consumes(&mut self, id: ConsumesId, value: Consumes) -> &mut Self {
        self.consumes.push((id, value));
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
        N: BuildPipelineNode + BuildPipelineDefaultId,
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
        N: BuildPipelineNode + BuildPipelineDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: BuildPipelineNode>(
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
    pub fn insert_named_with_id<N: BuildPipelineNode>(
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
        E: BuildPipelineEdge + BuildPipelineDefaultId,
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
        E: BuildPipelineEdge + BuildPipelineDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: BuildPipelineEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: BuildPipelineEdge>(
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
        T: BuildPipelineDefaultId,
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
        let mut __graphite_node_task: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_task {
            if !__graphite_node_task.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateTask(id));
            }
        }
        let mut __graphite_node_artifact: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_artifact {
            if !__graphite_node_artifact.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateArtifact(id));
            }
        }
        let mut __graphite_produces: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut produces_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut produces_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_produces_by_pair: std::collections::HashMap<
            (__TaskInternalPosition, __ArtifactInternalPosition),
            __ProducesInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.produces {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ProducesDuplicateKey(id));
                continue;
            }
            let Produces { task: from, artifact: to } = value;
            let from_position = __graphite_node_task
                .position(&from)
                .map(__TaskInternalPosition);
            let to_position = __graphite_node_artifact
                .position(&to)
                .map(__ArtifactInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::ProducesUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::ProducesUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                if __graphite_produces_by_pair
                    .contains_key(&(from_position, to_position))
                {
                    __violations
                        .push(Violation::ProducesUniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __ProducesInternalPosition(
                    graphite::TablePosition::from_index(__graphite_produces.len()),
                );
                __graphite_produces_by_pair
                    .insert((from_position, to_position), internal_edge_position);
                produces_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                produces_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_produces
                    .insert(
                        id,
                        __ProducesRecord {
                            task: from_position,
                            artifact: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_consumes: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut consumes_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut consumes_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_consumes_by_pair: std::collections::HashMap<
            (__TaskInternalPosition, __ArtifactInternalPosition),
            __ConsumesInternalPosition,
        > = std::collections::HashMap::new();
        for (id, value) in self.consumes {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ConsumesDuplicateKey(id));
                continue;
            }
            let Consumes { task: from, artifact: to } = value;
            let from_position = __graphite_node_task
                .position(&from)
                .map(__TaskInternalPosition);
            let to_position = __graphite_node_artifact
                .position(&to)
                .map(__ArtifactInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::ConsumesUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::ConsumesUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                if __graphite_consumes_by_pair
                    .contains_key(&(from_position, to_position))
                {
                    __violations
                        .push(Violation::ConsumesUniquePairViolation {
                            source: from.clone(),
                            target: to.clone(),
                        });
                }
                let internal_edge_position = __ConsumesInternalPosition(
                    graphite::TablePosition::from_index(__graphite_consumes.len()),
                );
                __graphite_consumes_by_pair
                    .insert((from_position, to_position), internal_edge_position);
                consumes_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                consumes_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_consumes
                    .insert(
                        id,
                        __ConsumesRecord {
                            task: from_position,
                            artifact: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let produces_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_task
                .positions()
                .map(|position| {
                    produces_from_index
                        .remove(&__TaskInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let produces_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_artifact
                .positions()
                .map(|position| {
                    produces_to_index
                        .remove(&__ArtifactInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let consumes_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_task
                .positions()
                .map(|position| {
                    consumes_from_index
                        .remove(&__TaskInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let consumes_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_artifact
                .positions()
                .map(|position| {
                    consumes_to_index
                        .remove(&__ArtifactInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_task,
            __graphite_node_artifact,
            produces: __graphite_produces,
            consumes: __graphite_consumes,
            produces_from_index,
            produces_to_index,
            __graphite_produces_by_pair,
            consumes_from_index,
            consumes_to_index,
            __graphite_consumes_by_pair,
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
