# エッジ宣言構文 v3 — ラベルの型としての矢印式

> **[v4 (`docs/schema_v4.md`) で置換済み]** このファイルは歴史的記録として残す。
> 現行のエッジ宣言構文 (`edge Kind = From -> To;` / `where each ...`/`unique pair`
> 制約・辺の第一級キー化) は `docs/schema_v4.md` を参照すること。

2026-07-16 のユーザー決定。`docs/edge_syntax_v2.md` (v2) のエッジ宣言部の改訂。
設計考察はオーケストレータによる (経緯・比較検討は
`docs/dev_history_2026-07-14_session2.md` 参照)。

## 1. 動機 (ユーザー指摘)

v2 の `edge Person -[boss: BossEdge]-> Person (0..1);` は、`boss: BossEdge` が
Rust のフィールド宣言 `name: Type` の顔をしているのに、**boss の型は BossEdge
ではない** (BossEdge は辺 1 本のペイロード型)。構文が読み手に嘘をつく。

> 「boss の型が BossEdge」ではないの部分を何とか解決したい。BossEdge は boss の
> 何なのか？が宣言的に書けるようになってないといけない。

## 2. 新構文 — 矢印式全体をラベルの型として読む

```rust
graphite::graph_schema! {
    schema Org {
        node Person;
        node Team;

        edge belongs_to: Person -> Team (1);
        edge boss:       Person -[BossEdge]-> Person (0..1);
        edge reports:    Person -> Person (0..*);
    }
}
```

- `label:` の右側**全体**がラベルの型 (関係型)。読み方は Rust の関数型
  `f: impl Fn(Person) -> Person` と同じ構図 — 「`名前: A -> B` は写像の型宣言」
  という Rust 既存の読解をそのまま借りる。boss の型は「Person から Person へ、
  BossEdge を運ぶ、高々 1 本の関係」。
- **矢印内 `-[..]->` は積み荷 (ペイロード型) のみ**。属性なしエッジは素の `->`
  (「何も運ばない」が見た目に出る)。決定3 の原理「矢印の中は辺が運ぶもの」に
  v2 より忠実 (ラベルは運ばれるものではないため、矢印内にラベルを置いた v2 の
  方が原理から逸れていた)。
- 多重度は従来どおり末尾 `(1)`/`(0..1)`/`(0..*)` — 関係型の一部
  (配列長 `[T; N]` に相当する制約) としてこの位置に置く。
- 「schema は `:` (型付け)、リテラルは `=` (代入)」の言語規則は不変。
  **graph! リテラルは変更なし** (`bob -[boss = BossEdge { .. }]-> alice` —
  あちらは「boss 表に積み荷 = 値で 1 行入れる」文であり矢印の役割が違う)。
- v2 形 (`edge From -[label]-> To` / `edge From -[label: Type]-> To`) は
  **完全廃止** (既定方針どおり検出・移行診断なし。素のパースエラーに任せる)。

検討済みの対案とその棄却理由 (`boss<BossEdge>` 型引数案 = リテラルとの対称性は
高いが `snake_case<T>` という Rust に実在しない形を発明することになる) は
dev_history 参照。

## 3. 実装上の要点

- schema_dsl.rs: `edge` の後を `label:` → From `Ident` → (`->` | `-[` AttrsPath `]->`)
  → To `Ident` → 多重度 → `;` でパース。G4 エラー回復の宣言境界 (`node`/`edge`
  キーワード) は不変。デリミタ内で Err を返す箇所は drain_rest 規約を厳守。
- スパン規約: ラベル ident (ビューを返す生成メソッド `fn {label}` のアンカー)、
  From/To ident、AttrsPath はいずれもユーザートークンをそのまま使う (現行踏襲)。
- codegen 側はパース結果の構造が同じ (label/from/to/mult/attrs_ty) なので
  原則としてパーサ変更のみで済むはず。

## 4. 移行対象

- crates/graphite/tests/ の全 schema 宣言・trybuild UI テスト (stderr 再確認)
- examples 4 本の schema 宣言
- hello-graph の §2 解説 — 「`label: Type` は辺が運ぶ型」という v2 前提の説明文を
  「`label:` の右は関係型。矢印内が積み荷」という v3 の読みに全面更新。
  §2.5 (脱糖の実像) の格納モデル説明は不変 (内部表現は変わらない)
- README (root) の構文説明
- docs/ide_support_spec.md — 実装後にラベル・型トークンの定義ジャンプを再計測
