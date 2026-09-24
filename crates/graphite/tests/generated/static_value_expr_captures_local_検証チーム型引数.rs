// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_value_expr_captures_local.rs:95
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_INSTANCE_FINGERPRINT: [u64; 4] = [
    13502672415092840725u64, 6409088999736536332u64, 9007310629765208211u64,
    18261846804022559207u64,
];
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `検証チーム型引数`
/// - 個体: `太郎`
/// - 実体型: `社員`
///
/// 宣言: `tests/static_value_expr_captures_local.rs` の `node 太郎: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 太郎Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 太郎Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チーム型引数`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        &self.graph.太郎
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `検証チーム型引数`
    /// - 個体: `太郎`
    /// - 具体辺: `太郎の上司`
    /// - 辺種別: `上司`
    /// - この個体の役割: `subordinate`
    /// - 戻り値: `太郎の上司Ref`
    ///
    /// 宣言: `tests/static_value_expr_captures_local.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    ///
    /// 関係する schema 宣言: `tests/static_value_expr_captures_local.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    pub fn 太郎の上司(&self) -> 太郎の上司Ref<'a> {
        太郎の上司Ref {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体個体参照。
///
/// - graph: `検証チーム型引数`
/// - 個体: `次郎`
/// - 実体型: `社員`
///
/// 宣言: `tests/static_value_expr_captures_local.rs` の `node 次郎: 社員 = ..`
#[derive(Clone, Copy)]
pub struct 次郎Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 次郎Ref<'a> {
    /// Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チーム型引数`
    ///
    /// 固定語彙: `entity` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn entity(&self) -> &'a 社員 {
        &self.graph.次郎
    }
    /// Graphite 静的グラフの具体辺参照を返す。
    ///
    /// - graph: `検証チーム型引数`
    /// - 個体: `次郎`
    /// - 具体辺: `太郎の上司`
    /// - 辺種別: `上司`
    /// - この個体の役割: `superior`
    /// - 戻り値: `太郎の上司Ref`
    ///
    /// 宣言: `tests/static_value_expr_captures_local.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    ///
    /// 関係する schema 宣言: `tests/static_value_expr_captures_local.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    pub fn 太郎の上司(&self) -> 太郎の上司Ref<'a> {
        太郎の上司Ref {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体辺参照。
///
/// - graph: `検証チーム型引数`
/// - 具体辺: `太郎の上司`
/// - 辺種別: `上司`
///
/// 宣言: `tests/static_value_expr_captures_local.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
///
/// 関係する schema 宣言: `tests/static_value_expr_captures_local.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
#[derive(Clone, Copy)]
pub struct 太郎の上司Ref<'a> {
    graph: &'a Graph,
}
impl<'a> 太郎の上司Ref<'a> {
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `上司`
    /// - 役割: `subordinate: 社員`
    /// - 具体辺: `太郎の上司`
    /// - 具体端点: `太郎`
    /// - 戻り値: `太郎Ref`
    /// - 検証制約: `each subordinate: 0..1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `tests/static_value_expr_captures_local.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    ///
    /// 関係する instance 宣言: `tests/static_value_expr_captures_local.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    pub fn subordinate(&self) -> 太郎Ref<'a> {
        太郎Ref { graph: self.graph }
    }
    /// Graphite 静的グラフの端点の役割アクセサ。
    ///
    /// - 辺種別: `上司`
    /// - 役割: `superior: 社員`
    /// - 具体辺: `太郎の上司`
    /// - 具体端点: `次郎`
    /// - 戻り値: `次郎Ref`
    /// - 検証制約: `each subordinate: 0..1` (instance の辺の集合が満たすことを展開時に検査済み。戻り値の型は制約ではなく具体辺の宣言が決める)
    ///
    /// 宣言: `tests/static_value_expr_captures_local.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    ///
    /// 関係する instance 宣言: `tests/static_value_expr_captures_local.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    pub fn superior(&self) -> 次郎Ref<'a> {
        次郎Ref { graph: self.graph }
    }
    /// Graphite 静的グラフの積み荷アクセサ。
    ///
    /// - 辺種別: `上司`
    /// - 積み荷: `任命: 任命記録`
    /// - 具体辺: `太郎の上司`
    ///
    /// 宣言: `tests/static_value_expr_captures_local.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
    ///
    /// 関係する instance 宣言: `tests/static_value_expr_captures_local.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    pub fn 任命(&self) -> &'a 任命記録 {
        &self.graph.太郎の上司
    }
}
/// Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。
///
/// - graph: `検証チーム型引数`
///
/// 固定語彙: `NodeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct NodeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> NodeRefs<'a> {
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `検証チーム型引数`
    /// - 個体: `太郎`
    ///
    /// 宣言: `tests/static_value_expr_captures_local.rs` の `node 太郎: 社員 = ..`
    pub fn 太郎(&self) -> 太郎Ref<'a> {
        太郎Ref { graph: self.graph }
    }
    /// Graphite 静的グラフの個体参照メソッド。`NodeRefs` がこのメソッドでこの個体の具体参照を返す。
    ///
    /// - graph: `検証チーム型引数`
    /// - 個体: `次郎`
    ///
    /// 宣言: `tests/static_value_expr_captures_local.rs` の `node 次郎: 社員 = ..`
    pub fn 次郎(&self) -> 次郎Ref<'a> {
        次郎Ref { graph: self.graph }
    }
}
/// Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。
///
/// - graph: `検証チーム型引数`
///
/// 固定語彙: `EdgeRefs` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct EdgeRefs<'a> {
    graph: &'a Graph,
}
impl<'a> EdgeRefs<'a> {
    /// Graphite 静的グラフの辺参照メソッド。`EdgeRefs` がこのメソッドでこの具体辺の具体参照を返す。
    ///
    /// - graph: `検証チーム型引数`
    /// - 具体辺: `太郎の上司`
    ///
    /// 宣言: `tests/static_value_expr_captures_local.rs` の `edge 太郎の上司 = 上司(太郎 -[..]-> 次郎)`
    pub fn 太郎の上司(&self) -> 太郎の上司Ref<'a> {
        太郎の上司Ref {
            graph: self.graph,
        }
    }
}
/// Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。
///
/// - graph: `検証チーム型引数`
///
/// 固定語彙: `Graph` (`docs/static_graph.md` 「生成される名前の公開契約」)
pub struct Graph {
    太郎: 社員,
    次郎: 社員,
    太郎の上司: 任命記録,
}
impl Graph {
    #[doc(hidden)]
    #[deprecated(
        note = "Graphite の内部構築子である。construct! を使うこと"
    )]
    pub(crate) fn __graphite_internal_new(
        太郎: 社員,
        次郎: 社員,
        太郎の上司: 任命記録,
    ) -> Self {
        Self {
            太郎,
            次郎,
            太郎の上司,
        }
    }
    /// Graphite 静的グラフの `Graph` が個体参照の集まりを返すメソッド `node_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チーム型引数`
    ///
    /// 固定語彙: `node_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn node_refs(&self) -> NodeRefs<'_> {
        NodeRefs { graph: self }
    }
    /// Graphite 静的グラフの `Graph` が辺参照の集まりを返すメソッド `edge_refs` (Graphite の固定語彙)。
    ///
    /// - graph: `検証チーム型引数`
    ///
    /// 固定語彙: `edge_refs` (`docs/static_graph.md` 「生成される名前の公開契約」)
    pub fn edge_refs(&self) -> EdgeRefs<'_> {
        EdgeRefs { graph: self }
    }
}
/// Graphite 静的グラフの `Graph` を実体化するマクロ `construct` (Graphite の固定語彙)。値ありの個体・積み荷はinstance宣言の式からこのマクロが計算し、値なしの個体だけを宣言順の引数で受け取る。
///
/// - graph: `検証チーム型引数`
/// - 戻り値: `Graph`
///
/// 固定語彙: `construct!` (`docs/static_graph.md` 「生成される名前の公開契約」)
///
/// 関係する instance 宣言: `tests/static_value_expr_captures_local.rs` の `graph 検証チーム型引数`
macro_rules! construct {
    () => {
        { let (太郎, 次郎,) =
        __graphite_values_検証チーム型引数_79e829f4106b6b7f!(); let
        (太郎の上司,) =
        __graphite_payloads_検証チーム型引数_79e829f4106b6b7f!();
        #[allow(deprecated)] {
        検証チーム型引数::Graph::__graphite_internal_new(太郎, 次郎,
        太郎の上司) } }
    };
}
pub(crate) use construct;
