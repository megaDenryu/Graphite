//! §3「エッジを辿る」— 役割探索・端点対検索の戻り型が `where` 制約で決まること。
//!
//! `each 1` → 直接参照、`each 0..1` → `Option`、制約なし → イテレータ、
//! `unique pair` → `between` が `Option`、無向辺 → `endpoints`/`incident` の対称性、
//! という宣言と戻り型の対応を1関数ずつ実演する。

use crate::Org;
use crate::Org::{FriendsId, PersonId};

// やりたいこと: `each member: 1` の役割クエリは辺参照を直接返す。
pub fn each_1の役割探索は直接参照を返す(g: &Org::Graph) {
    let alice = g.person_by_id(&PersonId("alice".to_string())).unwrap();
    let membership: Org::BelongsToRef<'_> = alice.belongs_to_as_member();
    println!(
        "(each 1) alice.belongs_to_as_member() = {}",
        membership.team().name
    );

    let unknown = g.person_by_id(&PersonId("dave".to_string()));
    println!("g.person_by_id(&dave) = {unknown:?}");
}

// やりたいこと: `each subordinate: 0..1` の役割クエリは `Option<EdgeRef>` を返す。
pub fn each_0か1の役割探索はoptionを返す(g: &Org::Graph) {
    let bob = g.person_by_id(&PersonId("bob".to_string())).unwrap();
    let boss: Option<Org::BossRef<'_>> = bob.boss_as_subordinate();
    if let Some(edge) = boss {
        println!(
            "(each 0..1) bob.boss_as_subordinate() = {} (就任年: {})",
            edge.superior().name,
            edge.payload().since
        );
    }
    let alice = g.person_by_id(&PersonId("alice".to_string())).unwrap();
    let no_boss = alice.boss_as_subordinate();
    println!(
        "(each 0..1) alice.boss_as_subordinate() で値が無い = {}",
        no_boss.is_none()
    );
}

// やりたいこと: `unique pair` のエッジは `between` が `Option` を返す
// (同じ対に2本目を張れないので「あるかないか」で十分)。
pub fn unique_pairのbetweenはoptionを返す(g: &Org::Graph) {
    let alice = g.person_by_id(&PersonId("alice".to_string())).unwrap();
    let bob = g.person_by_id(&PersonId("bob".to_string())).unwrap();
    let r: Option<Org::ReportsRef<'_>> = alice.reports_between(bob);
    println!("(unique pair) alice.reports_between(bob) = {}", r.is_some());
    let none = bob.reports_between(alice);
    println!(
        "(unique pair) bob.reports_between(alice) = {} (逆向きは無い)",
        none.is_some()
    );
}

// やりたいこと: 制約なしの役割クエリは `EdgeRef` の iterator を返す。
pub fn 制約なしの役割探索はvecを返す(g: &Org::Graph) {
    let bob = g.person_by_id(&PersonId("bob".to_string())).unwrap();
    for edge in bob.reviewed_by_as_reviewee() {
        println!(
            "(制約なし) bob.reviewed_by_as_reviewee() に {} ({}年度) が含まれる",
            edge.reviewer().name,
            edge.payload().year
        );
    }
}

// やりたいこと: 無向辺 (`Friends`) は方向を示すアクセサを持たず、
// `endpoints() -> (PersonRef, PersonRef)` を持つ (`docs/edge_endpoints_v4_1.md`
// §2)。位置0/1は `Friends(alice -- bob)` と書いた際の記述順そのままだが、
// 意味論としては順序なし対であることに注意 (次の関数で確認する)。
pub fn 無向辺のendpointsアクセサで両端を読む(g: &Org::Graph) {
    let friends_id = FriendsId("alice_bob_friends".to_string());
    let edge: Org::FriendsRef<'_> = g.friends_by_id(&friends_id).unwrap();
    let (p0, p1) = edge.endpoints();
    println!(
        "(無向) g.friends_by_id(&alice_bob_friends).endpoints() = ({:?}, {:?})",
        p0.id(),
        p1.id()
    );
}

// やりたいこと: `incident`/`between` はどちらの位置に置かれても対称に辿れる。
// `unique pair` の同値判定も順序を無視する (`alice -- bob` と `bob -- alice`
// は同じ対)。
pub fn 無向辺の接続探索と端点対検索は対称に辿れる(g: &Org::Graph) {
    let alice = g.person_by_id(&PersonId("alice".to_string())).unwrap();
    let bob = g.person_by_id(&PersonId("bob".to_string())).unwrap();

    for edge in bob.friends_incident() {
        let (a, b) = edge.endpoints();
        let friend = if a.id() == bob.id() { b } else { a };
        println!(
            "(無向) bob.friends_incident() に {} が含まれる (aliceが位置0でも辿れる)",
            friend.name
        );
    }

    let forward: Option<Org::FriendsRef<'_>> = alice.friends_between(bob);
    let backward: Option<Org::FriendsRef<'_>> = bob.friends_between(alice);
    println!(
        "(無向) between(alice, bob) = {:?} / between(bob, alice) = {:?} (順序を無視して同じ辺)",
        forward.map(|edge| edge.id()),
        backward.map(|edge| edge.id())
    );
}
