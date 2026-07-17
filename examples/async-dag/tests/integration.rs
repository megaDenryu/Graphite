//! 統合テスト。
//!
//! - 本編サンプル (`fixtures::sample_orchestration`) の波の内容・波数が
//!   手計算した期待値と一致すること
//! - 循環依存サンプル (`fixtures::cyclic_demo`) が `has_cycle`/
//!   `compute_waves` の両方で拒否され、具体的な循環パスが得られること
//! - `run_waves` の実測ログから「依存先が依存元より先に完了している」
//!   という順序制約が実際に守られていること
//! - 並列実行 (`run_waves`) が直列実行 (`run_serial`) より実測で速いこと
//! - `graph_schema!` の図式適合検査 (未知の依存先) が独立に機能すること

use async_dag::depgraph::{self, build_dependency_graph};
use async_dag::engine::run_serial;
use async_dag::fixtures::{cyclic_demo, sample_orchestration};
use async_dag::schema::{
    DependsOn, Orchestration, OrchestrationNode, OrchestrationViolation, Service, ServiceId,
};
use std::collections::HashSet;

fn id(name: &str) -> ServiceId {
    ServiceId(name.to_string())
}

fn names(ids: &[ServiceId]) -> HashSet<String> {
    ids.iter().map(|i| i.0.clone()).collect()
}

#[test]
fn 本編グラフの波数は5である() {
    let g = sample_orchestration();
    let waves = depgraph::compute_waves(&g).expect("本編グラフに循環はないはず");
    assert_eq!(waves.len(), 5, "波の内容: {waves:?}");
}

#[test]
fn 波1はconfigのみで波5はhealthcheckのみである() {
    let g = sample_orchestration();
    let waves = depgraph::compute_waves(&g).unwrap();
    assert_eq!(waves[0], vec![id("config")]);
    assert_eq!(waves[4], vec![id("healthcheck")]);
}

#[test]
fn 波2はconfig直下の4サービスがまとまる() {
    let g = sample_orchestration();
    let waves = depgraph::compute_waves(&g).unwrap();
    assert_eq!(
        names(&waves[1]),
        ["logger", "db", "cache", "queue"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    );
}

#[test]
fn 波3はmigrationとmetrics_波4はapiとworkerである() {
    let g = sample_orchestration();
    let waves = depgraph::compute_waves(&g).unwrap();
    assert_eq!(
        names(&waves[2]),
        ["migration", "metrics"].iter().map(|s| s.to_string()).collect()
    );
    assert_eq!(
        names(&waves[3]),
        ["api", "worker"].iter().map(|s| s.to_string()).collect()
    );
}

#[test]
fn 全サービスがちょうど1つの波に現れる() {
    let g = sample_orchestration();
    let waves = depgraph::compute_waves(&g).unwrap();
    let total_scheduled: usize = waves.iter().map(|w| w.len()).sum();
    assert_eq!(total_scheduled, Service::ids(&g).count());

    let mut seen: HashSet<String> = HashSet::new();
    for wave in &waves {
        for svc in wave {
            assert!(seen.insert(svc.0.clone()), "{svc:?} が複数の波に重複して現れた");
        }
    }
}

#[test]
fn 循環依存サンプルはhas_cycleでtrueになる() {
    let g = cyclic_demo();
    let dep_graph = build_dependency_graph(&g);
    assert!(dep_graph.has_cycle());
}

#[test]
fn 循環依存サンプルはcompute_wavesが具体的な循環パスつきで拒否する() {
    let g = cyclic_demo();
    let err = depgraph::compute_waves(&g).expect_err("循環があるのでErrになるはず");
    assert_eq!(err.cycle.len(), 3, "循環パス: {:?}", err.cycle);
    let cycle_names: HashSet<String> = err.cycle.iter().map(|i| i.0.clone()).collect();
    assert_eq!(
        cycle_names,
        ["a", "b", "c"].iter().map(|s| s.to_string()).collect()
    );
}

#[test]
fn 実行ログは依存先が依存元より先に完了していることを示す() {
    let g = sample_orchestration();
    let waves = depgraph::compute_waves(&g).unwrap();
    let report = async_dag::engine::run_waves(&g, &waves);

    assert_eq!(report.records.len(), Service::ids(&g).count());

    for (_id, edge) in DependsOn::iter(&g) {
        let dependent = edge.from();
        let prerequisite = edge.to();
        let dependent_record = report.record_of(dependent);
        let prerequisite_record = report.record_of(prerequisite);
        assert!(
            prerequisite_record.end <= dependent_record.start,
            "{prerequisite:?} は {dependent:?} より前に完了しているはず (prerequisite.end={:?}, dependent.start={:?})",
            prerequisite_record.end,
            dependent_record.start,
        );
    }
}

#[test]
fn 並列実行は直列実行より実測で速い() {
    let g = sample_orchestration();
    let waves = depgraph::compute_waves(&g).unwrap();
    let report = async_dag::engine::run_waves(&g, &waves);

    let serial_order: Vec<ServiceId> = Service::ids(&g).cloned().collect();
    let serial_total = run_serial(&g, &serial_order);

    assert!(
        report.total < serial_total,
        "並列実行({:?})は直列実行({:?})より速いはず",
        report.total,
        serial_total
    );
}

#[test]
fn 未知の依存先を参照するとunknowntarget違反になる() {
    let result: Result<Orchestration, OrchestrationViolation> =
        Orchestration::create(|b| {
            b.service(id("api"), Service { name: "api".into(), startup_ms: 10 });
            b.depends_on(
                async_dag::schema::DependsOnId("api_missing".to_string()),
                DependsOn(id("api"), id("存在しないサービス")),
            );
        });
    assert!(matches!(
        result,
        Err(OrchestrationViolation::DependsOnUnknownTarget { .. })
    ));
}
