//! `Formula` が要求するエッジ本数と、実際のエッジ本数の整合性検査。
//!
//! この整合性は `graph!`/`Sheet::Graph::create` の検証対象外 (端点存在と
//! `unique pair` だけを見る、`src/fixtures.rs` 参照) なので、ドメイン側の
//! 責務としてエンジンの構築時に検査する (呼び出し規約違反はパニック、
//! `docs/development/design_principles.md` 原則2)。

use crate::schema::{Formula, Sheet};

// - `Mul`/`Sum` — このセルを終点とする `Feeds` エッジが1本以上必要
//   (可換なので本数の上限は無い)。
// - `Sub` — このセルを終点とする `Lhs`/`Rhs` エッジがそれぞれ
//   ちょうど1本必要 (被減数/減数はどちらも一意でなければならない)。
// - `Input` — エッジ本数を問わない (値は `set_input` で直接与える)。
//
// 本数だけが要件で相手セルの値は不要なので、`{kind}_iter` を毎回
// 全走査して `.filter(.. edge.dependent == cell_id ..)` する代わりに
// `cell.{kind}_as_<role>().count()`
// を使う。freeze 時に構築済みの終点索引を引くだけの O(1) 償却になる。
//
// パニックする条件は次のとおりである。
// `Formula` が要求するエッジ本数と実際の本数が一致しない場合。
pub(super) fn validate_formula_wiring(graph: &Sheet::Graph) {
    for cell in graph.cell_iter() {
        let cell_id = cell.id();
        match cell.formula {
            Formula::Input => {}
            Formula::Mul | Formula::Sum => {
                let count = cell.feeds_as_dependent().count();
                assert!(
                    count >= 1,
                    "{cell_id:?}: {:?}セルには演算対象を表すFeedsエッジが1本以上必要です (実際: {count}本)",
                    cell.formula
                );
            }
            Formula::Sub => {
                let lhs_count = cell.lhs_as_operation().count();
                let rhs_count = cell.rhs_as_operation().count();
                assert_eq!(
                    lhs_count, 1,
                    "{cell_id:?}: Subセルには被減数を表すLhsエッジがちょうど1本必要です (実際: {lhs_count}本)"
                );
                assert_eq!(
                    rhs_count, 1,
                    "{cell_id:?}: Subセルには減数を表すRhsエッジがちょうど1本必要です (実際: {rhs_count}本)"
                );
            }
        }
    }
}
