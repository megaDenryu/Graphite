# ComputeGraph — 具象化された計算グラフ (遅延実行・差分再計算) (Fudaba #14)

2026-07-18 オーケストレータ決定 (ユーザーから続行指示)。#11 (flow!) の議論で
ユーザーが「計算グラフを見据えてました」と確定した先の姿。

## 位置づけ

- **flow!** = 即時実行の脱糖 (書いた瞬間に let 列として実行される)
- **ComputeGraph** = 計算グラフを**実行時の値として持ち**、遅延実行・差分再計算する
  ランタイムエンジン。reactive-cells の Engine の一般化 (あちらは f64 +
  Formula enum に特化した example、こちらは汎用ライブラリ)

「ノード = 値、辺 = 関数」の形を flow! と共有する。将来 flow! の記述を
エンジンに載せ替える道は閉じない (今回はやらない)。

## API の骨格 (crates/graphite/src/compute.rs)

```rust
let mut b = ComputeGraph::builder();
b.input("price", 100.0);
b.input("qty", 3.0);
b.computed("subtotal", ["price", "qty"], |args| args[0] * args[1]);
b.computed("tax",      ["subtotal"],     |args| args[0] * 0.1);
b.computed("total",    ["subtotal", "tax"], |args| args[0] + args[1]);
let mut g = b.freeze()?;          // 循環は CycleError (循環パスつき) で拒否

assert_eq!(*g.get("total"), 341.0);   // 遅延: ここで初めて必要分だけ計算
g.set_input("qty", 5.0);              // 差分: 影響ノードだけ dirty に
assert_eq!(*g.get("total"), 561.0);   // 再計算は影響分をトポロジカル順に各1回
```

設計決定:

- **値型は単一のジェネリック `V`** (`ComputeGraph<V>`)。異種の値はユーザーが
  enum で表現する (reactive-cells と同じ整理。実行時リフレクションを持ち込まない)
- **関数は `Box<dyn Fn(&[&V]) -> V>`**。原則 5 (ゼロコスト) は「マクロ生成コードは
  手書きと同形」という規律であり、ランタイムエンジンが dyn ディスパッチを使うのは
  Rust の正道 (手書きでもそう書く)。この線引きを rustdoc に明記
- **依存は位置引数** (`args[0]` = 依存リストの 0 番)。非可換な演算の左右は
  依存リストの順序で表現する — flow! の fan-in タプルと同じ規則。
  (モデリングガイド §5 の「役割は名前で」はグラフ**データ**の規律であり、
  関数適用の引数は Rust と同じ位置渡しが正道 — この区別も rustdoc に一言)
- **キーは名前 (String)** — #15 で確定した意味論に従う
- **pull 型の遅延 + 差分**: `set_input` は書き込みと dirty 伝播
  (reachable_from 相当) のみ。`get` が dirty な祖先だけをトポロジカル順に
  各 1 回再計算 (glitch-free)。トポロジカル順は freeze で 1 回だけ計算しキャッシュ
- freeze 時検証: 循環 (CycleError、パスつき)・未宣言依存・キー重複

## テスト (完了条件)

- ダイヤモンド依存で各ノードちょうど 1 回再計算 (カウンタで証明、glitch-free)
- 遅延: get していない枝は再計算されない / set_input だけでは計算が走らない
- 差分: 影響外のノードが再計算されない
- 循環拒否 (パスつき)・未宣言依存・キー重複の検証エラー
- rustdoc の doctest (上の骨格例)

## やらないこと (境界の記録)

- flow! → ComputeGraph の自動載せ替え (別札の種として残す価値があれば起票)
- 非同期実行・並列評価 (async-dag の波実行はグラフデータ側の機能として既にある)
- reactive-cells の書き換え (あちらは schema モデリングの教材として現状維持。
  README に「汎用版はランタイムの ComputeGraph」への参照を 1 行足すのみ)
