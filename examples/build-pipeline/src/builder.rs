//! パース結果 (`ParsedPipeline`) から `graph_schema!` 製の `BuildPipeline`
//! グラフを組み立てる。
//!
//! `pipeline.txt` はタスクだけを `task` 行で明示宣言し、成果物 (`Artifact`)
//! は `produces`/`consumes` 行に現れるパスから暗黙的に導出する設計にしている
//! (成果物ごとに専用の宣言行を書かせるのは冗長なため)。そのため、まず全
//! エッジ行から distinct なパス集合を集め、それを `Artifact` ノードとして
//! 先に登録してからエッジを積む、という 2 段階の構築になる。

use crate::parser::{EdgeKind, ParsedPipeline};
use crate::schema::{
    Artifact, ArtifactId, BuildPipeline, BuildPipelineViolation, Consumes, ConsumesId, Produces,
    ProducesId, Task, TaskId,
};
use std::collections::BTreeSet;

/// `ParsedPipeline` から `BuildPipeline` を構築する。
///
/// 図式適合検査 (`BuildPipeline::create` 内部の freeze) は以下を検出する:
/// - `task` 行の名前が重複している (`DuplicateTask`)
/// - `produces`/`consumes` 行が指す `task` 名が未宣言
///   (`ProducesUnknownSource` / `ConsumesUnknownSource`。フェーズ5でエッジ
///   単位の型付きバリアントに変わった。成果物側は本関数が全パスを事前登録
///   するので `*UnknownTarget` は通常経路では発生しない)
///
/// これ以外のドメイン上の妥当性 (孤児成果物・二重生成・循環依存) は
/// 図式適合の範囲外であり、`analysis::validate` が別途検査する。
pub fn build_graph(parsed: &ParsedPipeline) -> Result<BuildPipeline, BuildPipelineViolation> {
    let mut artifact_paths: BTreeSet<&str> = BTreeSet::new();
    for edge in &parsed.edges {
        artifact_paths.insert(edge.path.as_str());
    }

    BuildPipeline::create(|b| {
        for task in &parsed.tasks {
            b.task(
                TaskId(task.name.clone()),
                Task {
                    name: task.name.clone(),
                    cmd: task.cmd.clone(),
                    secs: task.secs,
                },
            );
        }
        for path in &artifact_paths {
            b.artifact(
                ArtifactId(path.to_string()),
                Artifact {
                    path: path.to_string(),
                },
            );
        }
        for edge in &parsed.edges {
            let task_id = TaskId(edge.task_name.clone());
            let artifact_id = ArtifactId(edge.path.clone());
            // エッジキーは端点 (task_id, artifact_id) から機械的に組み立てる
            // (連番は使わない)。ProducesId/ConsumesId は種別ごとに独立した
            // KeyedTable を持つので、同じキー文字列を両方で使っても衝突しない。
            let edge_key = format!("{}::{}", task_id.0, artifact_id.0);
            match edge.kind {
                EdgeKind::Produces => {
                    b.produces(ProducesId(edge_key), Produces(task_id, artifact_id));
                }
                EdgeKind::Consumes => {
                    b.consumes(ConsumesId(edge_key), Consumes(task_id, artifact_id));
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::schema::BuildPipelineNode;

    #[test]
    fn 正常なパースからグラフを構築できる() {
        let input = "\
task build_core: cargo build -p core (120s)
build_core produces target/core.rlib

task test_core: cargo test -p core (70s)
test_core consumes target/core.rlib
";
        let parsed = parse(input).unwrap();
        let g = build_graph(&parsed).expect("構築に成功するはず");

        assert_eq!(Task::ids(&g).count(), 2);
        assert_eq!(Artifact::ids(&g).count(), 1);

        let produced = Produces::of(&g, &TaskId("build_core".to_string()));
        assert_eq!(produced.len(), 1);
        assert_eq!(produced[0].path, "target/core.rlib");
    }

    #[test]
    fn 未宣言のタスク名を参照するとunknowntask違反になる() {
        let input = "\
task build_core: cargo build -p core (120s)
typo_task produces target/core.rlib
";
        let parsed = parse(input).unwrap();
        let result = build_graph(&parsed);
        assert!(matches!(
            result,
            Err(BuildPipelineViolation::ProducesUnknownSource { .. })
        ));
    }

    #[test]
    fn 同名のtask宣言が2回あるとduplicatetask違反になる() {
        let input = "\
task build_core: cargo build -p core (120s)
task build_core: cargo build -p core --release (90s)
";
        let parsed = parse(input).unwrap();
        let result = build_graph(&parsed);
        assert!(matches!(
            result,
            Err(BuildPipelineViolation::DuplicateTask(_))
        ));
    }
}
