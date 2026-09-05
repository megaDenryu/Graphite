// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/schema.rs:49
// 再生成: パッケージのディレクトリで `cargo graphite generate` を実行する
//         (Graphite リポジトリ自身の開発では `cargo xtask generate`)。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    1676735987963901790u64, 9705446312033110733u64, 5266845505104514108u64,
    1170608020221244200u64,
];
/// `Scene` ノードの公開ID。
///
/// 宣言: `src/schema.rs` の `node Scene`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SceneId(pub String);
/// `Ending` ノードの公開ID。
///
/// 宣言: `src/schema.rs` の `node Ending`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EndingId(pub String);
/// `Choice` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChoiceId(pub String);
/// `Finale` 辺の公開ID。
///
/// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FinaleId(pub String);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __SceneInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __EndingInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __ChoiceInternalPosition(graphite::TablePosition);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __FinaleInternalPosition(graphite::TablePosition);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __SceneNamedPosition(__SceneInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __EndingNamedPosition(__EndingInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ChoiceNamedPosition(__ChoiceInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __FinaleNamedPosition(__FinaleInternalPosition, u64);
/// 構築時に組み立てる `Choice` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
#[derive(Clone)]
pub struct Choice {
    /// この辺の始点ノードの公開ID。
    pub scene: SceneId,
    /// この辺の終点ノードの公開ID。
    pub next: SceneId,
    /// この辺が運ぶ積み荷。
    pub choice: ChoiceEdge,
}
impl Choice {
    /// 始点と終点の公開IDと積み荷から構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn new(from: SceneId, to: SceneId, payload: ChoiceEdge) -> Self {
        Self {
            scene: from,
            next: to,
            choice: payload,
        }
    }
    /// この辺値が運ぶ積み荷を借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn payload(&self) -> &ChoiceEdge {
        &self.choice
    }
}
impl graphite::DirectedEdgeLiteral<SceneId, SceneId, ChoiceEdge> for Choice {
    fn from_graph_literal(from: SceneId, to: SceneId, payload: ChoiceEdge) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Choice))
    }
}
/// 構築時に組み立てる `Finale` 辺の値。
///
/// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
#[derive(Clone, PartialEq)]
pub struct Finale {
    /// この辺の始点ノードの公開ID。
    pub scene: SceneId,
    /// この辺の終点ノードの公開ID。
    pub ending: EndingId,
}
impl Finale {
    /// 始点と終点の公開IDから構築用の辺値を作る。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn new(from: SceneId, to: EndingId) -> Self {
        Self { scene: from, ending: to }
    }
}
impl graphite::DirectedEdgeLiteral<SceneId, EndingId, ()> for Finale {
    fn from_graph_literal(from: SceneId, to: EndingId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Finale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Finale)).field(&self.scene).field(&self.ending).finish()
    }
}
#[allow(dead_code)]
struct __ChoiceRecord {
    scene: __SceneInternalPosition,
    next: __SceneInternalPosition,
    choice: ChoiceEdge,
}
#[allow(dead_code)]
struct __FinaleRecord {
    scene: __SceneInternalPosition,
    ending: __EndingInternalPosition,
}
/// 凍結時の図式適合検査が見つけた違反。
///
/// 宣言: `src/schema.rs` の `schema DialogueGraph`
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    /// このノード種別のキーが重複している。
    DuplicateScene(SceneId),
    /// このノード種別のキーが重複している。
    DuplicateEnding(EndingId),
    /// このエッジ種別のキーが重複している。
    ChoiceDuplicateKey(ChoiceId),
    /// このエッジが未知の始点キーを参照している。
    ChoiceUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: ChoiceId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: SceneId,
    },
    /// このエッジが未知の終点キーを参照している。
    ChoiceUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: ChoiceId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: SceneId,
    },
    /// このエッジ種別のキーが重複している。
    FinaleDuplicateKey(FinaleId),
    /// このエッジが未知の始点キーを参照している。
    FinaleUnknownSource {
        /// 未知のキーを参照した辺の公開ID。
        edge: FinaleId,
        /// この辺が始点として参照した、対応するノードが存在しないキー。
        source: SceneId,
    },
    /// このエッジが未知の終点キーを参照している。
    FinaleUnknownTarget {
        /// 未知のキーを参照した辺の公開ID。
        edge: FinaleId,
        /// この辺が終点として参照した、対応するノードが存在しないキー。
        target: EndingId,
    },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    FinaleSceneEachViolation {
        /// 出次数が制約に反した始点ノードの公開ID。
        source: SceneId,
        /// この辺種別で、この始点から実際に出ている辺の本数。
        count: usize,
    },
}
impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Violation::DuplicateScene(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Scene", id)
            }
            Violation::DuplicateEnding(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Ending", id)
            }
            Violation::ChoiceDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Choice", id)
            }
            Violation::ChoiceUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    source, "Scene", "Choice", edge, "始点"
                )
            }
            Violation::ChoiceUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    target, "Scene", "Choice", edge, "終点"
                )
            }
            Violation::FinaleDuplicateKey(id) => {
                write!(f, "{}のキーが重複しています: {:?}", "Finale", id)
            }
            Violation::FinaleUnknownSource { edge, source } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    source, "Scene", "Finale", edge, "始点"
                )
            }
            Violation::FinaleUnknownTarget { edge, target } => {
                write!(
                    f,
                    "未知のキー {:?} が {} として解決できません (辺 `{}` {:?} の{})",
                    target, "Ending", "Finale", edge, "終点"
                )
            }
            Violation::FinaleSceneEachViolation { source, count } => {
                write!(
                    f,
                    "多重度制約違反: 辺 `{}` は {} {:?} について出次数 {} を期待しますが実際は {} 本です",
                    "Finale", "Scene", source, "0..1", count
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
/// 宣言: `src/schema.rs` の `schema DialogueGraph`
pub struct Graph {
    __graphite_node_scene: graphite::KeyedTable<SceneId, super::Scene>,
    __graphite_node_ending: graphite::KeyedTable<EndingId, super::Ending>,
    choice: graphite::KeyedTable<ChoiceId, __ChoiceRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    choice_from_index: graphite::MultipleRoleIndex<__ChoiceInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    choice_to_index: graphite::MultipleRoleIndex<__ChoiceInternalPosition>,
    __graphite_choice_by_pair: std::collections::HashMap<
        (__SceneInternalPosition, __SceneInternalPosition),
        Vec<__ChoiceInternalPosition>,
    >,
    finale: graphite::KeyedTable<FinaleId, __FinaleRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    finale_from_index: graphite::OptionalRoleIndex<__FinaleInternalPosition>,
    /// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
    /// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
    finale_to_index: graphite::MultipleRoleIndex<__FinaleInternalPosition>,
    __graphite_finale_by_pair: std::collections::HashMap<
        (__SceneInternalPosition, __EndingInternalPosition),
        Vec<__FinaleInternalPosition>,
    >,
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
}
impl Graph {
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `node Scene`
    pub fn scene_by_id<'graph>(&'graph self, id: &SceneId) -> Option<SceneRef<'graph>> {
        let internal_position = __SceneInternalPosition(
            self.__graphite_node_scene.position(id)?,
        );
        Some(SceneRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `src/schema.rs` の `node Scene`
    pub fn scene_value_mut(&mut self, id: &SceneId) -> Option<&mut super::Scene> {
        self.__graphite_node_scene.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `node Scene`
    pub fn scene_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph SceneId> {
        self.__graphite_node_scene.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `node Scene`
    pub fn scene_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = SceneRef<'graph>> + 'graph {
        self.__graphite_node_scene
            .positions()
            .map(move |position| SceneRef {
                graph: self,
                internal_position: __SceneInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `node Scene`
    pub fn scene_len(&self) -> usize {
        self.__graphite_node_scene.len()
    }
    /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `node Ending`
    pub fn ending_by_id<'graph>(
        &'graph self,
        id: &EndingId,
    ) -> Option<EndingRef<'graph>> {
        let internal_position = __EndingInternalPosition(
            self.__graphite_node_ending.position(id)?,
        );
        Some(EndingRef {
            graph: self,
            internal_position,
        })
    }
    /// グラフの構造を保ったままノード値だけを可変借用する。
    ///
    /// 宣言: `src/schema.rs` の `node Ending`
    pub fn ending_value_mut(&mut self, id: &EndingId) -> Option<&mut super::Ending> {
        self.__graphite_node_ending.get_mut(id)
    }
    /// この種別のノードの公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `node Ending`
    pub fn ending_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph EndingId> {
        self.__graphite_node_ending.ids()
    }
    /// この種別のノード個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `node Ending`
    pub fn ending_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = EndingRef<'graph>> + 'graph {
        self.__graphite_node_ending
            .positions()
            .map(move |position| EndingRef {
                graph: self,
                internal_position: __EndingInternalPosition(position),
            })
    }
    /// この種別のノードの件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `node Ending`
    pub fn ending_len(&self) -> usize {
        self.__graphite_node_ending.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice_by_id<'graph>(
        &'graph self,
        id: &ChoiceId,
    ) -> Option<ChoiceRef<'graph>> {
        Some(ChoiceRef {
            graph: self,
            internal_position: __ChoiceInternalPosition(self.choice.position(id)?),
        })
    }
    /// 辺の構造を保ったまま積み荷だけを可変借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice_payload_mut(&mut self, id: &ChoiceId) -> Option<&mut ChoiceEdge> {
        self.choice.get_mut(id).map(|record: &mut __ChoiceRecord| &mut record.choice)
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph ChoiceId> {
        self.choice.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = ChoiceRef<'graph>> + 'graph {
        self.choice
            .positions()
            .map(move |position| ChoiceRef {
                graph: self,
                internal_position: __ChoiceInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice_len(&self) -> usize {
        self.choice.len()
    }
    /// 公開IDから完成済みグラフ上の辺個体を平均 O(1) で引く。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn finale_by_id<'graph>(
        &'graph self,
        id: &FinaleId,
    ) -> Option<FinaleRef<'graph>> {
        Some(FinaleRef {
            graph: self,
            internal_position: __FinaleInternalPosition(self.finale.position(id)?),
        })
    }
    /// この種別の辺の公開IDを挿入順に走査する。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn finale_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph FinaleId> {
        self.finale.ids()
    }
    /// この種別の辺個体を挿入順に走査する。追加確保はしない。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn finale_iter<'graph>(
        &'graph self,
    ) -> impl Iterator<Item = FinaleRef<'graph>> + 'graph {
        self.finale
            .positions()
            .map(move |position| FinaleRef {
                graph: self,
                internal_position: __FinaleInternalPosition(position),
            })
    }
    /// この種別の辺の件数を返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn finale_len(&self) -> usize {
        self.finale.len()
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
/// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
#[derive(Clone, Copy)]
pub struct ChoiceRef<'graph> {
    graph: &'graph Graph,
    internal_position: __ChoiceInternalPosition,
}
impl<'graph> ChoiceRef<'graph> {
    fn record(self) -> &'graph __ChoiceRecord {
        self.graph
            .choice
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn id(self) -> &'graph ChoiceId {
        self.graph
            .choice
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn scene(self) -> SceneRef<'graph> {
        SceneRef {
            graph: self.graph,
            internal_position: __SceneInternalPosition(self.record().scene.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn next(self) -> SceneRef<'graph> {
        SceneRef {
            graph: self.graph,
            internal_position: __SceneInternalPosition(self.record().next.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn from(self) -> SceneRef<'graph> {
        self.scene()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn to(self) -> SceneRef<'graph> {
        self.next()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn from_id(self) -> &'graph SceneId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn to_id(self) -> &'graph SceneId {
        self.to().id()
    }
    /// この辺個体が運ぶ積み荷を役割名で借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice(self) -> &'graph ChoiceEdge {
        &self.record().choice
    }
    /// この辺個体が運ぶ積み荷を、役割名によらない固定名で借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn payload(self) -> &'graph ChoiceEdge {
        &self.record().choice
    }
}
impl<'graph> std::fmt::Debug for ChoiceRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(ChoiceRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 完成済みグラフ上の有向辺個体。
///
/// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
#[derive(Clone, Copy)]
pub struct FinaleRef<'graph> {
    graph: &'graph Graph,
    internal_position: __FinaleInternalPosition,
}
impl<'graph> FinaleRef<'graph> {
    fn record(self) -> &'graph __FinaleRecord {
        self.graph
            .finale
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この辺個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn id(self) -> &'graph FinaleId {
        self.graph
            .finale
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// この辺個体の始点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn scene(self) -> SceneRef<'graph> {
        SceneRef {
            graph: self.graph,
            internal_position: __SceneInternalPosition(self.record().scene.0),
        }
    }
    /// この辺個体の終点側の端点を役割名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn ending(self) -> EndingRef<'graph> {
        EndingRef {
            graph: self.graph,
            internal_position: __EndingInternalPosition(self.record().ending.0),
        }
    }
    /// この辺個体の始点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn from(self) -> SceneRef<'graph> {
        self.scene()
    }
    /// この辺個体の終点側の端点を、役割名によらない固定名で返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn to(self) -> EndingRef<'graph> {
        self.ending()
    }
    /// この辺個体の始点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn from_id(self) -> &'graph SceneId {
        self.from().id()
    }
    /// この辺個体の終点側の端点の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn to_id(self) -> &'graph EndingId {
        self.to().id()
    }
}
impl<'graph> std::fmt::Debug for FinaleRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(FinaleRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// 凍結前のグラフを組み立てる `Builder`。凍結 (`freeze()`) までは where
/// 制約検査を一切行わない。
///
/// 宣言: `src/schema.rs` の `schema DialogueGraph`
pub struct Builder {
    __graphite_node_scene: Vec<(SceneId, super::Scene)>,
    __graphite_node_ending: Vec<(EndingId, super::Ending)>,
    choice: Vec<(ChoiceId, Choice)>,
    finale: Vec<(FinaleId, Finale)>,
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
pub trait DialogueGraphInsertable: Sized {
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
pub trait DialogueGraphDefaultId: DialogueGraphInsertable {
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
pub trait DialogueGraphNode: DialogueGraphInsertable {}
impl DialogueGraphInsertable for super::Scene {
    type Id = SceneId;
    type NamedPosition = __SceneNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __SceneNamedPosition(
            __SceneInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_scene.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.scene(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.scene(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __SceneNamedPosition {
    type Reference<'graph> = SceneRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        SceneRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl DialogueGraphDefaultId for super::Scene {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        DialogueGraphInsertable::insert_named_with_id(self, b, SceneId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        DialogueGraphInsertable::insert_with_id(self, b, SceneId(binding))
    }
}
impl DialogueGraphNode for super::Scene {}
/// 完成済みグラフ上の `Scene` ノード個体。
///
/// 宣言: `src/schema.rs` の `node Scene`
#[derive(Clone, Copy)]
pub struct SceneRef<'graph> {
    graph: &'graph Graph,
    internal_position: __SceneInternalPosition,
}
impl<'graph> SceneRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `node Scene`
    pub fn id(self) -> &'graph SceneId {
        self.graph
            .__graphite_node_scene
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `src/schema.rs` の `node Scene`
    pub fn value(self) -> &'graph super::Scene {
        self.graph
            .__graphite_node_scene
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice_as_scene(self) -> impl Iterator<Item = ChoiceRef<'graph>> + 'graph {
        let positions = self.graph.choice_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ChoiceRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice_as_next(self) -> impl Iterator<Item = ChoiceRef<'graph>> + 'graph {
        let positions = self.graph.choice_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| ChoiceRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice_try_between(
        self,
        other: SceneRef<'graph>,
    ) -> Result<
        impl Iterator<Item = ChoiceRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_choice_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| ChoiceRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::choice_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice_between(
        self,
        other: SceneRef<'graph>,
    ) -> impl Iterator<Item = ChoiceRef<'graph>> + 'graph {
        self.choice_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(SceneRef), stringify!(choice_between)
                )
            })
    }
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn finale_as_scene(self) -> Option<FinaleRef<'graph>> {
        self.graph
            .finale_from_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| FinaleRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// 順序付き端点対を平均 O(1)、追加確保なしで検索する。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn finale_try_between(
        self,
        other: EndingRef<'graph>,
    ) -> Result<
        impl Iterator<Item = FinaleRef<'graph>> + 'graph,
        graphite::GraphMismatch,
    > {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let positions = self
            .graph
            .__graphite_finale_by_pair
            .get(&(self.internal_position, other.internal_position))
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        Ok(
            positions
                .iter()
                .copied()
                .map(move |internal_position| FinaleRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    /// パニックを避けたい場合は対の [`Self::finale_try_between`] を使う。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn finale_between(
        self,
        other: EndingRef<'graph>,
    ) -> impl Iterator<Item = FinaleRef<'graph>> + 'graph {
        self.finale_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(SceneRef), stringify!(finale_between)
                )
            })
    }
}
impl<'graph> std::ops::Deref for SceneRef<'graph> {
    type Target = super::Scene;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_scene
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for SceneRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(SceneRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
impl DialogueGraphInsertable for super::Ending {
    type Id = EndingId;
    type NamedPosition = __EndingNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __EndingNamedPosition(
            __EndingInternalPosition(
                graphite::TablePosition::from_index(b.__graphite_node_ending.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.ending(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.ending(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __EndingNamedPosition {
    type Reference<'graph> = EndingRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        EndingRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl DialogueGraphDefaultId for super::Ending {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        DialogueGraphInsertable::insert_named_with_id(self, b, EndingId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        DialogueGraphInsertable::insert_with_id(self, b, EndingId(binding))
    }
}
impl DialogueGraphNode for super::Ending {}
/// 完成済みグラフ上の `Ending` ノード個体。
///
/// 宣言: `src/schema.rs` の `node Ending`
#[derive(Clone, Copy)]
pub struct EndingRef<'graph> {
    graph: &'graph Graph,
    internal_position: __EndingInternalPosition,
}
impl<'graph> EndingRef<'graph> {
    /// このノード個体の公開IDを借用する。
    ///
    /// 宣言: `src/schema.rs` の `node Ending`
    pub fn id(self) -> &'graph EndingId {
        self.graph
            .__graphite_node_ending
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    /// このノード個体のノード値を借用する。
    ///
    /// 宣言: `src/schema.rs` の `node Ending`
    pub fn value(self) -> &'graph super::Ending {
        self.graph
            .__graphite_node_ending
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn finale_as_ending(self) -> impl Iterator<Item = FinaleRef<'graph>> + 'graph {
        let positions = self.graph.finale_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| FinaleRef {
                graph: self.graph,
                internal_position,
            })
    }
}
impl<'graph> std::ops::Deref for EndingRef<'graph> {
    type Target = super::Ending;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_ending
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for EndingRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(EndingRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
/// `graph!` の `add` 経由のエッジ挿入で使うトレイト境界。利用者が
/// この trait のメソッドを直接呼ぶことは想定しない
/// (`{Builder}::add` 経由で使う)。
pub trait DialogueGraphEdge: DialogueGraphInsertable {}
impl DialogueGraphInsertable for Choice {
    type Id = ChoiceId;
    type NamedPosition = __ChoiceNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __ChoiceNamedPosition(
            __ChoiceInternalPosition(
                graphite::TablePosition::from_index(b.choice.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.choice(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.choice(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __ChoiceNamedPosition {
    type Reference<'graph> = ChoiceRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        ChoiceRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl DialogueGraphDefaultId for Choice {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        DialogueGraphInsertable::insert_named_with_id(self, b, ChoiceId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        DialogueGraphInsertable::insert_with_id(self, b, ChoiceId(binding))
    }
}
impl DialogueGraphEdge for Choice {}
impl DialogueGraphInsertable for Finale {
    type Id = FinaleId;
    type NamedPosition = __FinaleNamedPosition;
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __FinaleNamedPosition(
            __FinaleInternalPosition(
                graphite::TablePosition::from_index(b.finale.len()),
            ),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.finale(id, self);
        (returned_id, named_position)
    }
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id {
        let returned_id = id.clone();
        b.finale(id, self);
        returned_id
    }
}
impl graphite::NamedGraphElement<Graph> for __FinaleNamedPosition {
    type Reference<'graph> = FinaleRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        FinaleRef {
            graph,
            internal_position: self.0,
        }
    }
}
impl DialogueGraphDefaultId for Finale {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        DialogueGraphInsertable::insert_named_with_id(self, b, FinaleId(binding), permit)
    }
    fn insert_with_binding(self, b: &mut Builder, binding: String) -> Self::Id {
        DialogueGraphInsertable::insert_with_id(self, b, FinaleId(binding))
    }
}
impl DialogueGraphEdge for Finale {}
impl Builder {
    fn new() -> Self {
        Self {
            __graphite_node_scene: Vec::new(),
            __graphite_node_ending: Vec::new(),
            choice: Vec::new(),
            finale: Vec::new(),
            __graphite_construction_stamp: graphite::次の構築印を発行する(),
        }
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `node Scene`
    pub fn scene(&mut self, id: SceneId, value: super::Scene) -> &mut Self {
        self.__graphite_node_scene.push((id, value));
        self
    }
    /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `node Ending`
    pub fn ending(&mut self, id: EndingId, value: super::Ending) -> &mut Self {
        self.__graphite_node_ending.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Choice = (scene: Scene) -[choice: ChoiceEdge]-> (next: Scene)`
    pub fn choice(&mut self, id: ChoiceId, value: Choice) -> &mut Self {
        self.choice.push((id, value));
        self
    }
    /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
    ///
    /// 宣言: `src/schema.rs` の `edge Finale = (scene: Scene) -> (ending: Ending) where each scene: 0..1`
    pub fn finale(&mut self, id: FinaleId, value: Finale) -> &mut Self {
        self.finale.push((id, value));
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
        N: DialogueGraphNode + DialogueGraphDefaultId,
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
        N: DialogueGraphNode + DialogueGraphDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定ノード挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたノード項は下記
    /// `insert_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn insert_with_id<N: DialogueGraphNode>(
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
    pub fn insert_named_with_id<N: DialogueGraphNode>(
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
        E: DialogueGraphEdge + DialogueGraphDefaultId,
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
        E: DialogueGraphEdge + DialogueGraphDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
    /// 明示ID型と既定ID型のどちらにも使える、ID指定エッジ挿入の
    /// 手書き用API。`graph!` の `@ ID式` を書いたエッジ項は下記
    /// `add_named_with_id` へ脱糖するため、このメソッド自体は
    /// `graph!` を経由しない。
    pub fn add_with_id<E: DialogueGraphEdge>(&mut self, id: E::Id, value: E) -> E::Id {
        value.insert_with_id(self, id)
    }
    /// `graph!` の `@ ID式` 付き辺を名前付き位置と共に挿入する経路。
    /// [`graphite::NamedInsertPermit`] を要求する
    /// (許可証は通常の `create` 経路からの直接的・偶発的な誤用を防ぐためのものであり、名前付き位置の持ち出しの検出は構築印の照合が担う。`crates/graphite/src/schema_runtime/named_construction.rs` 参照)。
    #[doc(hidden)]
    pub fn add_named_with_id<E: DialogueGraphEdge>(
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
        T: DialogueGraphDefaultId,
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
        let mut __graphite_node_scene: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_scene {
            if !__graphite_node_scene.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateScene(id));
            }
        }
        let mut __graphite_node_ending: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.__graphite_node_ending {
            if !__graphite_node_ending.insert(id.clone(), value) {
                __violations.push(Violation::DuplicateEnding(id));
            }
        }
        let mut __graphite_choice: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut choice_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut choice_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_choice_by_pair: std::collections::HashMap<
            (__SceneInternalPosition, __SceneInternalPosition),
            Vec<__ChoiceInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.choice {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::ChoiceDuplicateKey(id));
                continue;
            }
            let Choice { scene: from, next: to, choice } = value;
            let from_position = __graphite_node_scene
                .position(&from)
                .map(__SceneInternalPosition);
            let to_position = __graphite_node_scene
                .position(&to)
                .map(__SceneInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::ChoiceUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::ChoiceUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __ChoiceInternalPosition(
                    graphite::TablePosition::from_index(__graphite_choice.len()),
                );
                __graphite_choice_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                choice_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                choice_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_choice
                    .insert(
                        id,
                        __ChoiceRecord {
                            scene: from_position,
                            next: to_position,
                            choice,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let mut __graphite_finale: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        let mut __seen_edge_ids = std::collections::HashSet::new();
        let mut finale_from_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut finale_to_index: std::collections::HashMap<_, Vec<_>> = std::collections::HashMap::new();
        let mut __graphite_finale_by_pair: std::collections::HashMap<
            (__SceneInternalPosition, __EndingInternalPosition),
            Vec<__FinaleInternalPosition>,
        > = std::collections::HashMap::new();
        for (id, value) in self.finale {
            if !__seen_edge_ids.insert(id.clone()) {
                __violations.push(Violation::FinaleDuplicateKey(id));
                continue;
            }
            let Finale { scene: from, ending: to } = value;
            let from_position = __graphite_node_scene
                .position(&from)
                .map(__SceneInternalPosition);
            let to_position = __graphite_node_ending
                .position(&to)
                .map(__EndingInternalPosition);
            if from_position.is_none() {
                __violations
                    .push(Violation::FinaleUnknownSource {
                        edge: id.clone(),
                        source: from.clone(),
                    });
            }
            if to_position.is_none() {
                __violations
                    .push(Violation::FinaleUnknownTarget {
                        edge: id.clone(),
                        target: to.clone(),
                    });
            }
            if let (Some(from_position), Some(to_position)) = (
                from_position,
                to_position,
            ) {
                let internal_edge_position = __FinaleInternalPosition(
                    graphite::TablePosition::from_index(__graphite_finale.len()),
                );
                __graphite_finale_by_pair
                    .entry((from_position, to_position))
                    .or_default()
                    .push(internal_edge_position);
                finale_from_index
                    .entry(from_position)
                    .or_default()
                    .push(internal_edge_position);
                finale_to_index
                    .entry(to_position)
                    .or_default()
                    .push(internal_edge_position);
                let inserted = __graphite_finale
                    .insert(
                        id,
                        __FinaleRecord {
                            scene: from_position,
                            ending: to_position,
                        },
                    );
                debug_assert!(inserted, "重複辺IDは挿入前に除外済み");
            }
        }
        let _: fn(&Finale) = |edge| {
            let _ = &edge.scene;
        };
        for position in __graphite_node_scene.positions() {
            let internal_position = __SceneInternalPosition(position);
            let key = __graphite_node_scene
                .get_at(position)
                .expect("列挙した内部位置はノード表に存在する")
                .0;
            let count = finale_from_index
                .get(&internal_position)
                .map(Vec::len)
                .unwrap_or(0);
            if !(0usize..=1usize).contains(&count) {
                __violations
                    .push(Violation::FinaleSceneEachViolation {
                        source: key.clone(),
                        count,
                    });
            }
        }
        if !__violations.is_empty() {
            return Err(__violations);
        }
        let choice_from_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_scene
                .positions()
                .map(|position| {
                    choice_from_index
                        .remove(&__SceneInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let choice_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_scene
                .positions()
                .map(|position| {
                    choice_to_index
                        .remove(&__SceneInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let finale_from_index = graphite::OptionalRoleIndex::from_buckets(
            __graphite_node_scene
                .positions()
                .map(|position| {
                    finale_from_index
                        .remove(&__SceneInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let finale_to_index = graphite::MultipleRoleIndex::from_buckets(
            __graphite_node_ending
                .positions()
                .map(|position| {
                    finale_to_index
                        .remove(&__EndingInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        Ok(Graph {
            __graphite_node_scene,
            __graphite_node_ending,
            choice: __graphite_choice,
            finale: __graphite_finale,
            choice_from_index,
            choice_to_index,
            __graphite_choice_by_pair,
            finale_from_index,
            finale_to_index,
            __graphite_finale_by_pair,
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
