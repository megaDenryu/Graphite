//! §3・§5 の実演が主張していることを、表示ではなくアサーションで固定するテスト。
//!
//! 全テストが共有する題材のグラフをここで1つ組み立て、確かめる内容ごとに
//! サブモジュールへ分けている。

mod constraint_violation;
mod edge_value;
mod flow;
mod key_violation;
mod traversal;

use crate::Org;
use crate::{BossEdge, Person, ReviewEdge, Team};

fn build() -> Org::Graph {
    #[rustfmt::skip]
    let g = graphite::graph!(Org {
        alice = Person { name: "Alice".into() },
        bob   = Person { name: "Bob".into() },
        carol = Person { name: "Carol".into() },
        eng   = Team { name: "Engineering".into() },

        alice_dept = BelongsTo(alice -> eng),
        bob_dept   = BelongsTo(bob -> eng),
        carol_dept = BelongsTo(carol -> eng),
        bob_boss   = Boss(bob -[BossEdge { since: 2021 }]-> alice),
        alice_reports_bob   = Reports(alice -> bob),
        alice_reports_carol = Reports(alice -> carol),
        review_2023 = ReviewedBy(bob -[ReviewEdge { year: 2023 }]-> alice),
        review_2024 = ReviewedBy(bob -[ReviewEdge { year: 2024 }]-> carol),
    });
    g.expect("正常なグラフは構築に成功するはず").into_graph()
}
