// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_individual_value_evaluated_once_per_assembly.rs:25
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    9950949410128385477u64, 9739219544751351908u64, 3544813624233021739u64,
    18254212163993174271u64,
];
/// Graphite 静的グラフの辺値。端点への参照を保持する。
///
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_individual_value_evaluated_once_per_assembly.rs` の `edge 所属 = (member: 社員) -> (team: 部署)`
pub struct 所属Edge<'a> {
    pub(crate) member: &'a 社員,
    pub(crate) team: &'a 部署,
}
