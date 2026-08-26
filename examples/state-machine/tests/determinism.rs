//! 統合テスト: 多重度 `each before: 0..1` が遷移の決定性を保証すること。

use state_machine::fsm::{self, Event};
use state_machine::schema::OrderStateId;

fn id(s: &str) -> OrderStateId {
    OrderStateId(s.to_string())
}

#[test]
fn 同じ状態と同じイベントは常に同じ遷移先を返す_決定性() {
    let g = fsm::build();
    let paid = id("paid");
    let first = fsm::step(&g, &paid, Event::Ship);
    let second = fsm::step(&g, &paid, Event::Ship);
    assert_eq!(
        first, second,
        "同じ(状態,イベント)からの遷移先は決定的であるはず"
    );
}

#[test]
fn payのiterは各始点キーにつき1本ずつしか無い_多重度01の保証() {
    let g = fsm::build();
    let mut seen_sources = std::collections::HashSet::new();
    for edge in g.pay_iter() {
        let before = edge.before().id();
        assert!(
            seen_sources.insert(before.clone()),
            "同じ始点キー {:?} から2本以上のPay辺があってはならない (each 0..1 違反)",
            before
        );
    }
}
