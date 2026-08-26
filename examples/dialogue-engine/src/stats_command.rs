//! `stats` サブコマンド — シーン数・分岐数・最短ルート長を表示する。

use dialogue_engine::schema::{DialogueGraph, SceneId};
use dialogue_engine::stats;

pub fn cmd_stats(story: &DialogueGraph::Graph, start: &SceneId) {
    let stats = stats::compute_stats(story, start);
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
