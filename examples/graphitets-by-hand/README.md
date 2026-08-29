# graphitets-by-hand

GraphiteTS (issue #23 の TypeScript 検証、submodule `GraphiteTS/`) の設計を、
マクロ無しの通常の Rust で写す練習用 example である。本文は `src/main.rs` の
1ファイルに手で書く。依存はゼロで、graphite クレートにも依存しない。

フェーズ②の手書きターゲット (水準2の図式グラフ) とは出発点が違う。あちらは
`graph_schema!` が生成すべきコードの形を確かめるための手書きであり、こちらは
TS の union type で書いた schema 表現 (`Graph<OrgNode, OrgEdge>`) を Rust の
enum でどこまで同じ意味論にできるかを確かめるための手書きである。

実行場所: このディレクトリ (`examples/graphitets-by-hand`)。

```
cargo run
cargo test
```
