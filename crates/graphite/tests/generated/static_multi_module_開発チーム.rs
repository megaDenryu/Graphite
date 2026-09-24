// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_multi_module.rs:59
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    5942440580631974011u64, 9317254389252461578u64, 2398209404785470253u64,
    3867099391987218097u64,
];
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `開発チーム`
/// - 個体: `太郎`
/// - 実体型: `社員`
///
/// 宣言: `tests/static_multi_module.rs` の `node 太郎: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 太郎Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 太郎Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        &self.graph.太郎
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `太郎`
    /// - 具体辺: `太郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `member`
    /// - 戻り値: `太郎の所属Ref`
    ///
    /// 宣言: `tests/static_multi_module.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    ///
    /// 関係する schema 宣言: `tests/static_multi_module.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `開発チーム`
/// - 個体: `開発部`
/// - 実体型: `部署`
///
/// 宣言: `tests/static_multi_module.rs` の `node 開発部: 部署 = ..`
#[derive(Clone, Copy)]
pub struct 開発部Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 開発部Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 部署 {
        &self.graph.開発部
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `開発部`
    /// - 具体辺: `太郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `team`
    /// - 戻り値: `太郎の所属Ref`
    ///
    /// 宣言: `tests/static_multi_module.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    ///
    /// 関係する schema 宣言: `tests/static_multi_module.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `開発チーム`
/// - 具体辺: `太郎の所属`
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_multi_module.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
///
/// 関係する schema 宣言: `tests/static_multi_module.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
#[derive(Clone, Copy)]
pub struct 太郎の所属Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 太郎の所属Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `member: 社員`
    /// - 具体辺: `太郎の所属`
    /// - 具体端点: `太郎`
    /// - 戻り値: `太郎Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `tests/static_multi_module.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `tests/static_multi_module.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    pub fn member(&self) -> 太郎Ref<'a> {
        太郎Ref { graph: self.graph }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 役割: `team: 部署`
    /// - 具体辺: `太郎の所属`
    /// - 具体端点: `開発部`
    /// - 戻り値: `開発部Ref`
    /// - 検証制約: `each member: 1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `tests/static_multi_module.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
    ///
    /// 関係する instance 宣言: `tests/static_multi_module.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    pub fn team(&self) -> 開発部Ref<'a> {
        開発部Ref { graph: self.graph }
    }
}
/// Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。
///
/// - graph: `開発チーム`
///
/// 固定語彙: `NodeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct NodeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> NodeRefs<'a> {
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `太郎`
    ///
    /// 宣言: `tests/static_multi_module.rs` の `node 太郎: 社員 = ..`
    pub fn 太郎(&self) -> 太郎Ref<'a> {
        太郎Ref { graph: self.graph }
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 個体: `開発部`
    ///
    /// 宣言: `tests/static_multi_module.rs` の `node 開発部: 部署 = ..`
    pub fn 開発部(&self) -> 開発部Ref<'a> {
        開発部Ref { graph: self.graph }
    }
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `開発チーム`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> EdgeRefs<'a> {
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `開発チーム`
    /// - 具体辺: `太郎の所属`
    ///
    /// 宣言: `tests/static_multi_module.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `開発チーム`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph {
    太郎: 社員,
    開発部: 部署,
}
impl Graph {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(太郎: 社員, 開発部: 部署) -> Self {
        Self { 太郎, 開発部 }
    }
    /// Graphite 静的グラフの `Graph` が個体参照の集まりを返すメソッド `node_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn node_refs(&self) -> NodeRefs<'_> {
        NodeRefs { graph: self }
    }
    /// Graphite 静的グラフの `Graph` が辺参照の集まりを返すメソッド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `開発チーム`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn edge_refs(&self) -> EdgeRefs<'_> {
        EdgeRefs { graph: self }
    }
}
/// Graphite 静的グラフの `Graph` を実体化するマクロ `construct` (Graphite の固定語彙)。値ありの個体・積み荷はinstance宣言の式からこのマクロが計算し、値なしの個体だけを宣言順の引数で受け取る。
///
/// - graph: `開発チーム`
/// - 戻り値: `Graph`
///
/// 固定語彙: `construct!` (`docs/static_graph.md` 「生成される名前の公開契約」)
///
/// 関係する instance 宣言: `tests/static_multi_module.rs` の `graph 開発チーム`
macro_rules! construct {
    () => {
        { let (太郎, 開発部,) =
        __graphite_values_開発チーム_74affc48978331c5!(); let () =
        __graphite_payloads_開発チーム_74affc48978331c5!(); #[allow(deprecated)] {
        開発チーム::Graph::__graphite_internal_new(太郎, 開発部) } }
    };
}
pub(crate) use construct;
