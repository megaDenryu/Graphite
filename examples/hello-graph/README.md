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
`where` 制約パターン: `each Person: 1`・`each Person: 0..1`・
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
| §2.5 | 脱糖の実像。全要素キー・`KeyedTable` 格納・辺はタプル struct として第一級、という v4 の実装を実測して解説 |
| §3 | クックブック — 生成される公開APIを1関数=1つのやりたいこと単位で全列挙 (`cargo run` で実行される) |
| §4 | 「できないこと」— コメントアウトしたコード + 実際に採取したコンパイルエラー |

## Kind (辺種別) は何者か

`edge Boss = Person -[BossEdge]-> Person where each Person: 0..1;` の
`Boss` は、**新しい nominal 型 (名前で区別される型) の定義**です。
同じ形 (`Person -> Person`) の辺を2つ宣言しても、それぞれ独立した別の
型になります (`docs/schema_v4.md` §0)。読み方の骨格は3規則だけです:

1. **`名前 = 定義`** — `edge Kind = From -> To ...;` は `Kind` という
   型を定義する宣言 (取り違えてもコンパイルエラーになる)
2. **矢印の中は積み荷だけ** — `-[X]->` の `X` はその辺が運ぶ積み荷の型
   (積み荷が無ければ素の `->`)。ラベル名を矢印の中に書くことは無い —
   `Kind` という名前は既に左辺で言い切っているため
3. **`where` は制約** — `each <FromType>: 1` (全域関数)・
   `each <FromType>: 0..1` (部分関数)・`unique pair` (平行辺禁止)。
   省略時は「制約なし」(平行辺も自由)

`Boss` から `graph_schema!` が機械的に生成するもの:

| 生成されるもの | 命名規則 | `Boss` の場合 |
|---|---|---|
| 辺キー newtype | `{Kind}Id` | `pub struct BossId(pub String);` |
| タプル struct 本体 | `Kind(From, To[, Payload])` | `pub struct Boss(pub PersonId, pub PersonId, pub BossEdge);` |
| 読み取りメソッド (固有 impl) | `from()`/`to()`/`payload()` | `Boss::from`/`Boss::to`/`Boss::payload` |
| クエリ関連関数 (固有 impl) | `of`/`get`/`between`/`iter`/`ids`/`len` | `Boss::of(&g, &id)` 等 |
| 違反 enum のバリアント | `{Kind}DuplicateKey`/`{Kind}UnknownSource`/`{Kind}UnknownTarget`/`{Kind}EachViolation`/`{Kind}UniquePairViolation` | `OrgViolation::BossEachViolation { .. }` |

`of`/`between` の戻り型は宣言した `where` 制約が決めます (これだけ覚えれば
全 Kind に応用できます):

| 制約 | `of` の戻り値 | `between` の戻り値 | `iter()` の要素 |
|---|---|---|---|
| `each X: 1` | `&T` (積み荷つきは `(&T, &Attrs)`)。未知キーはパニック (非パニック版 `get_of`) | `Vec<&Kind>` | `(&{Kind}Id, &Kind)` |
| `each X: 0..1` | `Option<&T>` (積み荷つきは `Option<(&T, &Attrs)>`) | `Vec<&Kind>` | 同上 |
| `unique pair` | `Vec<&T>` (積み荷つきは `Vec<(&T, &Attrs)>`) | `Option<&Kind>` (対で高々1本のため) | 同上 |
| 制約なし | `Vec<&T>` (積み荷つきは `Vec<(&T, &Attrs)>`) | `Vec<&Kind>` | 同上 |

「`Boss` を値として `.attr` で持っているのか?」という疑問には `src/main.rs`
§2.5 (脱糖の実像) で直接回答しています。要点だけ言うと **No** —
`Boss` はタプル struct であり積み荷は `.2`/`payload()` の位置アクセスで
しか持ちません。格納先は `Org` 構造体の非公開フィールド
(`graphite::KeyedTable<BossId, Boss>`) であり、`graph!` の
`key = Boss(from -[積み荷式]-> to)` は
`__graphite_b.add(key, Boss(from.clone(), to.clone(), 積み荷式))` という
ただのメソッド呼び出しに脱糖されるだけです。

## クックブック チートシート (`src/main.rs` §3 と1対1対応)

`src/main.rs` §3 の各関数が、それぞれ生成APIの1つずつに対応しています。
「やりたいこと」列の順は `main.rs` の呼び出し順 (構築 → ノードを読む →
エッジを辿る → 一覧する → 検証エラーを受ける) と同じです。v4 では
`g.メソッド()` は一切生成されず、**すべて型名前空間の関連関数**です
(ノードは `{Schema}Node` トレイト経由、辺は各 `Kind` への固有 impl)。

### 構築

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| `graph!` にノード式・エッジをインラインで書く | `graphite::graph!(Org { alice = Person { .. }, ad = BelongsTo(alice -> eng), .. })` | `Result<Org, OrgViolation>` |
| 外部で作った値を `graph!` に渡す | `let v = Person{..}; graph!(Org { alice = v, .. })` | 同上 |
| 外部で作ったエッジ積み荷を `graph!` に渡す | `graph!(Org { .. bb = Boss(bob -[promotion]-> alice), .. })` | 同上 |
| builder の型名メソッドで組み立てる | `Org::create(\|b\| { b.person(id, value); b.belongs_to(edge_id, BelongsTo(from, to)); })` | 同上 |
| builder の総称 `insert`/`add` で組み立てる | `let id: PersonId = b.insert("eve", Person{..}); let eid = b.add("k", BelongsTo(id, team_id));` | `N::Id`/`E::Id` (呼び出し側の値の型で決まる) |

