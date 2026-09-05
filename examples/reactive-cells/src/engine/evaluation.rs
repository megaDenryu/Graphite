//! セル1つの式を評価して値を1つ求める、読み取りだけの計算。
//!
//! 演算対象は `Formula` 自身ではなく、そのセルを終点とする
//! `Feeds`/`Lhs`/`Rhs` エッジから読む (`engine.rs` 冒頭のモジュール doc
//! 参照)。この「終点で絞り込む」操作には役割クエリを使うため、辺表の
//! 全走査は起きない。

use super::Engine;
use crate::schema::{CellId, Formula};

impl Engine {
    // `cell_id` の `formula` を評価する。
    pub(super) fn eval_formula(&self, cell_id: &CellId, formula: Formula) -> f64 {
        match formula {
            Formula::Input => {
                unreachable!("Inputセルはset_inputのトポロジカル走査で再計算対象にならない")
            }
            Formula::Mul => self.feeds_into(cell_id).product(),
            Formula::Sum => self.feeds_into(cell_id).sum(),
            Formula::Sub => self.lhs_value(cell_id) - self.rhs_value(cell_id),
        }
    }

    // `cell_id` を終点とする `Feeds` エッジの起点セルの値を、挿入順
    // (`docs/schema_v4.md` §3.2 の順序保証) で列挙する。
    //
    // `cell.feeds_as_dependent()` は辺参照を返す。起点NodeRefの `id()` から
    // 現在値ストアのキーを直接得られるため、辺表の全走査は不要である。
    fn feeds_into<'a>(&'a self, cell_id: &'a CellId) -> impl Iterator<Item = f64> + 'a {
        let cell = self
            .graph
            .cell_by_id(cell_id)
            .expect("評価対象セルはグラフに存在するはず");
        cell.feeds_as_dependent()
            .map(move |edge| self.value(edge.dependency().id()))
    }

    // `cell_id` を終点とする `Lhs` エッジの起点セルの値 (被減数)。
    // 役割クエリを使う理由は `Self::feeds_into` と同じ (相手の
    // `CellId` が要る)。
    //
    // パニックする条件は次のとおりである。
    // `Lhs` エッジがちょうど1本であることは
    // `super::formula_wiring::validate_formula_wiring`
    // が `Engine::new` の時点で検査済みなので、ここに到達した時点で
    // 見つからなければ実装の不整合 (バグ) である。
    fn lhs_value(&self, cell_id: &CellId) -> f64 {
        let cell = self
            .graph
            .cell_by_id(cell_id)
            .expect("評価対象セルはグラフに存在するはず");
        let operand = cell
            .lhs_as_operation()
            .next()
            .expect("validate_formula_wiringで存在を検査済みのはず");
        self.value(operand.operand().id())
    }

    // `cell_id` を終点とする `Rhs` エッジの起点セルの値 (減数)。
    // 役割クエリを使う理由は `Self::feeds_into` と同じ。
    //
    // パニックする条件は次のとおりである。
    // `Self::lhs_value` と同様、`Engine::new` の検査済み前提が破れて
    // いる場合のみパニックする (実装の不整合)。
    fn rhs_value(&self, cell_id: &CellId) -> f64 {
        let cell = self
            .graph
            .cell_by_id(cell_id)
            .expect("評価対象セルはグラフに存在するはず");
        let operand = cell
            .rhs_as_operation()
            .next()
            .expect("validate_formula_wiringで存在を検査済みのはず");
        self.value(operand.operand().id())
    }
}
