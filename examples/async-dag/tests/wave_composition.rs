//! 統合テスト: 本編サンプルの波数・各波の中身が手計算した期待値と一致し、
//! 全サービスがちょうど1つの波に現れること。

use std::collections::HashSet;

use async_dag::depgraph;
use async_dag::fixtures::sample_orchestration;
use async_dag::schema::ServiceId;

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
        ["migration", "metrics"]
            .iter()
            .map(|s| s.to_string())
            .collect()
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
    assert_eq!(total_scheduled, g.service_ids().count());

    let mut seen: HashSet<String> = HashSet::new();
    for wave in &waves {
        for svc in wave {
            assert!(
                seen.insert(svc.0.clone()),
                "{svc:?} が複数の波に重複して現れた"
            );
        }
    }
}
