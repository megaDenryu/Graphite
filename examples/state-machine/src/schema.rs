//! 注文ライフサイクル FSM のグラフスキーマ。
//!
//! 「ステートマシン地獄」(README §1 参照) をグラフで再定式化するときの
//! 対応関係:
//!
//! | FSM の概念 | Graphite の概念 |
//! |---|---|
//! | 状態 (draft/paid/...) | ノードインスタンス (`OrderState` の各キー) |
//! | イベント (submit/pay/...) | エッジ種別 (ラベル) |
//! | 「この状態でこのイベントの行き先は高々1つ」という決定性 | 多重度 `(0..1)` |
//! | ガード条件・監査ログ用の付随情報 | エッジ属性型 (`CancelEdge`/`RefundEdge`) |
//! | 遷移表そのもの | `schema` 宣言 + `graph!` リテラル (`fsm::build`) |
//!
//! 状態は全部同じノード型 `OrderState` のインスタンス (ノード同一性は
//! ユーザーキーが担う — `docs/graph_design_sketches.md` 決定1)。イベントは
//! `OrderState -> OrderState` の自己ループ的なエッジ種別として宣言する。
//! 多重度 `(0..1)` は「ある状態から、あるイベントで遷移できる先は高々1つ」
//! という FSM の決定性そのものを型に持たせている。始点ノードが違えば同じ
//! ラベルでも別の辺として独立にカウントされるので、`cancel` のように複数
//! の状態から出る遷移でも「状態ごとに高々1本」が保たれる。

/// 状態ノード。注文ライフサイクル中の1状態を表す
/// (draft/pending_payment/paid/shipped/delivered/cancelled/refunded)。
///
/// フィールドは表示用ラベルのみ。状態同士の区別はノードキー
/// (`OrderStateId`、`graph!` の `draft = ..` の `draft` 部分から生成される)
/// が担うので、フィールド自体は最小限で良い。
#[derive(Debug, Clone, PartialEq)]
pub struct OrderState {
    pub label: String,
}

/// `cancel` イベントの属性。「なぜキャンセルされたか」「返金が必要か」
/// というガード条件・監査ログ相当の情報を辺そのものに積む例。
#[derive(Debug, Clone, PartialEq)]
pub struct CancelEdge {
    pub reason: String,
    pub refund_required: bool,
}

/// `refund` イベントの属性。監査ログ用ラベル。
#[derive(Debug, Clone, PartialEq)]
pub struct RefundEdge {
    pub audit_label: String,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema OrderFsm {
        node OrderState;

        edge submit:  OrderState -> OrderState (0..1);
        edge pay:     OrderState -> OrderState (0..1);
        edge ship:    OrderState -> OrderState (0..1);
        edge deliver: OrderState -> OrderState (0..1);
        edge cancel:  OrderState -[CancelEdge]-> OrderState (0..1);
        edge refund:  OrderState -[RefundEdge]-> OrderState (0..1);
    }
}
