//! 並列実行可能なタスクの「波」への分割 (実行計画)。

use std::collections::{HashMap, HashSet};

use graphite::CycleError;

use super::task_dependency_graph::task_dependency_graph;
use crate::schema::{BuildPipeline, TaskId};

/// 並列実行可能なタスクの「波」1 つ分。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wave {
    pub index: usize,
    pub tasks: Vec<TaskId>,
    pub duration_secs: u32,
}

/// トポロジカル順序から、依存が解決済みのタスクをまとめて 1 波とする実行計画
/// を計算する (Kahn のアルゴリズムを波単位でまとめて実行するレベル分割版)。
/// 波の所要時間 = 波内タスクの `max(secs)` (無限並列ワーカーを仮定)。
pub fn plan(g: &BuildPipeline::Graph) -> Result<Vec<Wave>, CycleError<TaskId>> {
    let dep_graph = task_dependency_graph(g);
    // 循環があれば代表ノード付きで早期に報告する。
    dep_graph.topological_sort()?;

    let mut remaining: HashMap<TaskId, usize> =
        g.task_ids().map(|id| (id.clone(), 0usize)).collect();
    for id in g.task_ids() {
        for succ in dep_graph.out_neighbors(id) {
            *remaining.get_mut(succ).expect("g.succはtask_ids()由来") += 1;
        }
    }

    let mut done: HashSet<TaskId> = HashSet::new();
    let mut waves = Vec::new();

    loop {
        let mut frontier: Vec<TaskId> = remaining
            .iter()
            .filter(|(id, deg)| **deg == 0 && !done.contains(*id))
            .map(|(id, _)| id.clone())
            .collect();
        if frontier.is_empty() {
            break;
        }
        frontier.sort_by(|a, b| a.0.cmp(&b.0));

        let duration = frontier
            .iter()
            .map(|id| g.task_by_id(id).map(|t| t.secs).unwrap_or(0))
            .max()
            .unwrap_or(0);

        for id in &frontier {
            done.insert(id.clone());
            remaining.remove(id);
            for succ in dep_graph.out_neighbors(id) {
                if let Some(d) = remaining.get_mut(succ) {
                    *d -= 1;
                }
            }
        }

        waves.push(Wave {
            index: waves.len() + 1,
            tasks: frontier,
            duration_secs: duration,
        });
    }

    Ok(waves)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builder::build_graph;
    use crate::parser::parse;

    fn graph_from(input: &str) -> BuildPipeline::Graph {
        let parsed = parse(input).unwrap();
        build_graph(&parsed).unwrap()
    }

    #[test]
    fn planは依存のない先頭タスクをまとめて波にする() {
        let g = graph_from(
            "\
task fetch: cargo fetch (10s)
fetch produces target/idx
task build_a: cargo build a (20s)
build_a consumes target/idx
build_a produces target/a
task build_b: cargo build b (30s)
build_b consumes target/idx
build_b produces target/b
task link: cargo link (5s)
link consumes target/a
link consumes target/b
",
        );
        let waves = plan(&g).expect("循環がないので成功するはず");
        assert_eq!(waves.len(), 3);
        assert_eq!(waves[0].tasks, vec![TaskId("fetch".to_string())]);
        assert_eq!(waves[0].duration_secs, 10);

        let mut wave2_names: Vec<String> = waves[1].tasks.iter().map(|t| t.0.clone()).collect();
        wave2_names.sort();
        assert_eq!(
            wave2_names,
            vec!["build_a".to_string(), "build_b".to_string()]
        );
        assert_eq!(waves[1].duration_secs, 30); // max(20, 30)

        assert_eq!(waves[2].tasks, vec![TaskId("link".to_string())]);
    }
}
