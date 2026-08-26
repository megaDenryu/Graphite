//! キーそのものが壊れている違反が検出されることのテスト。
//!
//! ノードキーの重複・辺キーの重複・未宣言の始点キーを対象にする。

use crate::Org;
use crate::Org::{BelongsTo, BelongsToId, PersonId, TeamId};
use crate::{Person, Team};

#[test]
fn 重複ノードキーはduplicate違反になる() {
    let result = Org::Graph::create(|b| {
        b.person(
            PersonId("alice".to_string()),
            Person {
                name: "Alice".to_string(),
            },
        );
        b.person(
            PersonId("alice".to_string()),
            Person {
                name: "Alice2".to_string(),
            },
        );
    });
    assert!(matches!(result, Err(Org::Violation::DuplicatePerson(_))));
}

#[test]
fn 辺キー重複はduplicatekey違反になる() {
    let result = Org::Graph::create(|b| {
        b.person(
            PersonId("alice".to_string()),
            Person {
                name: "Alice".to_string(),
            },
        );
        b.team(
            TeamId("eng".to_string()),
            Team {
                name: "Engineering".to_string(),
            },
        );
        b.belongs_to(
            BelongsToId("dup".to_string()),
            BelongsTo::new(PersonId("alice".to_string()), TeamId("eng".to_string())),
        );
        b.belongs_to(
            BelongsToId("dup".to_string()),
            BelongsTo::new(PersonId("alice".to_string()), TeamId("eng".to_string())),
        );
    });
    assert!(matches!(
        result,
        Err(Org::Violation::BelongsToDuplicateKey(_))
    ));
}

#[test]
fn 未知の始点キーはunknownsource違反になる() {
    let result = Org::Graph::create(|b| {
        b.team(
            TeamId("eng".to_string()),
            Team {
                name: "Engineering".to_string(),
            },
        );
        b.belongs_to(
            BelongsToId("bt1".to_string()),
            BelongsTo::new(
                PersonId("存在しない社員".to_string()),
                TeamId("eng".to_string()),
            ),
        );
    });
    assert!(matches!(
        result,
        Err(Org::Violation::BelongsToUnknownSource { .. })
    ));
}
