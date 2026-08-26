//! §3「検証エラーを受ける」のうち、同じキーを2回宣言した違反。
//!
//! ノードキーの重複と、v4で辺も第一級のキー付き要素になったことで生えた
//! 辺キーの重複を受け取る。

use crate::Org;
use crate::Org::{BelongsTo, BelongsToId, PersonId, TeamId};
use crate::{Person, Team};

// やりたいこと: 同じキーを2回宣言すると `Duplicate{Node}` 違反になることを確認する。
pub fn 重複ノードキーの違反を受け取る() {
    let result: Result<Org::Graph, Org::Violation> = Org::Graph::create(|b: &mut Org::Builder| {
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
    match result {
        Err(Org::Violation::DuplicatePerson(id)) => println!("(違反) 重複ノードキー: {id:?}"),
        _ => panic!("重複ノードキー違反が検出されるはず"),
    }
}

// やりたいこと: v4で新規追加された「辺キーの重複」も検出できることを確認する
// (辺も第一級のキー付き要素になったため)。
pub fn 辺キー重複の違反を受け取る() {
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
        b.belongs_to(
            BelongsToId("dup".to_string()),
            BelongsTo::new(PersonId("alice".to_string()), TeamId("eng".to_string())),
        );
        b.belongs_to(
            BelongsToId("dup".to_string()),
            BelongsTo::new(PersonId("bob".to_string()), TeamId("eng".to_string())),
        );
    });
    match result {
        Err(Org::Violation::BelongsToDuplicateKey(id)) => println!("(違反) 辺キー重複: {id:?}"),
        _ => panic!("辺キー重複違反が検出されるはず"),
    }
}
