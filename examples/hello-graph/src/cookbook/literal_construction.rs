//! §3「構築」のうち、`graph!` リテラルで組み立てる3通りの書き方。
//!
//! ノード式・エッジをその場に書く形、グラフの外で作ったノード値を渡す形、
//! エッジの積み荷も外で作って渡す形を1関数ずつ並べる。

use crate::Org;
use crate::{BossEdge, Person, ReviewEdge, Team};

// やりたいこと: graph! にノード式・エッジをそのまま書いて組み立てる (最も基本の書き方)。
// この g を以降の「ノードを読む」「エッジを辿る」「一覧する」節で使い回す。
pub fn インライン式でgraphリテラルを組み立てる() -> Org::Graph {
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
        alice_bob_friends = Friends(alice -- bob),
    })
    .expect("正常なグラフは構築に成功するはず");
    let alice_person: Org::PersonRef<'_> = g.alice();
    println!("(構築1: インライン式) alice = {}", alice_person.name);
    g.into_graph()
}

// やりたいこと: グラフの外で作った値を graph! にそのまま渡す (`alice = alice_value` の形)。
pub fn 外部変数を渡してgraphリテラルを組み立てる() {
    let alice_value: Person = Person {
        name: "Alice".to_string(),
    };
    let eng_value: Team = Team {
        name: "Engineering".to_string(),
    };
    #[rustfmt::skip]
    let g = graphite::graph!(Org {
        alice = alice_value,
        eng   = eng_value,
        alice_dept = BelongsTo(alice -> eng),
    })
    .expect("外部変数を渡した graph! も構築に成功するはず");
    let alice_person: Org::PersonRef<'_> = g.alice();
    println!("(構築2: 外部変数渡し) alice = {}", alice_person.name);
}

// やりたいこと: エッジの積み荷 (`BossEdge`) もグラフの外で作った値を渡せることを確認する。
pub fn 外部で作ったエッジ属性をgraphリテラルに渡す() {
    let promotion: BossEdge = BossEdge { since: 2019 };
    #[rustfmt::skip]
    let g = graphite::graph!(Org {
        alice = Person { name: "Alice".into() },
        bob   = Person { name: "Bob".into() },
        eng   = Team { name: "Engineering".into() },
        alice_dept = BelongsTo(alice -> eng),
        bob_dept   = BelongsTo(bob -> eng),
        bob_boss   = Boss(bob -[promotion]-> alice),
    })
    .expect("外部エッジ属性を渡した graph! も構築に成功するはず");
    let boss: Org::BossRef<'_> = g.bob_boss();
    println!(
        "(構築3: 外部エッジ属性渡し) bob の上司就任年 = {}",
        boss.payload().since
    );
}
