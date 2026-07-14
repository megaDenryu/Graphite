//! ビルドパイプライン・オーケストレータ CLI。
//!
//! `pipeline.txt` (簡易行形式) を読み込み、Task/Artifact の異種ノードと
//! produces/consumes の型付きエッジからなるグラフ (`graphite::graph_schema!`
//! 製) を組み立てて、検証・実行計画・クリティカルパス・可視化を行う。
//!
//! パイプライン: lexer 相当 (`parser`) -> グラフ構築 (`builder`) ->
//! 検証・分析 (`analysis`) -> 出力整形 (`report`)。

use build_pipeline::{analysis, builder, parser, report, schema};
use schema::BuildPipeline;
use std::env;
use std::fs;
use std::process::ExitCode;

fn print_usage(program: &str) {
    eprintln!(
        "使い方: {program} <subcommand> [pipeline-file]\n\
\n\
subcommand:\n\
  validate        図式適合 + ドメイン検証 (孤児成果物/produce競合/循環依存) を行う\n\
  plan            並列実行可能なタスクの波を計算して表示する\n\
  critical-path   クリティカルパス (最長経路) と全体並列度を表示する\n\
  mermaid         グラフを mermaid flowchart として標準出力へ書き出す\n\
\n\
pipeline-file省略時は ./pipeline.txt を読み込む。"
    );
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let program = args.first().map(String::as_str).unwrap_or("build-pipeline");

    let Some(subcommand) = args.get(1) else {
        print_usage(program);
        return ExitCode::FAILURE;
    };

    let pipeline_path = args.get(2).map(String::as_str).unwrap_or("pipeline.txt");

    let input = match fs::read_to_string(pipeline_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("エラー: {pipeline_path} を読み込めません: {e}");
            return ExitCode::FAILURE;
        }
    };

    let parsed = match parser::parse(&input) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("パースエラー: {e}");
            return ExitCode::FAILURE;
        }
    };

    let graph = match builder::build_graph(&parsed) {
        Ok(g) => g,
        Err(v) => {
            eprintln!("図式適合エラー: {v}");
            return ExitCode::FAILURE;
        }
    };

    match subcommand.as_str() {
        "validate" => cmd_validate(&graph),
        "plan" => cmd_plan(&graph),
        "critical-path" => cmd_critical_path(&graph),
        "mermaid" => cmd_mermaid(&graph),
        other => {
            eprintln!("未知のサブコマンド: {other}");
            print_usage(program);
            ExitCode::FAILURE
        }
    }
}

fn cmd_validate(g: &BuildPipeline) -> ExitCode {
    let issues = analysis::validate(g);
    let has_issues = !issues.is_empty();
    println!("{}", report::format_domain_issues(&issues));
    if has_issues {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn cmd_plan(g: &BuildPipeline) -> ExitCode {
    match analysis::plan(g) {
        Ok(waves) => {
            println!("{}", report::format_plan(&waves));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("循環依存があるため実行計画を作成できません: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_critical_path(g: &BuildPipeline) -> ExitCode {
    match analysis::critical_path(g) {
        Ok(cp) => {
            println!("{}", report::format_critical_path(&cp, g));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("循環依存があるためクリティカルパスを計算できません: {e}");
            ExitCode::FAILURE
        }
    }
}

fn cmd_mermaid(g: &BuildPipeline) -> ExitCode {
    println!("{}", report::mermaid(g));
    ExitCode::SUCCESS
}
