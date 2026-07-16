# state-machine

Graphite (`graphite::graph_schema!`/`graph!`) が「ステートマシン地獄」を
どう倒すかを、動くプログラムとして実証するexample。題材は注文ライフサイクル
FSM (draft → pending_payment → paid → shipped → delivered、脱線として
cancelled/refunded)。

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
cargo run
```

`cargo run` はサブコマンド無しで全部 (シナリオ→検証→検証デモ) を通しで
実行する。個別に見たい場合:

```powershell
cargo run -- scenario         # (1)(2) 正常遷移・不正遷移のシナリオのみ
cargo run -- validate         # (3) 正規のFSMを検証 (健全なはず)
cargo run -- validate-broken  # (3) 壊れた変種2つに対する検出デモ
```

## 1. 敵の紹介 — ステートマシン地獄

注文の状態をふつうの Rust だけで表そうとすると、大抵は次の2つのどちらか
(あるいは両方) に行き着く。

### (a) bool フラグ持ち

```rust
struct Order {
    is_submitted: bool,
    is_paid: bool,
    is_shipped: bool,
    is_delivered: bool,
    is_cancelled: bool,
    is_refunded: bool,
}
```

フィールドが `n` 個の bool なら、コンパイラは `2^n` 通りの組み合わせを
全部許してしまう。`n = 6` で 64 通り、実際に意味を持つ状態は 7 個 (このFSMの
状態数) しかないのに、残り57通り —
`is_cancelled: true, is_delivered: true`(キャンセル済みなのに配達済み) や
`is_paid: false, is_shipped: true`(未払いなのに発送済み) のような**表現可能
な不正状態**が野放しになる。

### (b) enum + match 散在

```rust
enum OrderState { Draft, PendingPayment, Paid, Shipped, Delivered, Cancelled, Refunded }

fn submit(state: OrderState) -> OrderState {
    match state {
        OrderState::Draft => OrderState::PendingPayment,
        other => other, // 何もしない (無視) か panic! か、関数ごとに対応がバラバラ
    }
}

fn pay(state: OrderState) -> OrderState {
    match state {
        OrderState::PendingPayment => OrderState::Paid,
        other => other,
    }
}

