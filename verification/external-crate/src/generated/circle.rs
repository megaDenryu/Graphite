// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/lib.rs:117
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    18312041046197007315u64, 18285859197179539232u64, 3087956162089940141u64,
    17351323712649360769u64,
];
/// Graphite 静的グラフの個体実体の所有者 `Nodes` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `Nodes` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Nodes {
    /// Graphite 静的グラフの個体実体フィールド。`Nodes` がこの個体の実体を所有する。
    ///
    /// - graph: `Circle`
    /// - 個体: `本`
    /// - 実体型: `Book`
    ///
    /// 宣言: `src/lib.rs` の `node 本: Book = ..`
    pub 本: Book,
    /// Graphite 静的グラフの個体実体フィールド。`Nodes` がこの個体の実体を所有する。
    ///
    /// - graph: `Circle`
    /// - 個体: `読者`
    /// - 実体型: `Reader`
    ///
    /// 宣言: `src/lib.rs` の `node 読者: Reader = ..`
    pub 読者: Reader,
}
impl Nodes {
    /// Graphite 静的グラフの個体実体の所有者 `Nodes` を構築する (Graphite の固定語彙)。全個体を宣言順の位置引数にそのまま取り、値の計算は行わない。値ありの個体をinstance宣言の式から計算して渡すのは `Circleの個体を組み立てる` の役目。
    ///
    /// - graph: `Circle`
    /// - 引数 (宣言順): `本: Book, 読者: Reader`
    ///
    /// 固定語彙: `Nodes::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    ///
    /// 関係する instance 宣言: `src/lib.rs` の `graph Circle`
    pub fn new(本: Book, 読者: Reader) -> Self {
        Self { 本, 読者 }
    }
}
/// Graphite 静的グラフの辺実体の所有者 `Edges` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `Edges` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Edges<'a> {
    /// Graphite 静的グラフの辺実体フィールド。`Edges` がこの具体辺の実体を所有する。
    ///
    /// - graph: `Circle`
    /// - 具体辺: `割り当て`
    /// - 辺種別: `Assigned`
    ///
    /// 宣言: `src/lib.rs` の `edge 割り当て = Assigned(本 -> 読者)`
    pub 割り当て: ReadingCircle::AssignedEdge<'a>,
}
impl<'a> Edges<'a> {
    /// Graphite 静的グラフの辺実体の所有者 `Edges` を構築する (Graphite の固定語彙)。値の計算は行わない。積み荷ありの具体辺の値をinstance宣言の式から計算して渡すのは`Circleの辺を組み立てる` の役目。
    ///
    /// - graph: `Circle`
    /// - 第1引数: `nodes: &Nodes`
    ///
    /// 固定語彙: `Edges::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    ///
    /// 関係する instance 宣言: `src/lib.rs` の `graph Circle`
    pub fn new(nodes: &'a Nodes) -> Self {
        Self {
            割り当て: ReadingCircle::AssignedEdge {
                book: &nodes.本,
                reader: &nodes.読者,
            },
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `Circle`
/// - 個体: `本`
/// - 実体型: `Book`
///
/// 宣言: `src/lib.rs` の `node 本: Book = ..`
#[derive(Clone, Copy)]
pub struct 本Ref<'a> {
    pub(super) entity: &'a Book,
    pub(super) nodes: &'a Nodes,
    pub(super) edges: &'a Edges<'a>,
}
impl<'a> 本Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a Book {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `Circle`
    /// - 個体: `本`
    /// - 具体辺: `割り当て`
    /// - 辺種別: `Assigned`
    /// - この個体の役割: `book`
    /// - 戻り値: `割り当てRef`
    ///
    /// 宣言: `src/lib.rs` の `edge 割り当て = Assigned(本 -> 読者)`
    ///
    /// 関係する schema 宣言: `src/lib.rs` の `edge Assigned = (book: Book) -> (reader: Reader) where each book: 1`
    pub fn 割り当て(&self) -> 割り当てRef<'a> {
        割り当てRef {
            entity: &self.edges.割り当て,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `Circle`
/// - 個体: `読者`
/// - 実体型: `Reader`
///
/// 宣言: `src/lib.rs` の `node 読者: Reader = ..`
#[derive(Clone, Copy)]
pub struct 読者Ref<'a> {
    pub(super) entity: &'a Reader,
    pub(super) nodes: &'a Nodes,
    pub(super) edges: &'a Edges<'a>,
}
impl<'a> 読者Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a Reader {
        self.entity
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `Circle`
    /// - 個体: `読者`
    /// - 具体辺: `割り当て`
    /// - 辺種別: `Assigned`
    /// - この個体の役割: `reader`
    /// - 戻り値: `割り当てRef`
    ///
    /// 宣言: `src/lib.rs` の `edge 割り当て = Assigned(本 -> 読者)`
    ///
    /// 関係する schema 宣言: `src/lib.rs` の `edge Assigned = (book: Book) -> (reader: Reader) where each book: 1`
    pub fn 割り当て(&self) -> 割り当てRef<'a> {
        割り当てRef {
            entity: &self.edges.割り当て,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `Circle`
/// - 具体辺: `割り当て`
/// - 辺種別: `Assigned`
///
/// 宣言: `src/lib.rs` の `edge 割り当て = Assigned(本 -> 読者)`
///
/// 関係する schema 宣言: `src/lib.rs` の `edge Assigned = (book: Book) -> (reader: Reader) where each book: 1`
#[derive(Clone, Copy)]
pub struct 割り当てRef<'a> {
    pub(super) entity: &'a ReadingCircle::AssignedEdge<'a>,
    pub(super) nodes: &'a Nodes,
    pub(super) edges: &'a Edges<'a>,
}
impl<'a> 割り当てRef<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `Assigned`
    /// - 役割: `book: Book`
    /// - 具体辺: `割り当て`
    /// - 具体端点: `本`
    /// - 戻り値: `本Ref`
    /// - 検証制約: `each book: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/lib.rs` の `edge Assigned = (book: Book) -> (reader: Reader) where each book: 1`
    ///
    /// 関係する instance 宣言: `src/lib.rs` の `edge 割り当て = Assigned(本 -> 読者)`
    pub fn book(&self) -> 本Ref<'a> {
        本Ref {
            entity: self.entity.book,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `Assigned`
    /// - 役割: `reader: Reader`
    /// - 具体辺: `割り当て`
    /// - 具体端点: `読者`
    /// - 戻り値: `読者Ref`
    /// - 検証制約: `each book: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `src/lib.rs` の `edge Assigned = (book: Book) -> (reader: Reader) where each book: 1`
    ///
    /// 関係する instance 宣言: `src/lib.rs` の `edge 割り当て = Assigned(本 -> 読者)`
    pub fn reader(&self) -> 読者Ref<'a> {
        読者Ref {
            entity: self.entity.reader,
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}
/// Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `NodeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct NodeRefs<'a> {
    /// Graphite 静的グラフの個体参照フィールド。`NodeRefs` がこの個体の具体参照を持つ。
    ///
    /// - graph: `Circle`
    /// - 個体: `本`
    ///
    /// 宣言: `src/lib.rs` の `node 本: Book = ..`
    pub 本: 本Ref<'a>,
    /// Graphite 静的グラフの個体参照フィールド。`NodeRefs` がこの個体の具体参照を持つ。
    ///
    /// - graph: `Circle`
    /// - 個体: `読者`
    ///
    /// 宣言: `src/lib.rs` の `node 読者: Reader = ..`
    pub 読者: 読者Ref<'a>,
}
impl<'a> NodeRefs<'a> {
    /// Graphite 静的グラフの `NodeRefs` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `NodeRefs::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            本: 本Ref {
                entity: &nodes.本,
                nodes,
                edges,
            },
            読者: 読者Ref {
                entity: &nodes.読者,
                nodes,
                edges,
            },
        }
    }
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    /// Graphite 静的グラフの辺参照フィールド。`EdgeRefs` がこの具体辺の具体参照を持つ。
    ///
    /// - graph: `Circle`
    /// - 具体辺: `割り当て`
    ///
    /// 宣言: `src/lib.rs` の `edge 割り当て = Assigned(本 -> 読者)`
    pub 割り当て: 割り当てRef<'a>,
}
impl<'a> EdgeRefs<'a> {
    /// Graphite 静的グラフの `EdgeRefs` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `EdgeRefs::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            割り当て: 割り当てRef {
                entity: &edges.割り当て,
                nodes,
                edges,
            },
        }
    }
}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph<'a> {
    /// Graphite 静的グラフの `Graph` が持つ個体参照の集まりへのフィールド `node_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub node_refs: NodeRefs<'a>,
    /// Graphite 静的グラフの `Graph` が持つ辺参照の集まりへのフィールド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub edge_refs: EdgeRefs<'a>,
}
impl<'a> Graph<'a> {
    /// Graphite 静的グラフの `Graph` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `Graph::new` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            node_refs: NodeRefs::new(nodes, edges),
            edge_refs: EdgeRefs::new(nodes, edges),
        }
    }
}
