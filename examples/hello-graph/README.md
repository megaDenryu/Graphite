# hello-graph

**これは教材です。** アプリとしての面白さは無く、Graphite (`graph_schema!`/
`graph!`) の意味論を1つずつ確認するためのものです。実践的な使用例は他の
3本を見てください:

- `examples/build-pipeline` — ビルドパイプライン・オーケストレータ
- `examples/org-analyzer` — 組織分析ツール
- `examples/dialogue-engine` — 分岐ノベルエンジン

## これは何を確かめる example か

「`Boss` は変数なのか、型なのか、何なのか。積み荷 (`BossEdge`) にはどう
やってアクセスするのか。逆に何にアクセスできなくてエラーになるのか。
`graph_schema!`/`graph!` は結局どんな公開APIを生成するのか」を、
`Person`/`Team` の2ノード種別、4本のエッジ (`docs/schema_v4.md` の
`where` 制約パターン: `each member: 1`・`each subordinate: 0..1`・
`unique pair`・制約なし を一通りカバー) を使った最小の題材で確認します。

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
cargo run
```

`src/main.rs` は上から読める構成になっています:

| セクション | 内容 |
|---|---|
| §1 | ノード型・エッジ積み荷型の宣言 (普通の struct) |
| §2 | `graph_schema!` でのスキーマ宣言 (v4: `edge Kind = ...;` は新しい nominal 型の定義、`where` は制約) |
| §2.5 | 脱糖の実像。全要素キー・`KeyedTable` 格納・辺は名前付きフィールドの構造体として第一級、という実装を解説 |
| §3 | クックブック — 生成される公開APIを1関数=1つのやりたいこと単位で全列挙 (`cargo run` で実行される) |
| §4 | 「できないこと」— コメントアウトしたコード + 実際に採取したコンパイルエラー |
| §5 | `flow!` — 関数の辺 (`graph!` の宣言される辺との対比。`cargo run` で実行される) |

## Kind (辺種別) は何者か

`edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1;` の
`Boss` は、**新しい nominal 型 (名前で区別される型) の定義**です。
同じ形 (`Person -> Person`) の辺を2つ宣言しても、それぞれ独立した別の
型になります (`docs/schema_v4.md` §0)。読み方の骨格は3規則だけです:

1. **`名前 = 定義`** — `edge Kind = (role: From) -> (role: To) ...;` は `Kind` という
   型を定義する宣言 (取り違えてもコンパイルエラーになる)
2. **矢印の中は積み荷だけ** — `-[X]->` の `X` はその辺が運ぶ積み荷の型
   (積み荷が無ければ素の `->`)。ラベル名を矢印の中に書くことは無い —
   `Kind` という名前は既に左辺で言い切っているため
3. **`where` は制約** — `each <role>: N | N..M | N..*`・`unique pair`。
   省略時は「制約なし」(平行辺も自由)

`Boss` から `graph_schema!` が機械的に生成するもの:

| 生成されるもの | 命名規則 | `Boss` の場合 |
|---|---|---|
| 既定ID newtype | `{Node}Id` / `{Kind}Id` | `pub struct PersonId(pub String);` / `pub struct BossId(pub String);` |
| 名前付きフィールドの構造体本体 | スキーマの役割名 | `pub struct Boss { pub subordinate: PersonId, pub superior: PersonId, pub appointment: BossEdge }` |
| 読み取り方法 | 端点はスキーマの役割名、積み荷は `payload()` | `boss.subordinate` / `boss.superior` / `boss_ref.payload()` |
| 探索メソッド (`NodeRef`) / 種別API (`Graph`) | `{kind}_as_<role>`/`{kind}_between` / `{kind}_by_id`/`{kind}_iter`/`{kind}_ids`/`{kind}_len` | `person.boss_as_subordinate()` 等 |
| 違反 enum のバリアント | `{Kind}DuplicateKey`/`{Kind}UnknownSource`/`{Kind}UnknownTarget`/`{Kind}{Role}EachViolation`/`{Kind}UniquePairViolation` | `Violation::BossSubordinateEachViolation { .. }` |

`{kind}_as_<role>`/`{kind}_between` の戻り型は宣言した `where` 制約が決めます (これだけ覚えれば
全 Kind に応用できます)。戻り値は相手ノードではなく常に `KindRef<'graph>`
(積み荷は `edge.payload()` から辿る)。`{kind}_as_<role>` は問い合わせた役割自身の
`each` 制約で決まり、`between` は `unique pair` 制約の有無で決まる (2つは
独立した軸なので分けて示す):

| 役割の `each` 制約 | `{kind}_as_<role>` の戻り値 | `iter()` の要素 |
|---|---|---|
| `each X: 1` | `KindRef<'graph>` | `KindRef<'graph>` |
| `each X: 0..1` | `Option<KindRef<'graph>>` | 同上 |
| `each X: N` / `N..M` / `N..*` のその他の範囲、または制約なし | `impl Iterator<Item = KindRef<'graph>>` | 同上 |

