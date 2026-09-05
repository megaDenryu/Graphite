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

mod execution_report;

#[cfg(test)]
mod tests;

pub use execution_report::{ExecutionRecord, ExecutionReport};

use crate::schema::{Orchestration, ServiceId};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

// 本物のサービス起動の代わりに `startup_ms` だけ sleep する。
fn simulate_startup(startup_ms: u64) {
    thread::sleep(Duration::from_millis(startup_ms));
}

// 波ごとに `std::thread::scope` でスレッドを起こし、実際に並列実行する。
//
// 波内のスレッドは全て同時に `spawn` され、`thread::scope` の呼び出しが
// 戻る (=波内の全スレッドが join し終える) まで次の波へは進まない。この
// 「波の完了を待ってから次の波へ」という同期こそが、依存関係
// (「先行サービスが起動完了していること」) を実際に守っている箇所。
pub fn run_waves(g: &Orchestration::Graph, waves: &[Vec<ServiceId>]) -> ExecutionReport {
    let overall_start = Instant::now();
    let records: Mutex<Vec<ExecutionRecord>> = Mutex::new(Vec::new());

    for (wave_index, wave) in waves.iter().enumerate() {
        thread::scope(|scope| {
            for id in wave {
                let service = g
                    .service_by_id(id)
                    .unwrap_or_else(|| panic!("波に含まれるキー{id:?}g.はservice_ids()由来のはず"));
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

// 「敵1」のベースライン: 依存関係の並行性を一切活かさず、渡された順に
// 直列に起動する (素朴な `await` の連鎖に相当)。所要時間は起動時間の
// 総和に一致する。並列実行版 (`run_waves`) との比較対象として使う。
pub fn run_serial(g: &Orchestration::Graph, order: &[ServiceId]) -> Duration {
    let start = Instant::now();
    for id in order {
        if let Some(service) = g.service_by_id(id) {
            simulate_startup(service.startup_ms);
        }
    }
    start.elapsed()
}
