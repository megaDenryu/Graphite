//! ビルドパイプラインとしての意味の妥当性検査。
//!
//! 孤児成果物・produce競合・タスク依存の循環の3種を検出する。

use std::collections::HashMap;

use graphite::CycleError;

use super::domain_issue::DomainIssue;
use super::task_dependency_graph::task_dependency_graph;
use crate::schema::{ArtifactId, BuildPipeline, TaskId};

/// ドメイン検証を実行する。図式適合 (`BuildPipeline::Graph::create` 時点) は既に
/// 通っている前提で、意味的な妥当性だけを検査する。
pub fn validate(g: &BuildPipeline::Graph) -> Vec<DomainIssue> {
    let mut issues = Vec::new();

    let mut producers_of: HashMap<&ArtifactId, Vec<&TaskId>> = HashMap::new();
    for edge in g.produces_iter() {
        producers_of
            .entry(edge.artifact().id())
            .or_default()
            .push(edge.task().id());
    }
    let mut consumers_of: HashMap<&ArtifactId, Vec<&TaskId>> = HashMap::new();
    for edge in g.consumes_iter() {
        consumers_of
            .entry(edge.artifact().id())
            .or_default()
            .push(edge.task().id());
    }

    // 1. 孤児成果物: consume されているのに produce するタスクが無い。
    let mut orphan_artifacts: Vec<&ArtifactId> = consumers_of
        .keys()
        .copied()
        .filter(|a| !producers_of.contains_key(*a))
        .collect();
    orphan_artifacts.sort_by(|a, b| a.0.cmp(&b.0));
    for artifact in orphan_artifacts {
        let mut consumers: Vec<TaskId> =
            consumers_of[artifact].iter().map(|&t| t.clone()).collect();
        consumers.sort_by(|a, b| a.0.cmp(&b.0));
        issues.push(DomainIssue::OrphanArtifact {
            artifact: artifact.clone(),
            consumers,
        });
    }

    // 2. produce競合: 同じ成果物を複数タスクが生成している。
    let mut conflicting: Vec<&ArtifactId> = producers_of
        .iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(&k, _)| k)
        .collect();
    conflicting.sort_by(|a, b| a.0.cmp(&b.0));
    for artifact in conflicting {
        let mut producers: Vec<TaskId> =
            producers_of[artifact].iter().map(|&t| t.clone()).collect();
        producers.sort_by(|a, b| a.0.cmp(&b.0));
        issues.push(DomainIssue::ConflictingProducers {
            artifact: artifact.clone(),
            producers,
        });
    }

    // 3. タスク依存の循環 (汎用 Graph<TaskId> へ射影して has_cycle 相当の検査)。
    let dep_graph = task_dependency_graph(g);
    if let Err(CycleError { cycle }) = dep_graph.topological_sort() {
        issues.push(DomainIssue::CyclicDependency { cycle });
    }

    issues
}

#[cfg(test)]
mod tests;
