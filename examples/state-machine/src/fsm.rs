//! 遷移グラフの具体的な定義 (`graph!` リテラル) と遷移エンジン (`step`)。
//!
//! [`build`] が定義する `graph!` リテラルそのものが「遷移表」であり、
//! ドキュメントとコードが乖離しない (README §2 の「なぜ死ぬか」への回答)。
//! [`step`] はイベントの `match` でエッジビューを引くだけで、遷移規則の
//! 実体は `match` の腕には一切無い — 全部 `build` の1箇所にしかない。

use std::fmt;

use crate::schema::{CancelEdge, OrderFsm, OrderState, OrderStateId, RefundEdge};

/// FSM が受理するイベント一覧。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    Submit,
    Pay,
    Ship,
    Deliver,
    Cancel,
    Refund,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Event::Submit => "submit",
            Event::Pay => "pay",
            Event::Ship => "ship",
            Event::Deliver => "deliver",
            Event::Cancel => "cancel",
            Event::Refund => "refund",
        };
        write!(f, "{s}")
    }
}

/// `state` の状態で `event` に対応する遷移が定義されていないことを表す。
///
/// bool フラグ持ち・enum+match 散在の設計では「その状態でそのイベントは
/// 無効」ということが実行時まで (最悪本番まで) 分からない。ここでは
/// `OrderFsm` が持つ遷移表を引いた結果として型で返るので、呼び出し側は
/// 必ず `Result` を処理しなければならない。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransitionError {
    pub state: OrderStateId,
    pub event: Event,
}

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "状態 {:?} でイベント `{}` は定義されていません",
            self.state, self.event
        )
    }
}

impl std::error::Error for TransitionError {}

/// 遷移エンジン本体。
///
/// イベントの `match` でエッジビューを引くだけ (`{event}().id_of(current)`)。
/// 多重度 `(0..1)` なので戻り値は `Option<&OrderStateId>` — 「その状態で
/// そのイベントは未定義」を `None` としてそのまま `TransitionError` に
/// 変換できる。遷移規則そのものはここには一切書かれていない (schema と
/// `build` にしか無い) — enum+match 散在アンチパターンとの決定的な違い。
pub fn step(
    fsm: &OrderFsm,
    current: &OrderStateId,
    event: Event,
) -> Result<OrderStateId, TransitionError> {
    let next: Option<&OrderStateId> = match event {
        Event::Submit => fsm.submit().id_of(current),
        Event::Pay => fsm.pay().id_of(current),
        Event::Ship => fsm.ship().id_of(current),
        Event::Deliver => fsm.deliver().id_of(current),
        Event::Cancel => fsm.cancel().id_of(current),
        Event::Refund => fsm.refund().id_of(current),
    };
    next.cloned().ok_or_else(|| TransitionError {
        state: current.clone(),
        event,
    })
}

/// `cancel` イベントのガード条件・監査情報 (`CancelEdge`) も見たい場合は
/// ビューの `of` を直接使う (`step` はキーだけ返すため属性は運ばない)。
pub fn cancel_details<'a>(
    fsm: &'a OrderFsm,
    current: &OrderStateId,
) -> Option<(&'a OrderState, &'a CancelEdge)> {
    fsm.cancel().of(current)
}

/// `refund` イベントの監査ログ用ラベル (`RefundEdge`) を見たい場合。
pub fn refund_details<'a>(
    fsm: &'a OrderFsm,
    current: &OrderStateId,
) -> Option<(&'a OrderState, &'a RefundEdge)> {
    fsm.refund().of(current)
}

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
/// (README §3 の「グラフによる再定式化」)。
///
/// 遷移の意味:
/// - `draft -[submit]-> pending_payment -[pay]-> paid -[ship]-> shipped -[deliver]-> delivered`
///   という正常経路 (直線)。
/// - `cancel` は発送前 (draft/pending_payment/paid) からのみ可能。
///   発送後 (shipped/delivered) には `cancel` は無く、代わりに `refund` を使う
///   (現実の EC システムでよくある区別: 未発送は取消、発送後は返金)。
/// - `refund` は支払い済み以降 (paid/shipped/delivered) からのみ可能。
pub fn build() -> OrderFsm {
    #[rustfmt::skip]
    let g: OrderFsm = graphite::graph!(OrderFsm {
        draft           = OrderState { label: "draft".into() },
        pending_payment = OrderState { label: "pending_payment".into() },
        paid            = OrderState { label: "paid".into() },
        shipped         = OrderState { label: "shipped".into() },
        delivered       = OrderState { label: "delivered".into() },
        cancelled       = OrderState { label: "cancelled".into() },
        refunded        = OrderState { label: "refunded".into() },

        draft           -[submit]-> pending_payment,
        pending_payment -[pay]-> paid,
        paid            -[ship]-> shipped,
        shipped         -[deliver]-> delivered,

        draft           -[cancel = CancelEdge { reason: "顧客都合キャンセル".into(), refund_required: false }]-> cancelled,
        pending_payment -[cancel = CancelEdge { reason: "支払い期限切れ".into(), refund_required: false }]-> cancelled,
        paid            -[cancel = CancelEdge { reason: "発送前キャンセル".into(), refund_required: true }]-> cancelled,

        paid            -[refund = RefundEdge { audit_label: "AUDIT-REFUND-PAID".into() }]-> refunded,
        shipped         -[refund = RefundEdge { audit_label: "AUDIT-REFUND-SHIPPED".into() }]-> refunded,
        delivered       -[refund = RefundEdge { audit_label: "AUDIT-REFUND-DELIVERED".into() }]-> refunded,
    })
    .expect("正規のFSM定義は構築に成功するはず");
    g
}

