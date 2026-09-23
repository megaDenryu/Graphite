// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_same_individual_name_multiple_instances.rs:60
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    15273178103125775874u64, 6757744296328420325u64, 5228638097196167788u64,
    7199627321149765640u64,
];
/// Graphite 静的グラフの個体実体の所有者 `Nodes` (Graphite の固定語彙)。
///
/// - graph: `検証チームb`
///
/// 固定語彙: `Nodes` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Nodes {
    太郎: 社員,
    総務部: 部署,
}
impl Nodes {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct::nodes!/construct::edges! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(太郎: 社員, 総務部: 部署) -> Self {
        Self { 太郎, 総務部 }
    }
}
/// Graphite 静的グラフの辺実体の所有者 `Edges` (Graphite の固定語彙)。
///
/// - graph: `検証チームb`
///
/// 固定語彙: `Edges` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Edges<'a> {
    __graphite_nodes: &'a Nodes,
    太郎の所属: 検証組織::所属Edge<'a>,
}
impl<'a> Edges<'a> {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct::nodes!/construct::edges! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(
        nodes: &'a Nodes,
        太郎の所属: 任命記録,
    ) -> Self {
        Self {
            __graphite_nodes: nodes,
            太郎の所属: 検証組織::所属Edge {
                member: &nodes.太郎,
                team: &nodes.総務部,
                任命: 太郎の所属,
            },
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `検証チームb`
/// - 個体: `太郎`
/// - 実体型: `社員`
///
/// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `node 太郎: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 太郎Ref<'a> {
    entity: &'a 社員,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 太郎Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チームb`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `検証チームb`
    /// - 個体: `太郎`
    /// - 具体辺: `太郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `member`
    /// - 戻り値: `太郎の所属Ref`
    ///
    /// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 総務部)`
    ///
    /// 関係する schema 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            entity: &self.edges.太郎の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `検証チームb`
/// - 個体: `総務部`
/// - 実体型: `部署`
///
/// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `node 総務部: 部署 = ..`
#[derive(Clone, Copy)]
pub struct 総務部Ref<'a> {
    entity: &'a 部署,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 総務部Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チームb`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 部署 {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `検証チームb`
    /// - 個体: `総務部`
    /// - 具体辺: `太郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `team`
    /// - 戻り値: `太郎の所属Ref`
    ///
    /// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 総務部)`
    ///
    /// 関係する schema 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            entity: &self.edges.太郎の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `検証チームb`
/// - 具体辺: `太郎の所属`
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 総務部)`
///
/// 関係する schema 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
#[derive(Clone, Copy)]
pub struct 太郎の所属Ref<'a> {
    entity: &'a 検証組織::所属Edge<'a>,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
}
impl<'a> 太郎の所属Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `member: 社員`
    /// - 具体辺: `太郎の所属`
    /// - 具体端点: `太郎`
    /// - 戻り値: `太郎Ref`
    ///
    /// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    ///
    /// 関係する instance 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 総務部)`
    pub fn member(&self) -> 太郎Ref<'a> {
        太郎Ref {
            entity: self.entity.member,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `team: 部署`
    /// - 具体辺: `太郎の所属`
    /// - 具体端点: `総務部`
    /// - 戻り値: `総務部Ref`
    ///
    /// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    ///
    /// 関係する instance 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 総務部)`
    pub fn team(&self) -> 総務部Ref<'a> {
        総務部Ref {
            entity: self.entity.team,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの積み荷アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 積み荷: `任命: 任命記録`
    /// - 具体辺: `太郎の所属`
    ///
    /// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    ///
    /// 関係する instance 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 総務部)`
    pub fn 任命(&self) -> &'a 任命記録 {
        &self.entity.任命
    }
}
/// Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。
///
/// - graph: `検証チームb`
///
/// 固定語彙: `NodeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct NodeRefs<'a> {
    太郎: 太郎Ref<'a>,
    総務部: 総務部Ref<'a>,
}
impl<'a> NodeRefs<'a> {
    fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            太郎: 太郎Ref {
                entity: &nodes.太郎,
                nodes,
                edges,
            },
            総務部: 総務部Ref {
                entity: &nodes.総務部,
                nodes,
                edges,
            },
        }
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `検証チームb`
    /// - 個体: `太郎`
    ///
    /// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `node 太郎: 社員 = ..`
    pub fn 太郎(&self) -> 太郎Ref<'a> {
        self.太郎
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `検証チームb`
    /// - 個体: `総務部`
    ///
    /// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `node 総務部: 部署 = ..`
    pub fn 総務部(&self) -> 総務部Ref<'a> {
        self.総務部
    }
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `検証チームb`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    太郎の所属: 太郎の所属Ref<'a>,
}
impl<'a> EdgeRefs<'a> {
    fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            太郎の所属: 太郎の所属Ref {
                entity: &edges.太郎の所属,
                nodes,
                edges,
            },
        }
    }
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `検証チームb`
    /// - 具体辺: `太郎の所属`
    ///
    /// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 総務部)`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        self.太郎の所属
    }
}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `検証チームb`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph<'a> {
    node_refs: NodeRefs<'a>,
    edge_refs: EdgeRefs<'a>,
}
impl<'a> Graph<'a> {
    /// Graphite 静的グラフの `Graph` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `検証チームb`
    ///
    /// 固定語彙: `Graph::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn new(edges: &'a Edges<'a>) -> Self {
        let nodes = edges.__graphite_nodes;
        Self {
            node_refs: NodeRefs::new(nodes, edges),
            edge_refs: EdgeRefs::new(nodes, edges),
        }
    }
    /// Graphite 静的グラフの `Graph` が個体参照の集まりを返すメソッド `node_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チームb`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn node_refs(&self) -> &NodeRefs<'a> {
        &self.node_refs
    }
    /// Graphite 静的グラフの `Graph` が辺参照の集まりを返すメソッド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チームb`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn edge_refs(&self) -> &EdgeRefs<'a> {
        &self.edge_refs
    }
}
/// Graphite 静的グラフの構築の入口をまとめるmodule `construct` (Graphite の固定語彙)。
///
/// - graph: `検証チームb`
///
/// 固定語彙: `construct` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub mod construct {
    /// Graphite 静的グラフの個体実体の所有者 `Nodes` を構築するマクロ `nodes` (Graphite の固定語彙)。値ありの個体はinstance宣言の式からこのマクロが計算し、値なしの個体だけを引数で受け取る。
    ///
    /// - graph: `検証チームb`
    /// - 戻り値: `Nodes`
    ///
    /// 固定語彙: `construct::nodes!` (`docs/static_graph.md` 「生成される名前の公開契約」)
    ///
    /// 関係する instance 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `graph 検証チームb`
    macro_rules! nodes {
        () => {
            { let (太郎, 総務部,) = __graphite_values_検証チームb!();
            #[allow(deprecated)] let __graphite_nodes =
            検証チームb::Nodes::__graphite_internal_new(太郎, 総務部);
            __graphite_nodes }
        };
    }
    pub(crate) use nodes;
    /// Graphite 静的グラフの辺実体の所有者 `Edges` を構築するマクロ `edges` (Graphite の固定語彙)。積み荷ありの具体辺はすべてinstance宣言の式からこのマクロが計算する。
    ///
    /// - graph: `検証チームb`
    /// - 引数: `nodes: &Nodes`
    /// - 戻り値: `Edges`
    ///
    /// 固定語彙: `construct::edges!` (`docs/static_graph.md` 「生成される名前の公開契約」)
    ///
    /// 関係する instance 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `graph 検証チームb`
    macro_rules! edges {
        ($nodes:expr) => {
            { let (太郎の所属,) = __graphite_payloads_検証チームb!();
            #[allow(deprecated)] let __graphite_edges =
            検証チームb::Edges::__graphite_internal_new($nodes, 太郎の所属);
            __graphite_edges }
        };
    }
    pub(crate) use edges;
}
