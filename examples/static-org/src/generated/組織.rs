// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/main.rs:39
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    10936049115705435305u64, 3954739200782021960u64, 13495638593551112319u64,
    17553142642863232011u64,
];
/// Graphite 静的グラフの辺値。端点への参照を保持する。
///
/// - 辺種別: `所属`
///
/// 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`
pub struct 所属Edge<'a> {
    pub(crate) member: &'a 社員,
    pub(crate) team: &'a 部署,
}
/// Graphite 静的グラフの辺値。端点への参照と、schemaが定めるこの種別の積み荷を保持する。
///
/// - 辺種別: `上司`
///
/// 宣言: `src/main.rs` の `edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1`
pub struct 上司Edge<'a> {
    pub(crate) subordinate: &'a 社員,
    pub(crate) superior: &'a 社員,
    pub(crate) 任命: 任命記録,
}
/// Graphite 静的グラフの辺値。端点への参照を保持する。
///
/// - 辺種別: `友人`
///
/// 宣言: `src/main.rs` の `edge 友人 = (甲: 社員) -- (乙: 社員) where unique pair`
pub struct 友人Edge<'a> {
    pub(crate) 甲: &'a 社員,
    pub(crate) 乙: &'a 社員,
}
/// Graphite 静的グラフの辺値。端点への参照と、schemaが定めるこの種別の積み荷を保持する。
///
/// - 辺種別: `同僚`
///
/// 宣言: `src/main.rs` の `edge 同僚 = (甲: 社員) -[経緯: 経緯記録]- (乙: 社員)`
pub struct 同僚Edge<'a> {
    pub(crate) 甲: &'a 社員,
    pub(crate) 乙: &'a 社員,
    pub(crate) 経緯: 経緯記録,
}
