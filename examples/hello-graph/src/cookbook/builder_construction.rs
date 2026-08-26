//! §3「構築」のうち、`graph!` を使わず builder を直接呼ぶ2通りの書き方。
//!
//! 型名つきメソッド (`b.person(id, value)`) と、値の型から振り分ける総称
//! メソッド (`b.insert`/`b.add`) を1関数ずつ並べる。

use crate::Org;
use crate::Org::{BelongsTo, BelongsToId, PersonId, TeamId};
use crate::{Person, Team};

// やりたいこと: graph! を使わず、builder の型名つきメソッド (`b.person(id, value)`) で組み立てる。
pub fn builderの型名メソッドで組み立てる() {
    let g: Org::Graph = Org::Graph::create(|b: &mut Org::Builder| {
        b.person(
            PersonId("dave".to_string()),
            Person {
                name: "Dave".to_string(),
            },
        );
        b.team(
            TeamId("sales".to_string()),
            Team {
                name: "Sales".to_string(),
            },
        );
        b.belongs_to(
            BelongsToId("dave_dept".to_string()),
            BelongsTo::new(PersonId("dave".to_string()), TeamId("sales".to_string())),
        );
    })
    .expect("builder の型名メソッドでも構築に成功するはず");
    let dave: Org::PersonRef<'_> = g.person_by_id(&PersonId("dave".to_string())).unwrap();
    println!("(構築4: builderの型名メソッド) dave = {}", dave.name);
}

// やりたいこと: builder の総称メソッド `insert`/`add` に値を渡し、値の型から自動で
// 振り分けさせる (`insert` の型境界 `N: OrgNode`、`add` の型境界 `E: OrgEdge` は
// graph_schema! が生成したトレイトで満たされる。利用者がこのトレイトを直接呼ぶことは無い)。
pub fn builderの総称insertとaddで組み立てる() {
    let g: Org::Graph = Org::Graph::create(|b: &mut Org::Builder| {
        let eve_id: PersonId = b.insert(
            "eve",
            Person {
                name: "Eve".to_string(),
            },
        );
        let sales_id: TeamId = b.insert(
            "sales",
            Team {
                name: "Sales".to_string(),
            },
        );
        let _dept_id: BelongsToId =
            b.add("eve_dept", BelongsTo::new(eve_id.clone(), sales_id.clone()));
    })
    .expect("insert/add 経由の構築も成功するはず");
    let eve: Org::PersonRef<'_> = g.person_by_id(&PersonId("eve".to_string())).unwrap();
    println!("(構築5: builderの総称insert/add) eve = {}", eve.name);
}
