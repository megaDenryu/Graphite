// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/lib.rs:117
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    14525001603493775856u64, 8400508092904293481u64, 1885641960989767998u64,
    15291797682214371482u64,
];
/// Graphite 静的グラフの個体実体の所有者 `Nodes` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `Nodes` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Nodes {
    本: Book,
    読者: Reader,
}
impl Nodes {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct::nodes!/construct::edges! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(本: Book, 読者: Reader) -> Self {
        Self { 本, 読者 }
    }
}
/// Graphite 静的グラフの辺実体の所有者 `Edges` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `Edges` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Edges<'a> {
    __graphite_nodes: &'a Nodes,
    割り当て: ReadingCircle::AssignedEdge<'a>,
}
impl<'a> Edges<'a> {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct::nodes!/construct::edges! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(nodes: &'a Nodes) -> Self {
        Self {
            __graphite_nodes: nodes,
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
    entity: &'a Book,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
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
    entity: &'a Reader,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
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
    entity: &'a ReadingCircle::AssignedEdge<'a>,
    nodes: &'a Nodes,
    edges: &'a Edges<'a>,
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
    本: 本Ref<'a>,
    読者: 読者Ref<'a>,
}
impl<'a> NodeRefs<'a> {
    fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
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
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `Circle`
    /// - 個体: `本`
    ///
    /// 宣言: `src/lib.rs` の `node 本: Book = ..`
    pub fn 本(&self) -> 本Ref<'a> {
        self.本
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `Circle`
    /// - 個体: `読者`
    ///
    /// 宣言: `src/lib.rs` の `node 読者: Reader = ..`
    pub fn 読者(&self) -> 読者Ref<'a> {
        self.読者
    }
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    割り当て: 割り当てRef<'a>,
}
impl<'a> EdgeRefs<'a> {
    fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
        Self {
            割り当て: 割り当てRef {
                entity: &edges.割り当て,
                nodes,
                edges,
            },
        }
    }
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `Circle`
    /// - 具体辺: `割り当て`
    ///
    /// 宣言: `src/lib.rs` の `edge 割り当て = Assigned(本 -> 読者)`
    pub fn 割り当て(&self) -> 割り当てRef<'a> {
        self.割り当て
    }
}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph<'a> {
    node_refs: NodeRefs<'a>,
    edge_refs: EdgeRefs<'a>,
}
impl<'a> Graph<'a> {
    /// Graphite 静的グラフの `Graph` を構築する (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
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
    /// - graph: `Circle`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn node_refs(&self) -> &NodeRefs<'a> {
        &self.node_refs
    }
    /// Graphite 静的グラフの `Graph` が辺参照の集まりを返すメソッド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn edge_refs(&self) -> &EdgeRefs<'a> {
        &self.edge_refs
    }
}
/// Graphite 静的グラフの構築の入口をまとめるmodule `construct` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `construct` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub mod construct {
    /// Graphite 静的グラフの個体実体の所有者 `Nodes` を構築するマクロ `nodes` (Graphite の固定語彙)。値ありの個体はinstance宣言の式からこのマクロが計算し、値なしの個体だけを引数で受け取る。
    ///
    /// - graph: `Circle`
    /// - 戻り値: `Nodes`
    ///
    /// 固定語彙: `construct::nodes!` (`docs/static_graph.md` 「生成される名前の公開契約」)
    ///
    /// 関係する instance 宣言: `src/lib.rs` の `graph Circle`
    macro_rules! nodes {
        () => {
            { let (本, 読者,) = __graphite_values_Circle!(); #[allow(deprecated)] let
            __graphite_nodes = Circle::Nodes::__graphite_internal_new(本, 読者);
            __graphite_nodes }
        };
    }
    pub(crate) use nodes;
    /// Graphite 静的グラフの辺実体の所有者 `Edges` を構築するマクロ `edges` (Graphite の固定語彙)。積み荷ありの具体辺はすべてinstance宣言の式からこのマクロが計算する。
    ///
    /// - graph: `Circle`
    /// - 引数: `nodes: &Nodes`
    /// - 戻り値: `Edges`
    ///
    /// 固定語彙: `construct::edges!` (`docs/static_graph.md` 「生成される名前の公開契約」)
    ///
    /// 関係する instance 宣言: `src/lib.rs` の `graph Circle`
    macro_rules! edges {
        ($nodes:expr) => {
            { let () = __graphite_payloads_Circle!(); #[allow(deprecated)] let
            __graphite_edges = Circle::Edges::__graphite_internal_new($nodes,);
            __graphite_edges }
        };
    }
    pub(crate) use edges;
}
