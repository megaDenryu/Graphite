// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_value_expr_captures_local.rs:26
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    17817952243894878112u64, 4097866441055109391u64, 7074693672159346462u64,
    16042554660230918914u64,
];
/// Graphite 静的グラフの辺値。端点への参照と、schemaが定めるこの種別の積み荷を保持する。
///
/// - 辺種別: `上司`
///
/// 宣言: `tests/static_value_expr_captures_local.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
pub struct 上司Edge<'a> {
    pub(crate) subordinate: &'a 社員,
    pub(crate) superior: &'a 社員,
    pub(crate) 任命: 任命記録,
}
