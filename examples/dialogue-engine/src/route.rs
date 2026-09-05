//! `route <ending名>` サブコマンド向けの最短ルート探索。
//!
//! `DialogueGraph::scene_graph()` (choice 辺だけを射影した汎用
//! `graphite::Graph<SceneId, String, SceneId>`) の `path`/`edge_weight` を使う。

use crate::schema::{DialogueGraph, EndingId, SceneId};

// `start` から `ending` への最短ルートを
// `(通過シーンキー, 次のシーンへ進むために選ぶべき選択肢ラベル)` の列で
// 返す。最後の要素 (finale するシーン) のラベルは `None` になる。
// 同じエンディングに複数のシーンから finale されている場合は最短のものを
// 採用する。到達不能なら `None`。
pub fn route_to_ending(
    schema: &DialogueGraph::Graph,
    start: &SceneId,
    ending: &EndingId,
) -> Option<Vec<(SceneId, Option<String>)>> {
    let scene_graph = schema.scene_graph();

    let mut best: Option<Vec<SceneId>> = None;
    for edge in schema.finale_iter() {
        let scene_id = edge.scene().id();
        let e = edge.ending().id();
        if e != ending {
            continue;
        }
        if let Some(path) = scene_graph.path(start, scene_id) {
            let path: Vec<SceneId> = path.into_iter().cloned().collect();
            if best.as_ref().is_none_or(|b| path.len() < b.len()) {
                best = Some(path);
            }
        }
    }

    let path = best?;
    let mut result = Vec::with_capacity(path.len());
    for i in 0..path.len() {
        let label = if i + 1 < path.len() {
            scene_graph.edge_weight(&path[i], &path[i + 1]).cloned()
        } else {
            None
        };
        result.push((path[i].clone(), label));
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::route_to_ending;
    use crate::schema::{EndingId, SceneId};
    use crate::story::{build_story, start_scene_id};

    #[test]
    fn route_は最短経路と選択肢ラベルの列を返す() {
        let story = build_story().expect("本編シナリオは構築に成功するはず");
        let route = route_to_ending(
            &story,
            &start_scene_id(),
            &EndingId("ending_evacuate".to_string()),
        )
        .expect("ending_evacuate へは到達可能なはず");

        assert_eq!(route.first().unwrap().0, start_scene_id());
        assert_eq!(route.last().unwrap().0, SceneId("shuttle_bay".to_string()));
        assert!(route.last().unwrap().1.is_none());
        // 最後以外は全て選択肢ラベルを持つ。
        assert!(route[..route.len() - 1].iter().all(|(_, l)| l.is_some()));
    }
}
