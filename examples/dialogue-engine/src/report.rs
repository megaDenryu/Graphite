//! `map`/`route`/`stats` サブコマンド向けのレポート生成。
//!
//! いずれも `DialogueGraph::scene_graph()` (choice 辺だけを射影した汎用
//! `graphite::Graph<SceneId, String, SceneId>`) の `path`/`edge_weight` を
//! 使い回す。

use std::collections::{HashMap, HashSet};

use crate::schema::{Choice, DialogueGraph, DialogueGraphNode, Ending, EndingId, Finale, Scene, SceneId};

// ============================================================
// map: mermaid flowchart 出力
// ============================================================

/// シナリオ全体を mermaid の `flowchart` 記法で出力する。
/// Scene は矩形 (`id["..."]`)、Ending はスタジアム形状 (`id{{"..."}}`) で
/// 区別する。選択肢ラベルは辺ラベルとして、finale 辺は破線矢印で表現する。
pub fn to_mermaid(schema: &DialogueGraph) -> String {
    let mut out = String::new();
    out.push_str("flowchart TD\n");

    let mut scene_ids: Vec<&SceneId> = Scene::ids(schema).collect();
    scene_ids.sort();
    for id in &scene_ids {
        let scene = Scene::get(schema, id)
            .expect("Scene::ids() が返すキーは必ず Scene::get() で引ける");
        out.push_str(&format!(
            "    {}[\"{}: {}\"]\n",
            mermaid_id(&id.0),
            escape(&scene.speaker),
            escape(&truncate(&scene.text, 18))
        ));
    }

    let mut ending_ids: Vec<&EndingId> = Ending::ids(schema).collect();
    ending_ids.sort();
    for id in &ending_ids {
        let ending = Ending::get(schema, id)
            .expect("Ending::ids() が返すキーは必ず Ending::get() で引ける");
        out.push_str(&format!(
            "    {}{{{{\"{}\"}}}}\n",
            mermaid_id(&id.0),
            escape(&ending.title)
        ));
    }

    let mut choice_edges: Vec<(&SceneId, &SceneId, &str)> = Choice::iter(schema)
        .map(|(_key, edge)| (edge.from(), edge.to(), edge.payload().label.as_str()))
        .collect();
    choice_edges.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1)));
    for (from, to, label) in choice_edges {
        out.push_str(&format!(
            "    {} -->|{}| {}\n",
            mermaid_id(&from.0),
            escape(label),
            mermaid_id(&to.0)
        ));
    }

    let mut finale_edges: Vec<(&SceneId, &EndingId)> = Finale::iter(schema)
        .map(|(_key, edge)| (edge.from(), edge.to()))
        .collect();
    finale_edges.sort();
    for (from, to) in finale_edges {
        out.push_str(&format!(
            "    {} -.->|finale| {}\n",
            mermaid_id(&from.0),
            mermaid_id(&to.0)
        ));
    }

    out
}

/// mermaid のノードIDとして使う文字列を作る。シナリオのキーは英数字+
/// アンダースコアのみを使う運用なのでほぼそのまま通すが、念のため
/// mermaid が誤解釈しうる記号を `_` に置き換える。
fn mermaid_id(raw: &str) -> String {
    raw.chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
        .collect()
}

/// mermaid のラベル文字列中で `"` があると壊れるため `'` に、改行は空白に
/// 潰す。
fn escape(text: &str) -> String {
    text.replace('"', "'").replace(['\n', '\r'], " ")
}

/// 文字数ベースで truncate する (UTF-8 のバイト境界を考慮するため
/// `chars()` を使う)。
fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut s: String = text.chars().take(max_chars).collect();
    s.push('…');
    s
}

// ============================================================
// stats: シナリオ統計
// ============================================================

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
pub fn compute_stats(schema: &DialogueGraph, start: &SceneId) -> Stats {
    let scene_graph = schema.scene_graph();

    let scene_count = Scene::ids(schema).count();
    let ending_count = Ending::ids(schema).count();
    let choice_count = Choice::len(schema);

    // 合流点: ある終点シーンへ、異なる始点シーンから2本以上の choice 辺が
    // 入っているシーン。
    let mut incoming: HashMap<SceneId, HashSet<SceneId>> = HashMap::new();
    for (_key, edge) in Choice::iter(schema) {
        incoming
            .entry(edge.to().clone())
            .or_default()
            .insert(edge.from().clone());
    }
    let convergence_count = incoming.values().filter(|froms| froms.len() >= 2).count();

    let mut shortest_routes: Vec<(String, usize)> = Vec::new();
    for (_key, edge) in Finale::iter(schema) {
        let scene_id = edge.from();
        let ending_id = edge.to();
        if let Some(path) = scene_graph.path(start, scene_id) {
            let ending = Ending::get(schema, ending_id)
                .expect("Finale::iter() が返す EndingId は必ず Ending::get() で引ける");
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

// ============================================================
// route: 指定エンディングへの最短ルート
// ============================================================

/// `route <ending名>` サブコマンド: `start` から `ending` への最短ルートを
/// `(通過シーンキー, 次のシーンへ進むために選ぶべき選択肢ラベル)` の列で
/// 返す。最後の要素 (finale するシーン) のラベルは `None` になる。
/// 同じエンディングに複数のシーンから finale されている場合は最短のものを
/// 採用する。到達不能なら `None`。
pub fn route_to_ending(
    schema: &DialogueGraph,
    start: &SceneId,
    ending: &EndingId,
) -> Option<Vec<(SceneId, Option<String>)>> {
    let scene_graph = schema.scene_graph();

    let mut best: Option<Vec<SceneId>> = None;
    for (_key, edge) in Finale::iter(schema) {
        let scene_id = edge.from();
        let e = edge.to();
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
    use super::*;
    use crate::schema::{build_story, start_scene_id};

    #[test]
    fn mermaidに全シーンと全エンディングが出力される() {
        let story = build_story().expect("本編シナリオは構築に成功するはず");
        let mermaid = to_mermaid(&story);

        assert!(mermaid.starts_with("flowchart TD\n"));
        assert!(mermaid.contains("start["));
        assert!(mermaid.contains("ending_evacuate{{"));
        assert!(mermaid.contains("-.->|finale|"));
    }

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

    #[test]
    fn route_は最短経路と選択肢ラベルの列を返す() {
        let story = build_story().expect("本編シナリオは構築に成功するはず");
        let route = route_to_ending(
            &story,
            &start_scene_id(),
            &crate::schema::EndingId("ending_evacuate".to_string()),
        )
        .expect("ending_evacuate へは到達可能なはず");

        assert_eq!(route.first().unwrap().0, start_scene_id());
        assert_eq!(route.last().unwrap().0, SceneId("shuttle_bay".to_string()));
        assert!(route.last().unwrap().1.is_none());
        // 最後以外は全て選択肢ラベルを持つ。
        assert!(route[..route.len() - 1].iter().all(|(_, l)| l.is_some()));
    }
}
