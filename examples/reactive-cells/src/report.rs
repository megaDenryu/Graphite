//! `main.rs` 向けの読み物風出力ヘルパー。
//!
//! ロジック (`engine.rs`/`antipattern.rs`) と表示 (このファイル) を分離
//! するのは `examples/org-analyzer` と同じ方針。

use graphite::CycleError;

use crate::antipattern::DiamondDemo;
use crate::engine::{Engine, RecomputeStep};
use crate::schema::CellId;

pub fn print_section(title: &str) {
    println!("\n=== {title} ===");
}

/// [`Engine::set_input`] の結果を「どのセルがどの順で再計算されたか」の
/// 物語として表示する。
pub fn print_set_input(label: &str, id: &CellId, value: f64, steps: &[RecomputeStep]) {
    println!("{label}: {} <- {value}", id.0);
    if steps.is_empty() {
        println!("  (影響を受けるセルは無い)");
        return;
    }
    for step in steps {
        println!("  -> 再計算: {} = {}", step.id.0, step.value);
    }
}

pub fn print_engine_snapshot(engine: &Engine, ids: &[&str]) {
    for id in ids {
        println!("  {id} = {}", engine.value(&CellId(id.to_string())));
    }
}

pub fn print_cycle_error(err: &CycleError<CellId>) {
    println!("Engine::new は失敗した (期待通り):");
    println!("  {err}");
    let path: Vec<String> = err.cycle.iter().map(|id| id.0.clone()).collect();
    println!("  循環パス: {} -> {}", path.join(" -> "), path[0]);
}

pub fn print_diamond_demo(title: &str, demo: &DiamondDemo) {
    println!("{title}:");
    for (i, (b, c, d)) in demo.d_log.borrow().iter().enumerate() {
        let marker = if i == 0 { "  [1回目] " } else { "  [2回目] " };
        println!("{marker}d再計算時点の観測値: b={b}, c={c} -> d={d}");
    }
    let (_b, _c, last) = *demo
        .d_log
        .borrow()
        .last()
        .expect("triggerを呼んだ後ならd_logは空ではない");
    println!("  最終的なd = {last} (正しい値に収束するが、過程で1回グリッチを踏んでいる)");
}

pub fn print_infinite_loop_demo(cap: usize, actual_count: usize) {
    println!("循環購読 x<->y を起動 (安全弁: {cap}回で強制停止)");
    println!("  実際の通知回数 = {actual_count}");
    if actual_count >= cap {
        println!("  -> capにちょうど到達した。安全弁が無ければ止まらずスタックオーバーフローしていたはず。");
    } else {
        println!("  -> capに到達せず自然に停止した (循環していない可能性がある)。");
    }
}
