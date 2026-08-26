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
//!
//! シナリオと検証デモの本体は `scenario.rs`・`validation_demo.rs` にあり、
//! このファイルはサブコマンドの振り分けと使い方の表示だけを持つ。

mod scenario;
mod validation_demo;

use scenario::run_scenario;
use validation_demo::{run_validate, run_validate_broken};

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
