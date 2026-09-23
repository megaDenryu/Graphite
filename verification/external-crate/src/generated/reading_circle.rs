// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/lib.rs:96
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    906700647454890743u64, 5562571064577290610u64, 3675367196518324237u64,
    18405137051009618409u64,
];
/// Graphite 静的グラフの辺値。端点への参照を保持する。
///
/// - 辺種別: `Assigned`
///
/// 宣言: `src/lib.rs` の `edge Assigned = (book: Book) -> (reader: Reader) where each book: 1`
pub struct AssignedEdge<'a> {
    pub(crate) book: &'a Book,
    pub(crate) reader: &'a Reader,
}
