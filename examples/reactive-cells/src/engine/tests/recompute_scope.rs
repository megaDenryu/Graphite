//! 再計算の範囲と回数のテスト — `reachable_from` による絞り込みと、
//! ダイヤモンド依存でも各セルがちょうど1回だけ再計算されること。

use std::collections::HashSet;

use super::id;
use crate::engine::Engine;
use crate::fixtures::default_sheet;
use crate::schema::CellId;

#[test]
fn 影響のないセルはreachable_fromで絞られ再計算されない() {
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    engine.set_input(&id("unit_price"), 1000.0);
    engine.set_input(&id("quantity"), 3.0);
    engine.set_input(&id("tax_rate"), 0.1);
    engine.set_input(&id("discount_rate"), 0.05);
    engine.set_input(&id("shipping_fee"), 500.0);

    // tax_rateだけを変える -> 影響が及ぶのはtax/adjustment/grand_totalのみ。
    // subtotal/discount_amount/他の入力は無関係なので再計算されない。
    let steps = engine.set_input(&id("tax_rate"), 0.2);
    let ids: HashSet<CellId> = steps.iter().map(|s| s.id.clone()).collect();
    assert_eq!(
        ids,
        HashSet::from([id("tax"), id("adjustment"), id("grand_total")])
    );
    assert_eq!(steps.len(), 3, "各セルはちょうど1回だけ再計算されるはず");

    // 新しい税額: subtotal(3000) * 0.2 = 600、adjustment = 600 - 150 = 450、
    // grand_total = 3000 + 450 + 500 = 3950。
    assert_eq!(engine.value(&id("tax")), 600.0);
    assert_eq!(engine.value(&id("adjustment")), 450.0);
    assert_eq!(engine.value(&id("grand_total")), 3950.0);
}

#[test]
fn ダイヤモンド依存でもadjustmentはちょうど1回だけ再計算される() {
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    engine.set_input(&id("tax_rate"), 0.1);
    engine.set_input(&id("discount_rate"), 0.05);
    engine.set_input(&id("shipping_fee"), 500.0);
    engine.set_input(&id("quantity"), 4.0);

    // unit_priceの変更はsubtotal(a) -> discount_amount(b)/tax(c) -> adjustment(d)
    // というダイヤモンド全体に伝播する。
    let steps = engine.set_input(&id("unit_price"), 2000.0);
    let ids: Vec<CellId> = steps.iter().map(|s| s.id.clone()).collect();

    // 重複が無い (=それぞれちょうど1回) ことを確認する。
    let unique: HashSet<CellId> = ids.iter().cloned().collect();
    assert_eq!(
        ids.len(),
        unique.len(),
        "各セルの再計算は重複してはならない"
    );
    assert_eq!(
        unique,
        HashSet::from([
            id("subtotal"),
            id("discount_amount"),
            id("tax"),
            id("adjustment"),
            id("grand_total"),
        ])
    );

    // 順序もトポロジカル (subtotalが最初、adjustmentはb,cの後、
    // grand_totalが最後) であることを確認する。
    let pos = |k: &str| ids.iter().position(|c| c.0 == k).unwrap();
    assert!(pos("subtotal") < pos("discount_amount"));
    assert!(pos("subtotal") < pos("tax"));
    assert!(pos("discount_amount") < pos("adjustment"));
    assert!(pos("tax") < pos("adjustment"));
    assert!(pos("adjustment") < pos("grand_total"));

    // 具体的な数値でも矛盾がないことを確認する:
    // subtotal=2000*4=8000, discount_amount=8000*0.05=400,
    // tax=8000*0.1=800, adjustment=800-400=400, grand_total=8000+400+500=8900。
    // glitchが起きていれば (例えばadjustmentが古いdiscount_amount/taxの
    // どちらかを混ぜて計算していれば) これらの等式は成立しない。
    assert_eq!(engine.value(&id("subtotal")), 8000.0);
    assert_eq!(engine.value(&id("discount_amount")), 400.0);
    assert_eq!(engine.value(&id("tax")), 800.0);
    assert_eq!(engine.value(&id("adjustment")), 400.0);
    assert_eq!(engine.value(&id("grand_total")), 8900.0);
}
