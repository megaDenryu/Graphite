//! `route <ending名>` サブコマンド — 指定エンディングへの最短ルートを表示する。

use dialogue_engine::route;
use dialogue_engine::schema::{DialogueGraph, EndingId, SceneId};

pub fn cmd_route(story: &DialogueGraph::Graph, start: &SceneId, rest: &[String]) {
    let Some(ending_key) = rest.first() else {
        eprintln!("使い方: dialogue-engine route <ending名>");
        eprintln!("利用可能なエンディング: {}", available_endings(story));
        std::process::exit(1);
    };
    let ending_id = EndingId(ending_key.clone());
    if story.ending_by_id(&ending_id).is_none() {
        eprintln!("未知のエンディングです: {ending_key}");
        eprintln!("利用可能なエンディング: {}", available_endings(story));
        std::process::exit(1);
    }

    match route::route_to_ending(story, start, &ending_id) {
        Some(steps) => {
            for (i, (scene_id, label)) in steps.iter().enumerate() {
                let scene = story
                    .scene_by_id(scene_id)
                    .expect("route が返すキーは必ず scene_by_id() で引ける");
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

fn available_endings(story: &DialogueGraph::Graph) -> String {
    let mut ids: Vec<String> = story.ending_ids().map(|id| id.0.clone()).collect();
    ids.sort();
    ids.join(", ")
}
