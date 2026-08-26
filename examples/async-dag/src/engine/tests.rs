//! 並列実行が依存順序を守ること、および直列実行より速いことを固定する。

use super::{run_serial, run_waves};
use crate::schema::ServiceId;
use crate::depgraph::compute_waves;
use crate::schema::{Orchestration, Service};

#[test]
#[rustfmt::skip]
fn run_wavesは依存元が依存先より先に完了していることを記録から確認できる() {
    let g = graphite::graph!(Orchestration {
        config = Service { name: "config".into(), startup_ms: 10 },
        db     = Service { name: "db".into(), startup_ms: 15 },
        cache  = Service { name: "cache".into(), startup_ms: 8 },
        api    = Service { name: "api".into(), startup_ms: 5 },

        db_config    = DependsOn(db -> config),
        cache_config = DependsOn(cache -> config),
        api_db       = DependsOn(api -> db),
        api_cache    = DependsOn(api -> cache),
    })
    .unwrap();

    let waves = compute_waves(&g).unwrap();
    let report = run_waves(&g, &waves);

    assert_eq!(report.records.len(), 4);

    // DependsOn の全ペアについて、依存先 (prerequisite) の完了時刻が
    // 依存元 (dependent) の開始時刻より前 (以下) であることを確認する。
    for edge in g.depends_on_iter() {
        let dependent = edge.dependent().id();
        let prerequisite = edge.dependency().id();
        let dependent_record = report.record_of(dependent);
        let prerequisite_record = report.record_of(prerequisite);
        assert!(
            prerequisite_record.end <= dependent_record.start,
            "{prerequisite:?}(end={:?}) は {dependent:?}(start={:?}) より前に完了しているはず",
            prerequisite_record.end,
            dependent_record.start,
        );
    }
}

#[test]
#[rustfmt::skip]
fn run_wavesの実測時間は波の合計より直列実行より短い() {
    let g = graphite::graph!(Orchestration {
        config = Service { name: "config".into(), startup_ms: 20 },
        db     = Service { name: "db".into(), startup_ms: 40 },
        cache  = Service { name: "cache".into(), startup_ms: 40 },
        api    = Service { name: "api".into(), startup_ms: 10 },

        db_config    = DependsOn(db -> config),
        cache_config = DependsOn(cache -> config),
        api_db       = DependsOn(api -> db),
        api_cache    = DependsOn(api -> cache),
    })
    .unwrap();

    let waves = compute_waves(&g).unwrap();
    let report = run_waves(&g, &waves);

    let order: Vec<ServiceId> = g.service_ids().cloned().collect();
    let serial = run_serial(&g, &order);

    // 直列: 20+40+40+10=110ms。並列: 20 (config) + 40 (db,cacheの最大) +
    // 10 (api) = 70ms。スレッド起動オーバーヘッドを見込んでも直列より
    // 十分短いはず。
    assert!(
        report.total < serial,
        "並列実行 ({:?}) は直列実行 ({:?}) より短いはず",
        report.total,
        serial
    );
}
