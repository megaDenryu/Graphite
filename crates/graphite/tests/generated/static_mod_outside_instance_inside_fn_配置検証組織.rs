// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_mod_outside_instance_inside_fn.rs:22
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    5394646703443674688u64, 3785100643201632181u64, 1762068288741192330u64,
    1557742951019157214u64,
];
/// Graphite 静的グラフの辺値。端点への参照を保持する。
///
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_mod_outside_instance_inside_fn.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
pub struct 所属Edge<'a> {
    pub(crate) member: &'a 社員,
    pub(crate) team: &'a 部署,
}
