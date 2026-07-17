//! 実行エンジン — `depgraph::compute_waves` が導出した波を、
//! `std::thread::scope` で実際に並列実行する。
//!
//! 外部の非同期ランタイム (tokio 等) には依存しない。`examples/` の
//! 依存ポリシー (`graphite` のみ) を守りつつ、「本当に並行に走らせる」
//! ことを実証するには `std::thread::scope` で十分 (`std::thread::sleep`
//! で本物のサービス起動をシミュレートするだけなので、非同期I/Oの出番は
//! そもそも無い)。
//!
//! 波ごとに `thread::scope` を1回呼び、波内の全サービスをスレッドとして
//! 起動して join してから次の波へ進む — これが「前の波が終わるまで次の
//! 波を始めない」という依存関係の遵守そのものであり、`Graph` 側で計算
//! した波の境界をそのまま同期点として使っているだけである点がポイント。

use crate::schema::{Orchestration, OrchestrationNode, Service, ServiceId};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

/// 1サービスの起動記録 (実測)。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionRecord {
    pub service: ServiceId,
    /// 1始まりの波番号。
    pub wave: usize,
    /// エンジン開始からの起動開始時刻。
    pub start: Duration,
    /// エンジン開始からの起動完了時刻。
    pub end: Duration,
}

/// `run_waves` の実行結果一式。
#[derive(Debug, Clone)]
pub struct ExecutionReport {
    pub waves: Vec<Vec<ServiceId>>,
    pub records: Vec<ExecutionRecord>,
    pub total: Duration,
}

impl ExecutionReport {
    /// `id` の起動記録を引く (`run_waves` に渡した波に含まれるキーなら必ず存在する)。
    pub fn record_of(&self, id: &ServiceId) -> &ExecutionRecord {
        self.records
            .iter()
            .find(|r| &r.service == id)
            .unwrap_or_else(|| panic!("{id:?} の実行記録が見つからない"))
    }
}

/// 本物のサービス起動の代わりに `startup_ms` だけ sleep する。
fn simulate_startup(startup_ms: u64) {
    thread::sleep(Duration::from_millis(startup_ms));
}

/// 波ごとに `std::thread::scope` でスレッドを起こし、実際に並列実行する。
///
/// 波内のスレッドは全て同時に `spawn` され、`thread::scope` の呼び出しが
/// 戻る (=波内の全スレッドが join し終える) まで次の波へは進まない。この
/// 「波の完了を待ってから次の波へ」という同期こそが、依存関係
/// (「先行サービスが起動完了していること」) を実際に守っている箇所。
pub fn run_waves(g: &Orchestration, waves: &[Vec<ServiceId>]) -> ExecutionReport {
    let overall_start = Instant::now();
    let records: Mutex<Vec<ExecutionRecord>> = Mutex::new(Vec::new());

    for (wave_index, wave) in waves.iter().enumerate() {
        thread::scope(|scope| {
            for id in wave {
                let service = Service::get(g, id)
                    .unwrap_or_else(|| panic!("波に含まれるキー{id:?}はService::ids(g)由来のはず"));
                let records = &records;
                scope.spawn(move || {
                    let start = overall_start.elapsed();
                    simulate_startup(service.startup_ms);
                    let end = overall_start.elapsed();
                    records.lock().unwrap().push(ExecutionRecord {
                        service: id.clone(),
                        wave: wave_index + 1,
                        start,
                        end,
                    });
                });
            }
        });
    }

    let total = overall_start.elapsed();
    ExecutionReport {
        waves: waves.to_vec(),
        records: records.into_inner().expect("Mutexがpoisonすることはない"),
        total,
    }
}

/// 「敵1」のベースライン: 依存関係の並行性を一切活かさず、渡された順に
/// 直列に起動する (素朴な `await` の連鎖に相当)。所要時間は起動時間の
/// 総和に一致する。並列実行版 (`run_waves`) との比較対象として使う。
pub fn run_serial(g: &Orchestration, order: &[ServiceId]) -> Duration {
    let start = Instant::now();
    for id in order {
        if let Some(service) = Service::get(g, id) {
            simulate_startup(service.startup_ms);
        }
    }
    start.elapsed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::depgraph::compute_waves;
    use crate::schema::{DependsOn, OrchestrationNode, Service};

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
        for (_id, edge) in DependsOn::iter(&g) {
            let dependent = edge.from();
            let prerequisite = edge.to();
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

        let order: Vec<ServiceId> = Service::ids(&g).cloned().collect();
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
}
