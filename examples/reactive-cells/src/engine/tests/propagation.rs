//! 入力の設定が計算セルへ正しく伝播することのテスト。

use super::id;
use crate::engine::Engine;
use crate::fixtures::default_sheet;

#[test]
fn 全入力を設定すると見積の数値が正しく伝播する() {
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    engine.set_input(&id("unit_price"), 1000.0);
    engine.set_input(&id("quantity"), 3.0);
    engine.set_input(&id("tax_rate"), 0.1);
    engine.set_input(&id("discount_rate"), 0.05);
    engine.set_input(&id("shipping_fee"), 500.0);

    assert_eq!(engine.value(&id("subtotal")), 3000.0);
    assert_eq!(engine.value(&id("discount_amount")), 150.0);
    assert_eq!(engine.value(&id("tax")), 300.0);
    assert_eq!(engine.value(&id("adjustment")), 150.0);
    assert_eq!(engine.value(&id("grand_total")), 3650.0);
}

#[test]
fn 複数回の入力変更が累積して正しく反映される() {
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    engine.set_input(&id("unit_price"), 100.0);
    engine.set_input(&id("quantity"), 1.0);
    engine.set_input(&id("tax_rate"), 0.1);
    engine.set_input(&id("discount_rate"), 0.0);
    engine.set_input(&id("shipping_fee"), 0.0);
    assert_eq!(engine.value(&id("grand_total")), 110.0);

    engine.set_input(&id("quantity"), 2.0);
    assert_eq!(engine.value(&id("subtotal")), 200.0);
    assert_eq!(engine.value(&id("grand_total")), 220.0);

    engine.set_input(&id("discount_rate"), 0.1);
    assert_eq!(engine.value(&id("discount_amount")), 20.0);
    assert_eq!(engine.value(&id("adjustment")), 20.0 - 20.0); // tax(20) - discount(20)
    assert_eq!(engine.value(&id("grand_total")), 200.0 + 0.0 + 0.0);
}
