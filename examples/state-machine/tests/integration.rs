//! state-machine の統合テスト。
//!
//! 「正常遷移」「不正遷移がErrになる」「決定性(多重度)」「検証アルゴリズム
//! による到達不能/行き止まり検出」の4カテゴリを一通り確認する。

use state_machine::fsm::{self, Event, TransitionError};
use state_machine::schema::{Cancel, Deliver, OrderStateId, Pay, Refund, Ship, Submit};
use state_machine::validate;

fn id(s: &str) -> OrderStateId {
    OrderStateId(s.to_string())
}

// ============================================================
// 正常遷移
// ============================================================

#[test]
fn 正常系ライフサイクルはdraftからdeliveredまで一直線に進める() {
    let g = fsm::build();
    let mut current = fsm::initial_state();
    for event in [Event::Submit, Event::Pay, Event::Ship, Event::Deliver] {
        current = fsm::step(&g, &current, event).expect("正常系の遷移は全て定義済みのはず");
    }
    assert_eq!(current, id("delivered"));
}

#[test]
fn draftからのsubmitはpending_paymentへ進む() {
    let g = fsm::build();
    let next = fsm::step(&g, &fsm::initial_state(), Event::Submit).unwrap();
    assert_eq!(next, id("pending_payment"));
}

#[test]
fn cancelの属性から理由と返金要否を読める() {
    let g = fsm::build();
    let (_, attrs) = fsm::cancel_details(&g, &id("paid")).expect("paidからのcancelは定義済みのはず");
    assert_eq!(attrs.reason, "発送前キャンセル");
    assert!(attrs.refund_required);
}

#[test]
fn refundの属性から監査ラベルを読める() {
    let g = fsm::build();
    let (_, attrs) = fsm::refund_details(&g, &id("shipped")).expect("shippedからのrefundは定義済みのはず");
    assert_eq!(attrs.audit_label, "AUDIT-REFUND-SHIPPED");
}

// ============================================================
// 不正遷移が型 (Result::Err) で返る
// ============================================================

#[test]
fn draftから直接deliverしようとするとerrになる() {
    let g = fsm::build();
    let result = fsm::step(&g, &fsm::initial_state(), Event::Deliver);
    assert_eq!(
        result,
        Err(TransitionError {
            state: id("draft"),
            event: Event::Deliver,
        })
    );
}

#[test]
fn deliveredに達した後はrefund以外の全イベントがerrになる() {
    let g = fsm::build();
    let delivered = id("delivered");
    for event in [
        Event::Submit,
        Event::Pay,
        Event::Ship,
        Event::Deliver,
        Event::Cancel,
    ] {
        assert!(
            fsm::step(&g, &delivered, event).is_err(),
            "delivered からの {event} は必ずErrのはず (refundだけが唯一の出口)"
        );
    }
    assert!(
        fsm::step(&g, &delivered, Event::Refund).is_ok(),
        "delivered からのrefund (配達後の返品) は定義済みのはず"
    );
}

#[test]
fn shippedに達した後はcancelできずrefundになる() {
    let g = fsm::build();
    let shipped = id("shipped");
    assert!(fsm::step(&g, &shipped, Event::Cancel).is_err(), "発送後のcancelは未定義のはず");
    assert!(fsm::step(&g, &shipped, Event::Refund).is_ok(), "発送後はrefundが可能なはず");
}

#[test]
fn 支払い前のdraftからrefundしようとするとerrになる() {
    let g = fsm::build();
    let result = fsm::step(&g, &id("draft"), Event::Refund);
    assert!(result.is_err(), "支払い前の状態からrefundは未定義のはず");
}

// ============================================================
// 決定性 (多重度 0..1 が保証すること)
// ============================================================

#[test]
fn 同じ状態と同じイベントは常に同じ遷移先を返す_決定性() {
    let g = fsm::build();
    let paid = id("paid");
    let first = fsm::step(&g, &paid, Event::Ship);
    let second = fsm::step(&g, &paid, Event::Ship);
    assert_eq!(first, second, "同じ(状態,イベント)からの遷移先は決定的であるはず");
}

#[test]
fn payのiterは各始点キーにつき1本ずつしか無い_多重度01の保証() {
    let g = fsm::build();
    let mut seen_sources = std::collections::HashSet::new();
    for (_id, edge) in Pay::iter(&g) {
        assert!(
            seen_sources.insert(edge.from().clone()),
            "同じ始点キー {:?} から2本以上のPay辺があってはならない (each 0..1 違反)",
            edge.from()
        );
    }
}

// ============================================================
// 検証アルゴリズム: 到達不能状態・行き止まり状態の検出
// ============================================================

#[test]
fn 正規のfsmは到達不能状態も行き止まり状態も無く健全() {
    let g = fsm::build();
    let report = validate::validate(&g, &fsm::initial_state(), &fsm::terminal_states());
    assert!(report.is_ok());
    assert!(report.unreachable.is_empty());
    assert!(report.dead_ends.is_empty());
}

#[test]
fn 正規のfsmはdraftから全7状態に到達できる() {
    let g = fsm::build();
    let report = validate::validate(&g, &fsm::initial_state(), &fsm::terminal_states());
    // 7状態 (draft/pending_payment/paid/shipped/delivered/cancelled/refunded)
    // 全てが到達可能であること (=到達不能リストが空であること) を件数でも確認する。
    assert_eq!(report.unreachable.len(), 0);
}

#[test]
fn 到達不能な状態を埋め込んだ変種は到達不能として検出される() {
    let g = fsm::build_with_unreachable_state();
    let report = validate::validate(&g, &fsm::initial_state(), &fsm::terminal_states());
    assert!(
        report.unreachable.contains(&id("held_for_review")),
        "held_for_reviewはどこからも到達不能として検出されるはず"
    );
    assert!(
        report.dead_ends.is_empty(),
        "held_for_reviewはcancelへの辺を持つため行き止まりではないはず"
    );
}

#[test]
fn 出口を書き忘れた状態を埋め込んだ変種は行き止まりとして検出される() {
    let g = fsm::build_with_dead_end_bug();
    let report = validate::validate(&g, &fsm::initial_state(), &fsm::terminal_states());
    assert!(
        report.dead_ends.contains(&id("shipped")),
        "shippedは出口が無いので行き止まりとして検出されるはず"
    );
    assert!(
        report.unreachable.is_empty(),
        "この変種では到達不能状態は生じない設計のはず"
    );
}

#[test]
fn 終端状態集合に含まれる状態は正規fsmでは出て行く辺を持たない() {
    let g = fsm::build();
    for terminal in fsm::terminal_states() {
        assert!(Submit::of(&g, &terminal).is_none());
        assert!(Pay::of(&g, &terminal).is_none());
        assert!(Ship::of(&g, &terminal).is_none());
        assert!(Deliver::of(&g, &terminal).is_none());
        assert!(Cancel::of(&g, &terminal).is_none());
        assert!(Refund::of(&g, &terminal).is_none());
    }
}
