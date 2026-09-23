// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_value_expr_name_resolves_at_call_site.rs:26
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    1908813294365810057u64, 16360174594558148498u64, 3311584521671457943u64,
    5649491711407177235u64,
];
/// Graphite 静的グラフの辺値。端点への参照を保持する。
///
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_value_expr_name_resolves_at_call_site.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
pub struct 所属Edge<'a> {
    pub(crate) member: &'a 社員,
    pub(crate) team: &'a 部署,
}
