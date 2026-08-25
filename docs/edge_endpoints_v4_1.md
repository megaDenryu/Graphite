# 端点宣言 — 役割名必須の有向辺と無向辺

v4.1で導入した端点の役割名と無向辺を、Issue #1で有向辺の役割名必須へ更新した仕様。
旧省略構文は互換として残さない。

発端の設計原理 (ユーザー、原文趣旨):
> 矢印というのはそれがどういう意味を持っているかで向きの解釈も何もかも変わる。
> 言語として記述するときはそこを明確にできる記法になっていないといけない。

現状の欠落: 「from = 部下、to = 上司」という向きの意味がコメントにしか書けない。
対称な関係 (友人・相互接続) は矢印の向きに意味がないのに矢印で書くしかない。

## 1. 有向辺の端点の役割名 (必須)

```rust
edge DependsOn = (dependent: Service) -> (dependency: Service) where unique pair;

edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee) where each subordinate: 0..1;
```

規則:

- 端点の役割名は両端とも必須で、必ず `(役割名: Type)` と括弧で囲む。
- 積み荷がある場合も `[役割名: Type]` の役割名を必須とする。
- 生成する辺値型は役割名を公開フィールドに使う。`Boss { subordinate, superior, appointment }`。
- `where each <役割名>: N | N..M | N..*` は両端点へ独立指定できる。
- graph! リテラルは `Boss(bob -[attrs]-> alice)` のまま、柄に対応する辺リテラルトレイトの構築関数へ脱糖する。
- each違反variant名はKindと役割名から導出する (`BossSubordinateEachViolation`)。

## 2. 無向辺

```rust
edge Friends = Person -- Person where unique pair;      // 積み荷なし
edge Wire = Node -[cable: Cable]- Node;                 // 積み荷あり

// リテラル (graph! 内) も同形
f1 = Friends(alice -- bob),
w1 = Wire(n1 -[Cable { ohm: 5 }]- n2),
```

記法の導出規則: 有向の柄 `-`+`>` から矢尻を落とすと `--`。積み荷は柄の中に
挿入する (有向 `-[X]->` と同じ規則) ので `-[X]-`。

意味論:

- 端点は**順序なし対** {a, b}。`Friends(alice -- bob)` と `Friends(bob -- alice)`
  は同じ辺を意味し、`unique pair` の同値判定・`between(a, b)` は順序を無視する。
- **両端は同じノード型でなければならない** (対称性は型にも及ぶ。異型を繋ぎたい
  対称関係は v1 では対象外 — 有向で書くかノード昇格。検証エラーで案内)。
- **役割名は書けない** (役割の区別がある時点で対称ではない — その場合は
  役割名つき有向 (§1) を使う。構文エラーで案内)。
- 自己ループ (`Friends(a -- a)`) は許可する。
- 無向辺には役割名が無いため `each` は使えない。利用可能な制約は
  順序無視の対へ適用する `unique pair` のみ。
- クエリ (型名前空間、有向と同じ語彙):
  - `Friends::incident(x)` — x に接続する辺参照を挿入順のiteratorで返す
  - `Friends::between(a, b)` — 対称。非パニック版 `try_between(a, b)` も提供する
  - `get`/`iter`/`ids`/`len` は有向と同じ
- 構築用の辺値は、非公開の順序なし対
  `endpoints: graphite::UnorderedPair<PersonId>` を保持する。
  完成済みの辺参照は公開アクセサ
  `endpoints() -> (PersonRef<'graph>, PersonRef<'graph>)` で両端を読む。IDが必要なら各参照の `id()` を使う。
- 格納: 実装の自由 (正規化 or 両方向索引) だが、`iter`/`of` の**挿入順保持**の
  仕様 (schema_v4 §3.2) は無向でも維持すること。

## 3. 実装ノート

- パーサ: 有向端点は `(Ident: Ident)`、無向端点は `Ident`。柄は `->` /
  `-[役割名: Path]->` / `--` / `-[役割名: Path]-` の4形。G4エラー回復とdrain_rest
  規約は従来どおり。
- graph! 側: 辺コンストラクタ内の柄も同 4 形。向きをenumとして保持し、
  対応する辺リテラルトレイトへ脱糖することでスキーマの向きと静的に照合する。
- 始点・終点のeachを独立にfreeze検証する。無向eachは明示的に拒否する。
- IDE 実測 (実装後、オーケストレータが実施): 役割名トークンの定義ジャンプ・
  生成アクセサ名からの着地・`--` リテラルのトークン解決
