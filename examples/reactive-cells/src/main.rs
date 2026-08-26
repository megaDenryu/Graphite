//! reactive-cells — 「リアクティブプログラミングのスパゲッティ」を
//! Graphite (`graph_schema!`/`graph!`) で倒す実証example。
//!
//! 詳しい経緯・設計判断は `README.md` を参照。このファイルは
//! `README.md` の構成 (敵の紹介 → グラフによる再定式化 → 対応表) を
//! そのまま実行可能な物語として再演する。各章の本体は章ごとのモジュール
//! (`antipattern_chapter`・`sheet_chapter`・`propagation_chapter`) にあり、
//! このファイルは章を並べる順序だけを持つ。

mod antipattern_chapter;
mod propagation_chapter;
mod sheet_chapter;

fn main() {
    antipattern_chapter::demonstrate_observer_problems();
    let mut engine = sheet_chapter::declare_sheet_and_build_engine();
    propagation_chapter::demonstrate_propagation(&mut engine);
    sheet_chapter::demonstrate_cycle_rejection();
}
