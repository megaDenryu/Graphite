# 一括構築API

この文書は、実行時データからグラフを構築する `Builder::extend` の現行契約を定める生存型文書である。

`extend` はノードとエッジに共通する一括挿入APIである。値の型がどの内部表へ入るかはRustの型推論が決める。

```rust
let graph = Org::Graph::create(|builder| {
    builder.extend(people.into_iter().map(|person| (person.code.clone(), person)));
    builder.extend(
        pairs
            .into_iter()
            .enumerate()
            .map(|(index, (from, to))| {
                (
                    format!("dependency-{index}"),
                    Org::DependsOn::new(from, to),
                )
            }),
    );
});
```

入力は `IntoIterator<Item = (K, T)>` であり、`K: Into<String>` を満たす必要がある。戻り値は挿入順の `Vec<T::Id>` である。重複IDと端点の検証は、要素単位の `insert` / `add` と同じくfreeze時に行う。

`extend` は文字列からschema module内の既定IDを作れる要素だけを受け付ける。既存ID型を `(id: 型パス)` で明示した要素には、要素単位の `insert_with_id` / `add_with_id` を使う。明示IDを一括投入する専用構文は現時点では提供しない。

`graph!` の `..式` は `extend` への糖衣であり、同じ型境界と挿入順保証を持つ。詳細は `docs/graph_splice.md` を参照する。生成される `extend` の実際の形と、挿入APIの一覧は `docs/desugaring_reference.md` §16・§17 にある。
