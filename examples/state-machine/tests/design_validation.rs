//! 統合テスト: 到達不能状態・行き止まり状態の検出アルゴリズム。

use state_machine::fsm;
use state_machine::schema::OrderStateId;
use state_machine::validate;

fn id(spelling: &str) -> OrderStateId {
    OrderStateId(spelling.to_string())
}

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
        let terminal = g.order_state_by_id(&terminal).unwrap();
        assert!(terminal.submit_as_before().is_none());
        assert!(terminal.pay_as_before().is_none());
        assert!(terminal.ship_as_before().is_none());
        assert!(terminal.deliver_as_before().is_none());
        assert!(terminal.cancel_as_before().is_none());
        assert!(terminal.refund_as_before().is_none());
    }
}
