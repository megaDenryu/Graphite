# キーの意味論 v4.2 — ノード Id のユーザー宣言化 (Fudaba #15)

2026-07-18 のユーザー決定。

## 決定した意味論

**キーは個体の名前である。** グラフとは「名前を持つ個体たちについての主張の集合」
であり、複数のグラフ (組織図と承認フロー) が同じ個体宇宙について語ることは正当。
したがって `PersonId` は特定のグラフにではなく **`Person` という型に 1 個だけ属する**。

導かれる規則: **「型を宣言した者が、その Id も宣言する」**

- `Person` はユーザーがマクロの外で宣言する → `PersonId` も**ユーザーが宣言**する。
  `Person` の隣の 1 行は「この型は名前で識別される個体である」という意味のある宣言
- `Boss` (辺種別) は schema が作る → `BossId` は従来どおり**マクロが生成**する
  (辺の同一性は schema 局所で、共有のしようがない)

複数 schema での `PersonId` 共有は、衝突 (旧 #3 の問題) ではなく当然の帰結になる:
組織図で得たキーを承認フローのクエリにそのまま渡せる。

## 構文とユーザーの書くもの

schema 構文は**不変**。ユーザーはノード型の隣に Id を宣言するだけ:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);
pub struct Person { pub name: String }

graphite::graph_schema! {
    schema Org {
        node Person;        // ← PersonId は生成されず、上の宣言を参照する
        ...
    }
}
```

- **命名規約**: schema は `{ノード型名}Id` という名前で参照する (Person → PersonId)。
  未宣言なら rustc の「cannot find type `PersonId`」がノード宣言のスパンで出る
  (参照トークンのスパン = node 宣言の型トークン)。rename は "Person" が
  "PersonId" に原文ママ含まれるため RA のカスケードが機能する (実測済みの規則)
- **形の規約**: `String` 1 要素のタプル struct。生成コードは `PersonId(文字列)` で
  構築するため、形が違えば rustc エラー (構築箇所のスパンで顕在化)
- **必要 derive**: `Debug, Clone, PartialEq, Eq, Hash` (HashMap キー + 違反 enum の
  表示に必要な最小。README に明記)
- 辺キー ({Kind}Id) は従来どおりマクロ生成 (derive も従来どおり)

## 実装ノート

- schema_codegen: ノード Id の struct 生成を削除し、型名参照 (`format_ident!("{}Id", ..)`
  相当 — 参照であって生成でないことをコメントに) に変更。スパンは node 宣言の
  型トークン (G3)
- {Schema}Node trait の `insert_into` 等、Id を構築している生成コードは
  `PersonId(key)` 構築のまま (参照先がユーザー型に変わるだけ)
- 破壊的変更: 全テスト・examples 7 本の schema 近傍にノード Id 宣言を追加する
  移行が必要。既定方針どおり旧状態の検出・診断なし
- README「キーの設計」節を v4.2 の意味論 (キー = 個体の名前・宣言規則) で更新。
  modeling_guide にも「Id 宣言 = ノードになれることの宣言」を 1 文追加。
  複数 schema の衝突に関する旧記述 (モジュール分割の回避策) は**共有が正当**に
  なったため書き換え
- IDE 実測 (実装後): `PersonId` 使用側 → ユーザー宣言への定義ジャンプ、
  schema の `node Person;` からの参照解決
