//! async-dag — 「非同期オーケストレーション地獄」をグラフで倒す実践example。
//!
//! 題材: マイクロサービス群の起動オーケストレータ。
//! `config -> (logger, db, cache, queue) -> (migration, metrics) ->
//! (api, worker) -> healthcheck` という10サービスの依存関係を
//! `graph_schema!`/`graph!` で宣言し、汎用 `graphite::Graph` へ射影して
//! `topological_levels` で「並行実行できる波」を導出、`std::thread::scope`
//! で実際に並列起動する。
//!
//! `cargo run` すると以下を順に実行する:
//! 1. 循環依存を仕込んだ小さな例が `CycleError` で拒否されることを示す
//!    (ハングする前にデータ検証で死ぬ、という README の主張の実演)
//! 2. 本編の10サービスグラフから波を計算して表示する
//! 3. その波を `std::thread::scope` で実際に並列実行し、ログを表示する
//! 4. 直列実行 (ベースライン) と実測時間を比較する
//!
//! 詳細な設計意図・アンチパターンの解説は README.md 参照。

use async_dag::depgraph::{self, build_dependency_graph};
use async_dag::engine::{self, ExecutionReport};
use async_dag::fixtures::{cyclic_demo, sample_orchestration};
use async_dag::schema::{Orchestration, Service, ServiceId};

fn main() {
    循環依存デモ();
    println!();
    let g = 本編のサービスグラフを構築する();
    let waves = 波を計算して表示する(&g);
    let report = 波を並列実行してログを表示する(&g, &waves);
    直列実行と比較する(&g, &report);
}

// ============================================================
// 1. 循環依存デモ — ハングする前にデータ検証で死ぬ
// ============================================================

/// 循環依存を仕込んだ3サービスの小さな例を作り、`has_cycle`/
/// `topological_levels` が具体的な循環パスつきで拒否することを示す。
/// もしこれをそのまま `std::thread::scope` で実行しようとしていたら
/// (循環しているのでどのサービスも「先行が終わるまで待つ」条件を満たせず)
/// 永久にハングしていたはずである — その手前でデータとして拒否できる、
/// というのが「循環はハングではなくCycleErrorとして構築直後に死ぬ」の実演。
fn 循環依存デモ() {
    println!("=== 1. 循環依存デモ (ハングする前にデータ検証で死ぬ) ===");

    let broken = cyclic_demo();

    let dep_graph = build_dependency_graph(&broken);
    println!("has_cycle() = {}", dep_graph.has_cycle());

    match depgraph::compute_waves(&broken) {
        Ok(_) => unreachable!("循環があるのでOkにはならないはず"),
        Err(cycle_error) => {
            println!(
                "波の計算は CycleError で拒否された (実行を試みる前に判明): {cycle_error}"
            );
        }
    }
}

// ============================================================
// 2. 本編: 10サービスの依存グラフ
// ============================================================

/// `config -> (logger, db, cache, queue) -> (migration, metrics) ->
/// (api, worker) -> healthcheck` という10サービスの依存グラフを組み立てる。
fn 本編のサービスグラフを構築する() -> Orchestration {
    let g = sample_orchestration();

    println!(
        "=== 2. 本編サービスグラフを構築 (サービス数={}, depends_on本数={}) ===",
        g.service_ids().count(),
        g.depends_on().len()
    );
    g
}

// ============================================================
// 3. 波の計算 — 実行計画は書くものではなく導出される
// ============================================================

fn 波を計算して表示する(g: &Orchestration) -> Vec<Vec<ServiceId>> {
    println!("\n=== 3. topological_levels() で波を計算 ===");
    let waves = depgraph::compute_waves(g).expect("本編グラフに循環はないはず");
    for (i, wave) in waves.iter().enumerate() {
        let names: Vec<&str> = wave
            .iter()
            .filter_map(|id| g.service(id))
            .map(|s: &Service| s.name.as_str())
            .collect();
        let duration = depgraph::wave_duration_ms(g, wave);
        println!(
            "wave {}: [{}] (この波の所要時間 = {}ms)",
            i + 1,
            names.join(", "),
            duration
        );
    }
    waves
}

// ============================================================
// 4. 波の並列実行 — std::thread::scope で実際に並行に走らせる
// ============================================================

fn 波を並列実行してログを表示する(g: &Orchestration, waves: &[Vec<ServiceId>]) -> ExecutionReport {
    println!("\n=== 4. std::thread::scope で波を実際に並列実行 ===");
    let report = engine::run_waves(g, waves);

    let mut sorted_records = report.records.clone();
    sorted_records.sort_by_key(|r| r.start);
    for record in &sorted_records {
        let name = g
            .service(&record.service)
            .map(|s| s.name.as_str())
            .unwrap_or("?");
        println!(
            "  wave {}: {name} 開始={}ms 完了={}ms",
            record.wave,
            record.start.as_millis(),
            record.end.as_millis(),
        );
    }
    println!("実測合計時間 = {}ms", report.total.as_millis());
    report
}

// ============================================================
// 5. 直列実行との比較
// ============================================================

fn 直列実行と比較する(g: &Orchestration, report: &ExecutionReport) {
    println!("\n=== 5. 直列実行 (敵1のベースライン) との比較 ===");
    let serial_order: Vec<ServiceId> = report
        .waves
        .iter()
        .flat_map(|wave| wave.iter().cloned())
        .collect();
    let serial_total = engine::run_serial(g, &serial_order);
    let ideal_serial_ms = depgraph::total_serial_ms(g);

    println!("直列実行 (実測) = {}ms (起動時間の総和 = {ideal_serial_ms}ms)", serial_total.as_millis());
    println!("並列実行 (実測) = {}ms", report.total.as_millis());
    let speedup = serial_total.as_secs_f64() / report.total.as_secs_f64().max(0.000_001);
    println!("実測の高速化率 = {speedup:.2}倍");
}
