// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_value_expr_captures_local.rs:27
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    1150746192874740249u64, 6374311606260791932u64, 15180902445610072763u64,
    11303653444700109727u64,
];
/// Graphite 静的グラフの辺種別を表す型アンカー。端点の役割と、schemaが定めるこの種別の積み荷の形を示す (どのinstanceもこの型を構築しない)。
///
/// - 辺種別: `上司`
///
/// 宣言: `tests/static_value_expr_captures_local.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
pub struct 上司Edge<'a> {
    pub(crate) subordinate: &'a 社員,
    pub(crate) superior: &'a 社員,
    pub(crate) 任命: 任命記録,
}
