// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: src/lib.rs:96
// 再生成: パッケージのディレクトリで cargo graphite generate を実行してください (Graphite リポジトリ自身の開発では cargo xtask generate)

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_STATIC_SCHEMA_FINGERPRINT: [u64; 4] = [
    8608392596079759118u64, 206602206560254401u64, 10513658031446216456u64,
    7327125950463957108u64,
];
/// Graphite 静的グラフの辺種別を表す型アンカー。端点の役割の形を示す (どのinstanceもこの型を構築しない)。
///
/// - 辺種別: `Assigned`
///
/// 宣言: `src/lib.rs` の `edge Assigned = (book: Book) -> (reader: Reader) where each book: 1`
pub struct AssignedEdge<'a> {
    pub(crate) book: &'a Book,
    pub(crate) reader: &'a Reader,
}
