# スキーマ v4 — 辺の第一級化: 全要素キー・where 制約・Graph中心の種別API

2026-07-16 のユーザー決定。v3 (edge_syntax_v3.md / graph_literal_v3.md /
edge_view_api.md) を置き換える大改訂。設計議論の経緯は
dev_history と Fudaba #7 を参照。

この文書はschema構文とその生成物の設計決定を扱う。「構文を消去するとどの普通の
Rustコードになるか」を構文ごとに機械的に確認する場合は
`docs/desugaring_reference.md` (正本) を参照する。

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
    generated = "generated/org.rs";
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

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod Org {
    include!("generated/org.rs");
}
```

`graph_schema!` は宣言の検証と生成指紋の照合を行うため、モジュール直下に書く。
通常のRust module本体は`cargo xtask generate`が`generated/org.rs`へ生成し、
`include!`で読み込む。詳細は`docs/code_generation.md`を参照する。
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
})?;

let alice_ref = g.alice(); // PersonRef<'_>: 公開IDのハッシュ表での検索なし
let lead_ref = g.lead();   // AssignedRef<'_>
```

- 静的項目は `名前 = 値`、または明示IDを渡す `名前 @ ID式 = 値`。名前は構築中のノード/辺ID束縛であると同時に、完成後の静的アクセサ名になる。
- `graph!` は呼び出し箇所ごとの名前集合を持つローカルの名前付きラッパーを返す (用語は §3.1.1)。名前付きラッパーは `Deref<Target = Org::Graph>` / `DerefMut` と `into_graph()` を持つ。公開境界で素の `Org::Graph` を返す場合は `graph.into_graph()` を明示する。
- 静的アクセサはbuilderへの追加時に得た種別専用の名前付き位置型からNodeRef/EdgeRefを直接作る。公開ID索引を検索しない。名前と公開ID値は独立している。
- `..式` の要素は公開IDを保持するが左辺名を持たないため、新しい名前付きラッパーへ静的アクセサを再公開しない。
- 辺リテラルは `Kind(from -> to)` / `Kind(from -[積み荷式]-> to)`。
  `graph!` はこれを柄に対応する辺リテラルトレイトの構築関数へ脱糖する。
  スキーマ宣言と柄の向きが一致しなければコンパイルエラーになる。
- 旧形 (`-[label]->` 中置形・無名辺) は完全廃止。検出・移行診断なし (既定方針)。

## 3. 生成物とアクセス API (Graph中心)

### 3.1 生成される型

`schema Org` は`generated/org.rs`に通常のRustコードとしてmodule本文を生成する。以下の
生成物は `Org::Graph`、`Org::Builder`、`Org::Violation`、`Org::PersonRef`、
`Org::Boss` のように、この module 内へ配置される。グラフ本体のストレージと
索引フィールドは module 外へ公開しない。

- 辺種別ごと: 構築用の値 `pub struct Boss { pub subordinate: PersonId, pub superior: PersonId, pub appointment: BossEdge }` と、完成済みグラフを読む `pub struct BossRef<'graph>` を生成する。構築用の値はマクロ外でも普通に構築できる。`BossRef` は `edge.subordinate()` / `edge.superior()` で `PersonRef<'graph>` を返し、積み荷ありの辺だけ `payload()` も持つ。
- ノード種別ごと: `pub struct PersonRef<'graph>` を生成する。`PersonRef` は `&Org::Graph` と非公開位置を保持し、`id()`、`value()`、`Deref<Target = Person>` を提供する。
- ID型を省略したノード・辺: `pub struct PersonId(pub String);` / `pub struct BossId(pub String);`。どちらも schema module 内に生成される。`(id: 型パス)` を指定した宣言は生成型を持たない。
- 違反 enum: each系違反 + `unique pair` 違反 + 辺キー重複違反。
  each違反variantはKindと役割名から導出する (`BossSubordinateEachViolation` 等)。

