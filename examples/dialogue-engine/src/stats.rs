//! `stats` サブコマンド向けのシナリオ統計。

use std::collections::{HashMap, HashSet};

use crate::schema::{DialogueGraph, SceneId};

/// `stats` サブコマンドの集計結果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stats {
    pub scene_count: usize,
    pub ending_count: usize,
    /// choice 辺の総数 (= 分岐選択肢の総数)。
    pub choice_count: usize,
    /// 合流点の数 (2本以上の異なる choice 辺から到達されるシーンの数)。
    pub convergence_count: usize,
    /// `(到達可能な各エンディングのタイトル, そこへの最短ルート長)`。
    /// グラフには循環があるため「最長ルート」はループ回数を増やせば無限に
    /// 伸ばせてしまい定義できない。代わりに「各エンディングへの最短経路
    /// 長」の最小値・最大値を「一番近いエンディング/一番遠いエンディング」
    /// として報告する。
    pub shortest_routes: Vec<(String, usize)>,
}

impl Stats {
    /// 最短ルート長が最も短いエンディングまでの長さ (シーン数)。
    pub fn shortest_route_len(&self) -> Option<usize> {
        self.shortest_routes.iter().map(|(_, n)| *n).min()
    }

    /// 最短ルート長が最も長いエンディングまでの長さ (シーン数)。
    pub fn longest_shortest_route_len(&self) -> Option<usize> {
        self.shortest_routes.iter().map(|(_, n)| *n).max()
    }
}

/// `start` を起点にシナリオの統計を計算する。
pub fn compute_stats(schema: &DialogueGraph::Graph, start: &SceneId) -> Stats {
    let scene_graph = schema.scene_graph();

    let scene_count = schema.scene_ids().count();
    let ending_count = schema.ending_ids().count();
    let choice_count = schema.choice_len();

    // 合流点: ある終点シーンへ、異なる始点シーンから2本以上の choice 辺が
    // 入っているシーン。
    let mut incoming: HashMap<SceneId, HashSet<SceneId>> = HashMap::new();
    for edge in schema.choice_iter() {
        incoming
            .entry(edge.next().id().clone())
            .or_default()
            .insert(edge.scene().id().clone());
    }
    let convergence_count = incoming.values().filter(|froms| froms.len() >= 2).count();

    let mut shortest_routes: Vec<(String, usize)> = Vec::new();
    for edge in schema.finale_iter() {
        let scene_id = edge.scene().id();
        let ending_id = edge.ending().id();
        if let Some(path) = scene_graph.path(start, scene_id) {
            let ending = schema
                .ending_by_id(ending_id)
                .expect("finale_iter() が返す EndingId は必ず ending_by_id() で引ける");
            shortest_routes.push((ending.title.clone(), path.len()));
        }
    }
    shortest_routes.sort();

    Stats {
        scene_count,
        ending_count,
        choice_count,
        convergence_count,
        shortest_routes,
    }
}

#[cfg(test)]
mod tests {
    use super::compute_stats;
    use crate::story::{build_story, start_scene_id};

    #[test]
    fn statsは分岐と合流を数える() {
        let story = build_story().expect("本編シナリオは構築に成功するはず");
        let stats = compute_stats(&story, &start_scene_id());

        assert_eq!(stats.scene_count, 30);
        assert_eq!(stats.ending_count, 4);
        assert!(stats.convergence_count >= 3, "central 等の合流点があるはず");
        assert_eq!(stats.shortest_routes.len(), 4);
        assert!(stats.shortest_route_len().unwrap() <= stats.longest_shortest_route_len().unwrap());
    }
}
