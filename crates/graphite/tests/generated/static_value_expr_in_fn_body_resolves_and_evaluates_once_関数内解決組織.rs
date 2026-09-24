// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_value_expr_in_fn_body_resolves_and_evaluates_once.rs:30
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    10182979632703560619u64, 1341760981975792380u64, 13898063198035356625u64,
    7421846035162795445u64,
];
/// Graphite 静的グラフの辺種別を表す型アンカー。端点の役割の形を示す (どのinstanceもこの型を構築しない)。
///
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_value_expr_in_fn_body_resolves_and_evaluates_once.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
pub struct 所属Edge<'a> {
    pub(crate) member: &'a 社員,
    pub(crate) team: &'a 部署,
}
