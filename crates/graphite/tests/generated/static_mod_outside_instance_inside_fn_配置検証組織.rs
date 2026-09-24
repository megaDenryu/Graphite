// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_mod_outside_instance_inside_fn.rs:22
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    9689301481937737525u64, 16190436852501467078u64, 11300793968760762651u64,
    3159105468348197023u64,
];
/// Graphite 静的グラフの辺種別を表す型アンカー。端点の役割の形を示す (どのinstanceもこの型を構築しない)。
///
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_mod_outside_instance_inside_fn.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
pub struct 所属Edge<'a> {
    pub(crate) member: &'a 社員,
    pub(crate) team: &'a 部署,
}
