//! 検証アルゴリズムの実演用に、意図的に壊してある遷移表の変種2種。

use crate::schema::{CancelEdge, OrderFsm, OrderState, RefundEdge};

// 検証デモ用: 「実装したつもりで実は初期状態から繋がっていない状態」を
// 埋め込んだ壊れた変種 (`crate::validate::validate` の到達不能検出デモ用)。
//
// `held_for_review` という状態を追加しているが、どの既存状態からも
// `held_for_review` へ向かう辺を張り忘れている (= コードは書いたが
// どこからも呼ばれないデッドコードと同種のバグ)。`held_for_review` 自身は
// `cancelled` への `Cancel` 辺を持つので行き止まりではない — 「到達不能」
// と「行き止まり」が別の問題であることも同時に示す。
pub fn build_with_unreachable_state() -> OrderFsm::Graph {
    #[rustfmt::skip]
    let g: OrderFsm::Graph = graphite::graph!(OrderFsm {
        draft           = OrderState { label: "draft".into() },
        pending_payment = OrderState { label: "pending_payment".into() },
        paid            = OrderState { label: "paid".into() },
        shipped         = OrderState { label: "shipped".into() },
        delivered       = OrderState { label: "delivered".into() },
        cancelled       = OrderState { label: "cancelled".into() },
        refunded        = OrderState { label: "refunded".into() },
        held_for_review = OrderState { label: "held_for_review".into() },

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

        // held_for_review へ入る辺が無い (書いたつもりで繋げ忘れた想定)。
        cancel_held = Cancel(held_for_review -[CancelEdge { reason: "審査により保留後キャンセル".into(), refund_required: true }]-> cancelled),
    })
    .expect("辺の端点は全てノードキーとして宣言済みなので構築自体は成功する")
    .into_graph();
    g
}

// 検証デモ用: 「`shipped` に進んだ後の出口 (deliver も refund も) を
// 定義し忘れた」壊れた変種 (`crate::validate::validate` の行き止まり
// 検出デモ用)。
//
// `delivered` 状態そのものを削り、`shipped` からの `Deliver`/`Refund` の
// 両方の辺を落としている。`shipped` は `draft` から `Submit -> Pay -> Ship`
// で到達可能 (到達不能ではない) だが、そこから先へ進む辺が1本も無く、かつ
// `shipped` は終端状態リスト (`terminal_states`) にも入っていない —
// 「発送したら中で永遠に止まる注文」というバグが、到達不能とは独立に
// 構造だけから検出できることを示す (到達不能側の状態は一切生じないよう
// `held_for_review` のような追加ノードは置いていない)。
pub fn build_with_dead_end_bug() -> OrderFsm::Graph {
    #[rustfmt::skip]
    let g: OrderFsm::Graph = graphite::graph!(OrderFsm {
        draft           = OrderState { label: "draft".into() },
        pending_payment = OrderState { label: "pending_payment".into() },
        paid            = OrderState { label: "paid".into() },
        shipped         = OrderState { label: "shipped".into() },
        cancelled       = OrderState { label: "cancelled".into() },
        refunded        = OrderState { label: "refunded".into() },

        submit_draft = Submit(draft -> pending_payment),
        pay_pending  = Pay(pending_payment -> paid),
        ship_paid    = Ship(paid -> shipped),
        // Deliver(shipped -> delivered) / Refund(shipped -> refunded) を
        // 両方書き忘れた想定 (shipped から出る辺が無い)。

        cancel_draft   = Cancel(draft -[CancelEdge { reason: "顧客都合キャンセル".into(), refund_required: false }]-> cancelled),
        cancel_pending = Cancel(pending_payment -[CancelEdge { reason: "支払い期限切れ".into(), refund_required: false }]-> cancelled),
        cancel_paid    = Cancel(paid -[CancelEdge { reason: "発送前キャンセル".into(), refund_required: true }]-> cancelled),

        refund_paid = Refund(paid -[RefundEdge { audit_label: "AUDIT-REFUND-PAID".into() }]-> refunded),
    })
    .expect("辺の端点は全てノードキーとして宣言済みなので構築自体は成功する")
    .into_graph();
    g
}
