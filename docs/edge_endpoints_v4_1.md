# 端点宣言 — role必須の有向edgeと無向edge

v4.1で導入した端点roleと無向edgeを、Issue #1で有向role必須へ更新した仕様。
旧省略構文は互換として残さない。

発端の設計原理 (ユーザー、原文趣旨):
> 矢印というのはそれがどういう意味を持っているかで向きの解釈も何もかも変わる。
> 言語として記述するときはそこを明確にできる記法になっていないといけない。

現状の欠落: 「from = 部下、to = 上司」という向きの意味がコメントにしか書けない。
対称な関係 (友人・相互接続) は矢印の向きに意味がないのに矢印で書くしかない。

## 1. 有向edgeのendpoint role (必須)

```rust
edge DependsOn = (dependent: Service) -> (dependency: Service) where unique pair;

edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee) where each subordinate: 0..1;
```

規則:

- endpoint roleは両端とも必須で、必ず `(role: Type)` と括弧で囲む。
- 積み荷がある場合も `[role: Type]` のroleを必須とする。
- 生成Edge値型はrole名を公開fieldに使う。`Boss { subordinate, superior, appointment }`。
- `where each <role>: N | N..M | N..*` は両endpoint roleへ独立指定できる。
- graph! リテラルは `Boss(bob -[attrs]-> alice)` のまま、`Boss::new(..)` へ脱糖する。
- each違反variant名はKindとroleから導出する (`BossSubordinateEachViolation`)。

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
  は同じ辺を意味し、`unique pair` の同値判定・`between(&g, a, b)` は順序を無視する。
- **両端は同じノード型でなければならない** (対称性は型にも及ぶ。異型を繋ぎたい
  対称関係は v1 では対象外 — 有向で書くかノード昇格。検証エラーで案内)。
- **役割名は書けない** (役割の区別がある時点で対称ではない — その場合は
  役割名つき有向 (§1) を使う。構文エラーで案内)。
- 自己ループ (`Friends(a -- a)`) は許可する。
- 無向edgeにはendpoint roleが無いため `each` は使えない。利用可能な制約は
  順序無視の対へ適用する `unique pair` のみ。
- クエリ (型名前空間、有向と同じ語彙):
  - `Friends::of(&g, &x)` — x に接続する相手側を挿入順の `Vec` で返す
  - `Friends::between(&g, &a, &b)` — 対称
  - `get`/`iter`/`ids`/`len` は有向と同じ
- 生成named-field struct: `pub struct Friends { pub endpoints: (PersonId, PersonId) }`。
- 格納: 実装の自由 (正規化 or 両方向索引) だが、`iter`/`of` の**挿入順保持**の
  仕様 (schema_v4 §3.2) は無向でも維持すること。

## 3. 実装ノート

- パーサ: 有向端点は `(Ident: Ident)`、無向端点は `Ident`。柄は `->` /
  `-[role: Path]->` / `--` / `-[role: Path]-` の4形。G4エラー回復とdrain_rest
  規約は従来どおり。
- graph! 側: 辺コンストラクタ内の柄も同 4 形。脱糖は従来の機構のまま
  (無向は正規化を builder/freeze 側で行い、リテラルの脱糖は素通し)。
- 始点・終点のeachを独立にfreeze検証する。無向eachは明示的に拒否する。
- IDE 実測 (実装後、オーケストレータが実施): 役割名トークンの定義ジャンプ・
  生成アクセサ名からの着地・`--` リテラルのトークン解決