fn ship(state: OrderState) -> OrderState { /* ... 同様の match ... */ }
fn deliver(state: OrderState) -> OrderState { /* ... */ }
fn cancel(state: OrderState) -> OrderState { /* ... */ }
// refund を追加するのを忘れていても、コンパイラは何も言わない。
```

bool フラグの不正状態は防げるが、代わりに**遷移規則がイベントごとの関数に
1つずつ分散**する。「`paid` の状態で有効なイベントは何か」を知るには
`submit`/`pay`/`ship`/`deliver`/`cancel`/`refund` 全部の中身を読んで回る
必要があり、遷移表の全体像はどこにも書かれていない。

## 2. なぜ死ぬか

- **不正遷移が実行時まで見えない。** (a) は「有効な組み合わせ」をコード上
  どこにも書けないので、テストで気づくかバグ報告が来るまで分からない。
  (b) は「その状態でそのイベントは無効」を `match` の `other => other`
  (無視) や `panic!` に押し付けており、コンパイル時には検出できない。
- **到達不能状態・行き止まり状態に誰も気づかない。** 新しい状態やイベントを
  追加したとき、「この状態にはどこからも到達できない」「この状態から先へ
  進む手段が無い (行き止まり)」に気づく仕組みが (a)(b) どちらにも無い。
  気づくのはユーザー影響が出た後になりがち。
- **遷移表のドキュメントとコードが乖離する。** 仕様書やExcelの状態遷移表は
  実装が変わっても自動更新されない。(b) の場合、遷移規則の全体像を知るには
  複数関数の `match` 腕を人力で集めて回るしかなく、その作業自体が「乖離して
  いないか」を保証できない。

## 3. グラフによる再定式化

Graphite ではこう考える:

- **状態 = ノードインスタンス。** `draft`/`paid`/`shipped`/... は同じ
  ノード型 `OrderState` の別々のキー付きインスタンス (ノード同一性は
  ユーザーキーが担う)。
- **イベント = エッジ種別 (ラベル)。** `submit`/`pay`/`ship`/`deliver`/
  `cancel`/`refund` はそれぞれ独立したエッジラベルとして宣言する。
- **決定性 = 多重度 `(0..1)`。** 「ある状態から、あるイベントで遷移できる
  先は高々1つ」という FSM の決定性そのものが、`edge pay: OrderState ->
  OrderState (0..1);` という**schemaの型**に乗る。bool フラグのような
  「表現可能な不正状態」は最初から存在しない — `OrderFsm::create` が
  多重度違反を一括検査するので、同じ状態から同じイベントで2箇所以上に
  遷移するような矛盾したデータは構築時点で `Err` になる。
- **schema そのものが遷移表のドキュメント。** `src/fsm.rs` の
  `graph_schema!`/`graph!` を読めば、遷移規則の全体像 (どの状態からどの
  イベントでどこへ行けるか) が1箇所に宣言的に並んでいる。ドキュメントと
  コードが同じソースなので乖離が起きない。

## 4. 対応表

| FSM の概念 | Graphite の概念 |
|---|---|
| 状態 (draft/pending_payment/paid/...) | ノードインスタンス (`OrderState` のキー) |
| イベント (submit/pay/ship/...) | エッジ種別 (ラベル) |
| 「この状態でこのイベントの行き先は高々1つ」という決定性 | 多重度 `(0..1)` |
| ガード条件・監査ログ用の付随情報 (キャンセル理由・返金要否・監査ラベル) | エッジ属性型 (`CancelEdge`/`RefundEdge`) |
| 遷移表そのもの | `schema` 宣言 + `graph!` リテラル (`src/fsm.rs::build`) |
| 「未定義の遷移」 | `TransitionError` (`Result::Err`、型で必ず処理を強制) |
| 「どこからも呼ばれない状態がある」(デッドコード相当) | `reachable_from` による到達不能検出 |
| 「そこから先へ進む手段が無いのに終端でもない」(定義漏れ) | `out_neighbors` による行き止まり検出 |

## スキーマ

```rust
pub struct OrderState { pub label: String }
pub struct CancelEdge { pub reason: String, pub refund_required: bool }
pub struct RefundEdge { pub audit_label: String }

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
```

状態はすべて同じノード型 `OrderState` のインスタンスであり、イベントは
`OrderState -> OrderState` の自己ループ的なエッジ種別として宣言している。
`cancel`/`refund` には属性型 (`CancelEdge`/`RefundEdge`) を持たせ、「なぜ
キャンセルされたか」「返金が必要か」「監査ログ用ラベル」というガード条件・
付随情報を辺そのものに積む例にしている。

## 遷移グラフ (`src/fsm.rs::build`)

```rust
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
```

これがそのまま「設定ファイル」に相当する一枚絵になっている。読み方:

- 正常フローは `draft -[submit]-> pending_payment -[pay]-> paid -[ship]->
  shipped -[deliver]-> delivered` という一直線。
- `cancel` は発送前 (draft/pending_payment/paid) からのみ可能。
- `refund` は支払い済み以降 (paid/shipped/delivered) からのみ可能 (現実の
  EC システムでよくある区別: 未発送は取消、発送後は返品/返金)。
  `delivered` も `refund` という唯一の出口を持つ点に注意 (完全に出口が
  無い終端状態は `cancelled`/`refunded` のみ)。

## 遷移エンジン (`src/fsm.rs::step`)

```rust
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
    next.cloned().ok_or_else(|| TransitionError { state: current.clone(), event })
}
```

`match` の各腕は `{event}().id_of(current)` を呼ぶだけで、遷移規則その
ものは一切書かれていない (規則は `build()` の `graph!` リテラルにしか
存在しない)。多重度 `(0..1)` なので `id_of` の戻り値は
`Option<&OrderStateId>` — 「その状態でそのイベントは未定義」がそのまま
`None` として表れ、`TransitionError` に変換して返す。enum+match 散在
アンチパターンの「規則が複数関数に分散する」問題は、規則自体を `build()`
1箇所に集約することで構造的に起きない。

## グラフアルゴリズムによる FSM 検証 (`src/validate.rs`) — ここが売り

`schema`+`graph!` で遷移表を書いただけでは、以下は誰も検査してくれない:

- 初期状態からどのイベント列を試しても絶対に辿り着けない状態 (**到達不能**)。
- 終端でないのに、そこから先へ進む辺が1本も無い状態 (**行き止まり**、定義漏れ)。

`validate::validate` は6種のイベントエッジ (`submit`/`pay`/`ship`/
`deliver`/`cancel`/`refund`) を全部束ねて、ラベルの区別を捨てた汎用
`graphite::Graph<(), (), OrderStateId>` に射影し (`Graph::from_edges`)、
ラベルを問わない汎用アルゴリズム2つだけで両方を検出する:

- **到達不能検出**: `graph.reachable_from(initial)` (initial 自身も含む
  反射的な到達可能性) の外側に残ったキーが到達不能。
- **行き止まり検出**: 終端状態集合に含まれないのに `graph.out_neighbors(key)`
  が空なキーが行き止まり。

### 実行結果 — 正規のFSM (`cargo run -- validate`)

```
=== 検証: 正規のFSM定義 (fsm::build()) ===

[正規のFSM]
  到達不能状態: なし
  行き止まり状態: なし
  総合判定: 健全
```

### 実行結果 — 壊れた変種2つでの検出デモ (`cargo run -- validate-broken`)

1つ目 (`fsm::build_with_unreachable_state`) は `held_for_review` という
状態を追加したが、どの既存状態からもそこへ向かう辺を張り忘れている
(= コードは書いたがどこからも呼ばれないデッドコードと同種のバグ):

```
--- デモ1: held_for_review 状態への辺を張り忘れた変種 (到達不能検出) ---
[held_for_review 未接続の変種]
  到達不能状態: [OrderStateId("held_for_review")]
  行き止まり状態: なし
  総合判定: 問題あり
```

2つ目 (`fsm::build_with_dead_end_bug`) は `shipped` から先へ進む辺
(`deliver`/`refund`) を両方書き忘れている。`shipped` 自身は `draft` から
到達可能 (到達不能ではない) だが、終端状態でもないのに出口が無い:

```
--- デモ2: shipped の出口 (deliver/refund) を両方書き忘れた変種 (行き止まり検出) ---
[shipped 出口未定義の変種]
  到達不能状態: なし
  行き止まり状態: [OrderStateId("shipped")]
  総合判定: 問題あり
```

「到達不能」と「行き止まり」が独立した別の問題であることが、2つのデモが
互いに他方を誤検出しないことからも分かる (デモ1は行き止まりゼロ、デモ2は
到達不能ゼロ)。

## テスト

`tests/integration.rs` に15件。カテゴリ:

- 正常遷移 (ライフサイクル一直線・cancel/refund属性の読み取り)
- 不正遷移が `Result::Err` になること (未定義遷移・終端後の遷移)
- 決定性 (同じ状態・同じイベントは常に同じ遷移先、多重度 `(0..1)` の保証)
- 検証アルゴリズム (正規FSMの健全性、壊れた変種2つそれぞれの検出)

```powershell
cargo test
```