| `unique pair` の有無 | `between` の戻り値 |
|---|---|
| あり | `Option<KindRef<'graph>>` (対で高々1本のため) |
| なし | `impl Iterator<Item = KindRef<'graph>>` |

構築時の `Boss` と完成後の `BossRef` の違いは `src/main.rs` §2.5 で説明しています。
`Boss` は端点IDと積み荷を持つ名前付きフィールドの構造体です。freeze は端点IDを非公開位置へ変換し、完成済みグラフは `graphite::KeyedTable<BossId, BossRecord>` を保持します。利用者は `BossRef<'graph>` を通して完成済み記録を読みます。`graph!` の
`key = Boss(from -[積み荷式]-> to)` は
`__graphite_b.add_named(key, <Boss as graphite::DirectedEdgeLiteral<_, _, _>>::from_graph_literal(from.clone(), to.clone(), 積み荷式))` という
通常のメソッド呼び出しと、内部位置handleを持つローカルwrapperへ脱糖されます。

## クックブック チートシート (`src/main.rs` §3 と1対1対応)

`src/main.rs` §3 の各関数が、それぞれ生成APIの1つずつに対応しています。
「やりたいこと」列の順は `main.rs` の呼び出し順 (構築 → ノードを読む →
エッジを辿る → 一覧する → 検証エラーを受ける) と同じです。v4 では
IDによる動的検索は**`Graph` に生えた種別APIのメソッド**です
(`g.person_by_id(&id)` のようなノード種別のメソッド、`g.boss_by_id(&id)`
のような辺種別のメソッド)。一方、`graph!` 左辺名は
`g.alice()` / `g.bob_boss()` のような呼び出しsite固有の静的アクセサになります。

### 構築

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| `graph!` にノード式・エッジをインラインで書く | `graphite::graph!(Org { alice = Person { .. }, ad = BelongsTo(alice -> eng), .. })` | `Result<名前付きwrapper, OrgViolation>` (wrapperは `Deref<Target = Org::Graph>`) |
| 外部で作った値を `graph!` に渡す | `let v = Person{..}; graph!(Org { alice = v, .. })` | 同上 |
| 外部で作ったエッジ積み荷を `graph!` に渡す | `graph!(Org { .. bb = Boss(bob -[promotion]-> alice), .. })` | 同上 |
| builder の型名メソッドで組み立てる | `Org::create(\|b\| { b.person(id, value); b.belongs_to(edge_id, BelongsTo { member: from, team: to }); })` | 同上 |
| builder の総称 `insert`/`add` で組み立てる | `let id: PersonId = b.insert("eve", Person{..}); let eid = b.add("k", BelongsTo { member: id, team: team_id });` | `N::Id`/`E::Id` (呼び出し側の値の型で決まる) |
| 名前付きAPIを捨てて素のGraphへ戻す | `let graph: Org::Graph = named.into_graph();` | `Org::Graph` |

### ノードを読む

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 人ノードを1件読む | `g.person_by_id(&PersonId("alice".to_string()))` | `Option<PersonRef<'_>>` |
| 左辺名から人ノードを直接読む | `g.alice()` | `PersonRef<'_>` (ID hash lookupなし) |
| チームノードを1件読む | `g.team_by_id(&TeamId("eng".to_string()))` | `Option<TeamRef<'_>>` |
| `PersonId` を手で組み立てる (`graph!` のキーと同一視) | `PersonId("alice".to_string())` | `PersonId` |

### エッジを辿る ({kind}_as_<role> / {kind}_by_id / {kind}_between)

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| `each member: 1` を辿る | `person.belongs_to_as_member()` | `BelongsToRef<'_>` |
| `each subordinate: 0..1` +積み荷ありを辿る | `person.boss_as_subordinate()` | `Option<BossRef<'_>>` |
| `unique pair` を対で検索する | `from.reports_between(to)` | `Option<ReportsRef<'_>>` |
| 制約なしを辿る | `person.reviewed_by_as_reviewee()` | `impl Iterator<Item = ReviewedByRef<'_>>` |
| キーで辺1本を検索する | `g.belongs_to_by_id(&BelongsToId("bt1".to_string()))` | `Option<BelongsToRef<'_>>` |
| 無向辺 (`--`) の両端を読む/対称に辿る | `g.friends_by_id(&id).endpoints()` / `person.friends_incident()` | `(PersonRef<'_>, PersonRef<'_>)` / iterator |

