//! 統合テスト: 未定義の遷移が `Result::Err` として型で返ること。

use state_machine::fsm::{self, Event, TransitionError};
use state_machine::schema::OrderStateId;

fn id(s: &str) -> OrderStateId {
    OrderStateId(s.to_string())
}

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
    assert!(
        fsm::step(&g, &shipped, Event::Cancel).is_err(),
        "発送後のcancelは未定義のはず"
    );
    assert!(
        fsm::step(&g, &shipped, Event::Refund).is_ok(),
        "発送後はrefundが可能なはず"
    );
}

#[test]
fn 支払い前のdraftからrefundしようとするとerrになる() {
    let g = fsm::build();
    let result = fsm::step(&g, &id("draft"), Event::Refund);
    assert!(result.is_err(), "支払い前の状態からrefundは未定義のはず");
}