生成物は次の対称な役割へ分かれる。

| 要素 | 構築時の値 | 公開ID | 完成済みグラフの参照 | 非公開の格納形式 |
|---|---|---|---|---|
| ノード | 利用者が宣言した `Person` | `PersonId` | `PersonRef<'graph>` | ノード種別専用の内部位置 |
| 辺 | 生成された `Boss`。端点IDと積み荷を持つ | `BossId` | `BossRef<'graph>` | 端点位置と積み荷を持つ完成済み記録 |

以下、ノード種別ごとの `{Node}Ref<'graph>` を NodeRef、辺種別ごとの
`{Kind}Ref<'graph>` を EdgeRef と総称する。

### 3.1.1 用語

生成物を指す4つの用語をここで定義する。他ファイルで生成物に触れる場合は
この節を指す (「参照: `docs/schema_v4.md` §3.1.1」)。

- **名前付きラッパー**とは、`graph!` の呼び出しごとにマクロが生成する構造体
  のことである。素の `Graph` を保持し、左辺名と同名のメソッドで参照を返し、
  `Deref`/`DerefMut` で素の `Graph` の操作を使え、`into_graph()` で素の
  `Graph` を取り出せる。
- **名前付き位置型**とは、`graph!` が要素ごとに生成する、`Graph` 内部の
  格納位置と、生成元を識別する構築印を保持する型のことである。凍結を
  またいで運ばれ、静的アクセサが公開IDの検索なしに参照を作るために使う。
  生成元以外の `Graph` へ渡すと、保持している構築印の不一致が実行時に
  検出されて `panic!` する (`crates/graphite/src/lib.rs` の構築印発行関数と
  `NamedGraphElement::bind` の生成実装を参照)。
- **呼び出し箇所**とは、`graph!` を1回呼んだ場所のことである。
- **凍結**とは、builderに積んだ要素を検査して確定済み `Graph` へ変換する
  操作のことである。英語名は `freeze`。

凍結は公開IDを種別専用の位置へ変換する。完成済み辺記録と索引は位置を保持するため、`NodeRef` または `EdgeRef` を得た後の端点走査では公開IDのハッシュ表での検索を行わない。どちらの参照型も `&Graph` と位置だけを持つ `Copy + Clone` の値であり、ヒープ確保、自己参照、`Rc`、`RefCell`、実行時リフレクションを使わない。

公開される `NodeRef` の `Deref::Target` にノード値型が現れるため、公開schemaのノード値型には到達可能な可視性が必要である。通常は `pub struct Person` と宣言する。

ノード値型が `id`/`value` という名のメソッドを持つ場合、`NodeRef` の同名の固有メソッドが優先される (メソッド解決は `Deref` より先に固有メソッドを探すため)。値側のメソッドを呼びたいときは `(*node_ref).id()` のように明示的に `Deref` させる。

### 3.2 アクセス (種別APIは Graph、探索は Ref、静的な名前は名前付きラッパーのメソッド)

種別APIとは、ある種別に属する個体の全体を対象にする読み取り・可変操作の
ことである。個体と索引の所有者は完成済みの `Graph` なので、種別APIは `Graph`
のメソッドになる。名前は `{種別名}_{固定接尾辞}` の機械的連結であり、
`bosses()` のような自然言語の複数形は生成しない。一度 `NodeRef` を得た後の
関係の探索は、その参照が親 `Graph` と内部位置を保持しているので参照自身の
メソッドで辿る (親 `Graph` を引数で渡し直さない)。

