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
  到達点 (issue #24)。仕組み (`辺`・`結ぶ`・`ノードタグ`・`台帳`・`ノード参照`・
  `辺参照`) は `src/lib.rs`/`src/仕組み/` へ切り出し済み
- `src/bin/static_graph_macro/` — `macros/` の `graphitets-by-hand-macros`
  proc-macro クレートが提供する `静的グラフ型!` で組織ドメイン (社員・部署・
  任命記録・経緯記録) の骨組みと具体グラフを宣言するマクロ利用側。
  Graphite本体crateの `graph_schema!`/`graph!` に忠実な矢印記法・役割名の
  構文を採用する

## マクロの構成 (issue #24 段階2)

公開するマクロは `静的グラフ型!` 1個だけ。schemaを構文解析・検証し、schema名
そのものを名前にした `macro_rules!` を生成する (macro_rules!転送)。この
生成されたmacro_rulesが実際に個体宣言を受け取ると、schemaの生トークンと
個体宣言の生トークンを束ねて `#[doc(hidden)]` の内部proc macro
`静的グラフ内部!` へ転送し、そこでschemaとinstanceを1回の展開で同時に見て
相互検証と具象コード生成を行う。

段階1の2層構成 (`静的グラフ型!`/`静的グラフ!` が独立に展開される) では、
互いの展開結果をトークンレベルで見られないため、多重度検査が「位置キー
trait + const assert」という迂回機構を必要としていた。macro_rules!転送で
1回の展開に統合したことで、この迂回機構は不要になり、未知の種別・端点の
実体型不一致・積み荷有無の不一致・向きの不一致・多重度違反・対一意違反の
どれも通常の `compile_error!` (instance側のトークンを指す) として検出できる。
生成物も全部具象 (タグ・trait・PhantomData・仕組みへの依存なし) になり、
生成物1種類1ファイルの `internal::codegen` 配下がそのまま構造を持つ。

schema宣言 (`静的グラフ型!` の引数、変更なし):

```rust
静的グラフ型! {
    schema 組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
        edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1;
        edge 友人 = 社員 -- 社員 where unique pair;
        edge 同僚 = 社員 -[経緯: 経緯記録]- 社員;
    }
}
```

無向辺の積み荷付き記法 `-[役割: 型]-` は、有向の積み荷付き記法
`-[役割: 型]->` から矢尻 (`>`) を落とした形 (hello-graph の規約に倣う)。

instance宣言 (schema名がそのままマクロ名になる。ヘッダに schema名を
重ねて書く必要はない):

```rust
組織! {
    graph 開発チーム;
    太郎 = 社員 { 名前: "太郎".into() },
    次郎 = 社員 { 名前: "次郎".into() },
    一郎 = 社員 { 名前: "一郎".into() },
    開発部 = 部署 { 名前: "開発部".into() },
    太郎の所属 = 所属(太郎 -> 開発部),
    次郎の所属 = 所属(次郎 -> 開発部),
    一郎の所属 = 所属(一郎 -> 開発部),
    太郎の上司 = 上司(太郎 -[任命記録 { 任命日: 2020 }]-> 次郎),
    太郎と次郎 = 友人(太郎 -- 次郎),
    太郎と一郎の同僚 = 同僚(太郎 -[経緯記録 { 経緯: "同期入社".into() }]- 一郎),
}
```

多重度制約 (`each member: 1` 等) 違反・対一意制約 (`unique pair`) 違反・
未知の種別・端点の実体型不一致・積み荷有無の不一致・向きの不一致は、
展開時にすべて通常の `compile_error!` としてコンパイルエラーになる
(instance側の該当トークンを指す)。実測した文言は
`src/bin/static_graph_macro/main.rs` のコメントを参照。

**macro_rules!のテキスト順の制約**: `組織!` は `静的グラフ型!` が展開時に
生成する通常の `macro_rules!` であり、通常の `macro_rules!` と同じテキスト
順の制約を受ける。`静的グラフ型! { schema 組織 { .. } }` の呼び出しより
前の行から `組織! { .. }` を呼ぶことはできない (同じファイル内で
`静的グラフ型!` の呼び出しを先に書く必要がある)。

実行場所: このディレクトリ (`examples/graphitets-by-hand`)。

```
cargo run --bin graphitets-by-hand
cargo run --bin static_graph
cargo run --bin static_graph_macro
cargo test
```
