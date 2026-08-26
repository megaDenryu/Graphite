//! 統合テスト: observer パターンで起きるグリッチが、同じ形の依存を
//! Graphite のエンジンで再計算すると起きないことを対比で確認する。

use reactive_cells::antipattern::build_diamond_demo;
use reactive_cells::engine::Engine;
use reactive_cells::fixtures::default_sheet;
use reactive_cells::schema::CellId;

fn id(s: &str) -> CellId {
    CellId(s.to_string())
}

#[test]
fn observerパターンのグリッチはgraphiteエンジンでは再現しない() {
    // antipattern側はd (adjustment相当) を2回再計算し1回目が矛盾する。
    let naive = build_diamond_demo(false);
    naive.trigger(5.0);
    assert_eq!(
        naive.d_log.borrow().len(),
        2,
        "素朴なobserverパターンは2回再計算する"
    );

    // 同じ形の依存 (a=subtotal, b=discount_amount, c=tax, d=adjustment)
    // をgraphiteエンジンで再計算すると、adjustmentはちょうど1回だけ
    // 再計算される (engine.rsの単体テストで数値まで確認済みなので、
    // ここでは「1回だけ」という回数の主張を再確認する)。
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    engine.set_input(&id("tax_rate"), 0.1);
    engine.set_input(&id("discount_rate"), 0.05);
    let steps = engine.set_input(&id("unit_price"), 10.0);
    let adjustment_recomputes = steps.iter().filter(|s| s.id == id("adjustment")).count();
    assert_eq!(
        adjustment_recomputes, 1,
        "graphite版はadjustmentをちょうど1回だけ再計算する"
    );
}
