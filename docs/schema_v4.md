# スキーマ v4 — 辺の第一級化: 全要素キー・where 制約・型名前空間アクセス

2026-07-16 のユーザー決定。v3 (edge_syntax_v3.md / graph_literal_v3.md /
edge_view_api.md) を置き換える大改訂。設計議論の経緯は
dev_history と Fudaba #7 を参照。

## 0. 基盤の宣言

**Graphite の基盤は多重グラフ (辺は独立した要素) である。**
辺種別は nominal 型 (名前で区別される型)、辺 1 本はそのインスタンスであり、
ノードと同様に**キーによる同一性**を持つ。「関係」(対で一意) は基盤ではなく、
種別ごとに宣言される制約 (`where unique pair`) として表現される。

言語規則は 3 つに集約される:

1. **`名前 = 定義`** — 名前が要る定義は schema もリテラルも全部この形
   (リテラルの名前は常に**キーの束縛**)
2. **矢印の中は積み荷だけ** — `-[X]->` の X は schema では積み荷の型、
   リテラルでは積み荷の値
3. **`where` は制約** — 制約があるときだけ書く

## 1. schema 構文

```rust
pub struct Person { pub name: String }
pub struct Team { pub name: String }
pub struct BossEdge { pub since: i32 }
pub struct Role { pub name: String }
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExistingPersonId(pub u64);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExistingRelationId(pub u64);

graphite::graph_schema! {
    schema Org {
        node Person(id: ExistingPersonId);
        node Team;
        node Project;

        edge BelongsTo = (member: Person) -> (team: Team) where each member: 1;
        edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1;
        edge DependsOn = (dependent: Service) -> (dependency: Service) where unique pair;
        edge Assigned(id: ExistingRelationId) = (person: Person) -[role: Role]-> (project: Project);
    }
}
```

`graph_schema!` は Rust module を生成するため、モジュール直下に書く。
参照する型も関数の外に宣言する。関数本体のローカル型は、関数内に生成された
module から参照できない。

- `edge Kind = (始点の役割名: From) -> (終点の役割名: To);` / `edge Kind = (始点の役割名: From) -[積み荷の役割名: PayloadType]-> (終点の役割名: To);`
  — **Kind は新しい nominal 型として生成される** (透過的別名ではない。
  同じ形の Boss と Mentor は別型。docs にこの旨明記)。
- 旧多重度注釈 `(1)`/`(0..1)`/`(0..*)` は**廃止** (字面ごと消滅)。
- `where` 節 (省略可、カンマ区切りで複数可):
  - `each <役割名>: N` — 役割名ごとにちょうどN本
  - `each <役割名>: N..M` / `each <役割名>: N..*` — 役割名ごとの範囲制約
  - `unique pair` — 同じ (始点, 終点) の対に 2 本目を張ることを禁止
  - 始点・終点の役割名に独立指定できる。存在しない役割名は検証エラー。
    無向辺は役割名を持たないため `each` を使えない。
  - 同じ役割名への `each` 重複は拒否する。始点側と終点側への独立した `each`、
    および `each` と `unique pair` の併記は許可する。
- `node 型名;` は schema module 内に `{型名}Id(pub String)` を生成する。`node 型名(id: 型パス);` は既存ID型を使う。エッジも同様に `edge Kind(id: 型パス) = ...;` で既存ID型を選べる。

## 2. graph! リテラル

```rust
let promo = BossEdge { since: 2023 };

let g = graphite::graph!(Org {
    alice @ ExistingPersonId(1) = Person { name: "Alice".into() },
    bob @ ExistingPersonId(42) = Person { name: "Bob".into() },
    eng   = Team { name: "Engineering".into() },

    a_team = BelongsTo(alice -> eng),
    b_team = BelongsTo(bob -> eng),
    b_boss = Boss(bob -[promo]-> alice),
    lead @ ExistingRelationId(8) = Assigned(alice -[Role { name: "lead".into() }]-> proj),
});
```

- 静的項目は `名前 = 値`、または明示IDを渡す `名前 @ ID式 = 値`。名前はノードキーまたは辺キーの束縛になる。
- 辺リテラルは `Kind(from -> to)` / `Kind(from -[積み荷式]-> to)`。
  `graph!` はこれを柄に対応する辺リテラルトレイトの構築関数へ脱糖する。
  スキーマ宣言と柄の向きが一致しなければコンパイルエラーになる。
- 旧形 (`-[label]->` 中置形・無名辺) は完全廃止。検出・移行診断なし (既定方針)。

## 3. 生成物とアクセス API (型名前空間)

### 3.1 生成される型

`schema Org` は `#[allow(non_snake_case)] pub mod Org` を生成する。以下の
生成物は `Org::Graph`、`Org::Builder`、`Org::Violation`、`Org::Person`、
`Org::Boss` のように、この module 内へ配置される。グラフ本体のストレージと
索引フィールドは module 外へ公開しない。

