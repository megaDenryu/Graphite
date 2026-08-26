//! 物語の第2章 — 依存グラフの宣言と `Engine::new` が構築時に決めること。
//!
//! 正常なシートからトポロジカル順序つきのエンジンが得られること、および
//! 循環するシートが構築の時点で拒否されることを実演する。

use reactive_cells::engine::Engine;
use reactive_cells::fixtures::{cyclic_demo_sheet, default_sheet};
use reactive_cells::report;

pub fn declare_sheet_and_build_engine() -> Engine {
    report::print_section("グラフによる再定式化: ミニスプレッドシートをgraph!で宣言する");
    let sheet = default_sheet().expect("正常なシートは構築に成功するはず");
    println!(
        "セル数 = {}, 依存エッジ数 = {} (Feeds {} + Lhs {} + Rhs {}。依存関係は実行前に\n\
         一枚で見える構造データ。Lhs/Rhsは減算セルadjustmentの被減数/減数の区別を運ぶ辺種別)",
        sheet.cell_ids().count(),
        sheet.feeds_len() + sheet.lhs_len() + sheet.rhs_len(),
        sheet.feeds_len(),
        sheet.lhs_len(),
        sheet.rhs_len()
    );
    let engine = Engine::new(sheet).expect("循環が無いので構築に成功するはず");
    println!("トポロジカル順序 (これがそのままglitch-freeな再計算順になる):");
    let order: Vec<String> = engine
        .topological_order()
        .iter()
        .map(|id| id.0.clone())
        .collect();
    println!("  {}", order.join(" -> "));
    engine
}

pub fn demonstrate_cycle_rejection() {
    report::print_section("敵2つづき: 循環の拒否 (構築前にデータ検証で拒否する)");
    let cyclic_sheet = cyclic_demo_sheet().expect("feedsは0..*なので構造としては構築できる");
    println!("cyclic_demo_sheet: 構造としてはSheet::createに成功する (a->b->c->aの循環)。");
    match Engine::new(cyclic_sheet) {
        Ok(_) => println!("  想定外: 循環があるのにEngine::newが成功した"),
        Err(err) => report::print_cycle_error(&err),
    }
    println!(
        "  -> observer パターンなら実行して初めて (無限にnotifyが回って) 気づく循環が、\n\
         Graphite化した依存グラフでは Engine::new の構築時点で具体的な循環パスつきで拒否される。"
    );
}
