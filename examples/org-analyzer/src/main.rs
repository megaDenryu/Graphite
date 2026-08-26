//! org-analyzer — Graphite (`graph_schema!`) を使った組織分析ツール。
//!
//! 人事データを「社員・部署・プロジェクトの3ノード + 4種の型付きエッジ」の
//! グラフとして扱い、多重度制約 (全社員は必ず1部署) と構造検査を実演する
//! CLI アプリ。詳細は `README.md` 参照。
//!
//! ```text
//! org-analyzer summary   [--seed N] [--inject-anomalies]
//! org-analyzer chain <社員キー>      [--seed N] [--inject-anomalies]
//! org-analyzer anomalies [--seed N] [--inject-anomalies]
//! org-analyzer reorg <部署キー>      [--seed N] [--inject-anomalies]
//! ```
//!
//! 引数の解釈と使い方の表示は `options.rs` にあり、このファイルはサブコマンド
//! の振り分けだけを持つ。

mod options;

use options::{parse_options, print_usage};

use org_analyzer::schema::{DepartmentId, EmployeeId};
use org_analyzer::{analysis, dataset, reorg, report};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage();
        std::process::exit(1);
    }

    let subcommand = args[0].clone();
    if subcommand == "help" || subcommand == "-h" || subcommand == "--help" {
        print_usage();
        return;
    }

    let options = match parse_options(&args[1..]) {
        Ok(o) => o,
        Err(msg) => {
            eprintln!("エラー: {msg}");
            print_usage();
            std::process::exit(1);
        }
    };

    let generated = dataset::generate(options.seed, options.inject_anomalies);
    let org = &generated.chart;

    match subcommand.as_str() {
        "summary" => {
            let summary = analysis::summarize(org);
            report::print_summary(&summary);
        }
        "chain" => {
            let Some(key) = options.positional.first() else {
                eprintln!(
                    "エラー: chain には社員キーを指定してください (例: org-analyzer chain E001)"
                );
                std::process::exit(1);
            };
            let employee_id = EmployeeId(key.clone());
            match analysis::management_chain(org, &employee_id) {
                Some(result) => report::print_chain(&result),
                None => {
                    report::print_unknown_employee(&employee_id);
                    std::process::exit(1);
                }
            }
        }
        "anomalies" => {
            let anomalies = analysis::detect_anomalies(org);
            report::print_anomalies(org, &anomalies);
        }
        "reorg" => {
            let Some(key) = options.positional.first() else {
                eprintln!(
                    "エラー: reorg には部署キーを指定してください (例: org-analyzer reorg D01)"
                );
                std::process::exit(1);
            };
            let dept_id = DepartmentId(key.clone());
            match reorg::simulate_reorg(org, &dept_id) {
                Some(result) => report::print_reorg(org, &result),
                None => {
                    eprintln!("エラー: 部署キー '{}' は存在しません", dept_id.0);
                    std::process::exit(1);
                }
            }
        }
        other => {
            eprintln!("エラー: 未知のサブコマンドです: {other}");
            print_usage();
            std::process::exit(1);
        }
    }
}
