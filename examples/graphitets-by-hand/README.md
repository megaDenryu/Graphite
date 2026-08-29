# graphitets-by-hand

GraphiteTS (issue #23 の TypeScript 検証、submodule `GraphiteTS/`) の設計を、
マクロ無しの通常の Rust で写す練習用 example として始まった。graphite クレート
には依存しない。issue #24 の段階1 (全個体がコンパイル時に確定するグラフ向けの
静的グラフ生成の実験) を経て、現在は3つの bin と1つの lib、1つの proc-macro
サブクレートを持つ。

- `src/main.rs` — TS の union type で書いた schema 表現
  (`Graph<OrgNode, OrgEdge>`) を Rust の enum でどこまで同じ意味論にできるかを
  確かめる、動的グラフの手書き (フェーズ②の手書きターゲットとは出発点が違う。
  あちらは `graph_schema!` が生成すべきコードの形を確かめるための手書き)
- `src/bin/static_graph.rs` — 全個体がコンパイル時に確定する静的グラフの手書き
  到達点 (issue #24)。仕組み (`辺`・`結ぶ`・`ノードタグ`・`台帳`・`ノード参照`・
  `辺参照`) は `src/lib.rs`/`src/仕組み.rs` へ切り出し済み
- `src/bin/static_graph_macro.rs` — `静的グラフ!` マクロ (`macros/` の
  `graphitets-by-hand-macros` proc-macro クレート) で `static_graph.rs` と
  同じグラフを宣言し、同じテストが通ることを確認するマクロ利用側

実行場所: このディレクトリ (`examples/graphitets-by-hand`)。

```
cargo run --bin graphitets-by-hand
cargo run --bin static_graph
cargo run --bin static_graph_macro
cargo test
```
