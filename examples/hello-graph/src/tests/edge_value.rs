//! 辺値型がマクロの内部表現ではなく公開structとして実在することのテスト。
//!
//! `graph!` を通さずに名前付きフィールドで直接構築できることを示す
//! (§2.5「辺は名前付きフィールドの構造体として実在する」の実測)。

use crate::BossEdge;
use crate::Org::{BelongsTo, Boss, PersonId, TeamId};

#[test]
fn 名前付きフィールドの辺値はマクロ外でも普通に構築できる() {
    let e = BelongsTo {
        member: PersonId("alice".to_string()),
        team: TeamId("eng".to_string()),
    };
    assert_eq!(e.member, PersonId("alice".to_string()));
    assert_eq!(e.team, TeamId("eng".to_string()));

    let b = Boss {
        subordinate: PersonId("bob".to_string()),
        superior: PersonId("alice".to_string()),
        appointment: BossEdge { since: 2020 },
    };
    assert_eq!(b.payload().since, 2020);
}