/// 検証デモ用: 「実装したつもりで実は初期状態から繋がっていない状態」を
/// 埋め込んだ壊れた変種 ([`crate::validate::validate`] の到達不能検出デモ用)。
///
/// `held_for_review` という状態を追加しているが、どの既存状態からも
/// `held_for_review` へ向かう辺を張り忘れている (= コードは書いたが
/// どこからも呼ばれないデッドコードと同種のバグ)。`held_for_review` 自身は
/// `cancelled` への `cancel` 辺を持つので行き止まりではない — 「到達不能」
/// と「行き止まり」が別の問題であることも同時に示す。
pub fn build_with_unreachable_state() -> OrderFsm {
    #[rustfmt::skip]
    let g: OrderFsm = graphite::graph!(OrderFsm {
        draft           = OrderState { label: "draft".into() },
        pending_payment = OrderState { label: "pending_payment".into() },
        paid            = OrderState { label: "paid".into() },
        shipped         = OrderState { label: "shipped".into() },
        delivered       = OrderState { label: "delivered".into() },
        cancelled       = OrderState { label: "cancelled".into() },
        refunded        = OrderState { label: "refunded".into() },
        held_for_review = OrderState { label: "held_for_review".into() },

        draft           -[submit]-> pending_payment,
        pending_payment -[pay]-> paid,
        paid            -[ship]-> shipped,
        shipped         -[deliver]-> delivered,

        draft           -[cancel = CancelEdge { reason: "顧客都合キャンセル".into(), refund_required: false }]-> cancelled,
        pending_payment -[cancel = CancelEdge { reason: "支払い期限切れ".into(), refund_required: false }]-> cancelled,
        paid            -[cancel = CancelEdge { reason: "発送前キャンセル".into(), refund_required: true }]-> cancelled,

        paid            -[refund = RefundEdge { audit_label: "AUDIT-REFUND-PAID".into() }]-> refunded,
        shipped         -[refund = RefundEdge { audit_label: "AUDIT-REFUND-SHIPPED".into() }]-> refunded,
        delivered       -[refund = RefundEdge { audit_label: "AUDIT-REFUND-DELIVERED".into() }]-> refunded,

        // held_for_review へ入る辺が無い (書いたつもりで繋げ忘れた想定)。
        held_for_review -[cancel = CancelEdge { reason: "審査により保留後キャンセル".into(), refund_required: true }]-> cancelled,
    })
    .expect("辺の端点は全てノードキーとして宣言済みなので構築自体は成功する");
    g
}

/// 検証デモ用: 「`shipped` に進んだ後の出口 (deliver も refund も) を
/// 定義し忘れた」壊れた変種 ([`crate::validate::validate`] の行き止まり
/// 検出デモ用)。
///
/// `delivered` 状態そのものを削り、`shipped` からの `deliver`/`refund` の
/// 両方の辺を落としている。`shipped` は `draft` から `submit -> pay -> ship`
/// で到達可能 (到達不能ではない) だが、そこから先へ進む辺が1本も無く、かつ
/// `shipped` は終端状態リスト ([`terminal_states`]) にも入っていない —
/// 「発送したら中で永遠に止まる注文」というバグが、到達不能とは独立に
/// 構造だけから検出できることを示す (到達不能側の状態は一切生じないよう
/// `held_for_review` のような追加ノードは置いていない)。
pub fn build_with_dead_end_bug() -> OrderFsm {
    #[rustfmt::skip]
    let g: OrderFsm = graphite::graph!(OrderFsm {
        draft           = OrderState { label: "draft".into() },
        pending_payment = OrderState { label: "pending_payment".into() },
        paid            = OrderState { label: "paid".into() },
        shipped         = OrderState { label: "shipped".into() },
        cancelled       = OrderState { label: "cancelled".into() },
        refunded        = OrderState { label: "refunded".into() },

        draft           -[submit]-> pending_payment,
        pending_payment -[pay]-> paid,
        paid            -[ship]-> shipped,
        // shipped -[deliver]-> delivered / shipped -[refund]-> refunded を
        // 両方書き忘れた想定 (shipped から出る辺が無い)。

        draft           -[cancel = CancelEdge { reason: "顧客都合キャンセル".into(), refund_required: false }]-> cancelled,
        pending_payment -[cancel = CancelEdge { reason: "支払い期限切れ".into(), refund_required: false }]-> cancelled,
        paid            -[cancel = CancelEdge { reason: "発送前キャンセル".into(), refund_required: true }]-> cancelled,

        paid            -[refund = RefundEdge { audit_label: "AUDIT-REFUND-PAID".into() }]-> refunded,
    })
    .expect("辺の端点は全てノードキーとして宣言済みなので構築自体は成功する");
    g
}
