//! グラフ全体を mermaid flowchart として描く。

use crate::schema::BuildPipeline;

/// 識別子を mermaid ノードIDとして安全な文字列へ変換する
/// (英数字と `_` 以外を `_` に置換する素朴な実装)。
fn sanitize_id(raw: &str) -> String {
    raw.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/// グラフ全体を mermaid flowchart として出力する。
/// Task は矩形 (`["..."]`)、Artifact は円柱形 (`[("...")]`、
/// 「保存された成果物」を表す慣用のノード形状) で描き分ける。
/// `consumes` は「成果物がタスクへ流れ込む」という読みやすさを優先して
/// 矢印を Artifact -> Task 方向 (スキーマ上の `from`/`to` とは逆) に描く。
pub fn mermaid(g: &BuildPipeline::Graph) -> String {
    let mut out = String::new();
    out.push_str("flowchart TD\n");

    let mut task_ids: Vec<_> = g.task_ids().collect();
    task_ids.sort_by(|a, b| a.0.cmp(&b.0));
    for id in &task_ids {
        let task = g
            .task_by_id(id)
            .expect("g.task_ids()由来のキーは必ず存在する");
        out.push_str(&format!(
            "    T_{}[\"{} ({}s)\"]\n",
            sanitize_id(&id.0),
            task.name,
            task.secs
        ));
    }

    let mut artifact_ids: Vec<_> = g.artifact_ids().collect();
    artifact_ids.sort_by(|a, b| a.0.cmp(&b.0));
    for id in &artifact_ids {
        let artifact = g
            .artifact_by_id(id)
            .expect("g.artifact_ids()由来のキーは必ず存在する");
        out.push_str(&format!(
            "    A_{}[(\"{}\")]\n",
            sanitize_id(&id.0),
            artifact.path
        ));
    }

    let mut produces: Vec<(String, String)> = g
        .produces_iter()
        .map(|edge| {
            (
                sanitize_id(&edge.task().id().0),
                sanitize_id(&edge.artifact().id().0),
            )
        })
        .collect();
    produces.sort();
    for (t, a) in produces {
        out.push_str(&format!("    T_{t} -->|produces| A_{a}\n"));
    }

    let mut consumes: Vec<(String, String)> = g
        .consumes_iter()
        .map(|edge| {
            (
                sanitize_id(&edge.task().id().0),
                sanitize_id(&edge.artifact().id().0),
            )
        })
        .collect();
    consumes.sort();
    for (t, a) in consumes {
        out.push_str(&format!("    A_{a} -->|consumes| T_{t}\n"));
    }

    out
}
#[cfg(test)]
mod tests {
    use super::mermaid;
    use crate::builder::build_graph;
    use crate::parser::parse;
    use crate::schema::BuildPipeline;

    fn graph_from(input: &str) -> BuildPipeline::Graph {
        let parsed = parse(input).unwrap();
        build_graph(&parsed).unwrap()
    }

    #[test]
    fn mermaid出力にノードと辺が含まれる() {
        let g = graph_from(
            "task build: cargo build (10s)
build produces target/a
task test: cargo test (5s)
test consumes target/a
",
        );
        let out = mermaid(&g);
        assert!(out.starts_with(
            "flowchart TD
"
        ));
        assert!(out.contains("T_build["));
        assert!(out.contains("A_target_a[("));
        assert!(out.contains("T_build -->|produces| A_target_a"));
        assert!(out.contains("A_target_a -->|consumes| T_test"));
    }
}
