//! §3「ノードを読む」— 公開IDからノード参照を1件引く。
//!
//! `g.{型名}_by_id(&id)` の形がノード種別によらず共通であること、および
//! `graph!` の左辺名が手で組み立てたID newtype と同一視されることを確かめる。

use crate::Org;
use crate::Org::{PersonId, TeamId};

// やりたいこと: `g.{type}_by_id(&id)` で1件読む (無ければ None)。
pub fn 人ノードを1件読む(g: &Org::Graph) {
    let alice: Option<Org::PersonRef<'_>> = g.person_by_id(&PersonId("alice".to_string()));
    if let Some(person) = alice {
        println!("(ノード) g.person_by_id(&alice) = {}", person.name);
    }
    let unknown: Option<Org::PersonRef<'_>> = g.person_by_id(&PersonId("dave".to_string()));
    println!("(ノード) g.person_by_id(&dave) = {unknown:?}");
}

// やりたいこと: `g.team_by_id` も同じ形。ノード型が違っても命名規則は共通。
pub fn チームノードを1件読む(g: &Org::Graph) {
    let eng: Option<Org::TeamRef<'_>> = g.team_by_id(&TeamId("eng".to_string()));
    if let Some(team) = eng {
        println!("(ノード) g.team_by_id(&eng) = {}", team.name);
    }
}

// やりたいこと: `PersonId` はただの newtype なので手で組み立てられる。graph! の
// キー (`alice = ..`) はこの `PersonId("alice".to_string())` と同一視される。
pub fn personidの作り方とgraphのキーの対応を確認する(g: &Org::Graph) {
    let hand_built_id: PersonId = PersonId("alice".to_string());
    let alice: Org::PersonRef<'_> = g
        .person_by_id(&hand_built_id)
        .expect("graph!のキーaliceがPersonId(\"alice\")と一致するはず");
    println!(
        "(型) PersonId(\"alice\".to_string()) で graph! の alice = {} が引ける",
        alice.name
    );
}