- 辺種別ごと: `pub struct Boss { pub subordinate: PersonId, pub superior: PersonId, pub appointment: BossEdge }`。
  **名前付きフィールドの構造体として実在し、マクロ外でも普通に構築できる** (原則6)。
  端点は `edge.subordinate` / `edge.superior` のように役割名の公開フィールドで読む。
  積み荷ありの辺だけ `fn payload(&self) -> &BossEdge` も生成する。
- ID型を省略したノード・辺: `pub struct PersonId(pub String);` / `pub struct BossId(pub String);`。どちらも schema module 内に生成される。`(id: 型パス)` を指定した宣言は生成型を持たない。
- 違反 enum: each系違反 + `unique pair` 違反 + 辺キー重複違反。
  each違反variantはKindと役割名から導出する (`BossSubordinateEachViolation` 等)。

### 3.2 アクセス (すべて型名前空間の関連関数。g.メソッドは廃止)

```rust
// ノード (schema module 内のノードマーカー。
// ユーザー struct への固有 impl は行わない — 複数 schema 共有時の衝突回避)
let p: Option<&Person> = Org::Person::get(&g, &alice_id);
Org::Person::ids(&g);  Org::Person::iter(&g);   // (&PersonId, &Person)

// 辺 — 種別型 (マクロ生成) への固有 impl
Org::Boss::of(&g, &bob);               // 走査: where の制約が戻り型を決める
                                        //   each:1 → (&Person, &BossEdge)
                                        //   each:0..1 → Option<..>
                                        //   その他の範囲・制約なし → Vec<..>
Org::Boss::get(&g, &boss_id);          // キーで辺 1 本: Option<&Boss>
Org::Boss::between(&g, &bob, &alice);  // 対で検索: unique pair → Option、他 → Vec
Org::Boss::iter(&g);                   // (&BossId, &Boss)
Org::Boss::ids(&g);  Org::Boss::len(&g);
```

- 旧ビュー API (`g.boss().of(..)`、EdgeOne 等 6 型) は**全廃**。
  ランタイムの共通機構は「キー付き要素表」に対するジェネリクスとして
  再構成する (ノード表と辺表で共有できるはず。実装時に設計)。
- builder: `b.insert(key, node_value)` (v3 の総称insertを維持) +
  `b.add(key, edge_value)` (辺版の総称)。
- **順序保証 (仕様):** `KeyedTable` (`crates/graphite/src/keyed_table.rs`)
  の `ids`/`iter` は挿入順 (`insert` を呼んだ順) を保持する。これにより
  制約なし辺の `{Kind}::of`/`iter`/`between` (Vec を返す各所) は格納順
  (構築時の追加順) を保持する — 旧フェーズ5 項目 i で仕様化された
  「正式な順序保証」の言語の約束であり、実装の副産物ではなく仕様として
  扱う (同一始点からの平行辺が複数ある場合でも、`of()` はリテラル/builder
  での記述順どおりに返る)。

### 3.3 検証 (freeze 時)

- 従来: 未知キー参照・キー重複 (ノード) ・each 系 (旧多重度)
- 追加: **辺キー重複** / **unique pair 違反** (同対 2 本目)

## 4. スパン・IDE 規約 (G3 継承)

- Kind・キー識別子・型パスは全てユーザートークンのスパンをそのまま使う
- schema の `Boss` トークンは生成 struct の定義アンカー → VSCode で型色、
  リテラル `Boss(..)` からの定義ジャンプは schema の宣言へ
- 実装後に definition provider で実測 (計測手法は ide_support_spec.md)

## 5. 移行対象 (v3 の痕跡ゼロ)

- crates/graphite: ビュー 6 型の撤去と要素表機構への置換、docs コメント
- crates/graphite-macros: schema_dsl (where 節)・instance_dsl (全行 名前=値)・
  codegen 全面
- 全テスト・trybuild (stderr 再採取)・examples 7 本・README・hello-graph
  (§2/§2.5/§3/§4 を v4 の概念 — 全要素キー・where・型名前空間 — で書き直し、
  エラー引用は実採取)
- docs/edge_syntax_v3.md / graph_literal_v3.md / edge_view_api.md の冒頭に
  「v4 (本ファイル) で置換済み」の注記

## 6. 見送り・保留 (根拠つき)

- 役割名を使った `EdgeRef` アクセサ、役割検索API、名前付き静的アクセサは
  後続Issueで扱う。Issue #1では辺値の公開フィールドと型名前空間APIを生成する。
- ノード宣言へのキーワード統一 (`node Person;` の再検討) — v4 安定後、
  Fudaba #1 後継として
- 「グラフで書くべきもの vs 構造体で書くべきもの」のモデリング指針 —
  Fudaba 別札で議論 (ユーザー発案 2026-07-16)
