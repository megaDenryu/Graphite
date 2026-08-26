//! `where` 制約の違反が検出されること、および複数件をまとめて受け取れることのテスト。

use crate::Org;
use crate::Org::{BelongsTo, BelongsToId, PersonId, Reports, ReportsId, TeamId};
use crate::{Person, Team};

#[test]
fn unique_pair違反が検出される() {
    let result = Org::Graph::create(|b| {
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
    assert!(matches!(
        result,
        Err(Org::Violation::ReportsUniquePairViolation { .. })
    ));
}

#[test]
fn create_collectingは複数の違反を集める() {
    let result = Org::Graph::create_collecting(|b| {
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
    });
    let violations = match result {
        Err(violations) => violations,
        Ok(_) => panic!("2件の違反が集まるはず"),
    };
    assert_eq!(violations.len(), 2);
}
