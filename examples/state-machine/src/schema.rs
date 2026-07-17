//! 注文ライフサイクル FSM のグラフスキーマ (`docs/schema_v4.md` 準拠)。
//!
//! 「ステートマシン地獄」(README §1 参照) をグラフで再定式化するときの
//! 対応関係:
//!
//! | FSM の概念 | Graphite の概念 |
//! |---|---|
//! | 状態 (draft/paid/...) | ノードインスタンス (`OrderState` の各キー) |
//! | イベント (submit/pay/...) | 辺種別 (Kind。nominal 型として生成される) |
//! | 「この状態でこのイベントの行き先は高々1つ」という決定性 | `where each OrderState: 0..1` |
//! | ガード条件・監査ログ用の付随情報 | エッジ属性型 (`CancelEdge`/`RefundEdge`) |
//! | 遷移表そのもの | `schema` 宣言 + `graph!` リテラル (`fsm::build`) |
//!
//! 状態は全部同じノード型 `OrderState` のインスタンス (ノード同一性は
//! ユーザーキーが担う — `docs/graph_design_sketches.md` 決定1)。イベントは
//! `OrderState -> OrderState` の自己ループ的な辺種別として宣言する。
//! `where each OrderState: 0..1` は「ある状態から、あるイベントで遷移できる
//! 先は高々1つ」という FSM の決定性そのものを型に持たせている。始点ノードが
//! 違えば同じ Kind でも別の辺として独立にカウントされるので、`Cancel` の
//! ように複数の状態から出る遷移でも「状態ごとに高々1本」が保たれる。
//!
//! `unique pair` は付けない: `each OrderState: 0..1` (同一始点からの出辺は
//! 高々1本という決定性の制約) がすでに「同じ (始点, 終点) の対に2本目」を
//! 禁止しているため、`unique pair` の併記は冗長になる
//! (`docs/schema_v4.md` §1 「実装を単純にするため特別扱いしない」方針)。

/// ノードキー。v4.2 からは `graph_schema!` はこれも生成せず、
/// `{ノード型名}Id` という命名規約で参照するだけ (`docs/node_id_v4_2.md`)。
/// `PartialOrd`/`Ord` は必須ではないが (必須なのは `Debug, Clone,
/// PartialEq, Eq, Hash` だけ)、`validate.rs` が到達不能/行き止まり状態を
/// 決定的な順で報告するためにキーをソートする箇所がこのアプリ側の都合で
/// 要求している。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OrderStateId(pub String);

/// 状態ノード。注文ライフサイクル中の1状態を表す
/// (draft/pending_payment/paid/shipped/delivered/cancelled/refunded)。
///
/// フィールドは表示用ラベルのみ。状態同士の区別はノードキー
/// (`OrderStateId`、`graph!` の `draft = ..` の `draft` 部分から値が
/// 決まる) が担うので、フィールド自体は最小限で良い。
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

        edge Submit  = OrderState -> OrderState              where each OrderState: 0..1;
        edge Pay     = OrderState -> OrderState              where each OrderState: 0..1;
        edge Ship    = OrderState -> OrderState              where each OrderState: 0..1;
        edge Deliver = OrderState -> OrderState              where each OrderState: 0..1;
        edge Cancel  = OrderState -[CancelEdge]-> OrderState where each OrderState: 0..1;
        edge Refund  = OrderState -[RefundEdge]-> OrderState where each OrderState: 0..1;
    }
}
