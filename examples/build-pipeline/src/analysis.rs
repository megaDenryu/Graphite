//! ドメイン検証・実行計画・クリティカルパス計算。
//!
//! `graph_schema!` が保証する「図式適合」(端点種別・多重度) はあくまで
//! グラフの形が正しいかどうかであり、「誰も produce しない artifact を
//! consume している」「同じ artifact を2つのタスクが produce している」
//! 「タスク依存が循環している」といった *ビルドパイプラインとしての意味の
//! 妥当性* は検査しない。これらはこのモジュールで、`{label}_pairs()` /
//! `{node_snake}_ids()` イテレータと汎用 `graphite::Graph<TaskId>` への
//! 射影を使って別レイヤーとして実装する
//! (README「導出エッジ」節が想定する使い分けそのもの)。

use crate::schema::{ArtifactId, BuildPipeline, Task, TaskId};
use graphite::{CycleError, Graph};
use std::collections::{HashMap, HashSet};
use std::fmt;

/// タスク依存グラフ (`consumes ∘ produces⁻¹` の射影)。
///
/// 辺 `producer -> consumer` は「`producer` が生成した成果物を `consumer`
/// が読み込む (=`producer` は `consumer` より先に実行されなければならない)」
/// ことを表す。ノード値・辺値は不要 (依存関係の形だけが要る) なので両方 `()`
/// にし、キー型だけ `TaskId` にしている。
pub type TaskDependencyGraph = Graph<(), (), TaskId>;

/// [`BuildPipeline`] からタスク依存グラフを射影する。
///
/// `produces_pairs()`/`consumes_pairs()` はどちらも `BuildPipeline` の生成物
/// (図式グラフのクエリ API) であり、ここで初めて「タスク間の順序」という
/// 導出情報を組み立てる。エッジの終点キーは常に `g.task_ids()` 由来なので
/// `Graph::build` が `UnknownEndpoint` を返すことはない (`expect` で妥当)。
pub fn task_dependency_graph(g: &BuildPipeline) -> TaskDependencyGraph {
    let mut producers_of: HashMap<&ArtifactId, Vec<&TaskId>> = HashMap::new();
    for (task, artifact) in g.produces_pairs() {
        producers_of.entry(artifact).or_default().push(task);
    }

    let nodes: Vec<(TaskId, ())> = g.task_ids().map(|id| (id.clone(), ())).collect();

    // `flat_map` にすると内側のイテレータが `producers_of` への借用を
    // `FnMut` クロージャの呼び出しをまたいで持ち越そうとしてしまい
    // 借用検査器に拒否される (「呼び出し毎に排他アクセスを得る」という
    // `FnMut` の性質上、その借用は呼び出しの外へ逃がせない)。ループで
    // 即座に `Vec` へ確定させることで回避する。
    let mut edges: Vec<(TaskId, TaskId, ())> = Vec::new();
    for (consumer, artifact) in g.consumes_pairs() {
        if let Some(producers) = producers_of.get(artifact) {
            for producer in producers {
                edges.push(((*producer).clone(), consumer.clone(), ()));
            }
        }
    }

    Graph::build(nodes, edges)
        .expect("タスク依存グラフの辺の端点は必ずg.task_ids()由来なので未知キーにはならない")
}

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

