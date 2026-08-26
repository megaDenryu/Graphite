//! 物語の第1章 — 敵 (observer パターン) の実演。
//!
//! ダイヤモンド依存のグリッチ、購読登録順に依存する非決定性、循環購読が
//! 止まらないことを順に走らせる。

use reactive_cells::antipattern::{build_diamond_demo, build_infinite_loop_demo};
use reactive_cells::report;

pub fn demonstrate_observer_problems() {
    report::print_section("敵1: observer パターンのグリッチ (ダイヤモンド依存)");
    println!(
        "a→b, a→c, b→d, c→d という依存を「値が変わったら購読者へ通知する」\n\
         だけの素朴なセルで組む (b=a*2, c=a+100, d=b+c)。a に 5 を設定する。"
    );
    let diamond = build_diamond_demo(false);
    diamond.trigger(5.0);
    report::print_diamond_demo("結果", &diamond);
    println!(
        "  -> dは2回再計算された。1回目は「bは新しい値(10)・cはまだ古い値(0)」という\n\
         矛盾した中間状態 (d=10) を観測している。最終値(115)は正しいが、\n\
         その値を誰かが1回目のタイミングで読んでいたら間違った値を見ることになる。"
    );

    report::print_section("敵1つづき: 購読登録順を入れ替えると結果の過程が変わる");
    let swapped = build_diamond_demo(true);
    swapped.trigger(5.0);
    report::print_diamond_demo("登録順を入れ替えた結果", &swapped);
    println!("  -> 依存関係は同じなのに、コードの書き方(登録順)次第でグリッチの内容が変わる。");

    report::print_section("敵2: 循環購読は誰も気づかず回り続ける");
    let cap = 200;
    let actual = build_infinite_loop_demo(cap);
    report::print_infinite_loop_demo(cap, actual);
}
