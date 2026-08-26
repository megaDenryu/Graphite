//! 物語の第3章 — 入力を設定するたびに影響範囲だけが再計算されること。
//!
//! 5つの入力セルを順に設定して値の伝播を見せたあと、税率だけを変えて
//! `reachable_from` による絞り込みが効くことを実演する。

use reactive_cells::engine::Engine;
use reactive_cells::report;
use reactive_cells::schema::CellId;

fn id(spelling: &str) -> CellId {
    CellId(spelling.to_string())
}

pub fn demonstrate_propagation(engine: &mut Engine) {
    report::print_section("値変更 -> 伝播の物語");
    report::print_set_input(
        "(1) 単価を設定",
        &id("unit_price"),
        1000.0,
        &engine.set_input(&id("unit_price"), 1000.0),
    );
    report::print_set_input(
        "(2) 数量を設定",
        &id("quantity"),
        3.0,
        &engine.set_input(&id("quantity"), 3.0),
    );
    report::print_set_input(
        "(3) 税率を設定",
        &id("tax_rate"),
        0.1,
        &engine.set_input(&id("tax_rate"), 0.1),
    );
    report::print_set_input(
        "(4) 割引率を設定",
        &id("discount_rate"),
        0.05,
        &engine.set_input(&id("discount_rate"), 0.05),
    );
    report::print_set_input(
        "(5) 配送料を設定",
        &id("shipping_fee"),
        500.0,
        &engine.set_input(&id("shipping_fee"), 500.0),
    );

    println!("\n現在の値:");
    report::print_engine_snapshot(
        engine,
        &[
            "unit_price",
            "quantity",
            "tax_rate",
            "discount_rate",
            "shipping_fee",
            "subtotal",
            "discount_amount",
            "tax",
            "adjustment",
            "grand_total",
        ],
    );

    report::print_section("影響範囲だけを再計算する (reachable_fromによる絞り込み)");
    let steps = engine.set_input(&id("tax_rate"), 0.2);
    report::print_set_input("税率を変更", &id("tax_rate"), 0.2, &steps);
    println!(
        "  -> subtotal/discount_amount/他の入力セルはtax_rateから到達不能なので再計算されない\n\
         (このexampleでは実際に{}件だけが再計算された。ダイヤモンド依存を通っても\n\
         各セルはちょうど1回だけ再計算されglitchは起きない — これがグラフによる\n\
         再定式化の核心)。",
        steps.len()
    );
}
