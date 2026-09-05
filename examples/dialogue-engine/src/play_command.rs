//! `play` サブコマンド — シナリオを対話または選択列でプレイする。

use dialogue_engine::engine;
use dialogue_engine::schema::{DialogueGraph, SceneId};

pub fn cmd_play(story: &DialogueGraph::Graph, start: &SceneId, rest: &[String]) {
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

// stdin から選択肢番号 (1始まり) を読む。範囲外・非数値な入力は再入力を促す。
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
