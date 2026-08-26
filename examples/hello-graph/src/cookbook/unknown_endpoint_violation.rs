//! §3「検証エラーを受ける」のうち、宣言していないキーへ辺を張った違反。
//!
//! 始点側が未宣言のときと終点側が未宣言のときで、別々の違反として役割が
//! 区別されることを受け取る。

use crate::Org;
use crate::Org::{BelongsTo, BelongsToId, PersonId, TeamId};
use crate::{Person, Team};

// やりたいこと: 未宣言の始点キーからエッジを張ると `{Kind}UnknownSource` 違反になる。
pub fn 未知の始点キーの違反を受け取る() {
    let result: Result<Org::Graph, Org::Violation> = Org::Graph::create(|b: &mut Org::Builder| {
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
    match result {
        Err(Org::Violation::BelongsToUnknownSource { edge, source }) => {
            println!("(違反) 未知の始点キー: 辺={edge:?} 始点={source:?}");
        }
        _ => panic!("未知の始点キー違反が検出されるはず"),
    }
}

// やりたいこと: 未宣言の終点キーへエッジを張ると `{Kind}UnknownTarget` 違反になる。
pub fn 未知の終点キーの違反を受け取る() {
    let result: Result<Org::Graph, Org::Violation> = Org::Graph::create(|b: &mut Org::Builder| {
        b.person(
            PersonId("alice".to_string()),
            Person {
                name: "Alice".to_string(),
            },
        );
        b.belongs_to(
            BelongsToId("bt1".to_string()),
            BelongsTo::new(
                PersonId("alice".to_string()),
                TeamId("存在しないチーム".to_string()),
            ),
        );
    });
    match result {
        Err(Org::Violation::BelongsToUnknownTarget { edge, target }) => {
            println!("(違反) 未知の終点キー: 辺={edge:?} 終点={target:?}");
        }
        _ => panic!("未知の終点キー違反が検出されるはず"),
    }
}
