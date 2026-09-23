// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_same_individual_name_inside_function.rs:25
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    1195580249667707307u64, 8128484869889040202u64, 315562364577092041u64,
    7834621602747183133u64,
];
/// Graphite 静的グラフの辺値。端点への参照と、schemaが定めるこの種別の積み荷を保持する。
///
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_same_individual_name_inside_function.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
pub struct 所属Edge<'a> {
    pub(crate) member: &'a 社員,
    pub(crate) team: &'a 部署,
    pub(crate) 任命: 任命記録,
}
