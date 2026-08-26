//! dialogue-engine — Graphite の `graph_schema!`/`graph!` で分岐シナリオを
//! 記述し、プレイ・検証・可視化する CLI。
//!
//! サブコマンド:
//! - `play [--script 1,2,1]` — 対話プレイ (省略時は stdin から選択肢番号を
//!   読む。`--script` を渡すとその選択列で自動プレイする)
//! - `validate` — シナリオ構造検証 (到達不能シーン・デッドエンド・
//!   閉じたループ・到達不能エンディングを検出)
//! - `map` — mermaid flowchart 出力
//! - `route <ending名>` — 指定エンディングへの最短ルート表示
//! - `stats` — シーン数・分岐数などの統計表示
//!
//! 各サブコマンドの本体はサブコマンド名を持つモジュール
//! (`play_command`・`validate_command`・`route_command`・`stats_command`) に
//! あり、このファイルは引数の振り分けと使い方の表示だけを持つ。

mod play_command;
mod route_command;
mod stats_command;
mod validate_command;

use dialogue_engine::mermaid;
use dialogue_engine::story::{build_story, start_scene_id};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let subcommand = args.get(1).map(String::as_str);

    let story = match build_story() {
        Ok(story) => story,
        Err(violation) => {
            eprintln!("シナリオの構築に失敗しました: {violation}");
            std::process::exit(1);
        }
    };
    let start = start_scene_id();

    match subcommand {
        Some("play") => play_command::cmd_play(&story, &start, &args[2..]),
        Some("validate") => validate_command::cmd_validate(&story, &start),
        Some("map") => println!("{}", mermaid::to_mermaid(&story)),
        Some("route") => route_command::cmd_route(&story, &start, &args[2..]),
        Some("stats") => stats_command::cmd_stats(&story, &start),
        _ => print_usage(),
    }
}

fn print_usage() {
    println!("使い方: dialogue-engine <play|validate|map|route|stats> [引数]");
    println!();
    println!("  play                  対話プレイ (stdinから選択肢番号を入力)");
    println!("  play --script 1,2,1   選択列を指定した自動プレイ (1始まりの選択肢番号)");
    println!("  validate              シナリオ構造を検証する");
    println!("  map                   mermaid flowchart を出力する");
    println!("  route <ending名>      指定エンディングへの最短ルートを表示する");
    println!("  stats                 シーン数・分岐数などの統計を表示する");
}
