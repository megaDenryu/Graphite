//! `Orchestration` (図式グラフ) から「実行順序グラフ」への射影と、
//! 循環検出・波 (トポロジカルレベル) 計算。
//!
//! これが本 example の核心部分。`depends_on().iter()` という
//! 「依存関係の表」を、汎用 `graphite::Graph<(), (), ServiceId>` へ
//! 射影してしまえば、循環検出 (`has_cycle`/`topological_sort`) と
//! 波分割 (`topological_levels`) はどちらも Graphite ランタイム側の
//! 実装を1回書けば済む — アプリ側が「並行実行できる集合をどう求めるか」
//! を自分で再発明する必要が無い、というのが README の主張。

use crate::schema::{DependsOn, Orchestration, OrchestrationNode, Service, ServiceId};
use graphite::{CycleError, Graph};

/// ノード値・辺値のいずれも不要 (依存関係の「形」だけが要る) なので
/// 両方 `()` にし、キー型だけ `ServiceId` にする
/// (`examples/build-pipeline` の `TaskDependencyGraph` と同じ手法)。
pub type ServiceDependencyGraph = Graph<(), (), ServiceId>;

/// `Orchestration` から実行順序グラフを射影する。
///
/// `DependsOn::iter(g)` は `(from, to)` = `(dependent, prerequisite)` の
/// 辺 (`DependsOn(dependent -> prerequisite)`) を返す。実行順序としては
/// `prerequisite` (to) が先に完了していなければならないので、汎用
/// `Graph` 上の辺は向きを反転し `prerequisite -> dependent`
/// (`to -> from`) として積む。これにより `topological_sort`/
/// `topological_levels` が仮定する「辺の始点が先」という向きと
/// 実行順序が一致する。
///
/// `DependsOn` の終点キーは常に `Service::ids(g)` 由来 (schema の
/// 図式適合検査が保証する) なので、`Graph::build` が `UnknownEndpoint`
/// を返すことはない。
pub fn build_dependency_graph(g: &Orchestration) -> ServiceDependencyGraph {
    let nodes: Vec<(ServiceId, ())> = Service::ids(g).map(|id| (id.clone(), ())).collect();

    let mut edges: Vec<(ServiceId, ServiceId, ())> = Vec::new();
    for (_id, edge) in DependsOn::iter(g) {
        let dependent = edge.from();
        let prerequisite = edge.to();
        edges.push((prerequisite.clone(), dependent.clone(), ()));
    }

    Graph::build(nodes, edges)
        .expect("DependsOnの端点は必ずService::ids(g)由来なので未知キーにはならない")
}

/// 「並行実行できる波」を依存関係グラフから計算する。
///
/// 循環がある場合はハングせず `CycleError` を返す — README が主張する
/// 「循環はデータ検証で構築直後に死ぬ、実行時にハングしない」の実装
/// そのもの。各波の要素順序は `Orchestration` へサービスを登録した順
/// (`Service` ノードの挿入順) で決定的。
pub fn compute_waves(g: &Orchestration) -> Result<Vec<Vec<ServiceId>>, CycleError<ServiceId>> {
    let dep_graph = build_dependency_graph(g);
    let levels = dep_graph.topological_levels()?;
    Ok(levels
        .into_iter()
        .map(|level| level.into_iter().cloned().collect())
        .collect())
}

/// 波1つ分の想定所要時間 (無限並列ワーカーを仮定した `max(startup_ms)`)。
pub fn wave_duration_ms(g: &Orchestration, wave: &[ServiceId]) -> u64 {
    wave.iter()
        .filter_map(|id| Service::get(g, id))
        .map(|s: &Service| s.startup_ms)
        .max()
        .unwrap_or(0)
}

/// 全サービスの起動時間の総和 (直列実行した場合の下限見積り)。
pub fn total_serial_ms(g: &Orchestration) -> u64 {
    Service::ids(g)
        .filter_map(|id| Service::get(g, id))
        .map(|s| s.startup_ms)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn 循環がないグラフでは波が依存順に計算される() {
        let g = graphite::graph!(Orchestration {
            config = Service { name: "config".into(), startup_ms: 10 },
            db     = Service { name: "db".into(), startup_ms: 20 },
            cache  = Service { name: "cache".into(), startup_ms: 15 },
            api    = Service { name: "api".into(), startup_ms: 25 },

            db_config    = DependsOn(db -> config),
            cache_config = DependsOn(cache -> config),
            api_db       = DependsOn(api -> db),
            api_cache    = DependsOn(api -> cache),
        })
        .unwrap();

        let waves = compute_waves(&g).expect("循環がないので成功するはず");
        assert_eq!(waves.len(), 3);
        assert_eq!(waves[0], vec![ServiceId("config".to_string())]);

        let mut wave2: Vec<String> = waves[1].iter().map(|id| id.0.clone()).collect();
        wave2.sort();
        assert_eq!(wave2, vec!["cache".to_string(), "db".to_string()]);

        assert_eq!(waves[2], vec![ServiceId("api".to_string())]);
    }

    #[test]
    #[rustfmt::skip]
    fn 循環があるグラフはtopological_levelsが具体的な循環パスつきで拒否する() {
        let g = graphite::graph!(Orchestration {
            a = Service { name: "a".into(), startup_ms: 10 },
            b = Service { name: "b".into(), startup_ms: 10 },
            c = Service { name: "c".into(), startup_ms: 10 },

            a_b = DependsOn(a -> b),
            b_c = DependsOn(b -> c),
            c_a = DependsOn(c -> a),
        })
        .unwrap();

        let dep_graph = build_dependency_graph(&g);
        assert!(dep_graph.has_cycle());

        let err = compute_waves(&g).expect_err("循環があるのでErrになるはず");
        assert_eq!(err.cycle.len(), 3);
        let names: std::collections::HashSet<String> =
            err.cycle.iter().map(|id| id.0.clone()).collect();
        assert_eq!(
            names,
            ["a", "b", "c"].iter().map(|s| s.to_string()).collect()
        );
    }
}
