//! 統合テスト — 公開API (`reactive_cells::*`) だけを使って end-to-end で
//! 確認する。単体テスト (`src/*.rs` 内の `#[cfg(test)]`) は個々のロジック
//! を細かく確認しているのに対し、ここでは README で説明した3つの主張
//! (グリッチ不在・循環拒否・影響範囲の絞り込み) を利用者視点でもう一度
//! 検証する。

use std::collections::HashSet;

use reactive_cells::antipattern::{build_diamond_demo, build_infinite_loop_demo};
use reactive_cells::engine::Engine;
use reactive_cells::fixtures::{cyclic_demo_sheet, default_sheet};
use reactive_cells::schema::{CellId, Feeds, Lhs, Rhs};

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
    assert_eq!(steps.len(), unique.len(), "同じセルが2回再計算されてはならない (glitch-free)");
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
fn 循環する依存グラフはengine_newの時点でcycleerrorになる() {
    let sheet = cyclic_demo_sheet().expect("構造検証自体は循環でも通る");
    // `Engine`はDebugを実装しないため`expect_err`ではなくmatchで取り出す。
    let err = match Engine::new(sheet) {
        Err(err) => err,
        Ok(_) => panic!("循環があるのでEngine::newは失敗するはず"),
    };
    assert_eq!(err.cycle.len(), 3);
    // 循環パスが実際に閉路になっている (cycle[i] -> cycle[i+1] が
    // feedsエッジとして存在する) ことまでは、CycleError自体の保証
    // (`crates/graphite/src/graph.rs`のドキュメント参照) に委ねる。
}

#[test]
fn observerパターンのグリッチはgraphiteエンジンでは再現しない() {
    // antipattern側はd (adjustment相当) を2回再計算し1回目が矛盾する。
    let naive = build_diamond_demo(false);
    naive.trigger(5.0);
    assert_eq!(naive.d_log.borrow().len(), 2, "素朴なobserverパターンは2回再計算する");

    // 同じ形の依存 (a=subtotal, b=discount_amount, c=tax, d=adjustment)
    // をgraphiteエンジンで再計算すると、adjustmentはちょうど1回だけ
    // 再計算される (engine.rsの単体テストで数値まで確認済みなので、
    // ここでは「1回だけ」という回数の主張を再確認する)。
    let mut engine = Engine::new(default_sheet().unwrap()).unwrap();
    engine.set_input(&id("tax_rate"), 0.1);
    engine.set_input(&id("discount_rate"), 0.05);
    let steps = engine.set_input(&id("unit_price"), 10.0);
    let adjustment_recomputes = steps.iter().filter(|s| s.id == id("adjustment")).count();
    assert_eq!(adjustment_recomputes, 1, "graphite版はadjustmentをちょうど1回だけ再計算する");
}

#[test]
fn 循環購読の無限notifyは安全弁なしでは自然に止まらない() {
    let cap = 500;
    let count = build_infinite_loop_demo(cap);
    assert_eq!(count, cap, "capに到達する = 循環があれば自然には止まらないことの証拠");
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
    for (from, to) in Feeds::iter(engine.graph())
        .map(|(_id, edge)| (edge.from(), edge.to()))
        .chain(Lhs::iter(engine.graph()).map(|(_id, edge)| (edge.from(), edge.to())))
        .chain(Rhs::iter(engine.graph()).map(|(_id, edge)| (edge.from(), edge.to())))
    {
        assert!(
            pos(&from.0) < pos(&to.0),
            "{from:?} -> {to:?} はトポロジカル順序に反している"
        );
    }
}
