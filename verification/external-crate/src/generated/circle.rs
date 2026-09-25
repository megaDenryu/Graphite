// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/lib.rs:119
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    8280421238214578031u64, 15170349764766456336u64, 14068980191178634777u64,
    12746725877975084965u64,
];
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `Circle`
/// - 個体: `本`
/// - 実体型: `Book`
///
/// 宣言: `src/lib.rs` の `node 本: Book = ..`
#[derive(Clone, Copy)]
pub struct 本Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 本Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a Book {
        &self.graph.本
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
            graph: self.graph,
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
    graph: &'a Graph,
}
impl<'a> 読者Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a Reader {
        &self.graph.読者
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
            graph: self.graph,
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
    graph: &'a Graph,
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
        本Ref { graph: self.graph }
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
        読者Ref { graph: self.graph }
    }
}
/// Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `NodeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct NodeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> NodeRefs<'a> {
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `Circle`
    /// - 個体: `本`
    ///
    /// 宣言: `src/lib.rs` の `node 本: Book = ..`
    pub fn 本(&self) -> 本Ref<'a> {
        本Ref { graph: self.graph }
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `Circle`
    /// - 個体: `読者`
    ///
    /// 宣言: `src/lib.rs` の `node 読者: Reader = ..`
    pub fn 読者(&self) -> 読者Ref<'a> {
        読者Ref { graph: self.graph }
    }
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> EdgeRefs<'a> {
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `Circle`
    /// - 具体辺: `割り当て`
    ///
    /// 宣言: `src/lib.rs` の `edge 割り当て = Assigned(本 -> 読者)`
    pub fn 割り当て(&self) -> 割り当てRef<'a> {
        割り当てRef {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `Circle`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph {
    本: Book,
    読者: Reader,
}
impl Graph {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(本: Book, 読者: Reader) -> Self {
        Self { 本, 読者 }
    }
    /// Graphite 静的グラフの `Graph` が個体参照の集まりを返すメソッド `node_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn node_refs(&self) -> NodeRefs<'_> {
        NodeRefs { graph: self }
    }
    /// Graphite 静的グラフの `Graph` が辺参照の集まりを返すメソッド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `Circle`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn edge_refs(&self) -> EdgeRefs<'_> {
        EdgeRefs { graph: self }
    }
}
/// Graphite 静的グラフの `Graph` を実体化するマクロ `construct` (Graphite の固定語彙)。値ありの個体・積み荷はinstance宣言の式からこのマクロが計算し、値なしの個体だけを宣言順の引数で受け取る。
///
/// - graph: `Circle`
/// - 戻り値: `Graph`
///
/// 固定語彙: `construct!` (`docs/static_graph.md` 「生成される名前の公開契約」)
///
/// 関係する instance 宣言: `src/lib.rs` の `graph Circle`
macro_rules! construct {
    () => {
        { let (本, 読者,) = __graphite_values_Circle_93ad335dee0a2ff0!(); let () =
        __graphite_payloads_Circle_93ad335dee0a2ff0!(); #[allow(deprecated)] {
        Circle::Graph::__graphite_internal_new(本, 読者) } }
    };
}
pub(crate) use construct;
