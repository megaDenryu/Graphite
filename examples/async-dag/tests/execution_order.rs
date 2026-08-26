//! 統合テスト: 実測ログが依存順序を示し、並列実行が直列実行より速いこと。

use async_dag::depgraph;
use async_dag::engine::run_serial;
use async_dag::fixtures::sample_orchestration;
use async_dag::schema::ServiceId;

#[test]
fn 実行ログは依存先が依存元より先に完了していることを示す() {
    let g = sample_orchestration();
    let waves = depgraph::compute_waves(&g).unwrap();
    let report = async_dag::engine::run_waves(&g, &waves);

    assert_eq!(report.records.len(), g.service_ids().count());

    for edge in g.depends_on_iter() {
        let dependent = edge.dependent().id();
        let prerequisite = edge.dependency().id();
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

    let serial_order: Vec<ServiceId> = g.service_ids().cloned().collect();
    let serial_total = run_serial(&g, &serial_order);

    assert!(
        report.total < serial_total,
        "並列実行({:?})は直列実行({:?})より速いはず",
        report.total,
        serial_total
    );
}
