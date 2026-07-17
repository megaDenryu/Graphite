//! CLI 出力のフォーマット (表形式・mermaid) を集約するモジュール。
//! `analysis.rs` の計算結果を人間に読める文字列に変換するだけで、
//! 計算ロジックは一切持たない。

use crate::analysis::{CriticalPath, DomainIssue, Wave};
use crate::schema::{Artifact, BuildPipeline, BuildPipelineNode, Consumes, Produces, Task};

/// `validate` サブコマンドの結果表示。
pub fn format_domain_issues(issues: &[DomainIssue]) -> String {
    if issues.is_empty() {
        return "ドメイン検証: 違反なし (孤児成果物 / produce競合 / 循環依存のいずれも検出されませんでした)".to_string();
    }
    let mut out = format!("ドメイン検証: {}件の違反を検出しました\n", issues.len());
    for (i, issue) in issues.iter().enumerate() {
        out.push_str(&format!("  [{}] {}\n", i + 1, issue));
    }
    out
}

/// `plan` サブコマンドの表形式出力。
pub fn format_plan(waves: &[Wave]) -> String {
    if waves.is_empty() {
        return "実行計画: タスクがありません".to_string();
    }

    let mut out = String::new();
    out.push_str("波  所要時間   タスク (この波の中で並列実行可能)\n");
    out.push_str("--  --------   --------------------------------\n");
    let mut total = 0u32;
    for wave in waves {
        total += wave.duration_secs;
        let names: Vec<&str> = wave.tasks.iter().map(|t| t.0.as_str()).collect();
        out.push_str(&format!(
            "{:<3} {:>6}s   {}\n",
            wave.index,
            wave.duration_secs,
            names.join(", ")
        ));
    }
    out.push_str(&format!(
        "\n波の合計 (逐次実行した場合の下限見積り): {total}秒 / {}波\n",
        waves.len()
    ));
    out
}

/// `critical-path` サブコマンドの出力。
pub fn format_critical_path(cp: &CriticalPath, g: &BuildPipeline) -> String {
    if cp.path.is_empty() {
        return "クリティカルパス: タスクがありません".to_string();
    }

    let mut out = String::new();
    out.push_str("クリティカルパス (依存関係上、最も時間がかかる経路):\n");
    for (i, task_id) in cp.path.iter().enumerate() {
        let secs = Task::get(g, task_id).map(|t| t.secs).unwrap_or(0);
        if i > 0 {
            out.push_str("  -> ");
        } else {
            out.push_str("  ");
        }
        out.push_str(&format!("{} ({secs}s)", task_id.0));
    }
    out.push('\n');
    out.push_str(&format!("\n合計時間: {}秒\n", cp.total_secs));
    out.push_str(&format!("全タスクの所要時間合計 (総作業量): {}秒\n", cp.total_work_secs));
    out.push_str(&format!(
        "全体並列度 (総作業量 / クリティカルパス長): {:.2}倍\n",
        cp.parallelism()
    ));
    out
}

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
pub fn mermaid(g: &BuildPipeline) -> String {
    let mut out = String::new();
    out.push_str("flowchart TD\n");

    let mut task_ids: Vec<_> = Task::ids(g).collect();
    task_ids.sort_by(|a, b| a.0.cmp(&b.0));
    for id in &task_ids {
        let task = Task::get(g, id).expect("Task::ids(g)由来のキーは必ず存在する");
        out.push_str(&format!(
            "    T_{}[\"{} ({}s)\"]\n",
            sanitize_id(&id.0),
            task.name,
            task.secs
        ));
    }

    let mut artifact_ids: Vec<_> = Artifact::ids(g).collect();
    artifact_ids.sort_by(|a, b| a.0.cmp(&b.0));
    for id in &artifact_ids {
        let artifact =
            Artifact::get(g, id).expect("Artifact::ids(g)由来のキーは必ず存在する");
        out.push_str(&format!(
            "    A_{}[(\"{}\")]\n",
            sanitize_id(&id.0),
            artifact.path
        ));
    }

    let mut produces: Vec<(String, String)> = Produces::iter(g)
        .map(|(_id, edge)| (sanitize_id(&edge.from().0), sanitize_id(&edge.to().0)))
        .collect();
    produces.sort();
    for (t, a) in produces {
        out.push_str(&format!("    T_{t} -->|produces| A_{a}\n"));
    }

    let mut consumes: Vec<(String, String)> = Consumes::iter(g)
        .map(|(_id, edge)| (sanitize_id(&edge.from().0), sanitize_id(&edge.to().0)))
        .collect();
    consumes.sort();
    for (t, a) in consumes {
        out.push_str(&format!("    A_{a} -->|consumes| T_{t}\n"));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::build_graph;
    use crate::parser::parse;

    fn graph_from(input: &str) -> BuildPipeline {
        let parsed = parse(input).unwrap();
        build_graph(&parsed).unwrap()
    }

    #[test]
    fn mermaid出力にノードと辺が含まれる() {
        let g = graph_from(
            "\
task build: cargo build (10s)
build produces target/a
task test: cargo test (5s)
test consumes target/a
",
        );
        let out = mermaid(&g);
        assert!(out.starts_with("flowchart TD\n"));
        assert!(out.contains("T_build["));
        assert!(out.contains("A_target_a[("));
        assert!(out.contains("T_build -->|produces| A_target_a"));
        assert!(out.contains("A_target_a -->|consumes| T_test"));
    }

    #[test]
    fn 違反なしのメッセージが出る() {
        let out = format_domain_issues(&[]);
        assert!(out.contains("違反なし"));
    }
}
