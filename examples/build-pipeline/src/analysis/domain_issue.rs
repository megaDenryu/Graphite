//! `validate` サブコマンドが報告するドメイン違反の種類と、その表示。

use std::fmt;

use crate::schema::{ArtifactId, TaskId};

/// `validate` サブコマンドが報告するドメイン違反 1 件。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainIssue {
    /// 誰も produce しない artifact を consume しているタスクがある。
    OrphanArtifact {
        artifact: ArtifactId,
        consumers: Vec<TaskId>,
    },
    /// 同じ artifact を複数のタスクが produce している (競合)。
    ConflictingProducers {
        artifact: ArtifactId,
        producers: Vec<TaskId>,
    },
    /// タスク依存グラフに循環がある。`cycle` は循環を構成するタスク列
    /// (`cycle[0]` から辿って `cycle[0]` に戻る)。
    CyclicDependency { cycle: Vec<TaskId> },
}

impl fmt::Display for DomainIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainIssue::OrphanArtifact {
                artifact,
                consumers,
            } => write!(
                f,
                "孤児成果物: {} を produce するタスクが存在しないのに、{} が consume しています",
                artifact.0,
                consumers
                    .iter()
                    .map(|t| t.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            DomainIssue::ConflictingProducers {
                artifact,
                producers,
            } => write!(
                f,
                "produce競合: {} を複数タスクが生成しています ({})",
                artifact.0,
                producers
                    .iter()
                    .map(|t| t.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            DomainIssue::CyclicDependency { cycle } => write!(
                f,
                "循環依存: {} を経由する依存の循環が検出されました",
                cycle
                    .iter()
                    .map(|t| t.0.as_str())
                    .collect::<Vec<_>>()
                    .join(" -> ")
            ),
        }
    }
}
