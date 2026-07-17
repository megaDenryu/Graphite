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

use dialogue_engine::{engine, report, schema, validate};
use schema::{DialogueGraph, DialogueGraphNode, Ending, EndingId, Scene, SceneId};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let subcommand = args.get(1).map(String::as_str);

    let story = match schema::build_story() {
        Ok(story) => story,
        Err(violation) => {
            eprintln!("シナリオの構築に失敗しました: {violation}");
            std::process::exit(1);
        }
    };
    let start = schema::start_scene_id();

    match subcommand {
        Some("play") => cmd_play(&story, &start, &args[2..]),
        Some("validate") => cmd_validate(&story, &start),
        Some("map") => println!("{}", report::to_mermaid(&story)),
        Some("route") => cmd_route(&story, &start, &args[2..]),
        Some("stats") => cmd_stats(&story, &start),
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

fn cmd_play(story: &DialogueGraph, start: &SceneId, rest: &[String]) {
    let scripted: Option<Vec<usize>> = rest
        .iter()
        .position(|a| a == "--script")
        .and_then(|i| rest.get(i + 1))
        .map(|s| {
            s.split(',')
                .filter_map(|n| n.trim().parse::<usize>().ok())
                .map(|n| n.saturating_sub(1)) // 表示は1始まり、内部は0始まり
                .collect()
        });

    let outcome = if let Some(script) = scripted {
        engine::play(story, start, engine::scripted_choices(script), |line| {
            println!("{line}")
        })
    } else {
        engine::play(
            story,
            start,
            |labels: &[String]| read_choice_from_stdin(labels),
            |line| println!("{line}"),
        )
    };

    println!();
    println!("--- プレイ終了 ---");
    println!("既訪シーン数: {}", outcome.unique_scene_count());
    match &outcome.ending_title {
        Some(title) => println!("到達したエンディング: {title}"),
        None => println!("エンディングに到達できませんでした。"),
    }
}

/// stdin から選択肢番号 (1始まり) を読む。範囲外・非数値な入力は再入力を促す。
fn read_choice_from_stdin(labels: &[String]) -> usize {
    use std::io::Write;
    loop {
        println!();
        for (i, label) in labels.iter().enumerate() {
            println!("  {}. {}", i + 1, label);
        }
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_err() {
            return 0;
        }
        if let Ok(n) = line.trim().parse::<usize>() {
            if n >= 1 && n <= labels.len() {
                return n - 1;
            }
        }
        println!("番号を正しく入力してください (1-{}).", labels.len());
    }
}

fn cmd_validate(story: &DialogueGraph, start: &SceneId) {
    let report = validate::validate(story, start);
    if report.is_clean() {
        println!("検証結果: 問題なし (全シーン到達可能・デッドエンド無し・全エンディング到達可能)");
        return;
    }

    println!("検証結果: 問題あり");
    if !report.unreachable_scenes.is_empty() {
        println!("到達不能シーン:");
        for id in &report.unreachable_scenes {
            println!("  - {}", id.0);
        }
    }
    if !report.dead_end_scenes.is_empty() {
        println!("デッドエンドシーン (選択肢もfinaleも無い):");
        for id in &report.dead_end_scenes {
            println!("  - {}", id.0);
        }
    }
    if !report.unreachable_endings.is_empty() {
        println!("到達不能なエンディング:");
        for id in &report.unreachable_endings {
            println!("  - {}", id.0);
        }
    }
    if !report.trapped_scenes.is_empty() {
        println!("どのエンディングにも到達できない閉じたループ:");
        for id in &report.trapped_scenes {
            println!("  - {}", id.0);
        }
    }
}

fn cmd_route(story: &DialogueGraph, start: &SceneId, rest: &[String]) {
    let Some(ending_key) = rest.first() else {
        eprintln!("使い方: dialogue-engine route <ending名>");
        eprintln!("利用可能なエンディング: {}", available_endings(story));
        std::process::exit(1);
    };
    let ending_id = EndingId(ending_key.clone());
    if Ending::get(story, &ending_id).is_none() {
        eprintln!("未知のエンディングです: {ending_key}");
        eprintln!("利用可能なエンディング: {}", available_endings(story));
        std::process::exit(1);
    }

    match report::route_to_ending(story, start, &ending_id) {
        Some(steps) => {
            for (i, (scene_id, label)) in steps.iter().enumerate() {
                let scene = Scene::get(story, scene_id)
                    .expect("route が返すキーは必ず Scene::get() で引ける");
                match label {
                    Some(l) => {
                        println!("{}. [{}] {} --({})-->", i + 1, scene.speaker, scene_id.0, l)
                    }
                    None => println!("{}. [{}] {} (finale)", i + 1, scene.speaker, scene_id.0),
                }
            }
        }
        None => {
            println!("{ending_key} へのルートは見つかりませんでした (到達不能です)。");
        }
    }
}

fn available_endings(story: &DialogueGraph) -> String {
    let mut ids: Vec<String> = Ending::ids(story).map(|id| id.0.clone()).collect();
    ids.sort();
    ids.join(", ")
}

fn cmd_stats(story: &DialogueGraph, start: &SceneId) {
    let stats = report::compute_stats(story, start);
    println!("シーン数: {}", stats.scene_count);
    println!("エンディング数: {}", stats.ending_count);
    println!("選択肢 (choice辺) 数: {}", stats.choice_count);
    println!("合流点の数: {}", stats.convergence_count);
    println!();
    println!("エンディング別 最短ルート長 (シーン数):");
    for (title, len) in &stats.shortest_routes {
        println!("  - {title}: {len}");
    }
    if let (Some(min), Some(max)) = (
        stats.shortest_route_len(),
        stats.longest_shortest_route_len(),
    ) {
        println!();
        println!("最短ルート長 (最も近いエンディングまで): {min}");
        println!("最長ルート長 (最も遠いエンディングの最短経路まで): {max}");
        println!(
            "(注: グラフには循環があるため「純粋な最長経路」は無限に伸ばせて定義できません。ここでは各エンディングへの最短経路長の最大値を代用しています)"
        );
    }
}
