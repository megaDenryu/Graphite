//! 遷移エンジン (`step`) と、その周辺の公開面。
//!
//! 遷移表そのものは `transition_table` が持つ `graph!` リテラルであり、
//! 「遷移表」がドキュメントとコードで乖離しない (README §2 の「なぜ死ぬか」
//! への回答)。[`step`] はイベントの `match` で辺種別 (Kind) を引くだけで、
//! 遷移規則の実体は `match` の腕には一切無い。

mod broken_variant;
mod event;
mod transition_error;
mod transition_table;

pub use broken_variant::{build_with_dead_end_bug, build_with_unreachable_state};
pub use event::Event;
pub use transition_error::TransitionError;
pub use transition_table::{build, initial_state, terminal_states};

use crate::schema::OrderFsm::OrderStateRef;
use crate::schema::{CancelEdge, OrderFsm, OrderStateId, RefundEdge};

/// 遷移エンジン本体。
///
/// イベントの `match` で `{kind}_iter` を引き、`before` のIDが `current` に
/// 一致する辺を探して `after` のIDを返すだけ。`where each before: 0..1`
/// (schema 側の制約) により、一致する辺は高々1本しか無い。
///
/// `{kind}_iter` は完成済みグラフに束縛されたEdgeRefを返し、役割名getterが
/// NodeRefを返す。`step` は `before().id()` / `after().id()` からキーを得る。
/// 遷移規則そのものはここには一切書かれていない
/// (schema と `build` にしか無い) — enum+match 散在アンチパターンとの
/// 決定的な違い。
pub fn step(
    fsm: &OrderFsm::Graph,
    current: &OrderStateId,
    event: Event,
) -> Result<OrderStateId, TransitionError> {
    let next: Option<OrderStateId> = match event {
        Event::Submit => fsm
            .submit_iter()
            .find(|edge| edge.before().id() == current)
            .map(|edge| edge.after().id().clone()),
        Event::Pay => fsm
            .pay_iter()
            .find(|edge| edge.before().id() == current)
            .map(|edge| edge.after().id().clone()),
        Event::Ship => fsm
            .ship_iter()
            .find(|edge| edge.before().id() == current)
            .map(|edge| edge.after().id().clone()),
        Event::Deliver => fsm
            .deliver_iter()
            .find(|edge| edge.before().id() == current)
            .map(|edge| edge.after().id().clone()),
        Event::Cancel => fsm
            .cancel_iter()
            .find(|edge| edge.before().id() == current)
            .map(|edge| edge.after().id().clone()),
        Event::Refund => fsm
            .refund_iter()
            .find(|edge| edge.before().id() == current)
            .map(|edge| edge.after().id().clone()),
    };
    next.ok_or_else(|| TransitionError {
        state: current.clone(),
        event,
    })
}

/// `cancel` イベントのガード条件・監査情報 (`CancelEdge`) も見たい場合は
/// `state.cancel_as_before()` を直接使う (`step` はキーだけ返すため属性は運ばない)。
pub fn cancel_details<'a>(
    fsm: &'a OrderFsm::Graph,
    current: &OrderStateId,
) -> Option<(OrderStateRef<'a>, &'a CancelEdge)> {
    let current = fsm.order_state_by_id(current)?;
    current
        .cancel_as_before()
        .map(|edge| (edge.after(), edge.payload()))
}

/// `refund` イベントの監査ログ用ラベル (`RefundEdge`) を見たい場合。
pub fn refund_details<'a>(
    fsm: &'a OrderFsm::Graph,
    current: &OrderStateId,
) -> Option<(OrderStateRef<'a>, &'a RefundEdge)> {
    let current = fsm.order_state_by_id(current)?;
    current
        .refund_as_before()
        .map(|edge| (edge.after(), edge.payload()))
}
