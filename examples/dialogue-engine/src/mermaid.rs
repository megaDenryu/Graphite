//! `map` サブコマンド向けの mermaid flowchart 出力。

use crate::schema::{DialogueGraph, EndingId, SceneId};

// シナリオ全体を mermaid の `flowchart` 記法で出力する。
// Scene は矩形 (`id["..."]`)、Ending はスタジアム形状 (`id{{"..."}}`) で
// 区別する。選択肢ラベルは辺ラベルとして、finale 辺は破線矢印で表現する。
pub fn to_mermaid(schema: &DialogueGraph::Graph) -> String {
    let mut out = String::new();
    out.push_str("flowchart TD\n");

    let mut scene_ids: Vec<&SceneId> = schema.scene_ids().collect();
    scene_ids.sort();
    for id in &scene_ids {
        let scene = schema
            .scene_by_id(id)
            .expect("scene_ids() が返すキーは必ず scene_by_id() で引ける");
        out.push_str(&format!(
            "    {}[\"{}: {}\"]\n",
            mermaid_id(&id.0),
            escape(&scene.speaker),
            escape(&truncate(&scene.text, 18))
        ));
    }

    let mut ending_ids: Vec<&EndingId> = schema.ending_ids().collect();
    ending_ids.sort();
    for id in &ending_ids {
        let ending = schema
            .ending_by_id(id)
            .expect("ending_ids() が返すキーは必ず ending_by_id() で引ける");
        out.push_str(&format!(
            "    {}{{{{\"{}\"}}}}\n",
            mermaid_id(&id.0),
            escape(&ending.title)
        ));
    }

    let mut choice_edges: Vec<(&SceneId, &SceneId, &str)> = schema
        .choice_iter()
        .map(|edge| {
            (
                edge.scene().id(),
                edge.next().id(),
                edge.payload().label.as_str(),
            )
        })
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

    let mut finale_edges: Vec<(&SceneId, &EndingId)> = schema
        .finale_iter()
        .map(|edge| (edge.scene().id(), edge.ending().id()))
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

// mermaid のノードIDとして使う文字列を作る。シナリオのキーは英数字+
// アンダースコアのみを使う運用なのでほぼそのまま通すが、念のため
// mermaid が誤解釈しうる記号を `_` に置き換える。
fn mermaid_id(raw: &str) -> String {
    raw.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

// mermaid のラベル文字列中で `"` があると壊れるため `'` に、改行は空白に
// 潰す。
fn escape(text: &str) -> String {
    text.replace('"', "'").replace(['\n', '\r'], " ")
}

// 文字数ベースで truncate する (UTF-8 のバイト境界を考慮するため
// `chars()` を使う)。
fn truncate(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    let mut s: String = text.chars().take(max_chars).collect();
    s.push('…');
    s
}

#[cfg(test)]
mod tests {
    use super::to_mermaid;
    use crate::story::build_story;

    #[test]
    fn mermaidに全シーンと全エンディングが出力される() {
        let story = build_story().expect("本編シナリオは構築に成功するはず");
        let mermaid = to_mermaid(&story);

        assert!(mermaid.starts_with("flowchart TD\n"));
        assert!(mermaid.contains("start["));
        assert!(mermaid.contains("ending_evacuate{{"));
        assert!(mermaid.contains("-.->|finale|"));
    }
}
