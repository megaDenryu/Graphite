# hello-graph

**これは教材です。** アプリとしての面白さは無く、Graphite (`graph_schema!`/
`graph!`) の意味論を1つずつ確認するためのものです。実践的な使用例は他の
3本を見てください:

- `examples/build-pipeline` — ビルドパイプライン・オーケストレータ
- `examples/org-analyzer` — 組織分析ツール
- `examples/dialogue-engine` — 分岐ノベルエンジン

## これは何を確かめる example か

「`boss.since` のようにアクセスできるのか? ラベルは変数なのか、関数
なのか、何なのか。値だったらそこから何にアクセスできるのか? 逆に何に
アクセスできなくてエラーになるのか。`graph_schema!`/`graph!` は結局
どんな公開APIを生成するのか」を、`Person`/`Team` の2ノード種別、
属性なしエッジ2本 (`belongs_to`・`reports`) + 属性ありエッジ2本
(`boss`・`reviewed_by`)、多重度 `(1)`/`(0..1)`/`(0..*)` を一通り使った
最小の題材で確認します。

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
cargo run
```

`src/main.rs` は上から読める構成になっています:

| セクション | 内容 |
|---|---|
| §1 | ノード型・エッジ属性型の宣言 (普通の struct) |
| §2 | `graph_schema!` でのスキーマ宣言。ラベルとは何なのかの説明 |
| §2.5 | 脱糖の実像。`-[label = 式]->` が実際どう展開され、どこに格納されるか |
| §3 | クックブック — 生成される公開APIを1関数=1つのやりたいこと単位で全列挙 (`cargo run` で実行される) |
| §4 | 「できないこと」— コメントアウトしたコード + 実際に採取したコンパイルエラー |

## ラベルは何者か

`edge Person -[boss: BossEdge]-> Person (0..1);` の `boss` は、**値でも
変数でもなく、これから生成される一群のメソッド名の元になる識別子**です。
`graph_schema!` はこの1トークンから以下を機械的に命名・生成します
(`boss` の場合の実例つき):

| 生成されるもの | 命名規則 | `boss` の場合 |
|---|---|---|
| アクセサ (パニック版) | `{label}` | `boss(&PersonId) -> Option<(&Person, &BossEdge)>` (多重度による。下表参照) |
| アクセサ (非パニック版、多重度(1)のみ) | `try_{label}` | (`boss` は0..1なので生成されない。`belongs_to` なら `try_belongs_to`) |
| ID版アクセサ | `{label}_id` / `{label}_ids` | `boss_id(&PersonId) -> Option<&PersonId>` |
| ペアイテレータ | `{label}_pairs` | `boss_pairs() -> impl Iterator<Item = (&PersonId, &PersonId, &BossEdge)>` |
| builder のエッジ追加メソッド | `{label}` | `OrgBuilder::boss(from, to, attrs)` |
| 違反 enum のバリアント | `{Label}Multiplicity`/`{Label}UnknownSource`/`{Label}UnknownTarget` | `OrgViolation::BossMultiplicity { .. }` |

属性なしエッジ (`belongs_to`・`reports`) では属性を運ぶ部分が無いだけで、
上記の命名規則自体は同じです。ノード型宣言 (`node Person;`) からは
`{Node}Id` newtype キー (`PersonId`) と `{node_snake}(&id)`/
`{node_snake}_ids()` (`person(&id)`/`person_ids()`) が生成されます。

「`boss` を値として `.attr` で持っているのか?」という疑問には `src/main.rs`
§2.5 (脱糖の実像) で直接回答しています。要点だけ言うと **No** — `boss` は
オブジェクトではなく `Org` 構造体の非公開フィールド名 (内部的には
`HashMap<PersonId, (PersonId, BossEdge)>` という「表」) であり、
`-[boss = 式]->` は `__graphite_b.boss(from.clone(), to.clone(), 式)` という
ただのメソッド呼び出しに脱糖されるだけです。

多重度ごとのアクセサの戻り値:

| 多重度 | `{label}` の戻り値 | `{label}_pairs()` の要素 |
|---|---|---|
| `(1)` | `&T` (属性つきは `(&T, &Attrs)`)。未知キーはパニック | `(&SrcId, &DstId[, &Attrs])` |
| `(0..1)` | `Option<&T>` (属性つきは `Option<(&T, &Attrs)>`) | 同上 |
| `(0..*)` | `Vec<&T>` (属性つきは `Vec<(&T, &Attrs)>`) | 同上 (始点キーごとに全ペアへ展開) |

## クックブック チートシート (`src/main.rs` §3 と1対1対応)

`src/main.rs` §3 の各関数が、それぞれ生成APIの1つずつに対応しています。
「やりたいこと」列の順は `main.rs` の呼び出し順 (構築 → ノードを読む →
エッジを辿る → 一覧する → 検証エラーを受ける) と同じです。エッジの実体は
どれも「ラベル名の表 (非公開フィールド)」であり、`{label}(...)` は
その表への問い合わせだと考えてください (§2.5 参照)。

### 構築

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| `graph!` にノード式・エッジをインラインで書く | `graphite::graph!(Org { alice = Person { .. }, alice -[belongs_to]-> eng, .. })` | `Result<Org, OrgViolation>` |
| 外部で作った値を `graph!` に渡す | `let v = Person{..}; graph!(Org { alice = v, .. })` | 同上 |
| 外部で作ったエッジ属性を `graph!` に渡す | `graph!(Org { .. bob -[boss = promotion]-> alice, .. })` | 同上 |
| builder の型名メソッドで組み立てる | `Org::create(\|b\| { b.person(id, value); b.belongs_to(from, to); })` | 同上 |
| builder の総称 `insert` で組み立てる | `let id: PersonId = b.insert("eve", Person{..});` | `N::Id` (呼び出し側の値の型で決まる) |

### ノードを読む

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 人ノードを1件読む | `g.person(&PersonId("alice".to_string()))` | `Option<&Person>` |
| チームノードを1件読む | `g.team(&TeamId("eng".to_string()))` | `Option<&Team>` |
| `PersonId` を手で組み立てる (`graph!` のキーと同一視) | `PersonId("alice".to_string())` | `PersonId` |

### エッジを辿る (多重度別)

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 多重度(1)を辿る (パニック版) | `g.belongs_to(&id)` | `&Team` |
| 多重度(1)を安全に辿る | `g.try_belongs_to(&id)` | `Option<&Team>` |
| 多重度(1)をidだけで辿る | `g.belongs_to_id(&id)` / `g.try_belongs_to_id(&id)` | `&TeamId` / `Option<&TeamId>` |
| 多重度(0..1)+属性ありを辿る | `g.boss(&id)` | `Option<(&Person, &BossEdge)>` |
| 多重度(0..1)をidだけで辿る | `g.boss_id(&id)` | `Option<&PersonId>` |
| 多重度(0..*)を辿る | `g.reports(&id)` (for ループで受ける) | `Vec<&Person>` |
| 多重度(0..*)をidだけで辿る | `g.reports_ids(&id)` | `Vec<&PersonId>` |
| 多重度(0..*)+属性ありを辿る | `g.reviewed_by(&id)` | `Vec<(&Person, &ReviewEdge)>` |
| 多重度(0..*)+属性ありをidだけで辿る | `g.reviewed_by_ids(&id)` | `Vec<&PersonId>` |

### 一覧する (pairs / ids)

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 人ノードの全キーを列挙する | `g.person_ids()` | `impl Iterator<Item = &PersonId>` |
| チームノードの全キーを列挙する | `g.team_ids()` | `impl Iterator<Item = &TeamId>` |
| 属性なしエッジを全部列挙する | `g.belongs_to_pairs()` | `impl Iterator<Item = (&PersonId, &TeamId)>` |
| 属性ありエッジを全部列挙する | `g.boss_pairs()` | `impl Iterator<Item = (&PersonId, &PersonId, &BossEdge)>` |
| 多重度(0..*)のエッジを全部列挙する (始点ごとに展開) | `g.reports_pairs()` | `impl Iterator<Item = (&PersonId, &PersonId)>` |
| 多重度(0..*)+属性ありのエッジを全部列挙する | `g.reviewed_by_pairs()` | `impl Iterator<Item = (&PersonId, &PersonId, &ReviewEdge)>` |

### 検証エラーを受ける

| やりたいこと | 書き方 | 戻り値の型 |
|---|---|---|
| 重複ノードキーの違反を受け取る | `match Org::create(\|b\| ..) { Err(OrgViolation::DuplicatePerson(id)) => .., _ => {} }` | `Result<Org, OrgViolation>` |
| 未知の始点キー参照の違反を受け取る | `Err(OrgViolation::BelongsToUnknownSource { key })` を `match` で受ける | 同上 |
| 未知の終点キー参照の違反を受け取る | `Err(OrgViolation::BelongsToUnknownTarget { key })` を `match` で受ける | 同上 |
| 多重度違反を受け取る | `Err(OrgViolation::BelongsToMultiplicity { source, count })` を `match` で受ける | 同上 |
| 最初の1件の違反だけで止める | `Org::create(\|b\| ..)` | `Result<Org, OrgViolation>` |
| 違反を全件集める | `Org::create_collecting(\|b\| ..)` | `Result<Org, Vec<OrgViolation>>` |

## できる/できない一覧

| やりたいこと | できる? | 方法 / 実際に出るエラー |
|---|---|---|
| `boss` エッジの相手ノードを取得する | できる | `g.boss(&id)` (`src/main.rs` §3) |
| `boss` エッジの属性 (`since`) を読む | できる | `g.boss(&id)` が返す `(&Person, &BossEdge)` の2番目から `attrs.since` |
| 未知キーで安全に問い合わせる | できる | `g.try_belongs_to(&id)` (`Option` で返る) |
| キーだけを取得して次のノードへ辿る | できる | `g.boss_id(&id)` (値ではなくキーの `Option<&PersonId>`) |
| 全エッジをイテレータで走査する | できる | `g.boss_pairs()` (`(&PersonId, &PersonId, &BossEdge)` の3つ組) |
| `boss` を値として (`boss.since`) 読む | **できない** | `error[E0425]: cannot find value \`boss\` in this scope` (§4.1) |
| `g.boss` とフィールドのように書いて `Person` を得る | **できない** | `error[E0308]: mismatched types` (中身は `HashMap`。§4.2、§2.5 参照) |
| `graph!` に存在しないエッジラベルを書く | **できない** | `error[E0599]: no method named \`no_such_label\` found ...` (v3 でハンドシェイクマクロを全廃したため、素の rustc method-not-found のみ。§4.3) |
| `graph!` のエッジ端点に間違ったノード型を渡す | **できない** | `error[E0308]: mismatched types` (`expected TeamId, found PersonId`。§4.4) |

実際のエラー全文は `src/main.rs` の §4 に、コメントアウトしたコードと
併せて引用してあります (捏造ではなく、コメントを外して `cargo build`
した実測値です)。
