//! state-machine — 「ステートマシン地獄」を Graphite で倒す実証example。
//!
//! 注文ライフサイクル (draft → pending_payment → paid → shipped →
//! delivered、脱線として cancelled/refunded) を `graph_schema!`/`graph!` で
//! 定義し、(1) 正常系の遷移、(2) 未定義遷移が型でエラーになる様子、
//! (3) グラフアルゴリズムによる FSM 設計検査、を読み物として実演する。
//! 詳細は README.md 参照。
//!
//! ```text
//! state-machine              # 全部 (シナリオ + 検証 + 検証デモ) を順に実行
//! state-machine scenario     # (1)(2) シナリオのみ
//! state-machine validate     # (3) 正規のFSMを検証 (健全なはず)
//! state-machine validate-broken  # (3) 壊れた変種2つに対する検出デモ
//! ```

use state_machine::fsm::{self, Event};
use state_machine::schema::OrderFsm;
use state_machine::validate::{self, ValidationReport};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let subcommand = args.first().map(String::as_str).unwrap_or("all");

    match subcommand {
        "all" => {
            run_scenario();
            println!();
            run_validate();
            println!();
            run_validate_broken();
        }
        "scenario" => run_scenario(),
        "validate" => run_validate(),
        "validate-broken" => run_validate_broken(),
        "help" | "-h" | "--help" => print_usage(),
        other => {
            eprintln!("エラー: 未知のサブコマンドです: {other}");
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!(
        "使い方:\n\
         \x20 state-machine                   # 全部実行\n\
         \x20 state-machine scenario          # シナリオ (正常系/異常系) のみ\n\
         \x20 state-machine validate          # 正規FSMの検証\n\
         \x20 state-machine validate-broken   # 壊れた変種の検出デモ"
    );
}

// ============================================================
// (1)(2) シナリオ: 正常な遷移 + 未定義遷移が Err になる様子
// ============================================================

fn run_scenario() {
    println!("=== シナリオ: 注文ライフサイクル ===\n");
    let g: OrderFsm = fsm::build();

    println!("--- 正常系: draft から delivered まで一直線に進める ---");
    let mut current = fsm::initial_state();
    println!("現在の状態: {current:?}");
    for event in [Event::Submit, Event::Pay, Event::Ship, Event::Deliver] {
        current = fsm::step(&g, &current, event)
            .expect("正常系のシナリオなので毎回定義済みの遷移のはず");
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
        println!("  shipped から refund は可能。監査ラベル={:?}", attrs.audit_label);
    }
    let refunded = fsm::step(&g, &shipped_flow, Event::Refund).unwrap();
    println!("  --[refund]--> {refunded:?}");
}

// ============================================================
// (3) グラフアルゴリズムによる FSM 設計検査
// ============================================================

fn run_validate() {
    println!("=== 検証: 正規のFSM定義 (fsm::build()) ===\n");
    let g = fsm::build();
    let report = validate::validate(&g, &fsm::initial_state(), &fsm::terminal_states());
    print_report("正規のFSM", &report);
    assert!(report.is_ok(), "正規のFSM定義は健全なはず");
}

fn run_validate_broken() {
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

    println!("\n--- デモ2: shipped の出口 (deliver/refund) を両方書き忘れた変種 (行き止まり検出) ---");
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
    println!("  総合判定: {}", if report.is_ok() { "健全" } else { "問題あり" });
}
