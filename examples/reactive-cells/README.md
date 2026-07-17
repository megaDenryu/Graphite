# reactive-cells

Graphite (`graph_schema!`/`graph!`) が倒すべき敵その3 = **リアクティブ
プログラミングのスパゲッティ**を、動くプログラムで実証するexample。
題材はミニスプレッドシート (単価・数量・税率・割引率・配送料から
小計・割引額・税額・調整額・合計を求める見積書) の依存グラフ。

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
cargo run
```

`cargo run` は以下の物語をそのまま実行して出力する。`cargo test` は
23件のテスト (単体15件 + 統合8件) を実行する。

## 1. 敵の紹介 — observer パターン (コールバック購読) のグリッチ・無限ループ・非決定性

`src/antipattern.rs` の `NaiveCell` は、リアクティブな値を実装する際の
最もよくあるパターンそのもの: 「値を持ち、値が変わったら購読者
(subscriber) へコールバックで通知する」だけのセル。

```rust
pub struct NaiveCell {
    value: RefCell<f64>,
    subscribers: RefCell<Vec<Rc<dyn Fn(f64)>>>,
}

impl NaiveCell {
    pub fn set(&self, value: f64) {
        *self.value.borrow_mut() = value;
        self.notify(); // 登録順に同期的に通知する
    }
    pub fn subscribe(&self, f: impl Fn(f64) + 'static) {
        self.subscribers.borrow_mut().push(Rc::new(f));
    }
}
```

これで `a→b, a→c, b→d, c→d` というダイヤモンド依存 (`b = a*2`,
`c = a+100`, `d = b+c`) を組み、`a` に `5` を設定すると
(`cargo run` の実際の出力):

```
[1回目] d再計算時点の観測値: b=10, c=0 -> d=10     <- グリッチ (cが古い)
[2回目] d再計算時点の観測値: b=10, c=105 -> d=115  <- ようやく正しい値
最終的なd = 115 (正しい値に収束するが、過程で1回グリッチを踏んでいる)
```

### (a) グリッチ
`d` は2回再計算され、1回目は「`b` は新しい値・`c` はまだ古い値」という
**矛盾した中間状態**を観測する。最終値 (115) は正しいが、その途中の
瞬間 (`d=10`) を誰か (別の購読者・UIの再描画など) が読んでいたら、
間違った値を見ることになる。

### (b) 無限ループ
`src/antipattern.rs::build_infinite_loop_demo` は `x`/`y` が互いを
購読し合う (`x`が変わったら`y`を更新し、`y`が変わったら`x`を更新する)
だけの2セルを作る。循環に気づく仕組みは一切無いので、`x.set(2.0)` を
呼ぶと notify が同期的に往復し続ける。実際に無限に回すとスタック
オーバーフローするため、デモでは回数に安全弁 (`cap`) を入れて強制停止
させているが、**安全弁はナイーブな実装には存在しない** — 実際
`cap` にちょうど到達すること (`cargo run`/テストで確認済み) が
「自然には止まらない」ことの証拠になっている。

### (c) 更新順序が購読登録順に依存して非決定
`a` への購読を「`b`を更新する処理」→「`c`を更新する処理」の順で登録
するか、逆順で登録するかで、1回目のグリッチの内容が変わる
(`cargo run` の「登録順を入れ替えた結果」節、`b=0,c=105` vs
`b=10,c=0`)。**依存関係そのものは同じ**なのに、コードのどこで
`subscribe` を呼んだかという無関係な要因で結果の過程が変わる。

## 2. なぜ死ぬか

- 依存関係が「実行時のコールバック登録」としてしか存在しない。
  `NaiveCell` 単体を見ても「`d` は `b`・`c` に依存する」という事実は
  どこにも書かれておらず、`subscribe` の呼び出し列を追わないと分からない。
  **静的な全体像 (依存グラフ) が無い。**
- 正しい更新順序 (トポロジカル順) を observer パターンは知らない。
  `notify` は「今この瞬間に誰が購読しているか」しか知らず、「全ての
  依存元が更新し終わってから自分を更新する」という順序を保証する仕組みを
  持たない。
- 循環は実行して初めて (スタックオーバーフローで) 発覚する。
  `x`/`y` の相互購読を書いた時点では何のエラーも出ない。実際に
  `x.set(..)` を呼んで初めて、無限に notify が回ることに気づく
  (気づく前にクラッシュする)。

## 3. グラフによる再定式化

`src/schema.rs` は依存関係を「セル (`Cell`) ノード + `Feeds` エッジ」
という**構造データ**として宣言する:

```rust
graphite::graph_schema! {
    schema Sheet {
        node Cell;
        edge Feeds = Cell -> Cell where unique pair;
    }
}
```

`Feeds` は「`from` の値が `to` の入力になる」という向き
(依存元→依存先) で読む。`where unique pair` は「あるセルが別のセルへ
値を供給する」という依存関係は有るか無いかの二値であり、同じ
(from, to) の対に2本目の `Feeds` エッジを張ることに意味は無い、という
判断 (`examples/async-dag` の `DependsOn` と同じ)。`src/fixtures.rs::default_sheet`
がこれを `graph!` リテラルで具体化する — 依存グラフが**一枚のリテラルとして
実行前に全部見える**:

```rust
graphite::graph!(Sheet {
    unit_price = Cell { formula: Formula::Input },
    // .. 入力セル5個 ..
    subtotal   = Cell { formula: Formula::Mul(unit_price.clone(), quantity.clone()) },
    // .. 計算セル5個 ..

    f_unit_price_subtotal = Feeds(unit_price -> subtotal),
    // .. Feedsエッジ11本 ..
})
```

この依存グラフを `graphite::Graph<(), (), CellId>` に射影すれば
(`src/engine.rs::Engine::new`)、水準1ランタイムの2つの操作だけで
再計算エンジンの核が組める:

- **`topological_sort()` がそのままglitch-freeな再計算順になる。**
  「あるセルを計算する時点で、そのセルが依存する全セルは既に最新値に
  なっている」ことを保証する順序そのものだから。
- **`reachable_from(id)` が「この入力の変更で影響を受けるセル」を
  厳密に絞る。** 無関係なセルは再計算されない。
- **循環は `topological_sort()` が `Err(CycleError)` を返すことで、
  実行前 (`Engine::new` の時点) にデータ検証として拒否される。**
  `CycleError::cycle` には循環を構成するキーがそのまま入っているので、
  「どこで循環しているか」が具体的に分かる (`cargo run` の実際の出力):

  ```
  Engine::new は失敗した (期待通り):
    グラフに循環があります: CellId("c") -> CellId("a") -> CellId("b") -> CellId("c")
    循環パス: c -> a -> b -> c
  ```

これで敵1〜3が構造的に解決する。ダイヤモンド依存
(`subtotal → discount_amount → adjustment`、`subtotal → tax →
adjustment`) を含む見積シートで `unit_price` を変更しても、
`adjustment` はちょうど1回だけ再計算される
(`src/engine.rs` のテスト「ダイヤモンド依存でもadjustmentはちょうど
1回だけ再計算される」で数値まで検算済み)。登録順という概念自体が
存在しない (`topo_order` は依存構造だけから決まる) ので (c) の非決定性
も原理的に発生しない。循環は `graph!`/`Sheet::create` ではなく
`Engine::new` で拒否される — これは意図的な責務分離で、「schema/graph!
は構造の整合性 (端点の存在・where制約) だけを見る、非循環性は再計算エンジン
という**ドメイン**が要求する制約」という切り分け (`src/fixtures.rs`
の `cyclic_demo_sheet` のドキュメント参照)。

## 4. 対応表 — リアクティブの概念 ↔ Graphite の概念

| リアクティブプログラミング | Graphite | このexampleでの実体 |
|---|---|---|
| signal (入力値) | 入力ノード | `Formula::Input` を持つ `Cell` |
| computed (計算値) | 計算ノード + そのノードへの入辺 | `Formula::Mul`/`Sub`/`Sum` を持つ `Cell` |
| 依存関係の宣言 (JSで言えば `computed(() => a.get() + b.get())`) | `edge Feeds = Cell -> Cell where unique pair;` + `graph!` リテラル | `f_unit_price_subtotal = Feeds(unit_price -> subtotal)` 等 |
| 購読 (subscribe)・通知 (notify) | (存在しない — 不要になる) | `Engine::set_input` が影響範囲を一括で処理する |
| 正しい再計算順序の保証 | `topological_sort()` | `Engine::topological_order()` (構築時に1回だけ計算) |
| 影響範囲の特定 (dirty checking) | `reachable_from(id)` | `Engine::set_input` 内の `affected` 集合 |
| glitch (矛盾した中間状態) | 原理的に発生しない (トポロジカル順が保証) | `tests/integration.rs`「ダイヤモンド依存を通る更新でも…」 |
| 循環依存の実行時クラッシュ | `CycleError` (構築前に拒否) | `Engine::new` が `Err(CycleError)` を返す |
| effect (副作用の実行) | (このexampleの範囲外) | — (`RecomputeStep` の列を読むのがeffectの代わり) |

## セル構成

10セル・`Feeds` エッジ11本。ダイヤモンド依存
(`subtotal(a) → discount_amount(b) → adjustment(d)`、
`subtotal(a) → tax(c) → adjustment(d)`) を含む。

| セル | 種別 | 式 |
|---|---|---|
| `unit_price`/`quantity`/`tax_rate`/`discount_rate`/`shipping_fee` | 入力 | — |
| `subtotal` | 計算 | `unit_price * quantity` |
| `discount_amount` | 計算 | `subtotal * discount_rate` |
| `tax` | 計算 | `subtotal * tax_rate` |
| `adjustment` | 計算 | `tax - discount_amount` |
| `grand_total` | 計算 | `subtotal + adjustment + shipping_fee` |

## モジュール構成

| モジュール | 内容 |
|---|---|
| `src/antipattern.rs` | 敵: `NaiveCell` (observer パターン) とダイヤモンド依存・循環購読のデモ |
| `src/schema.rs` | `Cell`/`Formula` ノード型と `graph_schema!` 宣言 |
| `src/fixtures.rs` | `graph!` リテラルによる具体的な依存グラフ (`default_sheet`/`cyclic_demo_sheet`) |
| `src/engine.rs` | 再計算エンジン (`topological_sort`/`reachable_from` を使う) |
| `src/report.rs` | `main.rs` 向けの読み物風出力ヘルパー |
| `src/main.rs` | 上記を通して読む物語 (`cargo run`) |
| `tests/integration.rs` | 公開APIだけを使ったend-to-endテスト |

## 実装の割り切り

- `Formula` (式) と `Feeds` エッジ (依存グラフの構造) は独立に手で
  書いており、意図的に重複させている。`Formula::Mul(subtotal, ..)` が
  「`subtotal` に依存する」という情報を既に持っているので、実運用なら
  `Feeds` エッジを `Formula` から自動導出する設計もありうる。この
  exampleでは `graph!` リテラルが依存グラフを一枚のデータとして見せる
  ことを優先し、あえて両方を手書きにしている (`src/fixtures.rs` の
  `default_sheet` ドキュメント参照)。
- `Engine::set_input` は計算セル (`Formula::Input` 以外) への直接代入を
  契約違反としてパニックする (`docs/design_principles.md` 原則2)。
  計算セルの値は依存元セルの更新から常に自動的に決まるべきであり、
  これを破ると依存グラフと値ストアが不整合になる。
