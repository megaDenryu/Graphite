# graphitets-by-hand

**手書き検証の記録です。** GraphiteTS (issue #23 の TypeScript 検証、
submodule `GraphiteTS/`) の設計を、マクロ無しの通常の Rust で写す練習用
example として始まった。graphite クレートには依存しない。

issue #24 (全個体がコンパイル時に確定するグラフ向けの静的グラフ生成の実験)
は、このディレクトリで `macros/` (独立 proc-macro crate) と
`src/bin/static_graph_macro/` (利用側) として実装・確定した。段階2 (本体
統合) でマクロは `graphite::static_schema!` として Graphite 本体crate群
(`crates/graphite-codegen`・`crates/graphite-macros`・`crates/graphite`) へ
英語識別子で統合済みであり、**正式な利用例は `examples/static-org` に
ある**。`macros/` と `src/bin/static_graph_macro/` はこのディレクトリから
削除した。マクロの構文・生成される名前の公開契約・コンパイル時検査の一覧は
`docs/static_graph.md` を参照。

このディレクトリに残っているのは、マクロが無い状態で「静的グラフ機構が
生成すべきコードの実際の形」を確かめた手書きの到達点であり、記録として
凍結している (機能追加はしない)。

- `src/main.rs` — TS の union type で書いた schema 表現
  (`Graph<OrgNode, OrgEdge>`) を Rust の enum でどこまで同じ意味論にできるかを
  確かめる、動的グラフの手書き (フェーズ②の手書きターゲットとは出発点が違う。
  あちらは `graph_schema!` が生成すべきコードの形を確かめるための手書き)
- `src/bin/static_graph.rs` — 全個体がコンパイル時に確定する静的グラフの手書き
  到達点 (issue #24)。仕組み (`辺`・`結ぶ`・`ノードタグ`・`台帳`・`ノード参照`・
  `辺参照`) は `src/lib.rs`/`src/仕組み/` へ切り出し済み。`static_schema!` が
  生成するコードはこの仕組みへ依存しない全部具象のコードであり、この仕組みは
  この手書き到達点だけが使う

実行場所: このディレクトリ (`examples/graphitets-by-hand`)。

```
cargo run --bin graphitets-by-hand
cargo run --bin static_graph
cargo test
```