/// ドメイン検証を実行する。図式適合 (`BuildPipeline::create` 時点) は既に
/// 通っている前提で、意味的な妥当性だけを検査する。
pub fn validate(g: &BuildPipeline) -> Vec<DomainIssue> {
    let mut issues = Vec::new();

    let mut producers_of: HashMap<&ArtifactId, Vec<&TaskId>> = HashMap::new();
    for (task, artifact) in g.produces_pairs() {
        producers_of.entry(artifact).or_default().push(task);
    }
    let mut consumers_of: HashMap<&ArtifactId, Vec<&TaskId>> = HashMap::new();
    for (task, artifact) in g.consumes_pairs() {
        consumers_of.entry(artifact).or_default().push(task);
    }

    // 1. 孤児成果物: consume されているのに produce するタスクが無い。
    let mut orphan_artifacts: Vec<&ArtifactId> = consumers_of
        .keys()
        .copied()
        .filter(|a| !producers_of.contains_key(*a))
        .collect();
    orphan_artifacts.sort_by(|a, b| a.0.cmp(&b.0));
    for artifact in orphan_artifacts {
        let mut consumers: Vec<TaskId> = consumers_of[artifact].iter().map(|&t| t.clone()).collect();
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
        let mut producers: Vec<TaskId> = producers_of[artifact].iter().map(|&t| t.clone()).collect();
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
pub fn plan(g: &BuildPipeline) -> Result<Vec<Wave>, CycleError<TaskId>> {
    let dep_graph = task_dependency_graph(g);
    // 循環があれば代表ノード付きで早期に報告する。
    dep_graph.topological_sort()?;

    let mut remaining: HashMap<TaskId, usize> =
        g.task_ids().map(|id| (id.clone(), 0usize)).collect();
    for id in g.task_ids() {
        for succ in dep_graph.out_neighbors(id) {
            *remaining.get_mut(succ).expect("succはg.task_ids()由来") += 1;
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
            .map(|id| g.task(id).map(|t| t.secs).unwrap_or(0))
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

/// クリティカルパス (タスク時間の重み付き最長経路) の計算結果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CriticalPath {
    pub path: Vec<TaskId>,
    pub total_secs: u32,
    pub total_work_secs: u32,
}

impl CriticalPath {
    /// 全体並列度 = 全タスクの所要時間合計 / クリティカルパス長。
    /// 1.0 に近いほど並列化の余地が無い (直列に近い) パイプラインであることを
    /// 意味し、大きいほど並列実行による短縮効果が大きいことを意味する。
    pub fn parallelism(&self) -> f64 {
        if self.total_secs == 0 {
            0.0
        } else {
            self.total_work_secs as f64 / self.total_secs as f64
        }
    }
}

/// クリティカルパスを計算する。
///
/// トポロジカル順序に沿って `dist[v] = max(dist[v], dist[u] + secs(v))`
/// (`u -> v` の辺ごと) と緩和していく DAG 上の最長経路 DP。`Graph::out_neighbors`
/// だけで書けるため、常に「順方向に伝播する」形にしているのがポイント
/// (`Graph::in_neighbors` はフェーズ5で追加されたが、この DP は前進伝播で
/// 完結するため使っていない)。同種の計算を汎用的に行いたいだけなら
/// `graphite::Graph::critical_path_by` (フェーズ5追加) に委譲することも
/// できるが、`total_work_secs`/`parallelism` などこのアプリ固有の付随
/// データを持つ [`CriticalPath`] を組み立てる都合上、ここでは専用の DP
/// を保持している。
pub fn critical_path(g: &BuildPipeline) -> Result<CriticalPath, CycleError<TaskId>> {
    let dep_graph = task_dependency_graph(g);
    let order = dep_graph.topological_sort()?;

    let secs_of = |id: &TaskId| -> u32 { g.task(id).map(|t: &Task| t.secs).unwrap_or(0) };

    if order.is_empty() {
        return Ok(CriticalPath {
            path: Vec::new(),
            total_secs: 0,
            total_work_secs: 0,
        });
    }

    let mut dist: HashMap<TaskId, u32> = HashMap::new();
    let mut pred: HashMap<TaskId, TaskId> = HashMap::new();

    for &id in &order {
        dist.entry(id.clone()).or_insert_with(|| secs_of(id));
    }

    for &id in &order {
        let cur = dist[id];
        for succ in dep_graph.out_neighbors(id) {
            let candidate = cur + secs_of(succ);
            if candidate > *dist.get(succ).unwrap_or(&0) {
                dist.insert(succ.clone(), candidate);
                pred.insert(succ.clone(), id.clone());
            }
        }
    }

    let end = order
        .iter()
        .max_by_key(|&&id| dist[id])
        .map(|&id| id.clone())
        .expect("orderは空でないことを上で確認済み");

    let total_secs = dist[&end];
    let mut path = vec![end.clone()];
    let mut cur = end;
    while let Some(p) = pred.get(&cur) {
        path.push(p.clone());
        cur = p.clone();
    }
    path.reverse();

    let total_work_secs: u32 = g.task_ids().map(secs_of).sum();

    Ok(CriticalPath {
        path,
        total_secs,
        total_work_secs,
    })
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
    fn 孤児成果物を検出できる() {
        let g = graph_from(
            "\
task build: cargo build (10s)
build produces target/a
task test: cargo test (5s)
test consumes target/b
",
        );
        let issues = validate(&g);
        assert!(issues.iter().any(|i| matches!(
            i,
            DomainIssue::OrphanArtifact { artifact, .. } if artifact.0 == "target/b"
        )));
    }

    #[test]
    fn produce競合を検出できる() {
        let g = graph_from(
            "\
task build_a: cargo build a (10s)
build_a produces target/out
task build_b: cargo build b (10s)
build_b produces target/out
",
        );
        let issues = validate(&g);
        assert!(issues.iter().any(|i| matches!(
            i,
            DomainIssue::ConflictingProducers { artifact, .. } if artifact.0 == "target/out"
        )));
    }

    #[test]
    fn 循環依存を検出できる() {
        let g = graph_from(
            "\
task a: cmd a (10s)
a consumes target/from_b
a produces target/from_a
task b: cmd b (10s)
b consumes target/from_a
b produces target/from_b
",
        );
        let issues = validate(&g);
        assert!(issues
            .iter()
            .any(|i| matches!(i, DomainIssue::CyclicDependency { .. })));
    }

    #[test]
    fn 正常なパイプラインは違反ゼロ() {
        let g = graph_from(
            "\
task build: cargo build (10s)
build produces target/a
task test: cargo test (5s)
test consumes target/a
",
        );
        assert!(validate(&g).is_empty());
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

        let mut wave2_names: Vec<String> =
            waves[1].tasks.iter().map(|t| t.0.clone()).collect();
        wave2_names.sort();
        assert_eq!(wave2_names, vec!["build_a".to_string(), "build_b".to_string()]);
        assert_eq!(waves[1].duration_secs, 30); // max(20, 30)

        assert_eq!(waves[2].tasks, vec![TaskId("link".to_string())]);
    }

    #[test]
    fn critical_pathは最長経路と合計時間を返す() {
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
        let cp = critical_path(&g).expect("循環がないので成功するはず");
        assert_eq!(cp.total_secs, 45); // fetch(10) + build_b(30) + link(5)
        assert_eq!(
            cp.path,
            vec![
                TaskId("fetch".to_string()),
                TaskId("build_b".to_string()),
                TaskId("link".to_string()),
            ]
        );
        assert_eq!(cp.total_work_secs, 65); // 10+20+30+5
    }

    #[test]
    fn 循環があるとplanもcritical_pathもエラーになる() {
        let g = graph_from(
            "\
task a: cmd a (10s)
a consumes target/from_b
a produces target/from_a
task b: cmd b (10s)
b consumes target/from_a
b produces target/from_b
",
        );
        assert!(plan(&g).is_err());
        assert!(critical_path(&g).is_err());
    }
}
