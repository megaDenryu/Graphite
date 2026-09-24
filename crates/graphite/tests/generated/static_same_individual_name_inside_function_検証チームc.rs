// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_same_individual_name_inside_function.rs:43
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    8155841523334358406u64, 17711600545827588531u64, 12658600240805035916u64,
    6082763639209756928u64,
];
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `検証チームc`
/// - 個体: `太郎`
/// - 実体型: `社員`
///
/// 宣言: `tests/static_same_individual_name_inside_function.rs` の `node 太郎: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 太郎Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 太郎Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チームc`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        &self.graph.太郎
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `検証チームc`
    /// - 個体: `太郎`
    /// - 具体辺: `太郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `member`
    /// - 戻り値: `太郎の所属Ref`
    ///
    /// 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 開発部)`
    ///
    /// 関係する schema 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `検証チームc`
/// - 個体: `開発部`
/// - 実体型: `部署`
///
/// 宣言: `tests/static_same_individual_name_inside_function.rs` の `node 開発部: 部署 = ..`
#[derive(Clone, Copy)]
pub struct 開発部Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 開発部Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チームc`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 部署 {
        &self.graph.開発部
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `検証チームc`
    /// - 個体: `開発部`
    /// - 具体辺: `太郎の所属`
    /// - 辺種別: `所属`
    /// - この個体の役割: `team`
    /// - 戻り値: `太郎の所属Ref`
    ///
    /// 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 開発部)`
    ///
    /// 関係する schema 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `検証チームc`
/// - 具体辺: `太郎の所属`
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 開発部)`
///
/// 関係する schema 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
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
    ///
    /// 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    ///
    /// 関係する instance 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 開発部)`
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
    ///
    /// 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    ///
    /// 関係する instance 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 開発部)`
    pub fn team(&self) -> 開発部Ref<'a> {
        開発部Ref { graph: self.graph }
    }
    /// Graphite 静的グラフの積み荷アクセサ。
    ///
    /// - 辺種別: `所属`
    /// - 積み荷: `任命: 任命記録`
    /// - 具体辺: `太郎の所属`
    ///
    /// 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
    ///
    /// 関係する instance 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 開発部)`
    pub fn 任命(&self) -> &'a 任命記録 {
        &self.graph.太郎の所属
    }
}
/// Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。
///
/// - graph: `検証チームc`
///
/// 固定語彙: `NodeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct NodeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> NodeRefs<'a> {
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `検証チームc`
    /// - 個体: `太郎`
    ///
    /// 宣言: `tests/static_same_individual_name_inside_function.rs` の `node 太郎: 社員 = ..`
    pub fn 太郎(&self) -> 太郎Ref<'a> {
        太郎Ref { graph: self.graph }
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `検証チームc`
    /// - 個体: `開発部`
    ///
    /// 宣言: `tests/static_same_individual_name_inside_function.rs` の `node 開発部: 部署 = ..`
    pub fn 開発部(&self) -> 開発部Ref<'a> {
        開発部Ref { graph: self.graph }
    }
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `検証チームc`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> EdgeRefs<'a> {
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `検証チームc`
    /// - 具体辺: `太郎の所属`
    ///
    /// 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 太郎の所属 = 所属(太郎 -[..]-> 開発部)`
    pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a> {
        太郎の所属Ref {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `検証チームc`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph {
    太郎: 社員,
    開発部: 部署,
    太郎の所属: 任命記録,
}
impl Graph {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(
        太郎: 社員,
        開発部: 部署,
        太郎の所属: 任命記録,
    ) -> Self {
        Self {
            太郎,
            開発部,
            太郎の所属,
        }
    }
    /// Graphite 静的グラフの `Graph` が個体参照の集まりを返すメソッド `node_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チームc`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn node_refs(&self) -> NodeRefs<'_> {
        NodeRefs { graph: self }
    }
    /// Graphite 静的グラフの `Graph` が辺参照の集まりを返すメソッド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チームc`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn edge_refs(&self) -> EdgeRefs<'_> {
        EdgeRefs { graph: self }
    }
}
/// Graphite 静的グラフの `Graph` を実体化するマクロ `construct` (Graphite の固定語彙)。値ありの個体・積み荷はinstance宣言の式からこのマクロが計算し、値なしの個体だけを宣言順の引数で受け取る。
///
/// - graph: `検証チームc`
/// - 戻り値: `Graph`
///
/// 固定語彙: `construct!` (`docs/static_graph.md` 「生成される名前の公開契約」)
///
/// 関係する instance 宣言: `tests/static_same_individual_name_inside_function.rs` の `graph 検証チームc`
macro_rules! construct {
    () => {
        { let (太郎, 開発部,) =
        __graphite_values_検証チームc_60c059b37c66add4!(); let (太郎の所属,) =
        __graphite_payloads_検証チームc_60c059b37c66add4!(); #[allow(deprecated)] {
        検証チームc::Graph::__graphite_internal_new(太郎, 開発部,
        太郎の所属) } }
    };
}
pub(crate) use construct;
