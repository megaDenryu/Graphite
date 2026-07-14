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

use org_analyzer::{analysis, dataset, reorg, report};
use org_analyzer::schema::{DepartmentId, EmployeeId};

const DEFAULT_SEED: u64 = 42;

struct Options {
    seed: u64,
    inject_anomalies: bool,
    /// フラグ以外の残り引数 (サブコマンドの位置引数)。
    positional: Vec<String>,
}

fn parse_options(args: &[String]) -> Result<Options, String> {
    let mut seed = DEFAULT_SEED;
    let mut inject_anomalies = false;
    let mut positional = Vec::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--seed" => {
                let value = args
                    .get(i + 1)
                    .ok_or_else(|| "--seed の後に数値を指定してください".to_string())?;
                seed = value
                    .parse::<u64>()
                    .map_err(|_| format!("--seed の値が数値ではありません: {value}"))?;
                i += 2;
            }
            "--inject-anomalies" => {
                inject_anomalies = true;
                i += 1;
            }
            other => {
                positional.push(other.to_string());
                i += 1;
            }
        }
    }

    Ok(Options {
        seed,
        inject_anomalies,
        positional,
    })
}

fn print_usage() {
    eprintln!(
        "使い方:\n\
         \x20 org-analyzer summary   [--seed N] [--inject-anomalies]\n\
         \x20 org-analyzer chain <社員キー>      [--seed N] [--inject-anomalies]\n\
         \x20 org-analyzer anomalies [--seed N] [--inject-anomalies]\n\
         \x20 org-analyzer reorg <部署キー>      [--seed N] [--inject-anomalies]\n\
         \n\
         社員キーの例: E001..E120 / 部署キーの例: D01..D08\n\
         (実際に生成されたキーは `summary` の出力で確認できます)"
    );
}

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
                eprintln!("エラー: chain には社員キーを指定してください (例: org-analyzer chain E001)");
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
                eprintln!("エラー: reorg には部署キーを指定してください (例: org-analyzer reorg D01)");
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
