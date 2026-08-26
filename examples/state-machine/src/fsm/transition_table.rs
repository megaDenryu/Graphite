//! 正規の注文ライフサイクルの遷移表そのもの (`graph!` リテラル) と、
//! 初期状態・終端状態の宣言。

use crate::schema::{CancelEdge, OrderFsm, OrderState, OrderStateId, RefundEdge};

/// 初期状態のキー。
pub fn initial_state() -> OrderStateId {
    OrderStateId("draft".to_string())
}

/// 終端状態 (「意図的に出口が無い」と設計者が宣言している状態) のキー一覧。
/// [`crate::validate::validate`] はここに載っていない状態の出て行く辺が
/// 0本だったら「定義漏れの疑いがある」と報告する。
///
/// `delivered` はここに含めない — 配達後も `refund` (返品) 1本だけ出口を
/// 持つ設計にしているため、`delivered` は「出口が0本」ではなく「出口が
/// 1本 (refundのみ)」の状態であり、そもそも行き止まり判定の対象外
/// (`out_neighbors` が空ではないので検査自体が引っかからない)。
pub fn terminal_states() -> Vec<OrderStateId> {
    vec![
        OrderStateId("cancelled".to_string()),
        OrderStateId("refunded".to_string()),
    ]
}

/// 正規の注文ライフサイクル遷移グラフを構築する。
///
/// これが「遷移表」に相当する一枚絵 — 状態と遷移がここに宣言的に並ぶ
/// (README §3 の「グラフによる再定式化」)。辺キーは端点+イベント名から
/// 読める名前 (`submit_draft` = `draft` からの `Submit`) にしている。
///
/// 遷移の意味:
/// - `draft -[Submit]-> pending_payment -[Pay]-> paid -[Ship]-> shipped -[Deliver]-> delivered`
///   という正常経路 (直線)。
/// - `Cancel` は発送前 (draft/pending_payment/paid) からのみ可能。
///   発送後 (shipped/delivered) には `Cancel` は無く、代わりに `Refund` を使う
///   (現実の EC システムでよくある区別: 未発送は取消、発送後は返金)。
/// - `Refund` は支払い済み以降 (paid/shipped/delivered) からのみ可能。
pub fn build() -> OrderFsm::Graph {
    #[rustfmt::skip]
    let g: OrderFsm::Graph = graphite::graph!(OrderFsm {
        draft           = OrderState { label: "draft".into() },
        pending_payment = OrderState { label: "pending_payment".into() },
        paid            = OrderState { label: "paid".into() },
        shipped         = OrderState { label: "shipped".into() },
        delivered       = OrderState { label: "delivered".into() },
        cancelled       = OrderState { label: "cancelled".into() },
        refunded        = OrderState { label: "refunded".into() },

        submit_draft    = Submit(draft -> pending_payment),
        pay_pending     = Pay(pending_payment -> paid),
        ship_paid       = Ship(paid -> shipped),
        deliver_shipped = Deliver(shipped -> delivered),

        cancel_draft   = Cancel(draft -[CancelEdge { reason: "顧客都合キャンセル".into(), refund_required: false }]-> cancelled),
        cancel_pending = Cancel(pending_payment -[CancelEdge { reason: "支払い期限切れ".into(), refund_required: false }]-> cancelled),
        cancel_paid    = Cancel(paid -[CancelEdge { reason: "発送前キャンセル".into(), refund_required: true }]-> cancelled),

        refund_paid      = Refund(paid -[RefundEdge { audit_label: "AUDIT-REFUND-PAID".into() }]-> refunded),
        refund_shipped   = Refund(shipped -[RefundEdge { audit_label: "AUDIT-REFUND-SHIPPED".into() }]-> refunded),
        refund_delivered = Refund(delivered -[RefundEdge { audit_label: "AUDIT-REFUND-DELIVERED".into() }]-> refunded),
    })
    .expect("正規のFSM定義は構築に成功するはず")
    .into_graph();
    g
}
