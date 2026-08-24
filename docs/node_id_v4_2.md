# ID型の既定生成と明示指定

この文書は、`graph_schema!` がノード・エッジのID型を選ぶ規則と、`graph!` がID値を受け取る規則を定める生存型文書である。

## schema宣言

ID型を省略すると、`graph_schema!` は schema module 内に型付き文字列IDを生成する。

```rust
graphite::graph_schema! {
    schema Org {
        node Person;
        edge Knows = Person -> Person;
    }
}

// 生成される公開型
// Org::PersonId(pub String)
// Org::KnowsId(pub String)
```

生成ID型は `Debug, Clone, PartialEq, Eq, Hash` を導出する。同じノード型を複数のschemaが参照しても、`Org::PersonId` と `Approval::PersonId` は別型である。

生成ID型は `PartialOrd` / `Ord` を導出しない。IDの順序がアプリケーションで必要な場合は、利用者側で実装する。`examples/dialogue-engine` と `examples/state-machine` が実例である。

既存型を使う場合は、宣言に `(id: 型パス)` を付ける。

```rust
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EmployeeNumber(pub u64);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct RelationNumber(pub u64);

graphite::graph_schema! {
    schema Org {
        node Person(id: EmployeeNumber);
        edge Knows(id: RelationNumber) = Person -> Person;
    }
}
```

明示IDを選んだ宣言では、対応する `Org::PersonId` や `Org::KnowsId` を生成しない。既存型に必要な能力は `Clone + Eq + Hash` であり、`Debug`・`Display`・文字列変換は要求しない。同じID型を複数のschemaへ明示すれば、schema間でIDを共有できる。

自動生成名どうしが型名前空間で衝突する場合、マクロは衝突箇所を診断する。既存ID型を使う意図がある場合は、同名の型を暗黙に拾わせず `(id: 型パス)` を書く。

## graph!リテラル

既定IDは、従来どおり束縛名から作る。

```rust
let graph = graphite::graph!(Org {
    alice = Person,
    bob = Person,
    relation = Knows(alice -> bob),
})?;
```

この例では `alice` が `Org::PersonId("alice".into())` に、`relation` が `Org::KnowsId("relation".into())` になる。

ID値を明示するときは `名前 @ ID式 = 値` と書く。`@` の右側は普通のRust式である。

```rust
let graph = graphite::graph!(Org {
    alice @ EmployeeNumber(10) = Person,
    bob @ EmployeeNumber(20) = Person,
    relation @ RelationNumber(30) = Knows(alice -> bob),
})?;
```

明示ID型を使う宣言は文字列からIDを作れないため、`@` を省略するとトレイト境界のコンパイルエラーになる。既定IDにも `alice @ Org::PersonId("external-name".into()) = Person` のように明示値を渡せる。

## builderと一括構築

`Builder::insert`・`add`・`extend` は文字列から既定IDを作る。明示IDでは `insert_with_id`・`add_with_id` を使う。

```rust
Org::Graph::create(|builder| {
    let alice = builder.insert_with_id(EmployeeNumber(10), Person);
    let bob = builder.insert_with_id(EmployeeNumber(20), Person);
    builder.add_with_id(RelationNumber(30), Org::Knows(alice, bob));
})?;
```

現在の `extend` と `graph!` の `..式` は既定IDだけを受け付ける。スプライスへ明示IDを渡す構文と名前の意味論はIssue #6/#2で確定する。

## 格納方式

IDは密な配列添字ではない。GraphiteはIDを `Hash + Eq` のキーとして扱い、挿入順を別の配列に保持する。このため、文字列IDと利用者定義IDは同じ格納経路を使う。
