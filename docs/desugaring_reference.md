# 脱糖リファレンス — 構文・生成型・内部表現・公開API

脱糖とは、DSL (ドメイン特化言語。特定の用途に絞った専用の構文) の独自構文を、
それが実際に展開される普通のRustのコードへ置き換えることである。この文書は、
Graphiteの独自構文を消去したときにどの普通のRustのファイル・型・値・関数へ
対応するかを、曖昧さなく示す正本である。チュートリアルでも設計史でもない。

掲載するRustコードは、すべてリポジトリ内に実在する生成ファイルまたは実装ファイル
からの引用であり、引用ごとに出典のパスと行を併記する。生成ファイルは
`cargo xtask generate` が書き出したものであり、`cargo xtask generate --check` が
本文のバイト一致を検査している。一覧性を優先して本体を省いた箇所には
「(署名のみ抜粋)」と書く。それ以外の引用は出典の当該行を写しており、読みやすさの
ために先頭の字下げを落とした箇所だけが原文と異なる。1つの出典範囲が、続けて並ぶ
2つのコードブロックを覆うことがある。

出典を併記していないコードブロックは引用ではなく例示である。該当するのは、
8段組の第1段 (Graphite構文) に置く構文の字面と、マクロの展開結果を示すブロックの
2種類だけである。構文の字面の例示は `examples/hello-graph/src/main.rs:109-122` の
`schema Org` (ノード `Person`・`Team`、辺 `BelongsTo`・`Boss`・`Friends` 等) を
題材にしている。

書いてあるのは現行の実装だけである。過去の仕様は混在させない。歴史的な経緯は
`docs/edge_syntax_v2.md`・`docs/edge_syntax_v3.md`・`docs/graph_literal_v3.md`・
`docs/edge_view_api.md`・`docs/design_journal.html` にある。

## 目次

