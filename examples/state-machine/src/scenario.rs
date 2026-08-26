//! (1)(2) シナリオ: 正常な遷移と、未定義遷移が `Err` になる様子。

use state_machine::fsm::{self, Event};
use state_machine::schema::OrderFsm;

pub fn run_scenario() {
    println!("=== シナリオ: 注文ライフサイクル ===\n");
    let g: OrderFsm::Graph = fsm::build();

    println!("--- 正常系: draft から delivered まで一直線に進める ---");
    let mut current = fsm::initial_state();
    println!("現在の状態: {current:?}");
    for event in [Event::Submit, Event::Pay, Event::Ship, Event::Deliver] {
        current =
            fsm::step(&g, &current, event).expect("正常系のシナリオなので毎回定義済みの遷移のはず");
        println!("  --[{event}]--> {current:?}");
    }
    println!("最終状態: {current:?} (正常フローの終点。あとは refund (返品) だけが唯一の出口)");

    println!("\n--- 異常系: delivered からさらに ship しようとすると Err になる ---");
    match fsm::step(&g, &current, Event::Ship) {
        Ok(next) => panic!("delivered からの ship は未定義のはずだが {next:?} に遷移してしまった"),
        Err(e) => println!("  期待どおり Err: {e}"),
    }

    println!("\n--- 異常系: draft から直接 deliver しようとすると Err になる ---");
    let draft = fsm::initial_state();
    match fsm::step(&g, &draft, Event::Deliver) {
        Ok(next) => panic!("draft からの deliver は未定義のはずだが {next:?} に遷移してしまった"),
        Err(e) => println!("  期待どおり Err: {e}"),
    }

    println!("\n--- cancel: 発送前ならキャンセル可能、属性 (理由・返金要否) も読める ---");
    let mut cancel_current = fsm::initial_state();
    cancel_current = fsm::step(&g, &cancel_current, Event::Submit).unwrap();
    cancel_current = fsm::step(&g, &cancel_current, Event::Pay).unwrap();
    // paid の状態でキャンセルする。
    if let Some((_, attrs)) = fsm::cancel_details(&g, &cancel_current) {
        println!(
            "  {cancel_current:?} から cancel: 理由={:?}, 返金要否={}",
            attrs.reason, attrs.refund_required
        );
    }
    let cancelled = fsm::step(&g, &cancel_current, Event::Cancel).unwrap();
    println!("  --[cancel]--> {cancelled:?}");

    println!("\n--- 異常系: shipped まで進めた後は cancel が使えず refund になる ---");
    let mut shipped_flow = fsm::initial_state();
    for event in [Event::Submit, Event::Pay, Event::Ship] {
        shipped_flow = fsm::step(&g, &shipped_flow, event).unwrap();
    }
    match fsm::step(&g, &shipped_flow, Event::Cancel) {
        Ok(next) => panic!("shipped からの cancel は未定義のはずだが {next:?} に遷移してしまった"),
        Err(e) => println!("  shipped から cancel は期待どおり Err: {e}"),
    }
    if let Some((_, attrs)) = fsm::refund_details(&g, &shipped_flow) {
        println!(
            "  shipped から refund は可能。監査ラベル={:?}",
            attrs.audit_label
        );
    }
    let refunded = fsm::step(&g, &shipped_flow, Event::Refund).unwrap();
    println!("  --[refund]--> {refunded:?}");
}
