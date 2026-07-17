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

graphite::graph_schema! {
    schema Org {
        node Person;
        node Team;
        node Project;

        edge BelongsTo = Person -> Team              where each Person: 1;
        edge Boss      = Person -[BossEdge]-> Person where each Person: 0..1;
        edge DependsOn = Service -> Service          where unique pair;
        edge Assigned  = Person -[Role]-> Project;   // 制約なし (平行辺も自由)
    }
}
```

- `edge Kind = From -> To;` / `edge Kind = From -[PayloadType]-> To;`
  — **Kind は新しい nominal 型として生成される** (透過的別名ではない。
  同じ形の Boss と Mentor は別型。docs にこの旨明記)。
- 旧多重度注釈 `(1)`/`(0..1)`/`(0..*)` は**廃止** (字面ごと消滅)。
- `where` 節 (省略可、カンマ区切りで複数可):
  - `each <FromType>: 1` — 各始点ノードにつきちょうど 1 本 (数学: 全域関数)
  - `each <FromType>: 0..1` — 各始点につき高々 1 本 (部分関数)
  - `unique pair` — 同じ (始点, 終点) の対に 2 本目を張ることを禁止
  - `<FromType>` は始点の型名と一致しなければならない (検証エラー)。
    始点と終点が同型の場合も「each = 始点側の出次数」と読む (終点側の
    入次数制約は将来拡張として保留)
  - 矛盾する組合せ (例: `each X: 0..1` と平行辺は両立するか? →
    each 0..1 の下では同対 2 本は自動的に不可能なので `unique pair` の
    併記は冗長として警告なしで許容 or 拒否 — 実装時に単純な方を選び
    docs に明記)
- `node 型名;` / `node 型名(複数形);` は当面現状維持 (ノード側の扱いは
  Fudaba #1 の後継論点として v4 実装後に再訪。ストレージ名は内部専用に
  なるため複数形指定の意義も薄れる可能性が高い)。

## 2. graph! リテラル

```rust
let promo = BossEdge { since: 2023 };

let g = graphite::graph!(Org {
    alice = Person { name: "Alice".into() },
    bob   = Person { name: "Bob".into() },
    eng   = Team { name: "Engineering".into() },

    a_team = BelongsTo(alice -> eng),
    b_team = BelongsTo(bob -> eng),
    b_boss = Boss(bob -[promo]-> alice),          // 積み荷は任意の式
    lead   = Assigned(alice -[Role { name: "lead".into() }]-> proj),
});
```

- **全行が `名前 = 値`**。ノードの名前はノードキー、辺の名前は辺キーの束縛。
- 辺のコンストラクタはタプル struct の顔 `Kind(from -> to)` /
  `Kind(from -[積み荷式]-> to)`。from/to はリテラル内で束縛済みのキー識別子。
- 旧形 (`-[label]->` 中置形・無名辺) は完全廃止。検出・移行診断なし (既定方針)。

## 3. 生成物とアクセス API (型名前空間)

### 3.1 生成される型

- 辺種別ごと: `pub struct Boss(pub PersonId, pub PersonId, pub BossEdge);`
  (積み荷なしは 2 要素)。**タプル struct として実在し、マクロ外でも
  `Boss(from_id, to_id, payload)` で普通に構築できる** (原則6)。
  読み取りは位置 (.0/.1) を人間に晒さず、固定語彙のメソッドを生成:
  `fn from(&self) -> &PersonId` / `fn to(&self) -> &PersonId` /
  `fn payload(&self) -> &BossEdge` (積み荷ありのみ)。
- 辺キー newtype: `pub struct BossId(pub String);` (ノードの {Node}Id と同じ
  規約。型名原文ママ + Id なので rename もカスケードする)
- 違反 enum: 従来の each 系違反 (旧多重度違反) + `unique pair` 違反 +
  辺キー重複違反。バリアント名は Kind 原文ママの合成 (`BossEachViolation` 等、
  命名は実装時に原則3で調整) — ケース変換が消えるため rename 問題なし

### 3.2 アクセス (すべて型名前空間の関連関数。g.メソッドは廃止)

```rust
// ノード (graph_schema! 生成の {Schema}Node トレイト経由。
// ユーザー struct への固有 impl は行わない — 複数 schema 共有時の衝突回避)
let p: Option<&Person> = Person::get(&g, &alice_id);
Person::ids(&g);  Person::iter(&g);   // (&PersonId, &Person)

// 辺 — 種別型 (マクロ生成) への固有 impl
Boss::of(&g, &bob);                    // 走査: where の制約が戻り型を決める
                                        //   each:1 → (&Person, &BossEdge)
                                        //   each:0..1 → Option<..>
                                        //   制約なし → Vec<..>
Boss::get(&g, &boss_id);               // キーで辺 1 本: Option<&Boss>
Boss::between(&g, &bob, &alice);       // 対で検索: unique pair → Option、他 → Vec
Boss::iter(&g);                        // (&BossId, &Boss)
Boss::ids(&g);  Boss::len(&g);
```

- 旧ビュー API (`g.boss().of(..)`、EdgeOne 等 6 型) は**全廃**。
  ランタイムの共通機構は「キー付き要素表」に対するジェネリクスとして
  再構成する (ノード表と辺表で共有できるはず。実装時に設計)。
- builder: `b.insert(key, node_value)` (v3 の総称 insert を維持) +
  `b.add(key, edge_value)` (辺版の総称。命名は原則3で実装時調整)。

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

- 終点側 (入次数) の each 制約 — 需要仮説のみのため。文法上は
  `where each ...` の拡張で受けられる形を保つ
- ノード宣言へのキーワード統一 (`node Person;` の再検討) — v4 安定後、
  Fudaba #1 後継として
- 「グラフで書くべきもの vs 構造体で書くべきもの」のモデリング指針 —
  Fudaba 別札で議論 (ユーザー発案 2026-07-16)
