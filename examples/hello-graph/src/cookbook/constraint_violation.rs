//! §3「検証エラーを受ける」のうち、`where` 制約を満たさない違反。
//!
//! `each 役割名: N` の本数違反と `unique pair` の平行辺禁止違反を受け取る。
//! キーは全て正しいがグラフの形が宣言に合わない、という種類の違反である。

use crate::Org;
use crate::Org::{BelongsTo, BelongsToId, PersonId, Reports, ReportsId, TeamId};
use crate::{Person, Team};

// やりたいこと: `each member: 1` を満たさない (0本の) エッジは役割名つきの違反になる。
pub fn each違反を受け取る() {
    let result: Result<Org::Graph, Org::Violation> = Org::Graph::create(|b: &mut Org::Builder| {
        b.person(
            PersonId("alice".to_string()),
            Person {
                name: "Alice".to_string(),
            },
        );
        // aliceをどのチームにも所属させない (BelongsTo は each member: 1)
    });
    match result {
        Err(Org::Violation::BelongsToMemberEachViolation { source, count }) => {
            println!("(違反) each違反: {source:?} は {count} 本 (期待は1本)");
        }
        _ => panic!("each違反が検出されるはず"),
    }
}

// やりたいこと: `unique pair` の対に2本目を張ると `{Kind}UniquePairViolation` になる。
pub fn unique_pair違反を受け取る() {
    let result: Result<Org::Graph, Org::Violation> = Org::Graph::create(|b: &mut Org::Builder| {
        b.person(
            PersonId("alice".to_string()),
            Person {
                name: "Alice".to_string(),
            },
        );
        b.person(
            PersonId("bob".to_string()),
            Person {
                name: "Bob".to_string(),
            },
        );
        b.team(
            TeamId("eng".to_string()),
            Team {
                name: "Engineering".to_string(),
            },
        );
        // each member: 1 (BelongsTo) が先に違反しないよう、両者ともチームに所属させておく。
        b.belongs_to(
            BelongsToId("bt_alice".to_string()),
            BelongsTo::new(PersonId("alice".to_string()), TeamId("eng".to_string())),
        );
        b.belongs_to(
            BelongsToId("bt_bob".to_string()),
            BelongsTo::new(PersonId("bob".to_string()), TeamId("eng".to_string())),
        );
        b.reports(
            ReportsId("r1".to_string()),
            Reports::new(PersonId("alice".to_string()), PersonId("bob".to_string())),
        );
        b.reports(
            ReportsId("r2".to_string()),
            Reports::new(PersonId("alice".to_string()), PersonId("bob".to_string())),
        );
    });
    match result {
        Err(Org::Violation::ReportsUniquePairViolation { source, target }) => {
            println!("(違反) unique pair違反: {source:?} -> {target:?} に2本目");
        }
        _ => panic!("unique pair違反が検出されるはず"),
    }
}
