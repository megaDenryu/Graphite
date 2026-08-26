//! 統合テスト: 定義済みの遷移が期待どおり進み、辺の属性も読めること。

use state_machine::fsm::{self, Event};
use state_machine::schema::OrderStateId;

fn id(s: &str) -> OrderStateId {
    OrderStateId(s.to_string())
}

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
    let (_, attrs) =
        fsm::cancel_details(&g, &id("paid")).expect("paidからのcancelは定義済みのはず");
    assert_eq!(attrs.reason, "発送前キャンセル");
    assert!(attrs.refund_required);
}

#[test]
fn refundの属性から監査ラベルを読める() {
    let g = fsm::build();
    let (_, attrs) =
        fsm::refund_details(&g, &id("shipped")).expect("shippedからのrefundは定義済みのはず");
    assert_eq!(attrs.audit_label, "AUDIT-REFUND-SHIPPED");
}
