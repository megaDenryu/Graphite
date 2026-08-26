//! (3) グラフアルゴリズムによる FSM 設計検査と、その実演。

use state_machine::fsm;
use state_machine::validate::{self, ValidationReport};

pub fn run_validate() {
    println!("=== 検証: 正規のFSM定義 (fsm::build()) ===\n");
    let g = fsm::build();
    let report = validate::validate(&g, &fsm::initial_state(), &fsm::terminal_states());
    print_report("正規のFSM", &report);
    assert!(report.is_ok(), "正規のFSM定義は健全なはず");
}

pub fn run_validate_broken() {
    println!("=== 検証デモ: 壊れた変種2つで検出アルゴリズムを実演 ===\n");

    println!("--- デモ1: held_for_review 状態への辺を張り忘れた変種 (到達不能検出) ---");
    let broken_unreachable = fsm::build_with_unreachable_state();
    let report = validate::validate(
        &broken_unreachable,
        &fsm::initial_state(),
        &fsm::terminal_states(),
    );
    print_report("held_for_review 未接続の変種", &report);
    assert!(
        !report.unreachable.is_empty(),
        "held_for_review は到達不能として検出されるはず"
    );
    assert!(
        report.dead_ends.is_empty(),
        "held_for_review はcancelへの辺を持つので行き止まりではないはず"
    );

    println!(
        "\n--- デモ2: shipped の出口 (deliver/refund) を両方書き忘れた変種 (行き止まり検出) ---"
    );
    let broken_dead_end = fsm::build_with_dead_end_bug();
    let report = validate::validate(
        &broken_dead_end,
        &fsm::initial_state(),
        &fsm::terminal_states(),
    );
    print_report("shipped 出口未定義の変種", &report);
    assert!(
        report.unreachable.is_empty(),
        "この変種は到達不能状態を作らない設計のはず"
    );
    assert!(
        !report.dead_ends.is_empty(),
        "shipped は行き止まりとして検出されるはず"
    );
}

fn print_report(label: &str, report: &ValidationReport) {
    println!("[{label}]");
    if report.unreachable.is_empty() {
        println!("  到達不能状態: なし");
    } else {
        println!("  到達不能状態: {:?}", report.unreachable);
    }
    if report.dead_ends.is_empty() {
        println!("  行き止まり状態: なし");
    } else {
        println!("  行き止まり状態: {:?}", report.dead_ends);
    }
    println!(
        "  総合判定: {}",
        if report.is_ok() {
            "健全"
        } else {
            "問題あり"
        }
    );
}
