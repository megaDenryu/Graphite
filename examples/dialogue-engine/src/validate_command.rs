//! `validate` サブコマンド — シナリオ構造の検証結果を表示する。

use dialogue_engine::schema::{DialogueGraph, SceneId};
use dialogue_engine::validate;

pub fn cmd_validate(story: &DialogueGraph::Graph, start: &SceneId) {
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