### 一覧する ({kind}_iter / {kind}_ids / {kind}_len)

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 人ノードの全キーを列挙する | `g.person_ids()` | `impl Iterator<Item = &PersonId>` |
| チームノードの全キーを列挙する | `g.team_ids()` | `impl Iterator<Item = &TeamId>` |
| エッジを全部列挙する (キー付き) | `g.belongs_to_iter()` | `impl Iterator<Item = BelongsToRef<'_>>` |
| 積み荷ありエッジを全部列挙する | `g.boss_iter()` | `impl Iterator<Item = BossRef<'_>>` (積み荷は `edge.payload()`) |
| 表の辺の本数を確認する | `g.belongs_to_len()` | `usize` |

### 検証エラーを受ける

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 重複ノードキーの違反を受け取る | `match Org::create(\|b\| ..) { Err(OrgViolation::DuplicatePerson(id)) => .., _ => {} }` | `Result<Org, OrgViolation>` |
| 辺キー重複の違反を受け取る (v4新規) | `Err(OrgViolation::BelongsToDuplicateKey(id))` を `match` で受ける | 同上 |
| 未知の始点/終点キー参照の違反を受け取る | `Err(OrgViolation::BelongsToUnknownSource { edge, source })`/`UnknownTarget { edge, target }` を `match` で受ける | 同上 |
| each違反を受け取る | `Err(OrgViolation::BelongsToMemberEachViolation { source, count })` を `match` で受ける | 同上 |
| unique pair違反を受け取る | `Err(OrgViolation::ReportsUniquePairViolation { source, target })` を `match` で受ける | 同上 |
| 最初の1件の違反だけで止める | `Org::create(\|b\| ..)` | `Result<Org, OrgViolation>` |
| 違反を全件集める | `Org::create_collecting(\|b\| ..)` | `Result<Org, Vec<OrgViolation>>` |

## `flow!` — 関数の辺 (`src/main.rs` §5 と1対1対応)

`graph_schema!`/`graph!` の辺は**宣言**(構築時にまとめて検証されるデータの
繋がり) ですが、`graphite::flow!` (`docs/flow_macro.md`) の矢印 `-[関数式]->`
は**実行**です — 書かれた順に `let 束縛名 = (関数式)(始点..);` という関数
呼び出しへ即時に脱糖するだけで、スキーマ・builder は一切関与しません。
同じ矢印記法 `-[X]->` を「宣言される辺」と「即時実行される辺」という対照的
な2つの意味論に使い分けている、という読み方が両者を統一します。

| やりたいこと | 書き方 | 脱糖後 |
|---|---|---|
| 直線 (1本の矢印) | `x -[f]-> y` | `let y = (f)(x);` |
| チェーン形 | `x -[f]-> y -[g]-> z` | `let y = (f)(x); let z = (g)(y);` |
| fan-out (同じ始点を複数の矢印に流す) | `x -[f]-> y, x -[g]-> z` | `let y = (f)(x); let z = (g)(x);` |
| fan-in (タプル始点で多引数呼び出し) | `(a, b) -[f]-> y` | `let y = (f)(a, b);` |
| 束縛を後で使う | `y`/`z` は普通のローカル変数として `flow!` の後に見える | (`graph!` の左辺はローカル変数ではなく名前付きwrapperのメソッドとして残る) |

## できる/できない一覧

| やりたいこと | できる? | 方法 / 実際に出るエラー |
|---|---|---|
| `Boss` エッジの相手ノードを取得する | できる | `person.boss_as_subordinate().map(|edge| edge.superior())` |
| `Boss` エッジの積み荷 (`since`) を読む | できる | `edge.payload().since` |
| 未知キーで安全に問い合わせる | できる | `g.person_by_id(&id)` (`Option` で返る) |
| キーで辺1本を検索する | できる | `g.belongs_to_by_id(&edge_id)` |
| 全エッジをイテレータで走査する | できる | `g.boss_iter()` (`BossRef<'_>`) |
| `Boss` を積み荷のように (`Boss.since`) 読む | **できない** | `error[E0609]: no field \`since\` on type \`fn(PersonId, PersonId, BossEdge) -> Boss {Boss}\`` (§4.1) |
| `g.boss` とフィールドのように書いて `Person` を得る | **できない** | グラフの格納フィールドは非公開であり、公開APIは `BossRef<'graph>` を返す `Graph`/`NodeRef` のメソッドに限定される (§4.2、§2.5 参照) |
| `graph!` に存在しないエッジ種別を書く | **できない** | `error[E0433]: failed to resolve: use of undeclared type \`NoSuchKind\`` (素のrustc型解決。§4.3) |
| `graph!` のエッジ端点に間違ったノード型を渡す | **できない** | `error[E0308]: mismatched types` (`expected TeamId, found PersonId`。§4.4) |

実際のエラー全文は `src/main.rs` の §4 に、コメントアウトしたコードと
併せて引用してあります (捏造ではなく、コメントを外して `cargo build`
した実測値です)。
