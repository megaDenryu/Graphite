// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: tests/static_same_individual_name_multiple_instances.rs:29
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    12255833254660426394u64, 15626552284654202381u64, 2351929282747076464u64,
    11073041775152505892u64,
];
/// Graphite 静的グラフの辺種別を表す型アンカー。端点の役割と、schemaが定めるこの種別の積み荷の形を示す (どのinstanceもこの型を構築しない)。
///
/// - 辺種別: `所属`
///
/// 宣言: `tests/static_same_individual_name_multiple_instances.rs` の `edge 所属 = (member: 社員) -[任命: 任命記録]-> (team: 部署)`
pub struct 所属Edge<'a> {
    pub(crate) member: &'a 社員,
    pub(crate) team: &'a 部署,
    pub(crate) 任命: 任命記録,
}
