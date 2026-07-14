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
アクセスできなくてエラーになるのか」を、最小の題材 (`Person`/`Team` の
2ノード種別、属性なしエッジ2本 + 属性ありエッジ1本、多重度 `(1)`/
`(0..1)`/`(0..*)` を1回ずつ) で確認します。

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
| §3 | `graph!` での構築と、多重度ごとのアクセス方法 (`cargo run` で実行される) |
| §4 | 「できないこと」— コメントアウトしたコード + 実際に採取したコンパイルエラー |

## ラベルは何者か

`edge Person -[boss: BossEdge]-> Person (0..1);` の `boss` は、**値でも
変数でもなく、これから生成される一群のメソッド名の元になる識別子**です。
`graph_schema!` はこの1トークンから以下を機械的に命名・生成します
(`boss` の場合の実例つき):

| 生成されるもの | 命名規則 | `boss` の場合 |
|---|---|---|
| アクセサ (パニック版) | `{label}` | `boss(&PersonId) -> Option<(&Person, &BossEdge)>` (多重度による。下表参照) |
| アクセサ (非パニック版) | `try_{label}` | `try_boss(&PersonId) -> Option<(&Person, &BossEdge)>` |
| ID版アクセサ | `{label}_id` / `{label}_ids` | `boss_id(&PersonId) -> Option<&PersonId>` |
| ペアイテレータ | `{label}_pairs` | `boss_pairs() -> impl Iterator<Item = (&PersonId, &PersonId, &BossEdge)>` |
| builder のエッジ追加メソッド | `{label}` | `OrgBuilder::boss(from, to, attrs)` |
| 違反 enum のバリアント | `{Label}Multiplicity`/`{Label}UnknownSource`/`{Label}UnknownTarget` | `OrgViolation::BossMultiplicity { .. }` |

属性なしエッジ (`belongs_to`) では属性を運ぶ部分が無いだけで、上記の
命名規則自体は同じです。ノード型宣言 (`node Person;`) からは `{Node}Id`
newtype キー (`PersonId`) と `{node_snake}_ids()` (`person_ids()`) が
生成されます。

多重度ごとのアクセサの戻り値:

| 多重度 | `{label}` の戻り値 | `{label}_pairs()` の要素 |
|---|---|---|
| `(1)` | `&T` (属性つきは `(&T, &Attrs)`)。未知キーはパニック | `(&SrcId, &DstId[, &Attrs])` |
| `(0..1)` | `Option<&T>` (属性つきは `Option<(&T, &Attrs)>`) | 同上 |
| `(0..*)` | `Vec<&T>` (属性つきは `Vec<(&T, &Attrs)>`) | 同上 (始点キーごとに全ペアへ展開) |

## できる/できない一覧

| やりたいこと | できる? | 方法 / 実際に出るエラー |
|---|---|---|
| `boss` エッジの相手ノードを取得する | できる | `g.boss(&id)` (`src/main.rs` §3) |
| `boss` エッジの属性 (`since`) を読む | できる | `g.boss(&id)` が返す `(&Person, &BossEdge)` の2番目から `attrs.since` |
| 未知キーで安全に問い合わせる | できる | `g.try_belongs_to(&id)` (`Option` で返る) |
| キーだけを取得して次のノードへ辿る | できる | `g.boss_id(&id)` (値ではなくキーの `Option<&PersonId>`) |
| 全エッジをイテレータで走査する | できる | `g.boss_pairs()` (`(&PersonId, &PersonId, &BossEdge)` の3つ組) |
| `boss` を値として (`boss.since`) 読む | **できない** | `error[E0425]: cannot find value \`boss\` in this scope` (§4.1) |
| `g.boss` とフィールドのように書いて `Person` を得る | **できない** | `error[E0308]: mismatched types` (中身は `HashMap`。§4.2) |
| `graph!` に存在しないエッジラベルを書く | **できない** | `error[E0599]: no method named \`no_such_label\` found ...` (v3 でハンドシェイクマクロを全廃したため、素の rustc method-not-found のみ。§4.3) |
| `graph!` のエッジ端点に間違ったノード型を渡す | **できない** | `error[E0308]: mismatched types` (`expected TeamId, found PersonId`。§4.4) |

実際のエラー全文は `src/main.rs` の §4 に、コメントアウトしたコードと
併せて引用してあります (捏造ではなく、コメントを外して `cargo build`
した実測値です)。
