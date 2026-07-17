//! 遷移グラフの具体的な定義 (`graph!` リテラル) と遷移エンジン (`step`)。
//!
//! [`build`] が定義する `graph!` リテラルそのものが「遷移表」であり、
//! ドキュメントとコードが乖離しない (README §2 の「なぜ死ぬか」への回答)。
//! [`step`] はイベントの `match` で辺種別 (Kind) を引くだけで、遷移規則の
//! 実体は `match` の腕には一切無い — 全部 `build` の1箇所にしかない。

use std::fmt;

use crate::schema::{Cancel, CancelEdge, Deliver, OrderFsm, OrderState, OrderStateId, Pay, Refund, RefundEdge, Ship, Submit};

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
/// イベントの `match` で `{Kind}::iter` を引き、`from` が `current` に
/// 一致する辺を探して `to()` を返すだけ。`where each OrderState: 0..1`
/// (schema 側の制約) により、一致する辺は高々1本しか無い。
///
/// v4 の `{Kind}::of`/`get_of` は「終点ノードの値そのもの」(`&OrderState`)
/// を返す設計であり、終点の**キー**は返さない
/// (`docs/schema_v4.md` §3.2、`crates/graphite/tests/orgchart_macro.rs`)。
/// `step` はキーを次の状態として返す必要があるため、`{Kind}::of` ではなく
/// `{Kind}::iter` + `from()`/`to()` (辺タプル struct 自身が持つ、キーを
/// 返すアクセサ) を使う。遷移規則そのものはここには一切書かれていない
/// (schema と `build` にしか無い) — enum+match 散在アンチパターンとの
/// 決定的な違い。
pub fn step(
    fsm: &OrderFsm,
    current: &OrderStateId,
    event: Event,
) -> Result<OrderStateId, TransitionError> {
    let next: Option<OrderStateId> = match event {
        Event::Submit => Submit::iter(fsm)
            .find(|(_, e)| e.from() == current)
            .map(|(_, e)| e.to().clone()),
        Event::Pay => Pay::iter(fsm)
            .find(|(_, e)| e.from() == current)
            .map(|(_, e)| e.to().clone()),
        Event::Ship => Ship::iter(fsm)
            .find(|(_, e)| e.from() == current)
            .map(|(_, e)| e.to().clone()),
        Event::Deliver => Deliver::iter(fsm)
            .find(|(_, e)| e.from() == current)
            .map(|(_, e)| e.to().clone()),
        Event::Cancel => Cancel::iter(fsm)
            .find(|(_, e)| e.from() == current)
            .map(|(_, e)| e.to().clone()),
        Event::Refund => Refund::iter(fsm)
            .find(|(_, e)| e.from() == current)
            .map(|(_, e)| e.to().clone()),
    };
    next.ok_or_else(|| TransitionError {
        state: current.clone(),
        event,
    })
}

/// `cancel` イベントのガード条件・監査情報 (`CancelEdge`) も見たい場合は
/// `Cancel::of` を直接使う (`step` はキーだけ返すため属性は運ばない)。
pub fn cancel_details<'a>(
    fsm: &'a OrderFsm,
    current: &OrderStateId,
) -> Option<(&'a OrderState, &'a CancelEdge)> {
    Cancel::of(fsm, current)
}

/// `refund` イベントの監査ログ用ラベル (`RefundEdge`) を見たい場合。
pub fn refund_details<'a>(
    fsm: &'a OrderFsm,
    current: &OrderStateId,
) -> Option<(&'a OrderState, &'a RefundEdge)> {
    Refund::of(fsm, current)
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
    .expect("正規のFSM定義は構築に成功するはず");
    g
}

/// 検証デモ用: 「実装したつもりで実は初期状態から繋がっていない状態」を
/// 埋め込んだ壊れた変種 ([`crate::validate::validate`] の到達不能検出デモ用)。
///
/// `held_for_review` という状態を追加しているが、どの既存状態からも
/// `held_for_review` へ向かう辺を張り忘れている (= コードは書いたが
/// どこからも呼ばれないデッドコードと同種のバグ)。`held_for_review` 自身は
/// `cancelled` への `Cancel` 辺を持つので行き止まりではない — 「到達不能」
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
    .expect("辺の端点は全てノードキーとして宣言済みなので構築自体は成功する");
    g
}

/// 検証デモ用: 「`shipped` に進んだ後の出口 (deliver も refund も) を
/// 定義し忘れた」壊れた変種 ([`crate::validate::validate`] の行き止まり
/// 検出デモ用)。
///
/// `delivered` 状態そのものを削り、`shipped` からの `Deliver`/`Refund` の
/// 両方の辺を落としている。`shipped` は `draft` から `Submit -> Pay -> Ship`
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
    .expect("辺の端点は全てノードキーとして宣言済みなので構築自体は成功する");
    g
}