| 節 | 内容 |
|---|---|
| [0](#0-読み方) | 読み方 (8段組・生成方法の区別・用語・本文で使う実例) |
| [1](#1-schema-宣言) | `schema` 宣言 |
| [2](#2-node-宣言-既定id) / [3](#3-node-宣言-明示id) | `node` 宣言 (既定ID / 明示ID) |
| [4](#4-edge-宣言-有向役割名積み荷) | `edge` 宣言 (有向・役割名・積み荷) |
| [5](#5-where-each-役割名-多重度) / [6](#6-where-unique-pair) | `where each` (多重度) / `where unique pair` |
| [7](#7-edge-宣言-無向) / [8](#8-edge-宣言-明示id) | `edge` 宣言 (無向 / 明示ID) |
| [9](#9-辺値型は普通のrustの値である) | 辺値型は普通のRustの値である |
| [10](#10-noderef) / [11](#11-edgeref) | `NodeRef` / `EdgeRef` |
| [12](#12-graph-の名前付き要素と短縮形の脱糖) | `graph!` の名前付き要素と短縮形の脱糖 |
| [13](#13-graph-の明示id-名前--id式--値) | `graph!` の明示ID |
| [14](#14-graph-の名前付きラッパーと静的アクセサ) | `graph!` の名前付きラッパーと静的アクセサ |
| [15](#15-graph-の許可証と構築印) | `graph!` の許可証と構築印 |
| [16](#16-graph-のスプライス-式) | `graph!` のスプライス |
| [17](#17-種別api-graphが所有者) | 種別API |
| [18](#18-noderef-からの探索) | `NodeRef` からの探索 |
| [19](#19-値可変api) | 値可変API |
| [20](#20-凍結の完成処理) | 凍結の完成処理 |
| [21](#21-構造不変性と値可変性の分離) | 構造不変性と値可変性の分離 |
| [22](#22-公開生成物とprivate生成物の境界) | 公開生成物とprivate生成物の境界 |
| [23](#23-自動生成物の一覧) | 自動生成物の一覧 |
| [24](#24-計算量と確保契約) | 計算量と確保契約 |
| [25](#25-3つのアクセス経路) | 3つのアクセス経路 |
| [26](#26-生成コードの配置と追跡性) | 生成コードの配置と追跡性 |
| [27](#27-検証方法) | 検証方法 |

## 0. 読み方

### 0.1 8段組

構文ごとに、次の8つを必ずこの順で示す。

| 段 | 名前 | 意味 |
|---|---|---|
| 1 | Graphite構文 | 利用者が書くDSLの字面 (`schema` 宣言・`graph!` リテラル) |
| 2 | 利用者定義 | その構文が参照する、利用者が普通のRustで書く型 |
| 3 | 公開生成物 | 生成ファイルに置かれ、利用者が名前で触れるもの |
| 4 | private生成物 | 生成ファイルに置かれ、Rustのモジュール外から触れないもの |
| 5 | 構築時の処理 | `Builder` へ積む段階で起きること |
| 6 | 完成済みGraphの内部保存 | 凍結後に何として保持されるか |
| 7 | 公開API | 完成済みGraphに対して呼べるもの |
| 8 | 計算量 | 上記公開APIの計算量とヒープ確保 |

該当が無い段には「なし。」と書く。

### 0.2 生成方法の区別

| 生成物の種類 | 生成方法 | 追跡性 |
|---|---|---|
| schemaに由来する公開API | `cargo xtask generate` が通常のRustファイルへ生成 | 定義ジャンプが生成ファイルの実装行へ着地する |
| schemaに由来する非公開の内部実装 | 同じ生成ファイルへ生成し、Rustの可視性で閉じる | 同上 (ただしモジュール外から参照できない) |
| `graph!` の名前付きラッパー | 手続き型マクロが呼び出し箇所へ展開 | 左辺の識別子のスパンを継承する |

公開APIの理解に `cargo expand` は要らない。生成ファイルをそのまま読めばよい。

### 0.3 用語

名前付きラッパー・名前付き位置型・呼び出し箇所・凍結の4語は
`docs/schema_v4.md` §3.1.1 が正本であり、ここへ同じ定義を再掲する。残りの語は
この節で定義する。

- **役割名**とは、辺宣言で端点と積み荷に付ける名前のことである (`(buyer: Person)`
  の `buyer`)。
- **多重度**とは、`where each <役割名>: ...` が課す本数の制約のことである。
- **内部位置**とは、種別ごとの格納配列の添字を1つだけ包んだ非公開の型 (newtype。
  既存の型を薄く包んで別の型として扱う手法) のことである。
- **名前付きラッパー**とは、`graph!` が呼び出し箇所ごとに生成する、素の `Graph` と
  名前付き位置型を保持する構造体のことである。
- **名前付き位置型**とは、`graph!` が要素ごとに生成する、内部位置と構築印を保持する
  型のことである。
- **呼び出し箇所**とは、`graph!` を1回呼んだ場所のことである。
- **凍結**とは、`Builder` に積んだ要素を検査して確定済み `Graph` へ変換する操作の
  ことである。
- **種別API**とは、ある種別に属する個体の全体を対象にする読み取り・可変操作のこと
  である。所有者は完成済み `Graph` なので `Graph` のメソッドになる。
- **構築印**とは、1回の構築を識別する `u64` の値のことである。名前付き位置が生成元
  以外の `Graph` へ束縛されたことを実行時に検出するために使う。
- **スプライス**とは、`graph!` の項の先頭に `..式` と書き、名前を持たない要素を
  まとめて追加する項のことである。
- **許可証**とは、`Builder` の名前付き挿入メソッドを呼べることを示す値のことで
  ある。`graphite::build_named_graph` だけがこの値を作れる。
- **schema module**とは、schema名と同じ名前で利用者が書き、生成ファイルを
  `include!` で読み込むRustのモジュールのことである。

### 0.4 本文で使う実例

本文の引用は次の6組から採る。

| 宣言元 | 生成ファイル | 内容 |
|---|---|---|
| `crates/graphite/tests/edge_roles.rs:27` | `crates/graphite/tests/generated/edge_roles_commerce.rs` | 有向辺・積み荷・多重度・`unique pair` |
| `crates/graphite/tests/undirected_edges.rs:30` | `crates/graphite/tests/generated/undirected_edges_social.rs` | 無向辺 |
| `crates/graphite/tests/role_query.rs:45` | `crates/graphite/tests/generated/role_query_rev_query.rs` | 多重度ごとの索引形状と戻り型 |
| `crates/graphite/tests/schema_ids.rs:70` | `crates/graphite/tests/generated/schema_ids_mixed_ids.rs` | 既定IDと明示IDの混在 |
| `crates/graphite/tests/edge_roles.rs:50` | `crates/graphite/tests/generated/edge_roles_japanese_roles.rs` | 日本語の役割名 |
| `crates/graphite/tests/traversal_api.rs:23` | `crates/graphite/tests/generated/traversal_api_traversal.rs` | 日本語の種別名と探索メソッド |

以降、生成ファイルは `generated/<名前>.rs` と短く書く。実体は
`crates/graphite/tests/generated/<名前>.rs` である。

`Commerce` schemaの宣言は次のとおりである
(`crates/graphite/tests/edge_roles.rs:15-36` から引用)。

```rust
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(
    clippy::needless_lifetimes,
    clippy::wrong_self_convention,
    clippy::clone_on_copy,
    clippy::write_literal
)]
pub mod Commerce {
    include!("generated/edge_roles_commerce.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/edge_roles_commerce.rs";
    schema Commerce {
        node Person;
        node Product;

        edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where each buyer: 1..2, each product: 0..1, unique pair;
        edge Subscription = (member: Person) -> (product: Product) where each member: 1..*;
    }
}
```

## 1. `schema` 宣言

**1. Graphite構文**

```rust
graphite::graph_schema! {
    generated = "generated/edge_roles_commerce.rs";
    schema Commerce {
        node Person;
        node Product;
        edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where each buyer: 1..2, each product: 0..1, unique pair;
        edge Subscription = (member: Person) -> (product: Product) where each member: 1..*;
    }
}
```

`generated = "..."` は必須である。宣言元Rustファイルからの相対パスで、
`generated/<名前>.rs` の形式でなければならない。絶対パスと `..` は拒否する
(`crates/graphite-codegen/src/generated_path.rs`)。

**2. 利用者定義**

schema名は利用者が決める。同じ名前のRustのモジュールを利用者が自分で書き、生成ファイルを
`include!` で読み込む。moduleへ付ける2行の属性は
`docs/code_generation.md` が定める固定の並びである。

**3. 公開生成物**

`schema Commerce` は `Commerce` という名前のRust moduleを展開しない。schema名の
moduleは利用者が書いた `pub mod Commerce { include!(...); }` そのものであり、その
本文が生成ファイルである。生成ファイルの先頭は次の形になる
(`crates/graphite/tests/generated/edge_roles_commerce.rs:1-11`)。

```rust
// このファイルは Graphite が生成したため手編集しないこと。
// 生成元: crates/graphite/tests/edge_roles.rs:27
// 再生成: リポジトリルートで `cargo xtask generate` を実行する。

#[allow(unused_imports)]
use super::*;
#[doc(hidden)]
pub(super) const __GRAPHITE_SCHEMA_FINGERPRINT: [u64; 4] = [
    2989501799219349444u64, 7255621562707497463u64, 5755820933598649166u64,
    8070068216230268690u64,
];
```

`use super::*;` があるため、生成ファイルの中では利用者が親moduleに書いた値型・
積み荷型・明示ID型がそのまま名前で見える。

生成ファイルが公開する型と関数の一覧は §22 にある。

**4. private生成物**

指紋の定数は `#[doc(hidden)] pub(super)` である。`graph_schema!` の照合はschema
moduleを囲む親のモジュール (宣言を書いたファイル) に展開されるので、そこからだけ見えれば
足りる。より外の利用コードやクレートからは見えない。

**5. 構築時の処理**

`graph_schema!` が展開するのは、指紋を照合する `const` ブロック1つだけである。
展開するトークンのテンプレートは `crates/graphite-macros/src/lib.rs:92-104` に
ある。`Commerce` schemaへ適用した展開結果は次の形になる (テンプレートの `#変数` を
実際の値で埋めたものであり、生成器の出力そのものではない)。

```rust
const _: () = {
    let actual = Commerce::__GRAPHITE_SCHEMA_FINGERPRINT;
    if !(actual[0] == 2989501799219349444u64
        && actual[1] == 7255621562707497463u64
        && actual[2] == 5755820933598649166u64
        && actual[3] == 8070068216230268690u64)
    {
        panic!("Graphite schema の生成ファイルが古いため、リポジトリルートで cargo xtask generate を実行してください");
    }
};
```

指紋とは、生成先の相対パスと、整形済みの生成本文を連結した文字列に対して、
FNV-1a (64bit) を4種の初期値でそれぞれ計算した `[u64; 4]` である
(`crates/graphite-codegen/src/lib.rs:244-258`)。暗号強度のハッシュではなく、
schemaの意味を変えて生成し忘れた場合に通常の `cargo build` を失敗させるための
目印である。const評価で比較するため、照合の実行時費用はない。

**6. 完成済みGraphの内部保存**

なし。schema宣言そのものは値を持たない。

**7. 公開API**

なし。schema宣言そのものは呼べるAPIを持たない。

**8. 計算量**

指紋の照合はコンパイル時に完結する。実行時費用はない。

## 2. `node` 宣言 (既定ID)

**1. Graphite構文**

```rust
node Person;
```

**2. 利用者定義**

```rust
#[derive(Clone, PartialEq)]
pub struct Person;
```

ノード値型は利用者が普通のRustで書く。Graphiteは生成しない。`NodeRef` の
`Deref::Target` に現れるため、schema moduleから到達できる可視性が要る。

**3. 公開生成物**

既定ID型を1つ生成する (`generated/edge_roles_commerce.rs:12-13`)。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);
```

`PartialOrd` / `Ord` は導出しない。順序が必要なら利用者側で実装する。

同じノード値型を複数のschemaが参照しても、`Commerce::PersonId` と
`JapaneseRoles::PersonId` は別型である。

`NodeRef` (§10) と、`Builder` のノード挿入メソッド (§17) も同時に生成する。

**4. private生成物**

内部位置型を生成する (`generated/edge_roles_commerce.rs:20-21`)。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PersonInternalPosition(usize);
```

名前付き位置型は `#[doc(hidden)] pub` である (§14 で扱う)。

**5. 構築時の処理**

`Builder` は種別ごとの `Vec<(PersonId, super::Person)>` へ末尾追加するだけで、検査は
一切行わない (`generated/edge_roles_commerce.rs:1048-1051`)。

```rust
pub fn person(&mut self, id: PersonId, value: super::Person) -> &mut Self {
    self.__graphite_node_person.push((id, value));
    self
}
```

**6. 完成済みGraphの内部保存**

`Graph` は種別ごとに1つのキー付き要素表を持つ
(`generated/edge_roles_commerce.rs:218`)。

```rust
__graphite_node_person: graphite::KeyedTable<PersonId, super::Person>,
```

`KeyedTable` は「挿入順の本体 `Vec<(K, V)>`」と「キーから添字への `HashMap<K, usize>`」
の組である (`crates/graphite/src/keyed_table.rs:27-34`)。内部位置はこの `Vec` の
添字である。

**7. 公開API**

`Graph` のノード種別APIは次の5つである
(`generated/edge_roles_commerce.rs:248-283`、署名のみ抜粋)。

```rust
pub fn person_by_id<'graph>(&'graph self, id: &PersonId) -> Option<PersonRef<'graph>>;
pub fn person_value_mut(&mut self, id: &PersonId) -> Option<&mut super::Person>;
pub fn person_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph PersonId>;
pub fn person_iter<'graph>(&'graph self) -> impl Iterator<Item = PersonRef<'graph>> + 'graph;
pub fn person_len(&self) -> usize;
```

メソッド名はノード型名を snake_case (単語を小文字にして下線で繋ぐ命名規則) にした形と
固定接尾辞の機械的連結である。自然言語の
複数形は生成しない。日本語のノード型名なら `人物_by_id` になる
(`crates/graphite-codegen/src/naming.rs:103-105`)。

**8. 計算量**

`person_by_id` と `person_value_mut` は平均O(1) (`HashMap` 検索)。`person_ids` と
`person_iter` は開始O(1)・走査O(要素数)。`person_len` はO(1)。いずれもヒープを
確保しない。

## 3. `node` 宣言 (明示ID)

**1. Graphite構文**

```rust
node ExternalNode(id: ExternalNodeId);
```

**2. 利用者定義**

```rust
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExternalNodeId(pub u64);

pub struct ExternalNode {
    pub name: &'static str,
}
```

明示ID型に要求する能力は `Clone + Eq + Hash` だけである。`Debug`・`Display`・
文字列変換は要求しない。

**3. 公開生成物**

`ExternalNodeId` は**生成しない**。生成ファイルの既定ID型は、明示指定の無い宣言の
分だけである (`generated/schema_ids_mixed_ids.rs:12-15`)。

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutomaticNodeId(pub String);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AutomaticLinkId(pub String);
```

`ExternalNodeRef<'graph>` と種別APIは既定IDの場合と同じ形で生成する。種別APIの
ID引数の型が利用者の型になるだけである
(`generated/schema_ids_mixed_ids.rs:363-366`、署名のみ抜粋)。

```rust
pub fn external_node_by_id<'graph>(
    &'graph self,
    id: &ExternalNodeId,
) -> Option<ExternalNodeRef<'graph>> {
```

**4. private生成物**

`__ExternalNodeInternalPosition` を生成する。既定IDの場合と同じである。

**5. 構築時の処理**

明示ID型を持つ種別は、束縛名の文字列からIDを作る経路を持たない。生成ファイルは
`MixedIdsInsertable` を実装するが `MixedIdsDefaultId` は実装しない
(`generated/schema_ids_mixed_ids.rs:918-920` と、同ファイルに
`impl MixedIdsDefaultId for super::ExternalNode` が存在しないこと)。

```rust
impl MixedIdsInsertable for super::ExternalNode {
    type Id = ExternalNodeId;
    type NamedPosition = __ExternalNodeNamedPosition;
```

このため `graph!` で `@ ID式` を省略するとトレイト境界のコンパイルエラーになり、
`Builder::insert` (文字列キー版) も使えない。使うのは `insert_with_id` である。

**6. 完成済みGraphの内部保存**

`graphite::KeyedTable<ExternalNodeId, super::ExternalNode>`。格納経路は既定IDと同じで
ある。IDは密な配列添字ではなく `Hash + Eq` のキーとして扱い、挿入順は別の配列が
保持する。

**7. 公開API**

既定IDと同じ5つ。ID引数と `ids()` の要素型が利用者の型になる。

`Debug` と `Display` の契約は既定IDと異なる。生成コードは利用者定義のID型・値型・
積み荷型に `Debug` を要求しないため、表示に含めるのは既定生成ID型に限る。違反の
表示も同じ規則に従う (`generated/schema_ids_mixed_ids.rs:196-203`)。

```rust
Violation::DuplicateExternalNode(_) => {
    write!(f, "{}のキーが重複しています", "ExternalNode")
}
Violation::DuplicateAutomaticNode(id) => {
    write!(
        f, "{}のキーが重複しています: {:?}", "AutomaticNode", id
    )
}
```

**8. 計算量**

既定IDと同じ。`Hash + Eq` の実装費用だけが利用者側の型に依存する。

## 4. `edge` 宣言 (有向・役割名・積み荷)

**1. Graphite構文**

```rust
edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product);
```

端点の役割名は両端とも必須で、`(役割名: 型)` と括弧で囲む。積み荷があるときは
`[役割名: 型]` の役割名も必須である。積み荷が無い辺は柄を `->` と書く。

**2. 利用者定義**

積み荷型は利用者が普通のRustで書く。

```rust
#[derive(Clone, PartialEq)]
pub struct TransactionInfo {
    amount: u64,
}
```

Graphiteは積み荷型を生成せず、参照するだけである。

**3. 公開生成物**

辺種別ごとに、構築用の辺値型を1つ生成する
(`generated/edge_roles_commerce.rs:40-71`)。

```rust
#[derive(Clone, PartialEq)]
pub struct Purchase {
    pub buyer: PersonId,
    pub product: ProductId,
    pub info: TransactionInfo,
}
impl Purchase {
    pub fn new(from: PersonId, to: ProductId, payload: TransactionInfo) -> Self {
        Self {
            buyer: from,
            product: to,
            info: payload,
        }
    }
    pub fn payload(&self) -> &TransactionInfo {
        &self.info
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, ProductId, TransactionInfo> for Purchase {
    fn from_graph_literal(
        from: PersonId,
        to: ProductId,
        payload: TransactionInfo,
    ) -> Self {
        Self::new(from, to, payload)
    }
}
impl std::fmt::Debug for Purchase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(Purchase))
    }
}
```

辺値はGraphを借用しない普通のRustの値である。Graphへ登録する前に構築でき、
フィールドは役割名そのものである。辺種別は名前で区別される型であり、同じ形の
`Boss` と `Mentor` は別型である。

積み荷の無い辺値は積み荷フィールドと `payload()` を持たない
(`generated/edge_roles_commerce.rs:72-94`)。

```rust
#[derive(Clone, PartialEq)]
pub struct Subscription {
    pub member: PersonId,
    pub product: ProductId,
}
impl Subscription {
    pub fn new(from: PersonId, to: ProductId) -> Self {
        Self { member: from, product: to }
    }
}
impl graphite::DirectedEdgeLiteral<PersonId, ProductId, ()> for Subscription {
    fn from_graph_literal(from: PersonId, to: ProductId, (): ()) -> Self {
        Self::new(from, to)
    }
}
impl std::fmt::Debug for Subscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(stringify!(Subscription))
            .field(&self.member)
            .field(&self.product)
            .finish()
    }
}
```

辺値の `Debug` は、積み荷が無く両端の公開IDが既定生成ID型のときだけ端点を表示する。
それ以外は種別名だけを書く。利用者定義の型へ `Debug` を要求しないためである
(`crates/graphite-codegen/src/schema_codegen.rs:1500-1525`)。

既定ID型 `PurchaseId` (§2と同じ形)、`PurchaseRef<'graph>` (§11)、`Violation` の各
variant (列挙型 `Violation` の1分岐。§20)、`Builder` の `purchase` メソッドも同時に
生成する。

**4. private生成物**

内部位置型と、凍結後の辺記録を生成する
(`generated/edge_roles_commerce.rs:24-25, 95-100`)。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct __PurchaseInternalPosition(usize);

#[allow(dead_code)]
struct __PurchaseRecord {
    buyer: __PersonInternalPosition,
    product: __ProductInternalPosition,
    info: TransactionInfo,
}
```

辺記録は端点を**公開IDではなく内部位置で**保持する。積み荷だけを辺値から移す。

**5. 構築時の処理**

`Builder` は辺値をそのまま `Vec<(PurchaseId, Purchase)>` へ末尾追加する
(`generated/edge_roles_commerce.rs:1056-1059`)。端点の存在検査も多重度検査も
凍結まで行わない。

**6. 完成済みGraphの内部保存**

`Graph` は辺種別ごとに、辺表と役割索引と端点対索引を持つ
(`generated/edge_roles_commerce.rs:220-230`)。

```rust
purchase: graphite::KeyedTable<PurchaseId, __PurchaseRecord>,
/// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
/// キーの一覧 (凍結時に構築)。
purchase_from_index: graphite::MultipleRoleIndex<__PurchaseInternalPosition>,
/// 位置1キー (終点) -> そこへ入るエッジキーの一覧 (凍結時に
/// 構築。終点役割クエリの索引、`docs/reverse_query.md`)。
purchase_to_index: graphite::OptionalRoleIndex<__PurchaseInternalPosition>,
__graphite_purchase_by_pair: std::collections::HashMap<
    (__PersonInternalPosition, __ProductInternalPosition),
    __PurchaseInternalPosition,
>,
```

索引の型は多重度と `unique pair` の宣言で静的に決まる (§5、§6)。

**7. 公開API**

辺種別APIは `Graph` に生える
(`generated/edge_roles_commerce.rs:320-357`、署名のみ抜粋)。

```rust
pub fn purchase_by_id<'graph>(&'graph self, id: &PurchaseId) -> Option<PurchaseRef<'graph>>;
pub fn purchase_payload_mut(&mut self, id: &PurchaseId) -> Option<&mut TransactionInfo>;
pub fn purchase_ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph PurchaseId>;
pub fn purchase_iter<'graph>(&'graph self) -> impl Iterator<Item = PurchaseRef<'graph>> + 'graph;
pub fn purchase_len(&self) -> usize;
```

`payload_mut` は積み荷を持つ辺種別にだけ生成する。`Subscription` には無い。

**8. 計算量**

`purchase_by_id` と `purchase_payload_mut` は平均O(1)。`purchase_ids` と
`purchase_iter` は開始O(1)・走査O(要素数)。`purchase_len` はO(1)。いずれもヒープを
確保しない。

## 5. `where each <役割名>` (多重度)

**1. Graphite構文**

```rust
edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where each buyer: 1..2, each product: 0..1;
edge Subscription = (member: Person) -> (product: Product) where each member: 1..*;
edge ExactlyOne = (src: NodeA) -[weight: Weight]-> (dst: NodeB) where each dst: 1;
```

`each <役割名>: N` はちょうどN本、`N..M` は範囲、`N..*` は下限のみを課す。始点側と
終点側へ独立に書ける。同じ役割名への重複は拒否する。存在しない役割名も拒否する。
無向辺は役割名を持たないため `each` を書けない。

**2. 利用者定義**

なし。

**3. 公開生成物**

多重度は戻り型を決める。役割クエリの戻り型は次の3つである
(`generated/role_query_rev_query.rs:1343-1392`)。

```rust
    pub fn unconstrained_as_target(
        self,
    ) -> impl Iterator<Item = UnconstrainedRef<'graph>> + 'graph {
        let positions = self.graph.unconstrained_to_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| UnconstrainedRef {
                graph: self.graph,
                internal_position,
            })
    }
```

```rust
    pub fn at_most_one_as_dst(self) -> Option<AtMostOneRef<'graph>> {
        self.graph
            .at_most_one_to_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| AtMostOneRef {
                graph: self.graph,
                internal_position,
            })
    }
    /// この役割に接続する唯一の辺を O(1)、追加確保なしで返す。
    pub fn exactly_one_as_dst(self) -> ExactlyOneRef<'graph> {
        ExactlyOneRef {
            graph: self.graph,
            internal_position: *self
                .graph
                .exactly_one_to_index
                .get(self.internal_position.0),
        }
    }
```

対応は次のとおりである。

| 宣言 | 役割クエリの戻り型 | 索引の型 |
|---|---|---|
| `each <役割名>: 1` | `{Kind}Ref<'graph>` | `graphite::ExactlyOneRoleIndex<P>` |
| `each <役割名>: 0..1` | `Option<{Kind}Ref<'graph>>` | `graphite::OptionalRoleIndex<P>` |
| 上記以外 (`N..M`・`N..*`・制約なし) | `impl Iterator<Item = {Kind}Ref<'graph>> + 'graph` | `graphite::MultipleRoleIndex<P>` |

多重度違反のvariantも生成する (`generated/edge_roles_commerce.rs:117-120`)。

```rust
/// このエッジ種別の `each` 制約違反 (出次数)。
PurchaseBuyerEachViolation { source: PersonId, count: usize },
/// このエッジ種別の `each` 制約違反 (入次数)。
PurchaseProductEachViolation { target: ProductId, count: usize },
```

variant名は辺種別名と役割名から機械的に導出する
(`crates/graphite-codegen/src/naming.rs:57-70`)。役割名が日本語なら
`Ownership所有者EachViolation` になる
(`generated/edge_roles_japanese_roles.rs:76`)。

**4. private生成物**

索引フィールドの型が上表のとおり切り替わる。それ以外の非公開生成物は変わらない。

**5. 構築時の処理**

なし。`Builder` は多重度を検査しない。

**6. 完成済みGraphの内部保存**

`Graph` の役割索引フィールドの型が多重度で決まる
(`generated/role_query_rev_query.rs:336, 365, 376`)。

```rust
    unconstrained_to_index: graphite::MultipleRoleIndex<__UnconstrainedInternalPosition>,
```

```rust
    at_most_one_to_index: graphite::OptionalRoleIndex<__AtMostOneInternalPosition>,
```

```rust
    exactly_one_to_index: graphite::ExactlyOneRoleIndex<__ExactlyOneInternalPosition>,
```

3種の索引はいずれもノードの内部位置を添字にする配列である
(`crates/graphite/src/lib.rs:41-123`)。`MultipleRoleIndex` は範囲の配列と連続した
辺位置列の組であり、問い合わせでスライスを借用して返す。`OptionalRoleIndex` は
`Vec<Option<P>>`、`ExactlyOneRoleIndex` は `Vec<P>` である。

**7. 公開API**

`{node_ref}.{kind}_as_{役割名}()`。詳細は §18。

**8. 計算量**

3種いずれも添字参照でO(1)。ハッシュを計算しない。`MultipleRoleIndex` の場合は走査が
O(件数)で、問い合わせ時に `Vec` を確保しない。

## 6. `where unique pair`

**1. Graphite構文**

```rust
edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where unique pair;
```

`each` との併記もできる。

**2. 利用者定義**

なし。

**3. 公開生成物**

違反variantを1つ追加する (`generated/edge_roles_commerce.rs:121-123`)。

```rust
/// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
/// 2本目の辺が張られた)。
PurchaseUniquePairViolation { source: PersonId, target: ProductId },
```

`between` の戻り型が変わる (§18)。

**4. private生成物**

端点対索引の値型が変わる。`unique pair` があれば辺位置1つ、無ければ辺位置の `Vec`
である (`generated/edge_roles_commerce.rs:227-241`)。

```rust
__graphite_purchase_by_pair: std::collections::HashMap<
    (__PersonInternalPosition, __ProductInternalPosition),
    __PurchaseInternalPosition,
>,
```

```rust
__graphite_subscription_by_pair: std::collections::HashMap<
    (__PersonInternalPosition, __ProductInternalPosition),
    Vec<__SubscriptionInternalPosition>,
>,
```

**5. 構築時の処理**

なし。`Builder` は対の重複を検査しない。

**6. 完成済みGraphの内部保存**

上記の端点対索引。キーは端点の内部位置の対である。有向辺は順序付きのタプル、
無向辺は `graphite::UnorderedPair` である (§7)。

**7. 公開API**

| 宣言 | `{kind}_between` の戻り型 | `{kind}_try_between` の戻り型 |
|---|---|---|
| `unique pair` あり | `Option<{Kind}Ref<'graph>>` | `Result<Option<{Kind}Ref<'graph>>, graphite::GraphMismatch>` |
| `unique pair` なし | `impl Iterator<Item = {Kind}Ref<'graph>> + 'graph` | `Result<impl Iterator<Item = {Kind}Ref<'graph>> + 'graph, graphite::GraphMismatch>` |

**8. 計算量**

どちらも平均O(1)の `HashMap` 検索である。`unique pair` なしの場合は結果の走査が
O(一致件数)。どちらもヒープを確保しない。`HashMap` から借用したスライスを走査する
だけである。

## 7. `edge` 宣言 (無向)

**1. Graphite構文**

```rust
edge Friends = Person -- Person where unique pair;
edge Wire = Person -[cable: Cable]- Person;
```

無向辺の端点は括弧を付けず、役割名も書けない。両端は同じノード型でなければ
ならない。積み荷の役割名は必須である。`each` は使えず、書ける制約は
`unique pair` だけである。自己ループは許す。

**2. 利用者定義**

積み荷型 `Cable` を利用者が書く。

**3. 公開生成物**

辺値は端点を非公開の順序なし対で保持する
(`generated/undirected_edges_social.rs:33-51`)。

```rust
#[derive(Clone, PartialEq)]
pub struct Friends {
    endpoints: graphite::UnorderedPair<PersonId>,
}
impl Friends {
    pub fn new(a: PersonId, b: PersonId) -> Self {
        Self {
            endpoints: graphite::UnorderedPair::new(a, b),
        }
    }
    pub fn endpoints(&self) -> (&PersonId, &PersonId) {
        self.endpoints.endpoints()
    }
}
impl graphite::UndirectedEdgeLiteral<PersonId, ()> for Friends {
    fn from_graph_literal(a: PersonId, b: PersonId, (): ()) -> Self {
        Self::new(a, b)
    }
}
```

`graphite::UnorderedPair<T>` の `PartialEq` と `Hash` は順序を区別しない
(`crates/graphite/src/unordered_pair.rs:20-40`)。したがって
`Friends::new(alice, bob) == Friends::new(bob, alice)` である。

積み荷ありの無向辺は積み荷を公開フィールドに持つ
(`generated/undirected_edges_social.rs:60-78`)。

```rust
#[derive(Clone, PartialEq)]
pub struct Wire {
    endpoints: graphite::UnorderedPair<PersonId>,
    pub cable: Cable,
}
impl Wire {
    pub fn new(a: PersonId, b: PersonId, payload: Cable) -> Self {
        Self {
            endpoints: graphite::UnorderedPair::new(a, b),
            cable: payload,
        }
    }
    pub fn endpoints(&self) -> (&PersonId, &PersonId) {
        self.endpoints.endpoints()
    }
    pub fn payload(&self) -> &Cable {
        &self.cable
    }
}
```

違反variantは端点の位置を区別しない
(`generated/undirected_edges_social.rs:104-109`)。

```rust
/// このエッジが未知の端点キーを参照している (無向のため位置の
/// 区別は無い)。
FriendsUnknownEndpoint { edge: FriendsId, endpoint: PersonId },
/// このエッジ種別の `unique pair` 違反 (無向のため
/// 順序を無視した対で判定)。
FriendsUniquePairViolation { a: PersonId, b: PersonId },
```

**4. private生成物**

辺記録も順序なし対で保持する (`generated/undirected_edges_social.rs:89-97`)。

```rust
#[allow(dead_code)]
struct __FriendsRecord {
    endpoints: graphite::UnorderedPair<__PersonInternalPosition>,
}
#[allow(dead_code)]
struct __WireRecord {
    endpoints: graphite::UnorderedPair<__PersonInternalPosition>,
    cable: Cable,
}
```

**5. 構築時の処理**

有向辺と同じで、`Builder` は末尾追加だけを行う。

**6. 完成済みGraphの内部保存**

役割索引は方向を持たないので1本だけになり、端点対索引のキーが
`UnorderedPair` になる (`generated/undirected_edges_social.rs:162-169`)。

```rust
    friends: graphite::KeyedTable<FriendsId, __FriendsRecord>,
    /// 位置0キー -> このキーから (有向: 出る / 無向: 接続する) エッジ
    /// キーの一覧 (凍結時に構築)。
    friends_index: graphite::MultipleRoleIndex<__FriendsInternalPosition>,
    __graphite_friends_by_pair: std::collections::HashMap<
        graphite::UnorderedPair<__PersonInternalPosition>,
        __FriendsInternalPosition,
    >,
```

**7. 公開API**

`EdgeRef` は `from()` / `to()` / `from_id()` / `to_id()` を持たず、`endpoints()` で
両端を返す (`generated/undirected_edges_social.rs:340-352`)。

```rust
    pub fn endpoints(self) -> (PersonRef<'graph>, PersonRef<'graph>) {
        let (first, second) = self.record().endpoints.endpoints();
        (
            PersonRef {
                graph: self.graph,
                internal_position: __PersonInternalPosition(first.0),
            },
            PersonRef {
                graph: self.graph,
                internal_position: __PersonInternalPosition(second.0),
            },
        )
    }
```

`NodeRef` 側の探索は役割名を捏造せず `{kind}_incident()` になる
(`generated/undirected_edges_social.rs:541-551`)。

```rust
    /// 接続辺を O(1) で参照し、追加確保なしで挿入順に走査する。
    pub fn friends_incident(self) -> impl Iterator<Item = FriendsRef<'graph>> + 'graph {
        let positions = self.graph.friends_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| FriendsRef {
                graph: self.graph,
                internal_position,
            })
    }
```

`{kind}_between` は順序を無視して検索する
(`generated/undirected_edges_social.rs:552-571`)。

```rust
    ///順序なし端点対を平均 O(1)、追加確保なしで検索する。
    pub fn friends_try_between(
        self,
        other: PersonRef<'graph>,
    ) -> Result<Option<FriendsRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_friends_by_pair
            .get(
                &graphite::UnorderedPair::new(
                    self.internal_position,
                    other.internal_position,
                ),
            )
            .copied();
```

種別API (`by_id` / `iter` / `ids` / `len` / `payload_mut`) は有向と同じである。

**8. 計算量**

`endpoints()` はO(1)。`{kind}_incident()` は開始O(1)・走査O(次数)。
`{kind}_between` は平均O(1)。いずれもヒープを確保しない。自己ループは次数として
1本と数える。

## 8. `edge` 宣言 (明示ID)

**1. Graphite構文**

```rust
edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1;
```

**2. 利用者定義**

```rust
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExternalEdgeId(pub u64);
```

**3. 公開生成物**

`ExternalLinkId` は生成しない。辺値型・`EdgeRef`・種別APIは既定IDと同じ形で生成し、
ID引数の型が利用者の型になる。ノードの明示IDと同じく `{Schema}DefaultId` を実装
しないため、文字列キーからIDを作る経路 (`add`・`extend`・`graph!` の短縮形) は
使えない。

**4. private生成物**

既定IDと同じ。

**5. 構築時の処理**

`Builder::add_with_id(id, value)` を使う。

**6. 完成済みGraphの内部保存**

`graphite::KeyedTable<ExternalEdgeId, __ExternalLinkRecord>`
(`generated/schema_ids_mixed_ids.rs:305`)。

**7. 公開API**

既定IDと同じ。`Debug` の表示規則は §3 と同じで、明示ID型を含む辺値は種別名だけを
表示する (`generated/schema_ids_mixed_ids.rs:66-70`)。

```rust
impl std::fmt::Debug for ExternalLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(stringify!(ExternalLink))
    }
}
```

**8. 計算量**

既定IDと同じ。

## 9. 辺値型は普通のRustの値である

**1. Graphite構文**

なし。§4 の `edge` 宣言の帰結である。

**2. 利用者定義**

なし。

**3. 公開生成物**

§4 に示した
`pub struct Purchase { pub buyer: PersonId, pub product: ProductId, pub info: TransactionInfo }`
と `Purchase::new`。

**4. private生成物**

なし。

**5. 構築時の処理**

辺値はGraphを借用しないため、Graphへ登録する前に自由に構築・保持・移動できる。
`crates/graphite/tests/edge_roles.rs:60-71` がこれを固定している。

```rust
#[test]
fn 辺値はgraph外で名前付きフィールドから構築できる() {
    let purchase = Commerce::Purchase {
        buyer: Commerce::PersonId("alice".into()),
        product: Commerce::ProductId("book".into()),
        info: TransactionInfo { amount: 100 },
    };

    assert_eq!(purchase.buyer, Commerce::PersonId("alice".into()));
    assert_eq!(purchase.product, Commerce::ProductId("book".into()));
    assert_eq!(purchase.info.amount, 100);
}
```

**6. 完成済みGraphの内部保存**

辺値そのものは保存しない。凍結が辺値を分解し、端点を内部位置へ解決した
`__PurchaseRecord` を保存する。

**7. 公開API**

辺値の公開フィールドと `new`。積み荷を持つ種別は `payload()`、無向の種別は
`endpoints()` も持つ。完成済みGraph上の個体は別の型 (`PurchaseRef<'graph>`) で
ある。

対称性は次のとおりである。

| 要素 | 構築時の値 | 公開ID | 完成済みGraphの参照 | 非公開の格納形式 |
|---|---|---|---|---|
| ノード | 利用者が宣言した `Person` | `PersonId` | `PersonRef<'graph>` | `__PersonInternalPosition` |
| 辺 | 生成された `Purchase` | `PurchaseId` | `PurchaseRef<'graph>` | `__PurchaseRecord` (端点は内部位置) |

**8. 計算量**

辺値の構築とフィールド読み出しはO(1)。`String` を含む公開IDを複製する場合だけ、
その複製の費用がかかる。

## 10. `NodeRef`

**1. Graphite構文**

なし。`node` 宣言の帰結である。

**2. 利用者定義**

ノード値型。

**3. 公開生成物**

`NodeRef` は `&Graph` と内部位置だけを持つ
(`generated/edge_roles_commerce.rs:651-675`)。

```rust
///完成済みグラフ上の `Person` ノード個体。
#[derive(Clone, Copy)]
pub struct PersonRef<'graph> {
    graph: &'graph Graph,
    internal_position: __PersonInternalPosition,
}
impl<'graph> PersonRef<'graph> {
    pub fn id(self) -> &'graph PersonId {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn value(self) -> &'graph super::Person {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
```

`Deref` と `Debug` も生成する (`generated/edge_roles_commerce.rs:787-805`)。

```rust
impl<'graph> std::ops::Deref for PersonRef<'graph> {
    type Target = super::Person;
    fn deref(&self) -> &Self::Target {
        self.graph
            .__graphite_node_person
            .get_at(self.internal_position.0)
            .expect(
                "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
}
impl<'graph> std::fmt::Debug for PersonRef<'graph> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(PersonRef))
            .field("id", &self.id())
            .finish_non_exhaustive()
    }
}
```

`Debug` の契約は次のとおりである。`&Graph` は表示しない。公開IDを表示するのは既定
生成ID型のときだけで、明示ID型のときは型名だけを書く
(`crates/graphite-codegen/src/schema_codegen.rs:1174-1206`)。値型は利用者定義であり
`Debug` を持つとは限らないため、常に表示しない。

`NodeRef` は `&Graph` と `usize` の2語であり、`Copy + Clone` である。ヒープ確保・
自己参照・`Rc`・`RefCell`・実行時リフレクションを使わない。
`crates/graphite/tests/graph_refs.rs:48-59` がこの大きさを固定している。

ノード値型が `id` / `value` という名のメソッドを持つ場合、`NodeRef` の同名の固有
メソッドが優先される。値側のメソッドを呼ぶには `(*node_ref).id()` と明示的に
`Deref` させる。

**4. private生成物**

`__PersonInternalPosition`。フィールド `graph` と `internal_position` も非公開で
あり、schema moduleの外から `NodeRef` を組み立てることはできない。

**5. 構築時の処理**

なし。`NodeRef` は凍結後にしか存在しない。

**6. 完成済みGraphの内部保存**

`NodeRef` 自体はGraphに保存されない。`Graph` の借用と内部位置から、呼び出しごとに
組み立てる値である。

**7. 公開API**

`id()` / `value()` / `Deref<Target = ノード値型>` / `Debug` に加えて、役割クエリと
端点対検索 (§18) が生える。

**8. 計算量**

`id()` / `value()` / `Deref` はいずれも `Vec` の添字参照でO(1)、確保なし。

## 11. `EdgeRef`

**1. Graphite構文**

なし。`edge` 宣言の帰結である。

**2. 利用者定義**

積み荷型。

**3. 公開生成物**

`EdgeRef` も `&Graph` と内部位置だけを持つ
(`generated/edge_roles_commerce.rs:427-482`)。

```rust
/// 完成済みグラフ上の有向辺個体。
#[derive(Clone, Copy)]
pub struct PurchaseRef<'graph> {
    graph: &'graph Graph,
    internal_position: __PurchaseInternalPosition,
}
impl<'graph> PurchaseRef<'graph> {
    fn record(self) -> &'graph __PurchaseRecord {
        self.graph
            .purchase
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .1
    }
    pub fn id(self) -> &'graph PurchaseId {
        self.graph
            .purchase
            .get_at(self.internal_position.0)
            .expect(
                "EdgeRefの内部位置は凍結後に不変の辺表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)",
            )
            .0
    }
    pub fn buyer(self) -> PersonRef<'graph> {
        PersonRef {
            graph: self.graph,
            internal_position: __PersonInternalPosition(self.record().buyer.0),
        }
    }
    pub fn product(self) -> ProductRef<'graph> {
        ProductRef {
            graph: self.graph,
            internal_position: __ProductInternalPosition(self.record().product.0),
        }
    }
    pub fn from(self) -> PersonRef<'graph> {
        self.buyer()
    }
    pub fn to(self) -> ProductRef<'graph> {
        self.product()
    }
    pub fn from_id(self) -> &'graph PersonId {
        self.from().id()
    }
    pub fn to_id(self) -> &'graph ProductId {
        self.to().id()
    }
    pub fn info(self) -> &'graph TransactionInfo {
        &self.record().info
    }
    pub fn payload(self) -> &'graph TransactionInfo {
        &self.record().info
    }
}
```

役割名のgetter (`buyer` / `product` / `info`) が主であり、`from` / `to` /
`from_id` / `to_id` / `payload` は方向固定の別名である。自己型辺でも役割名は
曖昧にならない。無向辺は方向固定の別名を持たず `endpoints()` だけを持つ (§7)。

**4. private生成物**

`__PurchaseInternalPosition` と `__PurchaseRecord`、および非公開メソッド `record()`。

**5. 構築時の処理**

なし。

**6. 完成済みGraphの内部保存**

`EdgeRef` 自体は保存されない。保存されるのは `__PurchaseRecord` である。

**7. 公開API**

上記のとおり。`Debug` は `NodeRef` と同じ契約である。

**8. 計算量**

`id()`・役割getter・`payload()`・`endpoints()` はいずれもO(1)、確保なし。端点の
公開IDを検索しない。端点は辺記録に内部位置で入っているためである。

## 12. `graph!` の名前付き要素と短縮形の脱糖

**1. Graphite構文**

```rust
let g = graphite::graph!(Org {
    alice = Person { name: "Alice".into() },
    bob   = Person { name: "Bob".into() },
    eng   = Team { name: "Engineering".into() },

    a_team = BelongsTo(alice -> eng),
    b_boss = Boss(bob -[BossEdge { since: 2021 }]-> alice),
})?;
```

静的な項はすべて `名前 = 値` の形である (名前を持たないスプライス項は §16)。名前は
構築中に公開IDを束縛するローカル名であり、同時に完成後の静的アクセサ名になる。
辺リテラルは `Kind(from -> to)` /
`Kind(from -[積み荷式]-> to)` / `Kind(a -- b)` / `Kind(a -[積み荷式]- b)` の4形で
ある。

**2. 利用者定義**

ノード値と積み荷の式。`graph!` は値の型を一切解析せず、トークンをそのまま埋め込む。

**3. 公開生成物**

なし。`graph!` はschemaに由来する型を生成しない。§14 の名前付きラッパーだけを
呼び出し箇所へ展開する。

**4. private生成物**

なし (呼び出し箇所ローカルの名前付きラッパーは §14)。

**5. 構築時の処理**

`graph!` は `{Schema}::Graph::create_named` の呼び出しへ脱糖する。項の展開は
「全ノード項 → (全辺項と全スプライス項を記述順)」の2段である。辺はノードのID束縛を
参照するので、`let` が使用より前に来ている必要があるためである。検証は凍結時なので
意味論は変わらない (`crates/graphite-macros/src/instance_codegen.rs:36-51`)。

```text
Org::Graph::create_named(|__graphite_b, __graphite_permit| {
    // (1) 全ノード宣言 (記述順)
    let (alice, alice_position) =
        __graphite_b.insert_named("alice", Person { .. }, __graphite_permit);
    let (eng, eng_position) =
        __graphite_b.insert_named("eng", Team { .. }, __graphite_permit);
    // (2) 全エッジとスプライスを記述順に (`docs/graph_splice.md` §1)
    let (a_team, a_team_position) = __graphite_b.add_named(
        "a_team",
        BelongsTo(alice.clone(), eng.clone()),
        __graphite_permit,
    );
    __graphite_b.extend(staff);
    (alice_position, eng_position, a_team_position)
})
```

実際に生成するトークンは次の形である
(`crates/graphite-macros/src/instance_codegen.rs:178-189`)。

```rust
                let call = match explicit_id {
                    Some(id) => {
                        quote! { __graphite_b.insert_named_with_id(#id, #value, __graphite_permit) }
                    }
                    None => {
                        quote! { __graphite_b.insert_named(#key_str, #value, __graphite_permit) }
                    }
                };
                node_calls.push(quote! {
                    #[allow(unused_variables, non_snake_case)]
                    let (#key_ident, #named_position) = #call;
                });
```

辺項は、柄の向きに対応する辺リテラルトレイトの構築関数を経由する
(`crates/graphite-macros/src/instance_codegen.rs:225-266`)。

```rust
                let literal_trait = match edge.direction {
                    EdgeDirection::Directed => quote! { graphite::DirectedEdgeLiteral<_, _, _> },
                    EdgeDirection::Undirected => quote! { graphite::UndirectedEdgeLiteral<_, _> },
                };
```

```rust
                let ctor = match &edge.attrs {
                    None => {
                        quote! {
                            <#schema_name::#kind as #literal_trait>::from_graph_literal(
                                #from_ident.clone(),
                                #to_ident.clone(),
                                (),
                            )
                        }
                    }
                    Some(attrs_expr) => quote! {
                        <#schema_name::#kind as #literal_trait>::from_graph_literal(
                            #from_ident.clone(),
                            #to_ident.clone(),
                            #attrs_expr,
                        )
                    },
                };
```

宣言した向きと柄の向きが一致しなければ、実装していないトレイトを要求することに
なりコンパイルエラーになる。

**短縮形の正確な脱糖**。`alice = Person { .. }` は `insert_named("alice", ..)` へ
脱糖し、生成ファイルの `insert_named` が束縛名の文字列から既定IDを作る
(`generated/edge_roles_commerce.rs:1088-1098` と `637-645`)。

```rust
    pub fn insert_named<N>(
        &mut self,
        key: impl Into<String>,
        value: N,
        permit: &graphite::NamedInsertPermit,
    ) -> (N::Id, N::NamedPosition)
    where
        N: CommerceNode + CommerceDefaultId,
    {
        value.insert_named_with_binding(self, key.into(), permit)
    }
```

```rust
impl CommerceDefaultId for super::Person {
    fn insert_named_with_binding(
        self,
        b: &mut Builder,
        binding: String,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        CommerceInsertable::insert_named_with_id(self, b, PersonId(binding), permit)
    }
```

したがって短縮形の結果は `alice @ Org::PersonId("alice".into()) = Person { .. }` と
等しい。ただし経路は同一ではない。短縮形は `{Schema}DefaultId` を要求するため、
明示ID型を宣言した種別では短縮形を書けない (§3、§8)。名前と公開ID値は独立して
おり、`alice @ Org::PersonId("external-name".into()) = Person { .. }` と書けば、名前
`alice` と公開ID `"external-name"` は別々の値になる。

`graph!` の左辺名は、ノードと辺を通じて1つの平坦な名前空間である。同じ識別子を
2回宣言するとコンパイルエラーになる。`into_graph` は名前付きラッパーの予約メソッド
名なので左辺名に使えない
(`crates/graphite-macros/src/instance_codegen.rs:116-129`)。

**6. 完成済みGraphの内部保存**

名前付き位置型の値が、名前付きラッパーのフィールドとして凍結境界の外へ運ばれる
(§14)。`Graph` の側には名前の文字列は残らない。

**7. 公開API**

`graph!` の戻り値は `Result<名前付きラッパー, {Schema}::Violation>` である。

**8. 計算量**

`Builder` への追加は償却O(1) (`Vec::push`)。凍結の費用は §20。

## 13. `graph!` の明示ID (`名前 @ ID式 = 値`)

**1. Graphite構文**

```rust
let graph = graphite::graph!(MixedIds {
    left @ ExternalNodeId(10) = ExternalNode { name: "left" },
    right @ ExternalNodeId(20) = ExternalNode { name: "right" },
    external_edge @ ExternalEdgeId(30) = ExternalLink(left -> right),
    boolean @ 1 == 1 = BooleanNode,
});
```

`@` の右側は普通のRust式である (`boolean` の行は `1 == 1` という式をそのまま
`bool` のID値として渡している。`node BooleanNode(id: bool);` と宣言してある)。
`crates/graphite/tests/schema_ids.rs:84-119` が実際に動く形である。

**2. 利用者定義**

ID式。

**3. 公開生成物**

なし。

**4. private生成物**

なし。

**5. 構築時の処理**

`insert_named_with_id(#id, #value, __graphite_permit)` /
`add_named_with_id(#id, #ctor, __graphite_permit)` へ脱糖する
(`crates/graphite-macros/src/instance_codegen.rs:179-181, 255-258`)。文字列を経由
しないため、`{Schema}DefaultId` を要求しない。既定ID型の種別にも明示IDを渡せる。

**6. 完成済みGraphの内部保存**

`KeyedTable` のキーがそのID式の値になる。

**7. 公開API**

`node_ref.id()` が利用者のID型への参照を返す。

**8. 計算量**

ID式の評価費用は利用者の式に依存する。それ以外は §12 と同じ。

## 14. `graph!` の名前付きラッパーと静的アクセサ

**1. Graphite構文**

```rust
let g = graphite::graph!(Org {
    alice = Person { name: "Alice".into() },
    eng   = Team { name: "Engineering".into() },
    a_team = BelongsTo(alice -> eng),
})?;

let alice: Org::PersonRef<'_> = g.alice();
let a_team: Org::BelongsToRef<'_> = g.a_team();
let bare: Org::Graph = g.into_graph();
```

**2. 利用者定義**

左辺名。

**3. 公開生成物**

名前付き位置型は、この機構のために `#[doc(hidden)] pub` で生成ファイルに置かれる
(`generated/edge_roles_commerce.rs:28-39`)。

```rust
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PersonNamedPosition(__PersonInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __ProductNamedPosition(__ProductInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __PurchaseNamedPosition(__PurchaseInternalPosition, u64);
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct __SubscriptionNamedPosition(__SubscriptionInternalPosition, u64);
```

第1要素が内部位置、第2要素が構築印である。

名前付き位置から参照を作る実装も生成ファイルにある
(`generated/edge_roles_commerce.rs:623-636`)。

```rust
impl graphite::NamedGraphElement<Graph> for __PersonNamedPosition {
    type Reference<'graph> = PersonRef<'graph>;
    fn bind<'graph>(&self, graph: &'graph Graph) -> Self::Reference<'graph> {
        if graph.__graphite_construction_stamp != self.1 {
            panic!(
                "名前付き位置が生成元と異なる Graph へ bind されました。名前付き位置は生成元の graph! が返したグラフでのみ有効です"
            );
        }
        PersonRef {
            graph,
            internal_position: self.0,
        }
    }
}
```

**4. private生成物**

名前付きラッパーは呼び出し箇所ローカルの型であり、生成ファイルには置かない。左辺名の
集合が呼び出し箇所ごとに異なり、安定したモジュールのファイルへ事前生成できないためで
ある。マクロが呼び出し箇所のブロックスコープへ展開する
(`crates/graphite-macros/src/instance_codegen.rs:350-373`)。

```rust
    Ok(quote! {{
        #[allow(non_snake_case)]
        struct #wrapper_ident<__GraphiteGraph #(, #wrapper_parameters)*> {
            __graphite_graph: __GraphiteGraph,
            #(#named_positions: #wrapper_parameters,)*
        }

        impl<__GraphiteGraph #(, #wrapper_parameters)*>
            #wrapper_ident<__GraphiteGraph #(, #wrapper_parameters)*>
        {
            pub fn into_graph(self) -> __GraphiteGraph {
                self.__graphite_graph
            }
        }

        impl<__GraphiteGraph #(, #wrapper_parameters)*> std::ops::Deref
            for #wrapper_ident<__GraphiteGraph #(, #wrapper_parameters)*>
        {
            type Target = __GraphiteGraph;

            fn deref(&self) -> &Self::Target {
                &self.__graphite_graph
            }
        }
```

静的アクセサは名前付き位置から参照を直接作る
(`crates/graphite-macros/src/instance_codegen.rs:320-335`)。

```rust
    let accessors = named_keys
        .iter()
        .zip(named_positions.iter())
        .zip(wrapper_parameters.iter())
        .map(|((key, position), parameter)| {
            quote! {
                pub fn #key(
                    &self,
                ) -> <#parameter as graphite::NamedGraphElement<#schema_name::#graph_ident>>::Reference<'_> {
                    <#parameter as graphite::NamedGraphElement<#schema_name::#graph_ident>>::bind(
                        &self.#position,
                        &self.__graphite_graph,
                    )
                }
            }
        });
```

型名・フィールド名・型引数名の生成規則は
`crates/graphite-codegen/src/naming.rs:37-49` にある。ラッパー型名は
`__Graphite{Schema}NamedGraph`、位置フィールド名は `__graphite_named_{左辺名}`、
型引数名は `__GraphiteNamedPosition{通番}` である。展開全体がブロック式で閉じるため、
同じ関数内で `graph!` を複数回書いてもローカル型は衝突しない。

**5. 構築時の処理**

名前付き位置は、その種別の `Builder` 内部の `Vec` へ追加する直前の長さを記録する
(`generated/edge_roles_commerce.rs:603-616`)。

```rust
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        _permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition) {
        let named_position = __PersonNamedPosition(
            __PersonInternalPosition(b.__graphite_node_person.len()),
            b.__graphite_construction_stamp,
        );
        let returned_id = id.clone();
        b.person(id, self);
        (returned_id, named_position)
    }
```

凍結後の `KeyedTable` は同じ順序で詰めるため、この位置は凍結をまたいでそのまま
有効である。凍結が `Err` を返した場合は名前付きラッパー自体を構築しない。

名前付き位置を積む経路には許可証が要る (§15)。

**6. 完成済みGraphの内部保存**

名前付き位置は `Graph` ではなく名前付きラッパーが保持する。`Graph` は構築印だけを
保持する (`generated/edge_roles_commerce.rs:242-245`)。

```rust
    /// この `Graph` を生んだ構築の構築印。凍結元の `Builder` から
    /// そのまま引き継ぐ。名前付き位置がこの `Graph` の生成元と一致
    /// するかを `NamedGraphElement::bind` が照合するのに使う。
    __graphite_construction_stamp: u64,
```

**7. 公開API**

名前付きラッパーは次の3つを持つ。

- 左辺名と同名のメソッド。戻り値はその種別の `NodeRef` または `EdgeRef` である。
- `Deref<Target = {Schema}::Graph>` と `DerefMut`。したがって `Graph` の種別APIを
  そのまま呼べる。
- `into_graph()`。素の `Graph` を取り出す。公開境界で素の `Graph` を返す場合は
  これを明示する。

`crates/graphite/tests/named_graph.rs:43-51` が名前と公開IDの独立を固定している。

```rust
    let graph = graphite::graph!(NamedWorld {
        太郎 @ PersonId("public-person-42".into()) = Person { name: "太郎".into() },
        本 @ ItemId("public-item-7".into()) = Item { name: "本".into() },
        購入 @ PurchaseId("public-purchase-9001".into()) =
            Purchase(太郎 -[PurchaseInfo { amount: 100 }]-> 本),
    })
    .expect("名前付きグラフを構築できるはず");

    assert_eq!(graph.太郎().id(), &PersonId("public-person-42".into()));
```

**8. 計算量**

静的アクセサはO(1)であり、公開IDのハッシュ表検索を行わない。行うのは構築印の比較
1回と参照の組み立てだけである。ヒープを確保しない。§24 の確保契約テストが
`crates/graphite/tests/allocation_contract.rs` でこれを実測している。

## 15. `graph!` の許可証と構築印

**1. Graphite構文**

なし。§12 と §14 の内部機構である。

**2. 利用者定義**

なし。

**3. 公開生成物**

`{Schema}::Graph::create_named` は `#[doc(hidden)] pub` である
(`generated/edge_roles_commerce.rs:408-414`)。

```rust
    #[doc(hidden)]
    pub fn create_named<F, N>(f: F) -> Result<(Self, N), Violation>
    where
        F: for<'b> FnOnce(&'b mut Builder, &'b graphite::NamedInsertPermit) -> N,
    {
        graphite::build_named_graph(Builder::new, f)
    }
```

**4. private生成物**

なし。許可証の型はランタイムクレートにある。

**5. 構築時の処理**

`graphite::NamedInsertPermit` のフィールドは非公開であり、値を作れるのは
`graphite::build_named_graph` だけである
(`crates/graphite/src/lib.rs:191-228`)。

```rust
#[doc(hidden)]
pub struct NamedInsertPermit {
    _private: (),
}
```

```rust
#[doc(hidden)]
pub fn build_named_graph<B, F, N>(
    new_builder: impl FnOnce() -> B,
    f: F,
) -> Result<(B::Graph, N), B::Violation>
where
    B: FreezableBuilder,
    F: for<'b> FnOnce(&'b mut B, &'b NamedInsertPermit) -> N,
{
    let mut builder = new_builder();
    let permit = NamedInsertPermit { _private: () };
    let named_positions = f(&mut builder, &permit);
    builder
        .freeze_into_graph()
        .map(|graph| (graph, named_positions))
}
```

通常の構築経路 `{Schema}::Graph::create` のクロージャは `FnOnce(&mut Builder)` で
あり許可証を受け取らないため、`insert_named` 系へ到達できない。許可証が塞ぐのは
この通常経路からの偶発的な誤用である。

構築印は、名前付き位置を持ち出して別の `Graph` へ束縛する誤用を実行時に検出する
(`crates/graphite/src/lib.rs:149-173`)。`Builder::new()` が1つ発行し、その `Builder`
から生まれる `Graph` と全ての名前付き位置へ同じ値を刻む。`bind` は不一致を
`panic!` にする。これは呼び出し規約の違反であり通常のドメインエラーではないため、
パニックにしている (`docs/design_principles.md` 原則2)。

**6. 完成済みGraphの内部保存**

`Graph::__graphite_construction_stamp: u64`。

**7. 公開API**

利用者が直接呼ぶことは想定しない。`{kind}_try_between` の `GraphMismatch` 判定に
同じ構築印を使う (§18)。

**8. 計算量**

構築印の発行は原子的な加算1回。照合は `u64` の比較1回。確保なし。

## 16. `graph!` のスプライス (`..式`)

**1. Graphite構文**

```rust
let g = graphite::graph!(Org {
    root = Person { name: "CEO".into() },
    ..staff,
    ..reports,
});
```

スプライス (§0.3) は、項の先頭が `..` であることで見分ける。`staff` と `reports` は
実行時に用意したコレクションである。

**2. 利用者定義**

式の型は `IntoIterator<Item = (K, T)>` であり、`K: Into<String>`、`T` は
schema module内の既定IDを使うノード値型または辺値型である。ノードか辺かはマクロでは
なくRustの型推論が決める。

**3. 公開生成物**

なし。

**4. private生成物**

なし。スプライス項は左辺名を持たないため、`let` 束縛も名前付き位置も作らない
(`crates/graphite-macros/src/instance_codegen.rs:274-282`)。

```rust
            GraphItem::Spread(spread) => {
                // 統一 `extend` への脱糖 (`docs/graph_splice.md` §1/§2)。
                // スプライスの要素は名前を持たないため `let` 束縛は作らず、
                // 戻り値のキー列もその場で捨てる (式文として実行するのみ)。
                let expr = &spread.expr;
                rest_calls.push(quote! {
                    __graphite_b.extend(#expr);
                });
            }
```

**5. 構築時の処理**

`__graphite_b.extend(式);` へ脱糖する。戻り値の `Vec<T::Id>` はその場で捨てる。
静的な辺項とスプライス項は記述順のまま実行するため、挿入順の保証に記述順がその
まま現れる。

**6. 完成済みGraphの内部保存**

スプライスで入った要素は、静的な項と全く同じ形で保存される。公開IDは完成済み
`Graph` に保持される。

**7. 公開API**

スプライスの要素は名前を持たないため、静的アクセサを**再公開しない**。取り出すには
公開IDで `{type}_by_id` / `{kind}_by_id` を呼ぶか、`iter` で走査する。明示IDを
一括投入する構文は設けていない。

**8. 計算量**

`extend` は要素数に比例する `Vec::push` の反復である。要素単位の `insert` / `add`
の反復と全く同じ意味論を持つ。

## 17. 種別API (Graphが所有者)

**1. Graphite構文**

なし。`node` / `edge` 宣言の帰結である。

**2. 利用者定義**

なし。

**3. 公開生成物**

種別APIは個体と索引の所有者である完成済み `Graph` のメソッドである。利用者の
struct へ固有 `impl` を書かない (複数のschemaが同じ値型を共有したときに衝突する
ため)。schema moduleにノード名の型も作らない。

| 種別 | メソッド | 戻り型 |
|---|---|---|
| ノード | `{type}_by_id(&id)` | `Option<{Node}Ref<'_>>` |
| ノード | `{type}_value_mut(&id)` | `Option<&mut ノード値型>` |
| ノード | `{type}_ids()` | `impl Iterator<Item = &{Node}Id>` |
| ノード | `{type}_iter()` | `impl Iterator<Item = {Node}Ref<'_>>` |
| ノード | `{type}_len()` | `usize` |
| 辺 | `{kind}_by_id(&id)` | `Option<{Kind}Ref<'_>>` |
| 辺 | `{kind}_payload_mut(&id)` | `Option<&mut 積み荷型>` (積み荷がある種別だけ) |
| 辺 | `{kind}_ids()` | `impl Iterator<Item = &{Kind}Id>` |
| 辺 | `{kind}_iter()` | `impl Iterator<Item = {Kind}Ref<'_>>` |
| 辺 | `{kind}_len()` | `usize` |

`{type}` と `{kind}` は宣言した型名・種別名のsnake_case形である。接尾辞は固定の
英語であり、自然言語の複数形や省略形は生成しない。

`Builder` の入口も `Graph` と対になる関連関数として生成する
(`generated/edge_roles_commerce.rs:391-425`)。

```rust
    pub fn create<F>(f: F) -> Result<Self, Violation>
    where
        F: for<'b> FnOnce(&'b mut Builder),
    {
        let mut builder = Builder::new();
        f(&mut builder);
        builder.freeze()
    }
```

```rust
    pub fn create_collecting<F>(f: F) -> Result<Self, Vec<Violation>>
    where
        F: for<'b> FnOnce(&'b mut Builder),
    {
        let mut builder = Builder::new();
        f(&mut builder);
        builder.freeze_collecting()
    }
```

`create` は最初の1件の違反で `Err(Violation)` になり、`create_collecting` は全違反を
`Err(Vec<Violation>)` で返す。検証ロジックは `freeze_collecting` の1つだけで、
`freeze` はその先頭を取り出す薄い包みである
(`generated/edge_roles_commerce.rs:1453-1457`)。

`Builder` の公開メソッドは次のとおりである
(`generated/edge_roles_commerce.rs:1048-1181`)。

| メソッド | 用途 |
|---|---|
| `{type}(id, value)` / `{kind}(id, value)` | 型名付きの1件挿入 |
| `insert<N>(key: impl Into<String>, value: N) -> N::Id` | ノードの総称挿入 (既定IDのみ) |
| `insert_with_id<N>(id: N::Id, value: N) -> N::Id` | ノードのID指定挿入 |
| `add<E>(key: impl Into<String>, value: E) -> E::Id` | 辺の総称挿入 (既定IDのみ) |
| `add_with_id<E>(id: E::Id, value: E) -> E::Id` | 辺のID指定挿入 |
| `extend<K, T>(items) -> Vec<T::Id>` | ノードと辺に共通の一括挿入 (既定IDのみ) |

`insert` / `add` / `extend` の振り分けは値の型のトレイト実装で決まる。トレイトは
schemaごとに名前が異なる (`{Schema}Insertable` / `{Schema}DefaultId` /
`{Schema}Node` / `{Schema}Edge`) ため、ランタイムクレートではなく生成ファイルに
置く (`generated/edge_roles_commerce.rs:571-583`)。

```rust
pub trait CommerceInsertable: Sized {
    type Id;
    #[doc(hidden)]
    type NamedPosition;
    #[doc(hidden)]
    fn insert_named_with_id(
        self,
        b: &mut Builder,
        id: Self::Id,
        permit: &graphite::NamedInsertPermit,
    ) -> (Self::Id, Self::NamedPosition);
    fn insert_with_id(self, b: &mut Builder, id: Self::Id) -> Self::Id;
}
```

**4. private生成物**

`Builder` の種別ごとの `Vec` フィールドと構築印
(`generated/edge_roles_commerce.rs:548-557`)。`Builder::new` も非公開であり、
`create` / `create_collecting` / `create_named` を経由しないと作れない。

**5. 構築時の処理**

`Builder` は検査を一切行わず、種別ごとの `Vec` へ末尾追加するだけである。

**6. 完成済みGraphの内部保存**

§2・§4・§5・§6・§7 に示した表と索引。

**7. 公開API**

上表のとおり。

**8. 計算量**

§24 の表にまとめる。

## 18. `NodeRef` からの探索

**1. Graphite構文**

```rust
edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where each buyer: 1..2, each product: 0..1, unique pair;
```

**2. 利用者定義**

なし。

**3. 公開生成物**

探索メソッド名は辺種別名と役割名から機械的に導出する。自然言語の複数形や省略語を
推測しない (`crates/graphite-codegen/src/naming.rs:107-121`)。

| 元の宣言 | 生成するメソッド |
|---|---|
| `Purchase` の役割 `buyer` | `PersonRef::purchase_as_buyer` |
| `Purchase` の役割 `product` | `ProductRef::purchase_as_product` |
| 無向の `Friends` | `PersonRef::friends_incident` |
| `Purchase` の端点対 | `PersonRef::purchase_between` / `PersonRef::purchase_try_between` |
| 日本語の種別 `関係` と役割 `始点` | `PersonRef::関係_as_始点` (`generated/traversal_api_traversal.rs:1061`) |

`{kind}_between` / `{kind}_try_between` の主語は位置0側 (有向辺は始点側、無向辺は
唯一の端点型) の `NodeRef` である。

これらは手続き型マクロの展開の中に隠れておらず、生成ファイルに実在する
(`generated/edge_roles_commerce.rs:676-689`)。

```rust
    /// この役割に接続する辺を O(1) で参照し、挿入順に走査する。
    /// 問い合わせ時に結果 `Vec` を確保しない。
    pub fn purchase_as_buyer(
        self,
    ) -> impl Iterator<Item = PurchaseRef<'graph>> + 'graph {
        let positions = self.graph.purchase_from_index.get(self.internal_position.0);
        positions
            .iter()
            .copied()
            .map(move |internal_position| PurchaseRef {
                graph: self.graph,
                internal_position,
            })
    }
```

終点側は `each product: 0..1` により `Option` を返す
(`generated/edge_roles_commerce.rs:882-892`)。

```rust
    /// この役割に接続する高々1本の辺を O(1)、追加確保なしで返す。
    pub fn purchase_as_product(self) -> Option<PurchaseRef<'graph>> {
        self.graph
            .purchase_to_index
            .get(self.internal_position.0)
            .copied()
            .map(|internal_position| PurchaseRef {
                graph: self.graph,
                internal_position,
            })
    }
```

端点対検索は非パニック版が本体で、パニック版がそれを包む
(`generated/edge_roles_commerce.rs:690-727`)。

```rust
    ///順序付き端点対を平均 O(1)、追加確保なしで検索する。
    pub fn purchase_try_between(
        self,
        other: ProductRef<'graph>,
    ) -> Result<Option<PurchaseRef<'graph>>, graphite::GraphMismatch> {
        if self.graph.__graphite_construction_stamp
            != other.graph.__graphite_construction_stamp
        {
            return Err(graphite::GraphMismatch);
        }
        let found = self
            .graph
            .__graphite_purchase_by_pair
            .get(&(self.internal_position, other.internal_position))
            .copied();
        Ok(
            found
                .map(|internal_position| PurchaseRef {
                    graph: self.graph,
                    internal_position,
                }),
        )
    }
    /// # Panics
    /// 2つの参照が異なる `Graph` から得られた場合にパニックする。
    ///パニックを避けたい場合は対の [`Self::purchase_try_between`] を使う。
    pub fn purchase_between(
        self,
        other: ProductRef<'graph>,
    ) -> Option<PurchaseRef<'graph>> {
        self.purchase_try_between(other)
            .unwrap_or_else(|error| {
                panic!(
                    "{}::{}: {error}", stringify!(PersonRef),
                    stringify!(purchase_between)
                )
            })
    }
```

`graphite::GraphMismatch` はランタイムクレートの公開型である
(`crates/graphite/src/lib.rs:29-39`)。

**4. private生成物**

役割索引・端点対索引と、それらのフィールド。

**5. 構築時の処理**

なし。索引は凍結時に作る (§20)。

**6. 完成済みGraphの内部保存**

§5・§6 の索引。

**7. 公開API**

戻り値は常に相手ノードではなく `EdgeRef` である。相手端点は `EdgeRef` の役割getter
から、積み荷は `payload()` から辿る。

順序の保証は仕様である。`KeyedTable` の `ids` / `iter` が挿入順を保つため、
イテレータを返す全ての探索 (役割クエリ・`incident`・`between`・`{kind}_iter`) は
格納順すなわち構築時の追加順を保つ。同じ役割の平行辺が複数ある場合も、リテラルや
`Builder` での記述順どおりに返る。

**8. 計算量**

§24 の表にまとめる。

## 19. 値可変API

**1. Graphite構文**

なし。

**2. 利用者定義**

なし。

**3. 公開生成物**

`generated/edge_roles_commerce.rs:261-264, 330-336` から署名のみ抜粋する。

```rust
pub fn person_value_mut(&mut self, id: &PersonId) -> Option<&mut super::Person>;
pub fn purchase_payload_mut(&mut self, id: &PurchaseId) -> Option<&mut TransactionInfo>;
```

**4. private生成物**

`graphite::KeyedTable::get_mut` は `#[doc(hidden)] pub` である
(`crates/graphite/src/keyed_table.rs:94-99`)。

**5. 構築時の処理**

なし。

**6. 完成済みGraphの内部保存**

なし。可変借用は既存の格納先を指すだけである。

**7. 公開API**

可変APIの主語は `&mut Graph` だけである。引数は公開IDのままとする。
`NodeRef` / `EdgeRef` は共有借用のハンドルなので、そこから可変借用は作れない。
可変借用中は参照を生かせないため、内部位置を引数のキーにもできない。

**8. 計算量**

平均O(1) (`HashMap` 検索 + `Vec` の添字)。確保なし。

## 20. 凍結の完成処理

**1. Graphite構文**

なし。`Graph::create` / `create_collecting` / `create_named` の内部処理である。

**2. 利用者定義**

なし。

**3. 公開生成物**

`Violation` enum。ノードのキー重複、辺のキー重複、未知の端点、`each` 違反、
`unique pair` 違反の5種類のvariantを持つ
(`generated/edge_roles_commerce.rs:106-123`。`Subscription` 側の同型のvariantは省く)。

```rust
#[allow(clippy::enum_variant_names)]
#[derive(Clone, PartialEq, Eq)]
pub enum Violation {
    DuplicatePerson(PersonId),
    DuplicateProduct(ProductId),
    /// このエッジ種別のキーが重複している。
    PurchaseDuplicateKey(PurchaseId),
    /// このエッジが未知の始点キーを参照している。
    PurchaseUnknownSource { edge: PurchaseId, source: PersonId },
    /// このエッジが未知の終点キーを参照している。
    PurchaseUnknownTarget { edge: PurchaseId, target: ProductId },
    /// このエッジ種別の `each` 制約違反 (出次数)。
    PurchaseBuyerEachViolation { source: PersonId, count: usize },
    /// このエッジ種別の `each` 制約違反 (入次数)。
    PurchaseProductEachViolation { target: ProductId, count: usize },
    /// このエッジ種別の `unique pair` 違反 (同じ始点・終点の対に
    /// 2本目の辺が張られた)。
    PurchaseUniquePairViolation { source: PersonId, target: ProductId },
```

`Violation` は `Display` と `std::error::Error` を実装し、`Debug` は `Display` へ
委譲する (`generated/edge_roles_commerce.rs:209-214`)。

**4. private生成物**

`Builder::freeze_collecting` と `Builder::freeze` はどちらも非公開である
(`generated/edge_roles_commerce.rs:1186, 1455`)。凍結を外から呼ぶ入口は
`Graph::create` 系だけである。`graphite::build_named_graph` から具体型を知らずに
凍結を呼ぶための橋渡しだけが公開されている
(`generated/edge_roles_commerce.rs:1462-1468`)。

```rust
impl graphite::FreezableBuilder for Builder {
    type Graph = Graph;
    type Violation = Violation;
    fn freeze_into_graph(self) -> Result<Self::Graph, Self::Violation> {
        self.freeze()
    }
}
```

**5. 構築時の処理**

凍結の手順は次のとおりである。生成コードは
`generated/edge_roles_commerce.rs:1186-1452` にある。

1. ノード種別ごとに、`Builder` の `Vec` を `KeyedTable` へ順に挿入する。既に同じ公開IDが
   あれば挿入せず `Duplicate{Node}` を記録する。**この時点でノードの公開IDから内部
   位置への対応が確定する。**
2. 辺種別ごとに、次の2つをこの順で行う。宣言した辺種別の順に、1種別ずつ完了させて
   から次の種別へ進む。
   1. `Builder` の `Vec` を先頭から1本ずつ処理する。
      1. 辺の公開IDが既出なら `{Kind}DuplicateKey` を記録し、その辺を捨てて次へ進む。
      2. 辺値を分解し、端点の公開IDをノード表の位置で解決する。
      3. 解決できない端点があれば `{Kind}UnknownSource` / `{Kind}UnknownTarget`
         (無向は `{Kind}UnknownEndpoint`) を記録する。
      4. 両端が解決できたときだけ、以下を行う。`unique pair` があれば端点対索引に
         既に対があるかを検査し、あれば `{Kind}UniquePairViolation` を記録する。
         端点対索引へ登録する。役割ごとの一時索引へ辺の内部位置を追加する。辺表へ
         `__{Kind}Record` (端点は内部位置、積み荷はそのまま) を挿入する。
   2. その辺種別の `each` 制約を、ノード表の全位置を走査して検査する。件数が範囲外
      なら `{Kind}{役割名}EachViolation` を記録する。
3. 違反が1件でもあれば `Err` を返す。確定形の索引は作らない。
4. 役割ごとの一時索引を、ノードの内部位置順に詰めた確定形へ変換する
   (`MultipleRoleIndex` / `OptionalRoleIndex` / `ExactlyOneRoleIndex`)。
5. `Graph` を組み立てる。構築印は `Builder` からそのまま引き継ぐ。

辺の内部位置は「辺表へ挿入する直前の長さ」であり、未知端点で捨てた辺の分は詰まる
(`generated/edge_roles_commerce.rs:1248-1250`)。

```rust
                let internal_edge_position = __PurchaseInternalPosition(
                    __graphite_purchase.len(),
                );
```

確定形への変換は次の形である (`generated/edge_roles_commerce.rs:1403-1420`)。

```rust
        let purchase_from_index = graphite::MultipleRoleIndex::from_buckets(
            (0..__graphite_node_person.len())
                .map(|position| {
                    purchase_from_index
                        .remove(&__PersonInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
        let purchase_to_index = graphite::OptionalRoleIndex::from_buckets(
            (0..__graphite_node_product.len())
                .map(|position| {
                    purchase_to_index
                        .remove(&__ProductInternalPosition(position))
                        .unwrap_or_default()
                })
                .collect(),
        );
```

生成コードには、IDE支援のためだけのゼロコストな型検査文も混ざる
(`generated/edge_roles_commerce.rs:1273-1275`)。`where each <役割名>` の役割名
トークンを辺値型のフィールドへ結び付けるためのものであり、実行時の意味はない。

```rust
        let _: fn(&Purchase) = |edge| {
            let _ = &edge.buyer;
        };
```

**6. 完成済みGraphの内部保存**

`Graph` の全フィールド。

**7. 公開API**

`Graph::create` / `create_collecting`。`graph!` は `create_named` を使う。

**8. 計算量**

ノードの総数を V、辺の総数を E とすると、時間もメモリも O(V + E) である。表と索引を
1度ずつ組み立てる。完成済み `Graph` を読む操作にヒープ確保は無く、確保が起きるのは
`Builder` への追加 (`Vec` の伸長) とこの凍結だけである。

## 21. 構造不変性と値可変性の分離

**1. Graphite構文**

なし。

**2. 利用者定義**

なし。

**3. 公開生成物**

`Graph` のdocコメントがこの契約を書いている
(`generated/edge_roles_commerce.rs:215-217`)。

```rust
/// 凍結済み図式グラフ。構築後の構造は不変で、ノード値と辺の積み荷だけを
/// `&mut Graph` を要求する種別APIから更新できる。
pub struct Graph {
```

**4. private生成物**

なし。

**5. 構築時の処理**

なし。

**6. 完成済みGraphの内部保存**

なし。

**7. 公開API**

固定されるものと変更できるものは次のとおりである。

| 対象 | 完成後 | 理由 |
|---|---|---|
| ノード・辺の追加と削除 | 不可 | 追加も削除もするAPIを生成しない |
| 端点 | 不可 | 辺値の端点IDは構築の入力であり、辺記録は内部位置しか持たない |
| 公開IDと内部位置の対応 | 不可 | `KeyedTable` に挿入するAPIを公開しない |
| 索引 | 不可 | 索引を変更するAPIを公開しない |
| ノード値 | 可 | `{type}_value_mut(&mut self, &id)` |
| 辺の積み荷 | 可 | `{kind}_payload_mut(&mut self, &id)` |

構造の不変性は、値の更新可能性まで自動的に禁止しない。この2つは別の性質である。

**8. 計算量**

値の更新は平均O(1)。

## 22. 公開生成物とprivate生成物の境界

境界はRustの可視性で強制する。公開層と非公開層を別々の生成経路へ分けない。生成
ファイルが唯一の実装である。

### 22.1 生成ファイル (schema module) の公開物

| 生成物 | 可視性 | 備考 |
|---|---|---|
| `Graph` | `pub` | フィールドは全て非公開 |
| `Builder` | `pub` | フィールドは全て非公開。`new` も非公開 |
| `{Node}Id` / `{Kind}Id` | `pub` | 明示ID宣言では生成しない |
| `{Node}Ref<'graph>` / `{Kind}Ref<'graph>` | `pub` | フィールドは非公開 |
| 辺値型 `{Kind}` | `pub` | 有向は端点と積み荷が公開フィールド。無向は端点が非公開で `endpoints()` から読み、積み荷だけが公開フィールド |
| `Violation` | `pub` | 全variantが公開 |
| `Graph` の種別API | `pub` | §17 |
| `Builder` の挿入API | `pub` | §17 |
| `{Node}Ref` の探索API | `pub` | §18 |
| `{Schema}Insertable` / `{Schema}DefaultId` / `{Schema}Node` / `{Schema}Edge` | `pub` | `insert` / `add` / `extend` の境界に現れるため名前で必要 |
| 公開trait実装 (`Debug`・`Display`・`Deref`・`Error`) | `pub` | |

### 22.2 `#[doc(hidden)]` を付けて公開する例外

利用者が直接触ることは想定しないが、可視性としては到達できるものがある。マクロが
展開したコードから呼ぶ必要があるためである。指紋の定数だけは `pub(super)` であり、
残りは `pub` である。

| 生成物 | 場所 | 必要な理由 |
|---|---|---|
| `__{Type}NamedPosition` | 生成ファイル | 名前付きラッパーがフィールドの型として要求する |
| `Graph::create_named` | 生成ファイル | `graph!` の展開が呼ぶ |
| `Builder::insert_named` / `add_named` / `insert_named_with_id` / `add_named_with_id` | 生成ファイル | 同上 |
| `{Schema}Insertable::NamedPosition` / `insert_named_with_id` | 生成ファイル | 同上 |
| `{Schema}DefaultId::insert_named_with_binding` | 生成ファイル | 同上 |
| `__GRAPHITE_SCHEMA_FINGERPRINT` | 生成ファイル | `graph_schema!` の指紋照合が読む。`pub(super)` |
| `graphite::DirectedEdgeLiteral` / `UndirectedEdgeLiteral` | ランタイムクレート | 辺リテラルの脱糖先。柄の向きの静的照合を担う |
| `graphite::NamedGraphElement` | ランタイムクレート | 静的アクセサの脱糖先 |
| `graphite::NamedInsertPermit` / `build_named_graph` / `FreezableBuilder` | ランタイムクレート | 許可証付きの構築経路 |
| `graphite::次の構築印を発行する` | ランタイムクレート | `Builder::new` が呼ぶ |
| `graphite::MultipleRoleIndex` / `OptionalRoleIndex` / `ExactlyOneRoleIndex` | ランタイムクレート | `Graph` のフィールド型 |
| `graphite::KeyedTable::position` / `get_at` / `get_mut` / `positions` | ランタイムクレート | 生成コードが内部位置を扱うため |

### 22.3 生成ファイルの非公開物

| 生成物 | 可視性 |
|---|---|
| `__{Type}InternalPosition` | 非公開 |
| `__{Kind}Record` | 非公開 |
| `Graph` のノード表・辺表フィールド | 非公開 |
| `Graph` の役割索引フィールド | 非公開 |
| `Graph` の端点対索引フィールド | 非公開 |
| `Graph` の構築印フィールド | 非公開 |
| `Builder` の全フィールドと `Builder::new` | 非公開 |
| `Builder::freeze` / `freeze_collecting` | 非公開 |
| `{Kind}Ref::record` | 非公開 |

`private_interfaces` の許可が必要なのは、Graphite内部の型ではなく、利用者が
非公開で宣言した値型 (辺の積み荷型など) が生成コードの公開APIに現れうるためで
ある。schemaは値型の可視性を検査しない。

## 23. 自動生成物の一覧

### 23.1 利用者が定義するもの

- ノード値型
- 辺の積み荷型
- 明示指定するノードID型・辺ID型
- schema名と同名のmodule、およびその中の `include!`

### 23.2 Graphiteが公開生成するもの (通常のRustソースとして追跡可能)

- `Graph`
- `Builder`
- 既定ノードID型・既定辺ID型
- `NodeRef` / `EdgeRef`
- 辺値型 (端点と積み荷を持つ普通のRustの値)
- `Violation` (型付きの検証エラー)
- `Graph` の種別API (`by_id` / `value_mut` / `payload_mut` / `ids` / `iter` / `len`)
- `Builder` の挿入API (`{type}` / `{kind}` / `insert` / `insert_with_id` / `add` /
  `add_with_id` / `extend`)
- `NodeRef` の探索API (`{kind}_as_{役割名}` / `{kind}_incident` / `{kind}_between` /
  `{kind}_try_between`)
- 挿入トレイト (`{Schema}Insertable` / `{Schema}DefaultId` / `{Schema}Node` /
  `{Schema}Edge`)
- 公開trait実装 (`Debug` / `Display` / `Deref` / `Error` / 辺リテラルトレイト /
  `NamedGraphElement` / `FreezableBuilder`)

### 23.3 Graphiteがprivate生成するもの

- ノード・辺の内部位置型
- 凍結後の辺記録
- 公開ID索引 (`KeyedTable` の `HashMap` 部分)
- 役割索引
- 端点対索引
- 構築印
- 非公開の補助処理 (`record` / `freeze` / `freeze_collecting`)

### 23.4 `graph!` が呼び出し箇所へ展開するもの

- 名前付きラッパー型 (ローカル型)
- 左辺名と同名の静的アクセサ
- `into_graph` / `Deref` / `DerefMut`
- `create_named` の呼び出しと、項ごとの `let` 束縛列

## 24. 計算量と確保契約

### 24.1 一覧

| 操作 | 計算量 | ヒープ確保 | 実装の根拠 |
|---|---|---|---|
| `{node_ref}.id()` / `value()` / `Deref` | O(1) | なし | `KeyedTable::get_at` の添字参照 |
| `{edge_ref}.id()` / 役割getter / `payload()` / `from()` / `to()` / `endpoints()` | O(1) | なし | 辺記録が端点を内部位置で保持 |
| `graph.{type}_by_id(&id)` / `{kind}_by_id(&id)` | 平均O(1) | なし | `KeyedTable` の `HashMap` 検索 |
| `wrapper.{左辺名}()` | O(1) (ハッシュ検索なし) | なし | 名前付き位置が内部位置を直接保持 |
| `{kind}_as_{役割名}` (`each: 1`) | O(1) | なし | `ExactlyOneRoleIndex` の添字参照 |
| `{kind}_as_{役割名}` (`each: 0..1`) | O(1) | なし | `OptionalRoleIndex` の添字参照 |
| `{kind}_as_{役割名}` (その他) の開始 | O(1) | なし | `MultipleRoleIndex` の範囲参照 |
| 同 走査 | O(件数) | なし | 借用したスライスのイテレータ |
| `{kind}_incident()` (無向) | 開始O(1)・走査O(次数) | なし | `MultipleRoleIndex` |
| `{kind}_between` (`unique pair` あり) | 平均O(1) | なし | 端点対索引の `HashMap` 検索 |
| `{kind}_between` (`unique pair` なし) | 平均O(1) + 走査O(一致件数) | なし | 同上。値は借用したスライス |
| `{kind}_try_between` | `between` と同じ | なし | 構築印の比較を1回足すだけ |
| `graph.{type}_iter()` / `{kind}_iter()` / `{type}_ids()` / `{kind}_ids()` | 開始O(1)・走査O(要素数) | なし | `KeyedTable` の挿入順配列 |
| `graph.{type}_len()` / `{kind}_len()` | O(1) | なし | `Vec::len` |
| `graph.{type}_value_mut()` / `{kind}_payload_mut()` | 平均O(1) | なし | `HashMap` 検索 + 添字 |
| `Builder::{type}()` / `{kind}()` / `insert` / `add` | 償却O(1) | `Vec` の伸長時のみ | `Vec::push` |
| `Builder::extend(items)` | O(要素数) | `Vec` の伸長時のみ | `Vec::push` の反復 |
| 凍結 (`Graph::create` の内部) | O(V + E) (Vはノード総数、Eは辺総数) | あり (表と索引を1度ずつ) | §20 |

`NodeRef` と `EdgeRef` はどちらも `&Graph` と `usize` の2語であり `Copy` である。
ヒープ確保・自己参照・`Rc`・`RefCell`・実行時リフレクションを使わない。

### 24.2 順序の保証

`KeyedTable` の `ids` / `iter` は挿入順を保つ (`insert` を呼んだ順)。これは実装の
副産物ではなく言語の約束である。したがってイテレータを返す全てのAPIは格納順を保つ。
同じ役割の平行辺が複数ある場合も、リテラルや `Builder` での記述順どおりに返る。

### 24.3 確保契約の機械検証

`crates/graphite/tests/allocation_contract.rs` が、確保回数を数えるグローバル
割り当て器で上表の「なし」を実測して固定している。統合テストは1ファイルが1つの
crate rootなので、`#[global_allocator]` の差し替えはこのテストバイナリの中だけに
閉じ、他のテストへ影響しない。数える先はスレッドローカルであり、`cargo test` の
並行実行で他スレッドの確保が測定区間へ混入しない。

固定している区間は次の6つである。

| テスト | 対象 |
|---|---|
| `計測器は実際の確保を検出できる` | 計測器そのものが確保を検出できること (確保0回の測定に意味を与える前提) |
| `参照の生成はヒープを確保しない` | `{type}_by_id` / `{kind}_by_id` と、`NodeRef` / `EdgeRef` の読み出し |
| `静的アクセサはヒープを確保しない` | 名前付きラッパーの静的アクセサ |
| `役割クエリの開始と走査はヒープを確保しない` | 3種の役割クエリと無向の `incident` |
| `端点対検索はヒープを確保しない` | `between` / `try_between` の有向・無向 |
| `種別apiの走査はヒープを確保しない` | `iter` / `ids` / `len` |

時間を計るベンチマーク (criterion など) は置かない。計測ぶれで偽陽性を出し、
検証の入口を信頼できなくするためである。計算量は、上表に書いた実装構造 (添字参照か
ハッシュ検索か、走査対象は何件か) と、この確保契約テストの2つで担保する。

## 25. 3つのアクセス経路

完成済みGraphから個体へ到達する経路は3つあり、費用と前提が異なる。混同しては
ならない。

| 経路 | 書き方 | 前提 | 費用 |
|---|---|---|---|
| 静的アクセス | `wrapper.alice()` | `graph!` の同じ呼び出し箇所で左辺名が分かる | O(1)。公開IDのハッシュ表検索を行わない |
| IDアクセス | `graph.person_by_id(&id)` | 公開ID値を持っている | 平均O(1)のハッシュ表検索 |
| 内部位置 | (公開APIなし) | schema module内部だけ | O(1)の添字参照 |

- 静的アクセスは**IDアクセスの糖衣ではない**。名前付き位置が保持する内部位置を
  そのまま使い、公開IDを一切参照しない。名前と公開ID値は独立しているため、
  `alice @ PersonId("public-person-42")` のように別々の値にできる。
- 内部位置は公開APIに現れない。`NodeRef` / `EdgeRef` のフィールドとして運ばれ、
  役割探索と端点走査がこれを添字として使う。利用者が内部位置を作ることも読むことも
  できない。
- スプライスで入った要素は左辺名を持たないため、静的アクセスの経路が無い。
  IDアクセスか走査を使う。

## 26. 生成コードの配置と追跡性

規約の正本は `docs/code_generation.md` である。ここでは脱糖の観点で要点を再掲する。

### 26.1 配線

宣言に `generated = "generated/<名前>.rs";` を書き、同じRustファイルへ生成moduleの
`include!` を置く。`include!` の相対パスは宣言元ファイルの位置を基準に解決する。
これは `mod foo;` のファイル探索とは基準が異なり、moduleの入れ子に影響されない。

### 26.2 生成先

- 通常のクレートと例の `src/*.rs` にある宣言は、宣言元と同じ `src/generated/` へ
  生成する。
- `crates/graphite/tests/*.rs` にある統合テストの宣言は
  `crates/graphite/tests/generated/` へ生成する。統合テストは1ファイルが1つの
  crate rootであり `src/` を持たないためである。
- 生成ファイルはgitで管理する。

### 26.3 生成コマンド

作業ディレクトリをリポジトリルートにして実行する。

```powershell
cargo xtask generate
cargo xtask generate --check
```

`generate` は全宣言を読んで生成ファイルを更新する。`--check` は書き換えずに、
生成本文全体をバイト単位で比較し、不足・差分・宣言の無い孤児ファイルをエラーに
する。

### 26.4 陳腐化の検出

2段構えである。

1. `graph_schema!` が指紋をconst評価で照合する。schemaの意味を変えて生成し忘れた
   場合、通常の `cargo build` がコンパイルエラーになるため、古い公開APIがエラーの
   出ないまま残ることはない。
2. `cargo xtask generate --check` が生成本文をバイト比較する。schemaの位置移動、
   生成器の変更、先頭コメントに記録する元DSL位置の変化も検出する。

### 26.5 IDE導線

利用コードから公開APIへ定義ジャンプすると、生成ファイルの実装行へ着地する。
生成ファイル先頭の「生成元」コメントが元DSLのリポジトリ相対パスと行を記録して
いるので、そこから宣言へ戻れる。実測記録は `docs/ide_support_spec.md` §1.15 に
ある。

```text
利用コードの purchase_as_buyer()
    ↓ 定義へ移動
crates/graphite/tests/generated/edge_roles_commerce.rs の実装行
    ↓ ファイル先頭の「生成元」コメント
crates/graphite/tests/edge_roles.rs:27 の schema 宣言
```

`graph!` の名前付きラッパーだけは例外で、呼び出し箇所ローカル型のため生成ファイルへ
事前生成しない。追跡性はスパンの規約で担保する。`graph.alice()` の定義情報は左辺の
`alice` へ結び付ける。schemaに由来する公開APIをこの例外へ含めることはしない。

## 27. 検証方法

この文書が実装と一致していることを、次の6つで検査している。

| 検査 | 手段 | 実行方法 (作業ディレクトリはリポジトリルート) |
|---|---|---|
| 掲載コードと生成物の一致 | 引用ごとの出典パスと行の併記。生成物は `--check` がバイト一致を保証する | `cargo xtask generate --check` |
| 構文が通ること | 統合テスト (`crates/graphite/tests/`) | `cargo test --workspace` |
| 利用側から見て通ること | 独立したワークスペースを持つ例7本 | 例ごとに `cd examples/<名前>` してから `cargo test` |
| 拒否すべき構文が拒否されること | `trybuild` によるコンパイル失敗の確認テスト (`crates/graphite/tests/ui/`)。期待する診断は実際に採取した `.stderr` と突き合わせる | `cargo test --workspace` |
| 確保契約 | 確保回数を数えるグローバル割り当て器による実測 (§24.3) | `cargo test --test allocation_contract` |
| IDE導線 | 誤った引数数で呼び出して `E0061` を起こし、rustcの「note: method defined here」の着地行を確認する | `docs/ide_support_spec.md` §1.15 の記録 |

時間を計るベンチマークは意図的に置いていない。理由は §24.3 に書いた。

## 関連文書

| 文書 | 扱う範囲 |
|---|---|
| `docs/schema_v4.md` | schema構文と生成物の設計決定。用語の正本 (§3.1.1) |
| `docs/edge_endpoints_v4_1.md` | 端点の役割名と無向辺 |
| `docs/node_id_v4_2.md` | 既定IDと明示IDの規則 |
| `docs/reverse_query.md` | 役割クエリと索引 |
| `docs/graph_splice.md` | スプライス構文と `extend` の統一 |
| `docs/bulk_construction.md` | 一括構築API |
| `docs/code_generation.md` | 生成の規約 (正本) |
| `docs/ide_support_spec.md` | IDE支援の仕様と実測 |
| `docs/design_principles.md` | 設計6原則 |
| `docs/modeling_guide.md` | グラフで書くか構造体で書くかの判断基準 |
