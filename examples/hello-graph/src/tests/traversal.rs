//! 探索APIの戻り型が `where` 制約どおりであることのテスト。
//!
//! 役割探索・端点対検索・一覧・1件読みが、宣言した制約に応じて直接参照・
//! `Option`・イテレータのどれを返すかを固定する。

use super::build;
use crate::Org;
use crate::Org::PersonId;

#[test]
fn each_1の役割探索は参照そのものを返す() {
    let g = build();
    let alice = g.person_by_id(&PersonId("alice".to_string())).unwrap();
    let edge = alice.belongs_to_as_member();
    assert_eq!(edge.team().name, "Engineering");
}

#[test]
fn each_0か1の役割探索はoptionのタプルを返し積み荷フィールドへアクセスできる() {
    let g = build();
    let bob = g.person_by_id(&PersonId("bob".to_string())).unwrap();
    let edge = bob.boss_as_subordinate().expect("bobの上司はaliceのはず");
    assert_eq!(edge.superior().name, "Alice");
    assert_eq!(edge.payload().since, 2021);
    let alice = g.person_by_id(&PersonId("alice".to_string())).unwrap();
    assert!(alice.boss_as_subordinate().is_none());
}

#[test]
fn 制約なしの役割探索はiteratorを返す() {
    let g = build();
    let bob = g.person_by_id(&PersonId("bob".to_string())).unwrap();
    let mut names: Vec<String> = bob
        .reviewed_by_as_reviewee()
        .map(|edge| edge.reviewer().name.clone())
        .collect();
    names.sort();
    assert_eq!(names, vec!["Alice".to_string(), "Carol".to_string()]);
}

#[test]
fn iterで表全体を列挙できる() {
    let g = build();
    let boss_pairs: Vec<Org::BossRef<'_>> = g.boss_iter().collect();
    assert_eq!(boss_pairs.len(), 1);
    let edge = boss_pairs[0];
    assert_eq!(edge.subordinate().id(), &PersonId("bob".to_string()));
    assert_eq!(edge.superior().id(), &PersonId("alice".to_string()));
    assert_eq!(edge.payload().since, 2021);
}

#[test]
fn person_getで1件読める() {
    let g = build();
    assert_eq!(
        g.person_by_id(&PersonId("alice".to_string())).unwrap().name,
        "Alice"
    );
    assert!(g.person_by_id(&PersonId("dave".to_string())).is_none());
}

#[test]
fn reports_betweenはunique_pairなのでoptionを返す() {
    let g = build();
    let alice = g.person_by_id(&PersonId("alice".to_string())).unwrap();
    let bob = g.person_by_id(&PersonId("bob".to_string())).unwrap();
    assert!(alice.reports_between(bob).is_some());
    assert!(bob.reports_between(alice).is_none());
}

#[test]
fn review_の役割探索は制約なしでvecのタプルを返す() {
    let g = build();
    let bob = g.person_by_id(&PersonId("bob".to_string())).unwrap();
    let reviewers: Vec<_> = bob.reviewed_by_as_reviewee().collect();
    assert_eq!(reviewers.len(), 2);
    assert!(reviewers
        .iter()
        .any(|edge| edge.reviewer().name == "Alice" && edge.payload().year == 2023));
    assert!(reviewers
        .iter()
        .any(|edge| edge.reviewer().name == "Carol" && edge.payload().year == 2024));
}

#[test]
fn lenで辺の本数を確認できる() {
    let g = build();
    assert_eq!(g.belongs_to_len(), 3);
    assert_eq!(g.reports_len(), 2);
}
