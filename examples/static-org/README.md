# static-org

`graphite::static_schema!` (issue #24。全個体がコンパイル時に確定するグラフ)
の正式な利用例です。`graph_schema!`/`graph!` (実行時に個体を追加でき、
`freeze()` が制約を実行時検証する既定のグラフ) とは対照的に、
`static_schema!` は個体・辺の集合自体をコンパイル時に固定し、多重度・
対一意制約をコンパイルエラーとして検出します。

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
cargo run
```

## 構成

| ファイル | 内容 |
|---|---|
| `src/domain.rs` | `static_schema!` の外で宣言する実体型 (社員・部署・任命記録・経緯記録) |
| `src/main.rs` | schema宣言 (`static_schema!`)・instance宣言 (`組織!`)・構築から辿りまでの実演。コンパイルエラーの実測コメント (存在しない辿り・多重度違反・対一意違反) を含む |
| `src/tests.rs` | 役割アクセサ・積み荷アクセサ・無向辺・同一インスタンス性・後付けimplのアサーション |

DSL構文の正式な仕様 (schema宣言・instance宣言の文法、生成される名前の公開
契約、コンパイル時検査の一覧) は `docs/static_graph.md` を参照してください。
手書き検証の記録は `examples/graphitets-by-hand` (凍結済み) にあります。

## 最初に読むもの

`src/main.rs` の schema宣言 (`static_schema! { schema 組織 { .. } }`) と
instance宣言 (`組織! { graph 開発チーム; .. }`) の対を読むと、DSLの形が
一目で分かります。`組織!` は `static_schema!` が展開時に生成する
`macro_rules!` であり、schema名がそのままマクロ名になります。

```rust
static_schema! {
    schema 組織 {
        node 社員;
        node 部署;
        edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
        edge 上司 = (subordinate: 社員) -[任命: 任命記録]-> (superior: 社員) where each subordinate: 0..1;
        edge 友人 = (甲: 社員) -- (乙: 社員) where unique pair;
        edge 同僚 = (甲: 社員) -[経緯: 経緯記録]- (乙: 社員);
    }
}

組織! {
    graph 開発チーム;
    node 太郎 = 社員 { 名前: "太郎".into() };
    node 開発部: 部署;
    edge 太郎の所属 = 所属(太郎 -> 開発部);
}
```

`太郎` は `太郎Ref` という具象ローカル型になり、`g.node_refs.太郎` から
たどれます。`src/main.rs` の `impl<'a> 太郎Ref<'a> { fn あだ名(&self) -> .. }`
のように、マクロの外から普通の `impl` でメソッドを後付けできます
(`{個体名}Ref` は孤児規則 (E0116) に落ちない具象ローカルstructだからです)。