```rust
// ノード種別API (ユーザー struct への固有 impl は行わない —
// 複数 schema 共有時の衝突回避。schema module にノード名の型も作らない)
let p: Option<Org::PersonRef<'_>> = g.person_by_id(&alice_id);
g.person_ids();  g.person_iter();       // &PersonId / PersonRef<'_>
g.person_len();                         // usize
g.person_value_mut(&alice_id);          // Option<&mut Person>

// 辺 — NodeRef から役割名で探索し、常に EdgeRef を返す
let bob = g.person_by_id(&bob_id).unwrap();
bob.boss_as_subordinate();              // each subordinate:0..1 → Option<BossRef<'_>>
bob.boss_between(alice);                // unique pair → Option、他 → iterator
bob.boss_try_between(alice);            // 異なるGraphなら GraphMismatch

// 辺種別API
g.boss_by_id(&boss_id);                 // キーで辺 1 本: Option<BossRef<'_>>
g.boss_iter();                          // BossRef<'_>
g.boss_ids();  g.boss_len();
g.boss_payload_mut(&boss_id);           // Option<&mut BossEdge>

// graph! の同じ呼び出し箇所で名前が分かる場合 (ID検索なし)
g.alice();
g.b_boss();
```

`{kind}_between` / `{kind}_try_between` の主語は位置0側 (有向辺は始点側、
無向辺は唯一の端点型) の `NodeRef` である。可変APIの主語は `&mut Graph` だけ
とする。`NodeRef`/`EdgeRef` は共有借用のハンドルなのでそこから可変借用は
作れず、引数も公開IDのままにする (可変借用中は `Ref` を生かせないため、
内部位置をキーにできない)。

有向の `EdgeRef` は役割名による取得メソッドに加え、方向固定の別名
`from()` / `to()` / `from_id()` / `to_id()` を持つ。自己型辺でも
`subordinate()` / `superior()` のように両役割が曖昧にならない。無向の `EdgeRef` は
方向を捏造せず、`endpoints()` で2つの `NodeRef` を返し、`from` / `to` は持たない。
辺値の端点IDは構築の入力であり、完成後の端点は変更できない。ノード値と辺の
積み荷だけを `graph.{type}_value_mut` / `graph.{kind}_payload_mut` で変更できる。

- 旧ビュー API (`g.boss().of(..)`、EdgeOne 等 6 型) は**全廃**。
  ランタイムの共通機構は「キー付き要素表」に対するジェネリクスとして
  再構成する (ノード表と辺表で共有できるはず。実装時に設計)。
- builder: `b.insert(key, node_value)` (v3 の総称insertを維持) +
  `b.add(key, edge_value)` (辺版の総称)。
- **順序保証 (仕様):** `KeyedTable` (`crates/graphite/src/keyed_table.rs`)
  の `ids`/`iter` は挿入順 (`insert` を呼んだ順) を保持する。これにより
  制約なし辺の役割探索/`{kind}_iter`/`{kind}_between` (iterator を返す各所) は格納順
  (構築時の追加順) を保持する — 旧フェーズ5 項目 i で仕様化された
  「正式な順序保証」の言語の約束であり、実装の副産物ではなく仕様として
  扱う (同じ役割の平行辺が複数ある場合でも、役割クエリはリテラル/builder
  での記述順どおりに返る)。

### 3.3 検証 (凍結時)

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
  (§2/§2.5/§3/§4 を v4 の概念 — 全要素キー・where 制約・Graph中心の種別API —
  で書き直し、エラー引用は実採取)
- docs/edge_syntax_v3.md / graph_literal_v3.md / edge_view_api.md の冒頭に
  「v4 (本ファイル) で置換済み」の注記

## 6. 見送り・保留 (根拠つき)

- 役割別探索API (`purchase_as_buyer` 等) は後続Issueで扱う。役割名を使った
  `EdgeRef` アクセサと、`graph!` 左辺名の静的アクセサは実装済み。
- ノード宣言へのキーワード統一 (`node Person;` の再検討) — v4 安定後、
  Fudaba #1 後継として
- 「グラフで書くべきもの vs 構造体で書くべきもの」のモデリング指針 —
  Fudaba 別札で議論 (ユーザー発案 2026-07-16)
