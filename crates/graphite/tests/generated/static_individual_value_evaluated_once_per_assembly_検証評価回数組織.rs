// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_individual_value_evaluated_once_per_assembly.rs:25
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    4559885983535532246u64, 7542107861963189545u64, 11415654177700461400u64,
    3792800334921798268u64,
];
/// Graphite 静的グラフの辺種別を表す型アンカー。端点の役割の形を示す (どのinstanceもこの型を構築しない)。
///
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_individual_value_evaluated_once_per_assembly.rs` の `edge 所属 = (member: 社員) -> (team: 部署)`
pub struct 所属Edge<'a> {
    pub(crate) member: &'a 社員,
    pub(crate) team: &'a 部署,
}
