# graphitets-by-hand

GraphiteTS (issue #23 の TypeScript 検証、submodule `GraphiteTS/`) の設計を、
マクロ無しの通常の Rust で写す練習用 example として始まった。graphite クレート
には依存しない。issue #24 (全個体がコンパイル時に確定するグラフ向けの静的
グラフ生成の実験) を経て、現在は3つの bin と1つの lib、1つの proc-macro
サブクレートを持つ。

- `src/main.rs` — TS の union type で書いた schema 表現
  (`Graph<OrgNode, OrgEdge>`) を Rust の enum でどこまで同じ意味論にできるかを
  確かめる、動的グラフの手書き (フェーズ②の手書きターゲットとは出発点が違う。
  あちらは `graph_schema!` が生成すべきコードの形を確かめるための手書き)
- `src/bin/static_graph.rs` — 全個体がコンパイル時に確定する静的グラフの手書き
  到達点 (issue #24)。仕組み (`辺`・`無向辺`・`結ぶ`・`ノードタグ`・`台帳`・
  `ノード参照`・`辺参照`・`無向辺参照`・`種別契約`・`始点位置`/`終点位置`) は
  `src/lib.rs`/`src/仕組み/` へ切り出し済み
- `src/bin/static_graph_macro/` — 2層のマクロ (`macros/` の
  `graphitets-by-hand-macros` proc-macro クレートが提供する `静的グラフ型!`・
  `静的グラフ!`) で組織ドメイン (社員・部署・任命記録) の骨組みと具体グラフを
  宣言するマクロ利用側。Graphite本体crateの `graph_schema!`/`graph!` に忠実な
  矢印記法・役割名の構文を採用する

## 2層のマクロ

- **`静的グラフ型!`** (レイヤー1、`graph_schema!` 相当) — 型の骨組み (種別
  ラベル・種別契約・役割アクセサ・多重度の契約) をグラフ名に依存しない形で
  1回生成する。構文はGraphite本体のschema宣言の矢印記法を写す:
  ```rust
  静的グラフ型! {
      schema 組織 {
          node 社員;
          node 部署;
          edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
          edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1;
          edge 友人 = 社員 -- 社員;
      }
  }
  ```
- **`静的グラフ!`** (レイヤー2、`graph!` リテラル相当) — 個体タグ・ノード達・
  辺達・参照の層・グラフ本体を生成し、レイヤー1が用意した骨組みへ具体的な
  個体を接続する:
  ```rust
  静的グラフ! {
      graph 開発チーム: 組織;
      太郎 = 社員 { 名前: "太郎".into() },
      次郎 = 社員 { 名前: "次郎".into() },
      開発部 = 部署 { 名前: "開発部".into() },
      太郎の所属 = 所属(太郎 -> 開発部),
      次郎の所属 = 所属(次郎 -> 開発部),
      太郎の上司 = 上司(太郎 -[任命記録 { 任命日: 2020 }]-> 次郎),
      太郎と次郎 = 友人(太郎 -- 次郎),
  }
  ```
  多重度制約 (`each member: 1` 等) 違反は `const assert` としてコンパイル
  時に検出される (freeze の実行時検証を、構造的保証へ置き換える段階1の
  実験)。

実行場所: このディレクトリ (`examples/graphitets-by-hand`)。

```
cargo run --bin graphitets-by-hand
cargo run --bin static_graph
cargo run --bin static_graph_macro
cargo test
```
