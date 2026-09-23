// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/main.rs:155
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    16142987864403014076u64, 12476505270094444279u64, 12439120313279859190u64,
    89990468296524282u64,
];
/// Graphite 静的グラフの個体実体の所有者 `Nodes` (Graphite の固定語彙)。
///
/// - graph: `経理チーム`
///
/// 固定語彙: `Nodes` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Nodes {
    /// Graphite 静的グラフの個体実体フィールド。`Nodes` がこの個体の実体を所有する。
    ///
    /// - graph: `経理チーム`
    /// - 個体: `花子`
    /// - 実体型: `社員`
    ///
    /// 宣言: `src/main.rs` の `node 花子: 社員 = ..`
    pub 花子: 社員,
    /// Graphite 静的グラフの個体実体フィールド。`Nodes` がこの個体の実体を所有する。
    ///
    /// - graph: `経理チーム`
    /// - 個体: `総務部`
    /// - 実体型: `部署`
    ///
    /// 宣言: `src/main.rs` の `node 総務部: 部署 = ..`
    pub 総務部: 部署,
}
impl Nodes {
    /// Graphite 静的グラフの個体実体の所有者 `Nodes` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `経理チーム`
    /// - instance 宣言の右辺式から作る個体: `花子`・`総務部`
    ///
    /// 固定語彙: `Nodes::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    ///
    /// 関係する instance 宣言: `src/main.rs` の `graph 経理チーム`
    pub fn new() -> Self {
        Self {
            花子: Self::__graphite_initial_value_花子(),
            総務部: Self::__graphite_initial_value_総務部(),
        }
    }
}
/// Graphite 静的グラフの辺実体の所有者 `Edges` (Graphite の固定語彙)。
///
/// - graph: `経理チーム`
///
/// 固定語彙: `Edges` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Edges<'a> {
    /// Graphite 静的グラフの辺実体フィールド。`Edges` がこの具体辺の実体を所有する。
    ///
    /// - graph: `経理チーム`
    /// - 具体辺: `花子の所属`
    /// - 辺種別: `所属`
    ///
    /// 宣言: `src/main.rs` の `edge 花子の所属 = 所属(花子 -> 総務部)`
    pub 花子の所属: 組織::所属Edge<'a>,
}
impl<'a> Edges<'a> {
    /// Graphite 静的グラフの `Edges` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `経理チーム`
    ///
    /// 固定語彙: `Edges::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn new(nodes: &'a Nodes) -> Self {
        Self {
            花子の所属: 組織::所属Edge {
                member: &nodes.花子,
                team: &nodes.総務部,
            },
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `経理チーム`
/// - 個体: `花子`
/// - 実体型: `社員`
///
/// 宣言: `src/main.rs` の `node 花子: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 花子Ref<'a> {
    pub(super) entity: &'a 社員,
    pub(super) nodes: &'a Nodes,
    pub(super) edges: &'a Edges<'a>,
}
impl<'a> 花子Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `経理チーム`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `経理チーム`
    /// - 個体: `花子`
    /// - 具体辺: `花子の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `member`
    /// - 戻り値: `花子の所属Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 花子の所属 = 所属(花子 -> 総務部)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 花子の所属(&self) -> 花子の所属Ref<'a> {
        花子の所属Ref {
            entity: &self.edges.花子の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `経理チーム`
/// - 個体: `総務部`
/// - 実体型: `部署`
///
/// 宣言: `src/main.rs` の `node 総務部: 部署 = ..`
#[derive(Clone, Copy)]
pub struct 総務部Ref<'a> {
    pub(super) entity: &'a 部署,
    pub(super) nodes: &'a Nodes,
    pub(super) edges: &'a Edges<'a>,
}
impl<'a> 総務部Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `経理チーム`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 部署 {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `経理チーム`
    /// - 個体: `総務部`
    /// - 具体辺: `花子の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `team`
    /// - 戻り値: `花子の所属Ref`
    ///
    /// 宣言: `src/main.rs` の `edge 花子の所属 = 所属(花子 -> 総務部)`
    ///
    /// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 花子の所属(&self) -> 花子の所属Ref<'a> {
        花子の所属Ref {
            entity: &self.edges.花子の所属,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `経理チーム`
/// - 具体辺: `花子の所属`
/// - 辺種別: `所属`
///
/// 宣言: `src/main.rs` の `edge 花子の所属 = 所属(花子 -> 総務部)`
///
/// 関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
#[derive(Clone, Copy)]
pub struct 花子の所属Ref<'a> {
    pub(super) entity: &'a 組織::所属Edge<'a>,
    pub(super) nodes: &'a Nodes,
    pub(super) edges: &'a Edges<'a>,
}
impl<'a> 花子の所属Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `member: 社員`
    /// - 具体辺: `花子の所属`
    /// - 具体端点: `花子`
    /// - 戻り値: `花子Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 花子の所属 = 所属(花子 -> 総務部)`
    pub fn member(&self) -> 花子Ref<'a> {
        花子Ref {
            entity: self.entity.member,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `team: 部署`
    /// - 具体辺: `花子の所属`
    /// - 具体端点: `総務部`
    /// - 戻り値: `総務部Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `src/main.rs` の `edge 花子の所属 = 所属(花子 -> 総務部)`
    pub fn team(&self) -> 総務部Ref<'a> {
        総務部Ref {
            entity: self.entity.team,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。
///
/// - graph: `経理チーム`
///
/// 固定語彙: `NodeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct NodeRefs<'a> {
    /// Graphite 静的グラフの個体参照フィールド。`NodeRefs` がこの個体の具体参照を持つ。
    ///
    /// - graph: `経理チーム`
    /// - 個体: `花子`
    ///
    /// 宣言: `src/main.rs` の `node 花子: 社員 = ..`
    pub 花子: 花子Ref<'a>,
    /// Graphite 静的グラフの個体参照フィールド。`NodeRefs` がこの個体の具体参照を持つ。
    ///
    /// - graph: `経理チーム`
    /// - 個体: `総務部`
    ///
    /// 宣言: `src/main.rs` の `node 総務部: 部署 = ..`
    pub 総務部: 総務部Ref<'a>,
}
impl<'a> NodeRefs<'a> {
    /// Graphite 静的グラフの `NodeRefs` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `経理チーム`
    ///
    /// 固定語彙: `NodeRefs::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            花子: 花子Ref {
                entity: &nodes.花子,
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
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `経理チーム`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    /// Graphite 静的グラフの辺参照フィールド。`EdgeRefs` がこの具体辺の具体参照を持つ。
    ///
    /// - graph: `経理チーム`
    /// - 具体辺: `花子の所属`
    ///
    /// 宣言: `src/main.rs` の `edge 花子の所属 = 所属(花子 -> 総務部)`
    pub 花子の所属: 花子の所属Ref<'a>,
}
impl<'a> EdgeRefs<'a> {
    /// Graphite 静的グラフの `EdgeRefs` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `経理チーム`
    ///
    /// 固定語彙: `EdgeRefs::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            花子の所属: 花子の所属Ref {
                entity: &edges.花子の所属,
                nodes,
                edges,
            },
        }
    }
}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `経理チーム`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph<'a> {
    /// Graphite 静的グラフの `Graph` が持つ個体参照の集まりへのフィールド `node_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `経理チーム`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub node_refs: NodeRefs<'a>,
    /// Graphite 静的グラフの `Graph` が持つ辺参照の集まりへのフィールド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `経理チーム`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub edge_refs: EdgeRefs<'a>,
}
impl<'a> Graph<'a> {
    /// Graphite 静的グラフの `Graph` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `経理チーム`
    ///
    /// 固定語彙: `Graph::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            node_refs: NodeRefs::new(nodes, edges),
            edge_refs: EdgeRefs::new(nodes, edges),
        }
    }
}
