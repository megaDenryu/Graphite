//! CLI 出力のフォーマットを集約するモジュール。
//! `analysis` の計算結果を人間に読める文字列に変換するだけで、
//! 計算ロジックは一切持たない。
//!
//! このファイルは表形式の出力を持ち、mermaid 図の出力は `graph_diagram` が持つ。

mod graph_diagram;

pub use graph_diagram::mermaid;

use crate::analysis::{CriticalPath, DomainIssue, Wave};
use crate::schema::BuildPipeline;

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
pub fn format_critical_path(cp: &CriticalPath, g: &BuildPipeline::Graph) -> String {
    if cp.path.is_empty() {
        return "クリティカルパス: タスクがありません".to_string();
    }

    let mut out = String::new();
    out.push_str("クリティカルパス (依存関係上、最も時間がかかる経路):\n");
    for (i, task_id) in cp.path.iter().enumerate() {
        let secs = g.task_by_id(task_id).map(|t| t.secs).unwrap_or(0);
        if i > 0 {
            out.push_str("  -> ");
        } else {
            out.push_str("  ");
        }
        out.push_str(&format!("{} ({secs}s)", task_id.0));
    }
    out.push('\n');
    out.push_str(&format!("\n合計時間: {}秒\n", cp.total_secs));
    out.push_str(&format!(
        "全タスクの所要時間合計 (総作業量): {}秒\n",
        cp.total_work_secs
    ));
    out.push_str(&format!(
        "全体並列度 (総作業量 / クリティカルパス長): {:.2}倍\n",
        cp.parallelism()
    ));
    out
}
#[cfg(test)]
mod tests {
    use super::format_domain_issues;

    #[test]
    fn 違反なしのメッセージが出る() {
        let out = format_domain_issues(&[]);
        assert!(out.contains("違反なし"));
    }
}
