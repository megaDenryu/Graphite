//! 呼び出し規約違反がパニックになることのテスト
//! (`docs/development/design_principles.md` 原則2)。

use super::id;
use crate::engine::Engine;
use crate::fixtures::default_sheet;

#[test]
#[should_panic(expected = "未知のセルキーです")]
fn set_inputは未知のキーでパニックする() {
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    engine.set_input(&id("no_such_cell"), 1.0);
}

#[test]
#[should_panic(expected = "計算セルであり入力セルではありません")]
fn set_inputは計算セルに対してパニックする() {
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    engine.set_input(&id("subtotal"), 999.0);
}

#[test]
#[should_panic(expected = "value: 未知のセルキーです")]
fn valueは未知のキーでパニックする() {
    let engine = Engine::new(default_sheet().unwrap()).unwrap();
    let _ = engine.value(&id("no_such_cell"));
}