### ノードを読む

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 人ノードを1件読む | `Person::get(&g, &PersonId("alice".to_string()))` | `Option<&Person>` |
| チームノードを1件読む | `Team::get(&g, &TeamId("eng".to_string()))` | `Option<&Team>` |
| `PersonId` を手で組み立てる (`graph!` のキーと同一視) | `PersonId("alice".to_string())` | `PersonId` |

### エッジを辿る (Kind::of/get/between)

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| `each Person: 1` を辿る (パニック版) | `BelongsTo::of(&g, &id)` | `&Team` |
| `each Person: 1` を安全に辿る | `BelongsTo::get_of(&g, &id)` | `Option<&Team>` |
| `each Person: 0..1` +積み荷ありを辿る | `Boss::of(&g, &id)` | `Option<(&Person, &BossEdge)>` |
| `unique pair` を対で検索する | `Reports::between(&g, &from, &to)` | `Option<&Reports>` |
| 制約なしを辿る | `ReviewedBy::of(&g, &id)` (for ループで受ける) | `Vec<(&Person, &ReviewEdge)>` |
| キーで辺1本を検索する | `BelongsTo::get(&g, &BelongsToId("bt1".to_string()))` | `Option<&BelongsTo>` |
| 無向辺 (`--`) の両端を読む/対称に辿る (v4.1) | `Friends::get(&g,&id).endpoints()` / `Friends::of(&g, &id)` (どちらの位置でも対称) | `(&PersonId, &PersonId)` / `Vec<&Person>` |

### 一覧する (iter/ids/len)

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 人ノードの全キーを列挙する | `Person::ids(&g)` | `impl Iterator<Item = &PersonId>` |
| チームノードの全キーを列挙する | `Team::ids(&g)` | `impl Iterator<Item = &TeamId>` |
| エッジを全部列挙する (キー付き) | `BelongsTo::iter(&g)` | `impl Iterator<Item = (&BelongsToId, &BelongsTo)>` |
| 積み荷ありエッジを全部列挙する | `Boss::iter(&g)` | `impl Iterator<Item = (&BossId, &Boss)>` (積み荷は `edge.payload()`) |
| 表の辺の本数を確認する | `BelongsTo::len(&g)` | `usize` |

### 検証エラーを受ける

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 重複ノードキーの違反を受け取る | `match Org::create(\|b\| ..) { Err(OrgViolation::DuplicatePerson(id)) => .., _ => {} }` | `Result<Org, OrgViolation>` |
| 辺キー重複の違反を受け取る (v4新規) | `Err(OrgViolation::BelongsToDuplicateKey(id))` を `match` で受ける | 同上 |
| 未知の始点/終点キー参照の違反を受け取る | `Err(OrgViolation::BelongsToUnknownSource { edge, source })`/`UnknownTarget { edge, target }` を `match` で受ける | 同上 |
| each違反を受け取る | `Err(OrgViolation::BelongsToEachViolation { source, count })` を `match` で受ける | 同上 |
| unique pair違反を受け取る | `Err(OrgViolation::ReportsUniquePairViolation { source, target })` を `match` で受ける | 同上 |
| 最初の1件の違反だけで止める | `Org::create(\|b\| ..)` | `Result<Org, OrgViolation>` |
| 違反を全件集める | `Org::create_collecting(\|b\| ..)` | `Result<Org, Vec<OrgViolation>>` |

## できる/できない一覧

| やりたいこと | できる? | 方法 / 実際に出るエラー |
|---|---|---|
| `Boss` エッジの相手ノードを取得する | できる | `Boss::of(&g, &id)` (`src/main.rs` §3) |
| `Boss` エッジの積み荷 (`since`) を読む | できる | `Boss::of(&g, &id)` が返す `(&Person, &BossEdge)` の2番目、または `edge.payload().since` |
| 未知キーで安全に問い合わせる | できる | `BelongsTo::get_of(&g, &id)` (`Option` で返る) |
| キーで辺1本を検索する | できる | `BelongsTo::get(&g, &edge_id)` |
| 全エッジをイテレータで走査する | できる | `Boss::iter(&g)` (`(&BossId, &Boss)` の組) |
| `Boss` を積み荷のように (`Boss.since`) 読む | **できない** | `error[E0609]: no field \`since\` on type \`fn(PersonId, PersonId, BossEdge) -> Boss {Boss}\`` (§4.1) |
| `g.boss` とフィールドのように書いて `Person` を得る | **できない** | `error[E0308]: mismatched types` (中身は `KeyedTable<BossId, Boss>`。§4.2、§2.5 参照) |
| `graph!` に存在しないエッジ種別を書く | **できない** | `error[E0425]: cannot find function, tuple struct or tuple variant \`NoSuchKind\` in this scope` (ハンドシェイクマクロは無く、素の rustc 型解決のみ。§4.3) |
| `graph!` のエッジ端点に間違ったノード型を渡す | **できない** | `error[E0308]: mismatched types` (`expected TeamId, found PersonId`。§4.4) |

実際のエラー全文は `src/main.rs` の §4 に、コメントアウトしたコードと
併せて引用してあります (捏造ではなく、コメントを外して `cargo build`
した実測値です)。
