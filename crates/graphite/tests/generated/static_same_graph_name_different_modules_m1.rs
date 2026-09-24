// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_same_graph_name_different_modules.rs:40
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    9988878751256322136u64, 9541720496466318655u64, 16936923424237650262u64,
    12830147923045911858u64,
];
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `衝突検査グラフ`
/// - 個体: `甲`
/// - 実体型: `社員`
///
/// 宣言: `tests/static_same_graph_name_different_modules.rs` の `node 甲: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 甲Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 甲Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `衝突検査グラフ`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        &self.graph.甲
    }
}
/// Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。
///
/// - graph: `衝突検査グラフ`
///
/// 固定語彙: `NodeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct NodeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> NodeRefs<'a> {
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `衝突検査グラフ`
    /// - 個体: `甲`
    ///
    /// 宣言: `tests/static_same_graph_name_different_modules.rs` の `node 甲: 社員 = ..`
    pub fn 甲(&self) -> 甲Ref<'a> {
        甲Ref { graph: self.graph }
    }
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `衝突検査グラフ`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> EdgeRefs<'a> {}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `衝突検査グラフ`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph {
    甲: 社員,
}
impl Graph {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(甲: 社員) -> Self {
        Self { 甲 }
    }
    /// Graphite 静的グラフの `Graph` が個体参照の集まりを返すメソッド `node_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `衝突検査グラフ`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn node_refs(&self) -> NodeRefs<'_> {
        NodeRefs { graph: self }
    }
    /// Graphite 静的グラフの `Graph` が辺参照の集まりを返すメソッド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `衝突検査グラフ`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn edge_refs(&self) -> EdgeRefs<'_> {
        EdgeRefs { graph: self }
    }
}
/// Graphite 静的グラフの `Graph` を実体化するマクロ `construct` (Graphite の固定語彙)。値ありの個体・積み荷はinstance宣言の式からこのマクロが計算し、値なしの個体だけを宣言順の引数で受け取る。
///
/// - graph: `衝突検査グラフ`
/// - 戻り値: `Graph`
///
/// 固定語彙: `construct!` (`docs/static_graph.md` 「生成される名前の公開契約」)
///
/// 関係する instance 宣言: `tests/static_same_graph_name_different_modules.rs` の `graph 衝突検査グラフ`
macro_rules! construct {
    () => {
        { let (甲,) = __graphite_values_衝突検査グラフ_b77ce2f422ea0c67!(); let
        () = __graphite_payloads_衝突検査グラフ_b77ce2f422ea0c67!();
        #[allow(deprecated)] { 衝突検査グラフ::Graph::__graphite_internal_new(甲)
        } }
    };
}
pub(crate) use construct;
