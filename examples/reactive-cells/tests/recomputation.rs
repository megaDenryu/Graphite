//! 統合テスト: 再計算の正しさ・範囲・順序を、公開API だけで確認する。

use std::collections::HashSet;

use reactive_cells::engine::Engine;
use reactive_cells::fixtures::default_sheet;
use reactive_cells::schema::CellId;

fn id(s: &str) -> CellId {
    CellId(s.to_string())
}

fn seeded_engine() -> Engine {
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    engine.set_input(&id("unit_price"), 1000.0);
    engine.set_input(&id("quantity"), 3.0);
    engine.set_input(&id("tax_rate"), 0.1);
    engine.set_input(&id("discount_rate"), 0.05);
    engine.set_input(&id("shipping_fee"), 500.0);
    engine
}

#[test]
fn 見積の最終値が正しい() {
    let engine = seeded_engine();
    assert_eq!(engine.value(&id("grand_total")), 3650.0);
}

#[test]
fn ダイヤモンド依存を通る更新でも影響セル数と再計算回数が一致する() {
    let mut engine = seeded_engine();
    // unit_priceはsubtotal経由でdiscount_amount/tax/adjustment/grand_totalに
    // 到達する (ダイヤモンド全体)。影響を受けるのはunit_price自身を除く5セル。
    let steps = engine.set_input(&id("unit_price"), 1500.0);
    let unique: HashSet<CellId> = steps.iter().map(|s| s.id.clone()).collect();
    assert_eq!(
        steps.len(),
        unique.len(),
        "同じセルが2回再計算されてはならない (glitch-free)"
    );
    assert_eq!(unique.len(), 5);
}

#[test]
fn 無関係な更新は依存グラフの反対側に伝播しない() {
    let mut engine = seeded_engine();
    // shipping_feeはgrand_totalにしか繋がっていないので、変更してもsubtotal
    // 以下 (discount_amount/tax/adjustment) は再計算されない。
    let steps = engine.set_input(&id("shipping_fee"), 999.0);
    let ids: HashSet<CellId> = steps.iter().map(|s| s.id.clone()).collect();
    assert_eq!(ids, HashSet::from([id("grand_total")]));
}

#[test]
fn set_inputで直接値を書き込めるのは入力セルだけである() {
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        engine.set_input(&id("grand_total"), 1.0);
    }));
    assert!(result.is_err(), "計算セルへの直接代入はパニックするはず");
}

#[test]
fn topological_orderはgraph_dependency構造と整合する() {
    let engine = Engine::new(default_sheet().unwrap()).unwrap();
    let order = engine.topological_order();
    assert_eq!(order.len(), 10);
    let pos = |k: &str| order.iter().position(|c| c.0 == k).unwrap();
    // 全ての `Feeds(from -> to)`/`Lhs(from -> to)`/`Rhs(from -> to)` エッジ
    // について pos(from) < pos(to) (3種とも「依存元→依存先」という同じ
    // 向きの意味を持つ、`src/schema.rs` 参照)。
    for (from, to) in engine
        .graph()
        .feeds_iter()
        .map(|edge| (edge.dependency().id(), edge.dependent().id()))
        .chain(
            engine
                .graph()
                .lhs_iter()
                .map(|edge| (edge.operand().id(), edge.operation().id())),
        )
        .chain(
            engine
                .graph()
                .rhs_iter()
                .map(|edge| (edge.operand().id(), edge.operation().id())),
        )
    {
        assert!(
            pos(&from.0) < pos(&to.0),
            "{from:?} -> {to:?} はトポロジカル順序に反している"
        );
    }
}
