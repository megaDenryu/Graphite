# IDE サポート仕様 (VSCode / rust-analyzer)

> **Development document** — 索引: `docs/README.md`

Graphite の DSL (`dynamic_graph_schema!` / `graph!`) を書くとき、VSCode 上で参照ジャンプ・
型追跡・rename などが「普通の Rust コードと同じように」機能することを目標とする。

計測方法: vscode-lsp-mcp 経由で rust-analyzer の definition/references プロバイダを
直接叩き、`targetSelectionRange` (F12 でカーソルが着地する正確な位置) まで確認した。
計測対象: `crates/graphite/tests/orgchart_macro.rs` (2026-07-14 計測)。

## 1. 現状マトリクス

本節は 2026-07-14 時点の rust-analyzer 実測記録である。以降の節が定める規範
(§1.9 / G3 / G4) は現行であり、生成コードが根拠として名指ししている。

| 操作 | 結果 | 備考 |
|---|---|---|
| 使用側の型名 (`Employee {..}`) → 定義 | ✅ 精密 | schema 内 `node Employee` の `Employee` トークンに着地 |
| 使用側のアクセサ (`g.belongs_to(..)`) → 定義 | ✅ 精密 | schema 内 `edge belongs_to` の `belongs_to` に着地 |
| 派生名 (`try_belongs_to`, `*_id`, `*_ids` 等) → 定義 | ✅ 精密 | `format_ident!` が最初に補間した Ident のスパンを継承するため |
| 属性フィールド (`attrs.since`) → 定義 | ✅ 精密 | schema 内 `{ since: i32 }` の `since` に着地 |
| 違反 enum (`OrgChart::Violation::..`) → 定義 | ✅ | schema 名トークンに着地 |
| schema のノード型 → 参照検索 | ✅ | `graph!` リテラル内の型使用も全件検出 |
| `graph!` の型名 (`tanaka: Employee`) → 定義 | ✅ | |
| **`graph!` エッジ内ノードキー (`tanaka -[..]-> sales`) → 定義** | ❌ 解決不能 | キーが文字列リテラルへ脱糖され、識別子が展開後に残らない |
| **`graph!` ノードキーの rename / 参照検索 / hover** | ❌ | 同上 |
| **examples/* の解析全般** | ❌ | ルート workspace から除外されたスタンドアロンクレートを rust-analyzer が読んでいない |
| **編集途中 (パース不能状態) の耐性** | ❌ 全滅 | パース失敗 → `compile_error!` のみ展開 → 生成型が全部消え、利用側が全て赤くなる。補完も効かない |

スパン保存については既存実装がほぼ正解を出している。残る欠陥は「展開形の構造」
(識別子が束縛として生き残らない) と「プロジェクト構成」(examples 除外) と
「パーサの回復性」の 3 つで、いずれもスパン付け替えでは直らない。

## 1.5 G1/G2 実装後の再計測 (2026-07-14、コミット `1268cba` / `c75d927`)

| 操作 | 結果 |
|---|---|
| `graph!` エッジ内ノードキー → 定義 | ✅ 精密 (ノード宣言の `tanaka` トークンに着地) |
| `graph!` ノードキー → 参照検索 | ✅ 宣言 + 全エッジ内出現を検出 (計3件を確認) |
| `graph!` ノードキー → hover | ✅ トークン範囲で応答 (ローカル変数 `EmployeeId` として) |
| examples/* の解析 | ✅ `Scene`/`SceneId` 等がワークスペースシンボルとして引ける。dynamic_graph_schema! 内トークンへのスパンも機能 |
| rename | ⚠️ VSCode UI (F2) での rename は機能する。schema エッジラベルの rename は派生名 (`{label}_pairs`、アクセサ、builder メソッド) の**参照側までカスケードする**が、**大文字小文字変換を挟む派生名には追従しない** (下記「rename カスケードの境界条件」) |

### rename カスケードの境界条件 (2026-07-14 実測、コミット `1c7d76d` 後に再確認)

rust-analyzer の rename は、派生名の中にリネーム対象トークンが**そのままの文字**で
含まれる場合のみ、その部分を置換してカスケードできる:

- ✅ `boss` → `boss_edge`: `boss_pairs()` の呼び出し側が `boss_edge_pairs()` に、
  `g.boss(..)` / `b.boss(..)` が `g.boss_edge(..)` / `b.boss_edge(..)` に一括で変わる
- ❌ 同 rename で `BossAttrs` (属性型) の参照側は取り残される。`boss` が PascalCase
  変換された「Boss」としてしか現れず、RA は「boss_edge → BossEdge」という
  ケース変換込みの新名を計算できないため。**スパンは正しく effect している**
  (`AssignedAttrs` → schema の `assigned` トークンへの定義ジャンプは精密) ので、
  これはスパンでは解決できない RA 側の構造的制約である
- 同じ理屈で、ケース変換を挟む他の派生名も追従しないと予測される:
  違反バリアント (`{Label}Multiplicity` 等)、ノード型 rename 時の snake_case
  アクセサ/builder メソッド (`Employee` → `employee()`)

実害の評価: 取り残しは「unresolved import `BossAttrs`」のような**正確な位置の
コンパイルエラー**として即座に現れるため、静かな破壊は起きない (rename 後に
エラー箇所を手修正すれば完了)。緩和策の候補は §2 G7 を参照。

注意: `.vscode/settings.json` の `linkedProjects` 変更と proc-macro の変更は、
`rust-analyzer: Restart Server` を実行するまで反映されないことがある
(reloadWorkspace / rebuildProcMacros では不十分な場合を実測した)。

## 1.6 G4 実装後の再計測 (2026-07-14、コミット `6e4b120`)

壊れたノード宣言 (`node Employee { name String }`) + 正常な宣言 + 利用側コード
を含むプローブファイルを VSCode で開いて実測:

| 操作 | 結果 |
|---|---|
| 編集途中 (1宣言だけ構文エラー) の診断 | ✅ エラーはちょうど1件 (壊れたトークン位置の「expected `:`」のみ)。正常な宣言由来の型 (`Department`) の利用コード・`graph!` リテラルに二次エラーは0件 |
| G6: `graph!` 内フィールド名位置の補完 | ❌ 0件 (rust-analyzer の関数様 proc-macro 入力内補完の制約。同ファイルの通常コード位置では補完が正常に返ることを確認済みなので、プロバイダの問題ではない) |
| 副産物の発見 | ⚠️ ハンドシェイクマクロ `__graphite_check_edge_{Schema}!` が通常コード位置の補完候補に露出する (macro_rules のテキストスコープゆえ隠せない。G5 の名前空間汚染の具体的な現れ。実害は軽微だが記録する) |

G6 の結論: 現状の rust-analyzer では関数様 proc-macro の入力トークン木内での
補完は機能しない (speculative expansion がこの形には効かない)。これは Graphite
側で直せる問題ではないため「制約の記録」とする。定義ジャンプ・参照検索・hover・
診断が全て機能しているため、IDE 体験の主要導線は確保できている。

## 1.7 スキーマ宣言構文 v2 実装後の再計測 (2026-07-14、コミット `75f597e`/`86b715a`)

構文 v2 (`docs/history/edge_syntax_v2.md`: ノード型・エッジ属性型を外部 struct 参照化)
の実装後、`crates/graphite/tests/orgchart_macro.rs` で再計測:

| 操作 | 結果 |
|---|---|
| schema 内の属性型 (`-[boss: BossEdge]`) → 定義 | ✅ ユーザー宣言の `struct BossEdge` に精密着地 |
| `node Employee;` → 定義 | ✅ ユーザー宣言の `struct Employee` に着地 (schema 内アンカーと併せて2件提示) |
| 使用側アクセサ (`g.boss(..)`) → 定義 | ✅ schema の `-[boss: ..]` の `boss` トークンに精密着地 (v1 と同等) |
| 属性フィールド使用 (`attrs.since`) → 定義 | ✅ ユーザー struct の `pub since: i32` フィールドに直行 (v1 では schema 内の無名ブロックだった。本物のフィールド宣言に飛ぶようになり改善) |
| `graph!` リテラル内の属性フィールド (`-[boss { since: .. }]` の `since`) → 定義 | ❌ 解決不能。`graph!` (proc-macro) → `__graphite_edge_{Schema}!` (macro_rules) → `BossEdge { .. }` という**二段マクロ展開を rust-analyzer が追跡できない**。コンパイル・ラベル照合は正しく機能しており、IDE ナビゲーションのみの制約。RA 側の進化を待って再計測する |
| rename への効果 | 属性型・ノード型がユーザートークンになったため、型の rename は普通の struct rename (ケース変換の壁 §1.5 の主要ケースが構造的に消滅)。ラベル rename は型に触れない |

## 1.8 graph! リテラル構文 v3 実装後の再計測 (2026-07-15)

`docs/history/graph_literal_v3.md` (`-[label = 式]->` への変更、ハンドシェイクマクロ
全廃) の実装後、`cargo expand -p graphite --test orgchart_macro` で実際の
展開結果を確認した。§1.7 で「二段マクロ展開を rust-analyzer が追跡できない」
と記録した制約は、**展開そのものが単段になったことで構造的に解消した**
(rust-analyzer 側の実測は今回未実施だが、展開結果に中間マクロ呼び出しが
一切残っていないことをコード上で確認済み):

```rust
// cargo expand の実際の出力 (抜粋)
let g = OrgChart::Graph::create(|__graphite_b| {
    let tanaka = __graphite_b.insert("tanaka", Employee { name: "田中".into(), id: 1 });
    let sato = __graphite_b.insert("sato", Employee { name: "佐藤".into(), id: 2 });
    let sales = __graphite_b.insert("sales", Department { name: "営業".into() });
    __graphite_b.belongs_to(tanaka.clone(), sales.clone());
    __graphite_b.belongs_to(sato.clone(), sales.clone());
    __graphite_b.boss(tanaka.clone(), sato.clone(), BossEdge { since: 2020 });
})
.expect("...");
```

`BossEdge { since: 2020 }` がユーザーの書いたトークンそのまま
(`__graphite_edge_OrgChart!(attrs boss { .. })` のような中間呼び出しを経由
せず) `.boss(..)` の第3引数に直接埋め込まれている。

rust-analyzer での実測 (2026-07-15、`orgchart_macro.rs` の
`tanaka -[boss = BossEdge { since: 2020 }]-> sato` 行):

| リテラル内トークン | 結果 |
|---|---|
| `since` → 定義 | ✅ ユーザー struct の `pub since: i32` に精密着地 (§1.7 の二段展開制約の解消を実測確認) |
| `BossEdge` → 定義 | ✅ ユーザーの `struct BossEdge` 宣言に精密着地 |
| `boss` → 定義 | ✅ schema の `-[boss: BossEdge]` の `boss` トークンに精密着地 |

これで DSL 内の全トークン種 (ノードキー・ノード型・エッジラベル・属性型・
属性フィールド) が定義解決可能になった。

既知の軽微な回帰 (実測で発見): エッジに使われない孤立ノード
(`suzuki = Employee { .. }` のみで辺を張らない) に rustc の
「unused variable: `suzuki`」警告が出る。v3 の脱糖
`let suzuki = __graphite_b.insert(..)` の束縛が後続で読まれないため。
孤立ノードは正当なグラフなので、生成する `let` に
`#[allow(unused_variables)]` を付けて抑制する (修正済み)。

同様に §1.6 で記録した「ハンドシェイクマクロが補完候補に露出する」副産物
(項目G4 再計測の表) も、ハンドシェイクマクロ自体が存在しなくなったため
構造的に解消した。G5 (同一ファイル制約) も同時に解消している (G5 節参照)。

## 1.9 スキーマ v4 実装後の再計測 (2026-07-17)

v4 (`docs/schema_v4.md`: 辺の第一級化・where 制約・Graph中心の種別API) 実装後、
`crates/graphite/tests/orgchart_macro.rs` で実測:

| 操作 | 結果 |
|---|---|
| schema `-[appointment: BossEdge]->` の積み荷型 → 定義 | ✅ ユーザーstructへ精密 |
| `node.boss_as_subordinate()` / マクロ外の `OrgChart::Boss { subordinate, superior, appointment }` 構築 / リテラルの `Boss(..)` → 定義 | ✅ いずれもschemaの `edge Boss` トークンへ精密着地する。辺種別は生成された名前付きフィールドの構造体として全文脈で解決される。 |
| リテラルのノードキー (`tanaka`)・積み荷フィールド (`since`)・辺キー束縛 (`tanaka_boss`) | ✅ v3 同様に精密 (let 束縛・式素通しの機構は v4 でも維持) |
| schema `Boss` → 参照検索 | ✅ 宣言 + 全使用 15 件 (アクセス・リテラル・素の構築・型注釈) |
| `where each subordinate` の `subordinate` → 定義 | ✅ 端点の役割名フィールドへ型検査文を生成し、宣言した役割名と同じトークンを検証コードへ補間する |
| `node.boss_as_subordinate()` の `boss_as_subordinate` 等、生成メソッド名トークン → 定義 | ✅ `2dce96a` で修正し実測確認済み: schema の `edge Boss` トークンへ精密着地 (修正前はマクロブロックに着地)。生成 fn ident に由来する Kind/ノード型トークンのスパンを付与 (G3 ポリシー適用) |

v4 の DSL 全トークン種 (辺種別・積み荷型・where 節端点・ノードキー・辺キー・
積み荷フィールド・生成関連関数名) が定義解決可能になり、参照検索も全使用を
検出する。**IDE 対応の目標 (「普通の Rust コードと同じように機能する」) は
v4 時点で達成**とみなす。以後は構文変更のたびに本節の形式で再計測を行う。

計測手順の注意 (再確認): rust-analyzer の再起動直後は生成型 (`Boss` 等) の解決が
数分間 [] を返すことがある。ユーザー struct が解決するのに生成型が解決しない場合は
故障ではなくインデックス途中 — 対照実験 (`OrgChart` 等) で切り分けてから待つこと。

## 1.10 端点宣言 v4.1 実装後の再計測 (2026-07-17)

v4.1 (`docs/edge_endpoints_v4_1.md`: 端点役割名・無向辺) 実装後、
`orgchart_macro.rs` / `undirected_edges.rs` で実測:

| 操作 | 結果 |
|---|---|
| `where each subordinate` の役割名 → 定義 | ✅ schema の `(subordinate: Employee)` の役割名トークンへ精密着地 |
| 使用側の役割アクセサ (`b.subordinate()`) → 定義 | ✅ 同上 (生成 fn ident が役割名トークンのスパンを持つ仕様どおり) |
| 無向リテラル `Friends(alice -- bob)` の `Friends` → 定義 | ✅ `edge Friends = Person -- Person` 宣言へ精密着地 |
| 同 `alice` → 定義 | ✅ ノードキー束縛へ (有向と同じ機構) |

役割名・無向辺とも、既存のスパン規約 (G3) に乗って初回実装から全導線が機能した。

## 1.11 flow! 実装後の計測 (2026-07-18)

`docs/flow_macro.md` (データフロー矢印) 実装後、`crates/graphite/tests/flow.rs` で実測:

| 操作 | 結果 |
|---|---|
| `-[double]->` の関数式 → 定義 | ✅ `fn double` の定義に精密着地 |
| チェーン形の中間束縛名 (`-> parsed ->` の `parsed`) → 定義 | ✅ 自分自身 (実在の let 束縛として機能) |
| fan-out の始点名 (別項の `parsed`) → 定義 | ✅ 前の項の束縛位置へ精密着地 (項をまたぐ名前解決) |

flow! も初回実装から全導線が機能 (式素通し + let 漏らしという graph! で実証済みの
機構の再利用のため)。

## 1.12 スプライス・v4.2 実装後の計測 (2026-07-18)

この表は当時の実測記録である。現行仕様では、既定IDをマクロがschema module内へ生成し、既存ID型は `(id: 型パス)` で明示する形へIssue #4で変更した。

| 操作 | 結果 |
|---|---|
| 使用側の `EmployeeId` → 定義 (v4.2) | ✅ ユーザー宣言の `pub struct EmployeeId(pub String);` へ精密着地 (v4.1 まではマクロ生成型としてスキーマにアンカーされていた。ユーザー宣言化により本物の struct 宣言に飛ぶ) |
| スプライス項 `..staff` の `staff` → 定義 | ✅ ローカル変数宣言へ精密着地 (式素通しの既存機構) |

## 1.13 NodeRef/EdgeRef 導入後のスパン適用 (2026-08-25)

第4段階 (NodeRef/EdgeRef 導入、コミット `1ca50e9` のレビュー是正) で、
NodeRef/EdgeRef の公開メソッド群にも既存の G3 ポリシー (`§1.9`) を適用した:
`NodeRef::id`/`value` はノード型トークン (`node Person;` の `Person`) の
スパンを、`EdgeRef::id`/`from`/`to`/`from_id`/`to_id`/`endpoints`/`payload`
は辺種別トークン (`edge Boss = ..;` の `Boss`) のスパンを、それぞれ
`Ident::new(名前, span)` で明示的に継承する。

issue #9 で `Graph` へ移した種別API (`{type}_by_id`/`{type}_value_mut`/
`{type}_ids`/`{type}_iter`/`{type}_len`、`{kind}_by_id`/`{kind}_payload_mut`/
`{kind}_ids`/`{kind}_iter`/`{kind}_len`) と、`NodeRef` へ移した
`{kind}_between`/`{kind}_try_between` にも同じ規約を適用する。名前は
`naming.rs` の `kind_api_method_ident(accessor, suffix)` が組み立て、
`accessor` がノード型トークンまたは辺種別トークンのスパンを引き継ぐ。
§1.15 以降、公開APIは通常のRustファイルであり、定義ジャンプは生成ファイルの
実装行へ着地する。スパン継承は展開経路が残る場合の保険として維持する
(実測は §1.15 参照)。

**検証範囲**: rust-analyzer 実機での F12 (go to definition) 再計測は
今回実施していない。代わりに、対象メソッドをわざと誤った引数数で呼び出し
`E0061` を発生させ、rustc が出す「note: method defined here」の位置
(rust-analyzer の definition provider と同じスパン情報を使う診断) が
狙った宣言トークンへ一致することを確認した (`EdgeRef::from_id` →
`edge Boss = ..;` の `Boss`、`NodeRef::value` → `node Person;` の
`Person`。ワークスペース外の使い捨てクレートで実施、検証用コードは
リポジトリに残していない)。rustc の定義スパン表示では検証済みだが、
rust-analyzer 実機での確認は未実施のままである。

## 1.14 graph! の名前付き静的アクセサ (2026-08-25)

`graph!` 左辺の識別子は、builder内の構築用ID束縛だけでなく、呼び出し箇所に
生成する名前付きラッパーの固有メソッド名にも同じ `Ident` を使う。したがって
`alice = Person { .. }` の `alice` と、完成後の `graph.alice()` は同じsource
tokenを起点にrename・参照検索できる。日本語識別子も文字列から再構成せず、入力の
`Ident` をそのまま使う。名前付きラッパーの型名、位置フィールド名、型引数名の
生成規則は `naming.rs` に集約し、各 `graph!` 展開をブロックスコープで閉じるため、
同じ関数内で複数回展開してもローカル生成型が衝突しない。

**検証範囲**: §1.13 と同じく、rustc の定義スパン表示 (誤った引数数で呼び出して
`E0061` の「note: method defined here」位置を確認する手法) で確認済みであり、
rust-analyzer 実機での F12 (go to definition) / rename 再計測は今回実施して
いない。

## 1.15 通常のRust生成ファイルへの定義ジャンプ (2026-08-26)

schemaに由来する公開APIは、手続き型マクロの展開だけに置かず、`src/generated/`
または`tests/generated/`の通常のRustファイルへ生成する。利用コードから
NodeRef・EdgeRefのメソッド、役割アクセサ、`Graph`の種別API
(`{type}_by_id`・`{kind}_iter`・`{kind}_ids`・`{kind}_len` 等)、
`NodeRef`の`{kind}_between`・`{kind}_try_between`へ定義ジャンプした場合、
生成ファイル内の実装へ着地することを受理条件とする。生成ファイル先頭の「生成元」から元DSLのファイルと行へ
戻れる。着地した要素そのものからも戻れるよう、生成する公開型と公開メソッドの doc は
「宣言: `<宣言元ファイルのパッケージ相対の綴り>` の `<宣言の形>`」の段落を持つ
(issue #17、書式と対応表は `docs/desugaring_reference.md` §26.6)。

`graph!`が作る名前付きラッパーは呼び出し箇所ローカル型なので通常ファイルへ
事前生成しない。`graph.alice()`の定義情報は左辺`alice`のスパンへ結び付ける。
この例外はschemaに由来する公開APIへ広げない。生成と陳腐化検出の規約は
`docs/code_generation.md`を参照する。

**実測記録 (2026-08-26 レビュー)**: 次の7メソッドについて、定義ジャンプが
`crates/graphite/tests/generated/traversal_api_traversal.rs`
(schema `Traversal`) の実装へ着地することを確認した。いずれも生成ファイルの
実装そのものへ着地し、schema宣言トークンへは戻らない。

| メソッド | 着地する要素 |
|---|---|
| `person_by_id` | `impl Graph` の `person_by_id` の定義 |
| `person_value_mut` | `impl Graph` の `person_value_mut` の定義 |
| `person_len` | `impl Graph` の `person_len` の定義 |
| `purchase_iter` | `impl Graph` の `purchase_iter` の定義 |
| `purchase_as_buyer` | `impl<'graph> PersonRef<'graph>` の `purchase_as_buyer` の定義 |
| `関係_try_between` | `impl<'graph> PersonRef<'graph>` の `関係_try_between` の定義 |
| `関係_between` | `impl<'graph> PersonRef<'graph>` の `関係_between` の定義 |

着地行の行番号は記録しない。生成ファイルの行は再生成のたびに動くため、記録した
数値は次の再生成で実物とずれ、検証手段として機能しなくなるためである
(実際に issue #17 の再生成で全行がずれた)。着地位置を確かめるときは、下の
「着地位置の計測方法」でそのつど実測する。

### 着地位置の計測方法

1. ワークスペースの外に使い捨てのクレートを作り、`graphite` をパス依存で参照する。
   ワークスペースの中に作ると、次の手順でわざと起こすコンパイルエラーによって
   `cargo build` と `cargo test --workspace` が常に失敗するようになる。
2. 使い捨てクレートのディレクトリで、対象メソッドをわざと誤った引数数で呼び出す
   コードを書き、`cargo build` を実行して `E0061` を起こす。
3. rustc が出す「note: method defined here」が指すファイルと要素を読む。これは
   rust-analyzer の definition provider と同じスパン情報を使う診断である。
4. 上表の「着地する要素」と一致することを確かめる。確かめ終えたら使い捨ての
   クレートは消す。

この手順は rustc の定義スパン表示による確認であり、rust-analyzer 実機での F12
(go to definition) の再計測ではない。

## 1.16 静的グラフの受理マトリクス (issue #41 段階5、2026-09-23。instance展開のimpl撤去に伴い2026-09-24に再測定。PR #45レビューA・C・D・Fの構築境界の刷新に伴いさらに再測定。PR #45検収の指摘1〜3・7の是正 (コミット`bb551a4`) に伴いさらに再測定。`Graph`・`NodeRefs`・`EdgeRefs`のフィールド非公開化に伴いさらに再測定。構築の唯一の入口`construct!`への統合に伴いさらに再測定。値マクロへのinstance印付与・`{種別}Edge`概要文修正に伴いさらに再測定)

静的グラフ (`static_graph_schema!`/instance) の公開生成APIも動的グラフと同じ生成ファイル方式へ移した (issue #41 段階1〜4)。
本節は、issue #41 の「検証」節が定めるチェックリスト (A分類9項目・B分類8項目の計17項目。A分類の9項目には、instanceの辺種別からschemaへの追跡1項目を含む) を、rust-analyzer実機で実測した記録である。

**計測方法**: この節の実測を行ったセッションは、vscode-lsp-mcpの`go_to_definition`で`examples/static-org/src/main.rs` (rust-analyzer.linkedProjectsに登録済み) の各識別子からF12を実行し、着地した生成ファイル (`examples/static-org/src/generated/*.rs`) の位置をReadで確認した。
このセッションは、GitHub issue #41 のコメント「rust-analyzer 実機の実測と、実装方式の決定」が記録した制約と同じく、中継の道具がGraphite以外の型 (`String`等) でも中身を空で返す制約のため、hoverを実機で測れなかった (この制約は本節が本文書で初めて記録するものであり、§1.13・§1.15はF12のみを扱いhoverには触れていない)。
その代わりに、このセッションは、F12の着地先に付いた doc コメント (意味カード) をReadで読み、その内容をhoverの代替として記録した (rust-analyzerはこの doc コメントをそのままhoverへ表示するため、内容としては等価である)。
このセッションは、計測前後で`get_diagnostics`が0件であることを確認し、rust-analyzerが最新のソースを読み込んでいることを確かめた。

**2026-09-24の再測定 (PR #45)**: PR #45のレビューが、値ありの個体・積み荷を`Nodes::new`/`Edges::new`から差し替えられる穴 (レビューA)・別の`Nodes`から作った`Edges`を組み合わせられる穴 (レビューC)・組み立て関数が利用者の名前空間へ漏れる問題 (レビューD)・値の式からローカル変数を参照できない問題 (レビューF) を指摘し、構築境界を全面的に刷新した。旧・組み立て関数 (`{グラフ名}の個体を組み立てる`・`{グラフ名}の辺を組み立てる`、その場展開の出力でinstance宣言そのものへ着地していたA分類2項目) は廃止し、代わりに生成ファイルの中に構築の入口 (`construct::nodes!`・`construct::edges!`、固定語彙のB分類) を置いた。`Nodes::new`/`Edges::new`はC分類の内部専用構築子 (`__graphite_internal_new`) へ降格し、公開契約から外れた。`examples/static-org/src/main.rs`の該当箇所を新しいAPI (`開発チーム::construct::nodes!(..)`・`開発チーム::construct::edges!(&nodes)`・`開発チーム::Graph::new(&edges)`) へ書き換えたうえで、下表の`construct::nodes!`・`construct::edges!`・`Nodes`・`Edges`・`Graph::new`の5行を`go_to_definition`で再実測した (`get_diagnostics`で警告0件を確認済み)。他の行は、DSL宣言・生成ファイルの構造そのものは変わっていないため番号だけを機械的に補正した。

**2026-09-24の再測定 (コミット`bb551a4`、PR #45検収の指摘1〜3・7の是正後)**: 上記の再測定の後、PR #45検収の指摘2 (内部構築子`__graphite_internal_new`への`#[deprecated]`付与。可視性だけでは「呼べるのは`construct::nodes!`だけ」を強制できないための追加の柵)・指摘3 ({個体名}Ref・{辺名}Refの配線フィールド`entity`/`nodes`/`edges`を`pub(super)`から非公開へ変更。親moduleから構造体リテラルで別の`Nodes`を混ぜた不整合な参照を組み立てられた穴を閉じた) がコミット`bb551a4`で入った。指摘1 (値マクロから`pub(crate) use`を外し、呼び出し位置と同じテキスト順スコープへ閉じた) は本節が計測する識別子の着地先そのものは変えない。同コミットは`examples/static-org/src/main.rs`に関数`辺を組み立てる`を新設し、95行目の直後へ8行を挿入したため、96行目以降の行番号は旧測定からすべて+8だけ後ろへずれた。このセッションは、下表のA分類9項目・B分類10項目の計19項目全てについて、main.rsと生成ファイルの現在の識別子出現位置を実測し直し (`find_symbol`によるシンボル名検索ではなく、行内の実際の出現位置を数え上げてから`go_to_definition`で着地先を実測する方法)、`get_diagnostics`で警告0件・エラー0件を確認した (唯一出る警告は本コミットで新設した`辺を組み立てる`関数がmain()から呼ばれないことによる`dead_code`であり、`tests.rs`からは呼ばれているので実装上の欠陥ではなく、本節の計測結果にも影響しない)。19項目は全て着地に成功し、測れなかった行は無い。

**2026-09-24の再測定 (`Graph`・`NodeRefs`・`EdgeRefs`のフィールド非公開化後)**: PR #45最終検収は、`Graph`・`NodeRefs`・`EdgeRefs`のフィールドが`pub`のままなので、2つのグラフの部品を構造体リテラルで混ぜた不整合な`Graph`を組み立てられる穴を指摘した。この是正で、3つの型のフィールドをすべて非公開にし、読み出しを`Graph::node_refs()`/`Graph::edge_refs()`、`NodeRefs`/`EdgeRefs`が個体名・辺名をそのまま名前にして持つ読み出し専用メソッド (`g.node_refs().太郎()`のように呼ぶ) へ変えた。main.rsの呼び出し側 (`g.node_refs.太郎`のようなフィールドアクセスの箇所全て) も新しいメソッド呼び出しの形へ書き換えたが、この書き換えは既存の行を書き換えただけで行の増減が無いため、main.rs側の行番号はこのセッションでは動いていない。動いたのは生成ファイル側の`NodeRefs`・`EdgeRefs`・`Graph`・`construct`の各定義位置であり (フィールドの`pub`+doc除去とメソッド本体の追加が相殺し合わず全体として増加した)、下表の`g.node_refs.太郎`・`g.edge_refs.太郎の所属`・`NodeRefs`・`EdgeRefs`・`Graph::new`・`node_refs`・`edge_refs`・`construct::nodes!`・`construct::edges!`の9行を`go_to_definition`で再実測した (`get_diagnostics`で警告0件・エラー0件を確認済み)。他の10行 (太郎Ref・太郎の所属Ref・role/payloadアクセサ・所属Edge・instance辺種別・Nodes・Edges・entity) は、この是正で本文が変わっていない生成ファイルの先頭寄りの範囲 (指紋定数を除く) にあるため、`git diff`で無変更を確認したうえで番号を据え置いた。

### A分類 (利用者語彙から派生した識別子)

F12起点の列は、main.rsに識別子が直接出現する行が無い3項目 (`太郎の所属Ref`・`NodeRefs`・`EdgeRefs`) では、生成ファイル自身の中で識別子が実際に使われている行 (`find_symbol`によるシンボル名検索ではなく、`go_to_definition`が使う定義プロバイダを通した実測) を起点にしている (旧`所属Edge`行は`instance辺種別`所属`→schema`行へ統合したため、この一覧から外れた)。
生成ファイルも通常のRustソースであり、rust-analyzerはmain.rsの利用箇所と同じ仕組みで定義ジャンプを解決するため、これは正当なF12実測である。

| 識別子 | F12起点 | F12の着地先 | 意味カードの要約 | 合否 |
|---|---|---|---|---|
| `太郎Ref` | `impl<'a> 開発チーム::太郎Ref<'a>`の型 (main.rs:83、再実測: 2026-09-24 construct!統合) | `generated/開発チーム.rs`の`struct 太郎Ref<'a>`定義 (11〜21行目、0始まり) | 「Graphite 静的グラフの具体個体参照。graph: 開発チーム / 個体: 太郎 / 実体型: 社員」+ 宣言: `node 太郎: 社員 = ..` | 合格 |
| `太郎の所属Ref` | `pub fn 太郎の所属(&self) -> 太郎の所属Ref<'a>`の戻り値型 (`generated/開発チーム.rs:44`、再実測: 2026-09-24 construct!統合) | `generated/開発チーム.rs`の`struct 太郎の所属Ref<'a>`定義 (299〜311行目、0始まり) | 「具体辺参照。graph/具体辺/辺種別」+ 宣言: `edge 太郎の所属 = 所属(太郎 -> 開発部)` + 関係する schema 宣言: `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1` | 合格 |
| `g.node_refs().太郎()` | `g.node_refs().太郎()` の `太郎` (main.rs:106、再実測: 2026-09-24 construct!統合) | `generated/開発チーム.rs`の`impl<'a> NodeRefs<'a>`の`太郎`メソッド (600〜608行目、0始まり) | 「個体参照メソッド。NodeRefsがこのメソッドでこの個体の具体参照を返す」+ 宣言: `node 太郎: 社員 = ..` | 合格 |
| `g.edge_refs().太郎の所属()` | `g.edge_refs().太郎の所属()` の `太郎の所属` (main.rs:111、再実測: 2026-09-24 construct!統合) | `generated/開発チーム.rs`の`impl<'a> EdgeRefs<'a>`の`太郎の所属`メソッド (646〜656行目、0始まり) | 「辺参照メソッド。EdgeRefsがこのメソッドでこの具体辺の具体参照を返す」+ 宣言: `edge 太郎の所属 = 所属(太郎 -> 開発部)` | 合格 |
| `太郎の参照.太郎の所属()` | `.太郎の所属(` (main.rs:110、再実測: 2026-09-24 construct!統合) | `impl<'a> 太郎Ref<'a>`の`太郎の所属`メソッド (31〜47行目、0始まり) | 「具体辺参照を返す。個体/具体辺/辺種別/この個体の役割/戻り値」+ 宣言: instance edge + 関係する schema 宣言: schema edge | 合格 |
| `.member()`/`.team()` | `.team(` (main.rs:110)・`.member(` (main.rs:111) の両方を実測 (再実測: 2026-09-24 construct!統合) | `impl<'a> 太郎の所属Ref<'a>`の`team`メソッド (328〜342行目、0始まり)・`member`メソッド (313〜327行目、0始まり) | 「端点の役割アクセサ。辺種別/役割/具体辺/具体端点/戻り値/検証制約」+ 宣言: schema edge + 関係する instance 宣言: instance edge (両メソッドとも同型) | 合格 |
| payload accessor (`.任命()`) | `.任命(` (main.rs:108、再実測: 2026-09-24 construct!統合) | `impl<'a> 太郎の上司Ref<'a>`の`任命`メソッド (478〜489行目、0始まり) | 「積み荷アクセサ。辺種別/積み荷/具体辺」+ 宣言: schema edge + 関係する instance 宣言: instance edge | 合格 |
| instance辺種別`所属`→schema (兼`所属Edge`) | `edge 太郎の所属 = 所属(太郎 -> 開発部);` の中間の`所属` (main.rs:71、変更なし) | `generated/組織.rs`の`struct 所属Edge<'a>`定義 (11〜19行目、0始まり。DSLトークンの錨、`docs/static_graph.md`「追跡の契約」) | 「辺種別を表す型アンカー。端点の役割の形を示す (どのinstanceもこの型を構築しない)。辺種別: 所属」+ 宣言: `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1` | 合格 (旧`所属Edge`行が単独で持っていた起点`entity: &'a 組織::所属Edge<'a>`は、辺の実体が端点参照を持たなくなり生成ファイルから消えたため、この1行へ統合した。概要文は、`所属Edge`がどのinstanceからも構築されない型アンカーであることを明示する形へ改めた) |

### B分類 (Graphiteが定義する固定語彙)

構築の入口が単一の`construct!`へ統合されたため (旧: `construct::nodes!`/`construct::edges!`の2項目)、個体実体・積み荷の所有者が独立型を持たなくなったため (旧: `Nodes`/`Edges`の2項目)、`Graph`を構築する固定語彙が単一のマクロになったため (旧: `new (Graph::new)`の1項目) の3つの理由により、B分類は10項目から7項目へ減った。代わりに、main.rsへ新しく直接出現するようになった`Graph`型そのもの (`グラフを組み立てる() -> 開発チーム::Graph`の戻り値型注釈) を新規の1項目として加えた。

| 識別子 | F12起点 | F12の着地先 | 意味カードの要約 | 合否 |
|---|---|---|---|---|
| `construct!` | `開発チーム::construct!(` の呼び出し (main.rs:100、再実測: 2026-09-24 construct!統合) | `generated/開発チーム.rs`の`macro_rules! construct`定義 (765〜783行目、0始まり。doc付きの項目全体。instance印を値マクロ名へ混ぜる是正でマクロ本体が1行伸びたため783行目へ+1) | 「`Graph`を実体化するマクロ`construct`。graph: 開発チーム / 実行時に渡す個体 (宣言順): `開発部: 部署` / 戻り値: Graph」+ 固定語彙: `construct!` + 関係する instance 宣言: `graph 開発チーム` | 合格 |
| `Graph` (型) | `-> 開発チーム::Graph`の戻り値型注釈中の`Graph` (main.rs:99、新規測定: 2026-09-24 construct!統合) | `generated/開発チーム.rs`の`struct Graph`定義 (713〜725行目、0始まり。個体・積み荷を直接持つがフィールドは非公開) | 「具体グラフ本体 `Graph` (Graphiteの固定語彙)」+ 固定語彙: `Graph` | 合格 |
| `NodeRefs` | `pub fn node_refs(&self) -> NodeRefs<'_>`の戻り値型 (`generated/開発チーム.rs:754`、再実測: 2026-09-24 construct!統合) | `generated/開発チーム.rs`の`struct NodeRefs<'a>`定義 (591〜598行目、0始まり) | 「個体参照の集まり `NodeRefs` (Graphiteの固定語彙)」+ 固定語彙: `NodeRefs` | 合格 |
| `EdgeRefs` | `pub fn edge_refs(&self) -> EdgeRefs<'_>`の戻り値型 (`generated/開発チーム.rs:762`、再実測: 2026-09-24 construct!統合) | `generated/開発チーム.rs`の`struct EdgeRefs<'a>`定義 (637〜644行目、0始まり) | 「辺参照の集まり `EdgeRefs` (Graphiteの固定語彙)」+ 固定語彙: `EdgeRefs` | 合格 |
| `entity` | `.entity()` (main.rs:107、再実測: 2026-09-24 construct!統合) | `impl<'a> 次郎Ref<'a>`の`entity`メソッド (112〜119行目、0始まり) | 「具体個体参照から実体を取り出す `entity` (Graphiteの固定語彙)」+ 固定語彙: `entity` | 合格 |
| `node_refs` | `g.node_refs()` の`node_refs` (main.rs:106、再実測: 2026-09-24 construct!統合) | `impl Graph`の`node_refs`メソッド (748〜755行目、0始まり) | 「`Graph`が個体参照の集まりを返すメソッド `node_refs` (Graphiteの固定語彙)」+ 固定語彙: `node_refs` | 合格 |
| `edge_refs` | `g.edge_refs()` の`edge_refs` (main.rs:111、再実測: 2026-09-24 construct!統合) | `impl Graph`の`edge_refs`メソッド (756〜763行目、0始まり) | 「`Graph`が辺参照の集まりを返すメソッド `edge_refs` (Graphiteの固定語彙)」+ 固定語彙: `edge_refs` | 合格 |

### 結果

A分類8項目 (instance辺種別からschemaへの追跡と`所属Edge`を1行へ統合した項目を含む)・B分類7項目 (`construct!`への統合で新設した1項目、`Graph`型そのものの新規測定1項目を含む) の合計15項目すべてが合格した。
このセッションは、issue #41本文が2026-09-23付のコメントで記録した先行実測 (生成ファイル化前、B分類8項目全滅・A分類の一部も生成塊全体への着地) と比べ、段階1〜4の生成ファイル化によってB分類が0/8→8/8へ改善したことを確認した。
2026-09-24のPR #45再測定で、旧・組み立て関数2項目 (A分類、instance宣言そのものへ着地していた) を廃止し、代わりに構築の入口`construct::nodes!`・`construct::edges!`をB分類の固定語彙として生成ファイルへ置いた。この2項目は他のB分類と同じく生成ファイルへ着地するため、「その場展開の出力は例外」という以前の注記は解消した (`docs/static_graph.md`「追跡の契約」参照)。
2026-09-24のコミット`bb551a4`(PR #45検収の指摘1〜3・7の是正) 後の再測定でも19項目全てが合格を維持した。この是正は`{個体名}Ref`・`{辺名}Ref`の配線フィールド (`entity`/`nodes`/`edges`) を`pub(super)`から非公開へ変え、内部構築子`__graphite_internal_new`へ`#[deprecated]`を付けたが、いずれも公開契約 (A分類・B分類の識別子そのもの) には触れていないため、着地先の識別子・意味カードの内容は変わらない。変わったのは`examples/static-org/src/main.rs`に関数`辺を組み立てる`が新設されたことによる行番号のずれ (96行目以降が旧測定から+8) と、それに伴い書き直した生成ファイル側の行範囲だけである。測れなかった行は無い。

`Graph`・`NodeRefs`・`EdgeRefs`のフィールド非公開化後の再測定でも19項目全てが合格を維持した。この是正は`g.node_refs.太郎`のようなフィールドアクセスを`g.node_refs().太郎()`のようなメソッド呼び出しへ変え (公開契約の名前自体は`node_refs`/`edge_refs`/個体名/辺名のまま変わらない)、`NodeRefs`・`EdgeRefs`・`Graph`のフィールドを非公開にした。生成ファイルの中の`NodeRefs`・`EdgeRefs`・`Graph`・`construct`の各定義がこの変更の影響を受けて位置を変えたため、この4識別子に関わる9行を再実測した。測れなかった行は無い。

構築の唯一の入口`construct!`への統合後の再測定では、A分類8項目・B分類7項目の合計15項目すべてが合格した。個体実体・積み荷の所有者(`Nodes`/`Edges`)が消えたことに伴い、B分類から`construct::nodes!`・`construct::edges!`・`Nodes`・`Edges`・`new (Graph::new)`の5項目が消え、統合後の`construct!`1項目と新規測定の`Graph`型1項目に置き換わった (10項目→7項目、差引3項目減)。A分類は、`所属Edge`が単独で持っていた起点 (`Edges`構造体のフィールド型参照) が辺の実体から端点参照が消えたことで丸ごと無くなったため、`所属Edge`行を`instance辺種別`所属`→schema`行へ統合した (9項目→8項目、差引1項目減。着地先`generated/組織.rs`の`所属Edge`定義、11〜19行目自体は無変更)。他のA分類7項目は、main.rsの行数減少に伴う行番号のずれと、`Graph`構造体を直接参照する形へ変わった生成ファイル側の内部構造の変化を反映して、全項目を`go_to_definition`で実測し直した。測れなかった行は無い。

**2026-09-24の再測定 (構築の唯一の入口`construct!`への統合、オーナーの追加レビュー対応):** オーナーは、`construct::nodes!`→`construct::edges!(&nodes)`→`Graph::new(&edges)`という3段階の公開構築APIを、生成コード内部の構築手順の漏れ出しと見なし、「静的グラフを実体化する」1操作の単一の入口への統合を求めた。この対応で、個体実体・積み荷の所有者 (`Nodes`/`Edges`) を独立型として持つのをやめ、`Graph`自身の非公開フィールドへ統合した (辺の実体はもう端点個体への参照を保持せず、各具体辺の端点はinstance宣言の時点で確定した個体名を使って`{辺名}Ref`のロールアクセサが`Graph`から直接読む。これにより`Graph`が個体・積み荷を直接所有しても自己参照にならない)。構築の入口は`{instance名}::construct!(..)`という単一のマクロへ統合し、`Nodes`・`Edges`という名前・型は公開契約からもソースコードからも消えた。
この統合により、`examples/static-org/src/main.rs`の呼び出し側 (`ノードを組み立てる`・`辺を組み立てる`の2関数と3行の組み立て) は`グラフを組み立てる`という1関数・1行 (`開発チーム::construct!(部署 { .. })`) へ書き換わり、ファイル全体の行数が減った。これに伴い、A分類・B分類のうちmain.rsの行番号に依存する項目は軒並みずれ、B分類は`construct::nodes!`/`construct::edges!`の2項目が`construct!`の1項目へ、`Nodes`/`Edges`/`new (Graph::new)`の3項目が`Graph`(型そのもの)の1項目へ、それぞれ統合・置換された。このセッションは、下表のA分類8項目・B分類7項目の計15項目全てについて、`get_diagnostics`で警告0件・エラー0件を確認したうえで、main.rsと生成ファイルの現在の識別子出現位置を`go_to_definition`で実測し直した。15項目は全て着地に成功し、測れなかった行は無い。

**2026-09-24の再測定 (PR #45検収の指摘1・3是正):** 検収は2点を指摘した。(1) 値マクロの名前がグラフ名だけに由来するため、別moduleが同じグラフ名のinstanceを作ると、`construct!`が呼び出し位置から見える別instanceの値マクロ・`Graph`を無言ですり替える穴があった。この是正は、値マクロの名前 (`__graphite_values_{グラフ名}!`・`__graphite_payloads_{グラフ名}!`) へ、`generated = "..."`文字列から計算する`instance印`を追加した (`__graphite_values_{グラフ名}_{instance印}!`)。(2) `{種別}Edge`の概要文が「辺値。端点への参照を保持する」と書いており、どのinstanceも構築しない型アンカーである実態と食い違っていた。この是正は概要文を「辺種別を表す型アンカー。端点の役割の形を示す (どのinstanceもこの型を構築しない)」へ改めた。このセッションは、`vscode-lsp-mcp`への接続を試みたが`ws://127.0.0.1:16598`への接続が拒否され、`go_to_definition`による実測ができなかった。`instance印`の付与は`construct!`定義 (765〜783行目、0始まり) の本体が1行伸びる変化だけを`generated/開発チーム.rs`に与え (それより前の行は無変更)、`所属Edge`等の概要文変更は`generated/組織.rs`側の該当doc行を1行対1行で置き換えるだけで行数を変えないため、`git diff`によるテキスト差分の直接確認で行番号への影響が無い (または`construct!`行だけ+1) ことを確かめ、上表の該当行を手動で補正した。`go_to_definition`による実測は次にLSPへ接続できるセッションへ持ち越す。

「実装追跡」(issue #41本文の3つの追跡のうち3つ目) は生成ファイルそのものが
正式経路であり、上表のF12の着地先がそのまま実装追跡を兼ねる。`cargo expand`は
その場展開に残る部分 (指紋照合・DSLトークンの型参照・値マクロ) を読むための
補助である (`docs/static_graph.md`「実装追跡の正式経路」参照)。

### hoverの`宣言:`段落規則への追記

§1.15で定めた「宣言: `<宣言元ファイルのパッケージ相対の綴り>` の `<宣言の形>`」
の段落規則を、静的グラフ向けに2つ拡張する (`crate::static_graph::trace`が
組み立てる意味カード、書式は`docs/static_graph.md`「追跡の契約」参照)。

- **固定語彙の生成物**は「宣言:」段落を持たず、代わりに「固定語彙: `<名前>`
  (`docs/static_graph.md` 「生成される名前の公開契約」)」という1行を持つ
  (由来がGraphite言語仕様自体であり、特定の利用者トークンを偽の宣言元として
  示さないため。issue #41本文「B. Graphiteが定義する固定語彙」参照)。
- **schemaとinstanceの両方に由来する生成物**(role/payloadアクセサ・具体辺参照
  等) は、「宣言:」に加えて「関係する schema 宣言: `<...>`」または「関係する
  instance 宣言: `<...>`」の段落を持つ。1つのspanでは表現できない、2つの宣言
  から合成された意味を説明するためである (issue #41本文「hover/docは合成され
  た意味を説明する」節)。

## 2. 仕様項目

### G1: `graph!` ノードキーの let 束縛化 (実装対象)

> 以下はG1を導入した時点の展開設計を保存した記録である。名前付き静的
> アクセサ導入後の現行展開 (`create_named` / `insert_named`) は §1.14 と
> `crates/graphite-macros/src/instance_codegen.rs` を参照する。

G1導入前の展開はキー識別子をその場で文字列化していた:

```rust
b.employee(EmployeeId("tanaka".to_string()), Employee { .. });
b.belongs_to(EmployeeId("tanaka".to_string()), DepartmentId("sales".to_string()));
```

これを、ノードキーごとに 1 つの `let` 束縛を作り、以後は識別子参照で運ぶ形に変える:

```rust
OrgChart::Graph::create(|__graphite_b| {
    // (1) ノード宣言 (記述順)
    let tanaka = EmployeeId("tanaka".to_string()); // ← `tanaka` はノード宣言の出現スパン
    __graphite_b.employee(tanaka.clone(), Employee { .. });
    let sales = DepartmentId("sales".to_string());
    __graphite_b.department(sales.clone(), Department { .. });
    // (2) エッジ (記述順)
    __graphite_check_edge_OrgChart!(belongs_to);
    __graphite_b.belongs_to(tanaka.clone(), sales.clone()); // ← 各識別子はエッジ内の出現スパン
})
```

これにより rust-analyzer 上で:
- エッジ内キー → ノード宣言への定義ジャンプ
- キーの rename (リテラル内全出現の一括変更)
- キーの参照検索・hover での型表示 (`tanaka: EmployeeId`)

が全て「普通のローカル変数」として機能する。

設計上の注意:
- **builder 変数の改名**: クロージャ引数を `b` から `__graphite_b` に変える。
  ユーザーが `b: Employee { .. }` というノードキーを書いた場合に、生成する
  `let b = ..` が builder を隠してしまう衝突を避けるため (proc macro の入力
  トークンは call site ハイジーンなので、名前が同じなら本当に衝突する)。
- **並べ替え**: `graph!` は従来エッジとノードの記述順が自由 (キー逆引き表を
  先に作るため)。let 束縛は使用より前に必要なので、展開は「全ノード → 全エッジ」
  の 2 段に並べ替える。builder の検証は freeze 時なので意味論は変わらない。
  `(0..*)` エッジ同士の記述順保持 (項目i の仕様) はエッジ列内の順序なので影響なし。
- **スパン規約**: `let` の束縛識別子はノード宣言の出現スパン、エッジ内の参照は
  各エッジでの出現スパンを使う。これがジャンプの起点/終点の正確さを決める。
- **既存診断の維持**: 重複キー診断 (項目h)・未宣言キー参照診断は現行のまま。
- `.clone()` のコストはリテラル構築時のみで、キーは短い String。原則5
  (ゼロコスト志向: 手書きと同形) の範囲内と判断する。

### G2: examples を rust-analyzer に解析させる (実装対象)

examples/* は意図的にルート workspace から除外したスタンドアロンクレート
(スタンドアロン利用の実証のため。この構成自体は変えない)。rust-analyzer には
`.vscode/settings.json` の `rust-analyzer.linkedProjects` で明示的に教える:

```json
{
  "rust-analyzer.linkedProjects": [
    "Cargo.toml",
    "examples/build-pipeline/Cargo.toml",
    "examples/org-analyzer/Cargo.toml",
    "examples/dialogue-engine/Cargo.toml"
  ]
}
```

`.vscode/settings.json` はリポジトリにコミットする (このリポジトリでは IDE 挙動
そのものが検証対象なので、エディタ設定も再現可能であるべき)。README に
「VSCode で開くと examples も解析される」旨を一行追記。今後 example を増やす
ときはここに 1 行足す、を運用ルールにする (README と proc-macro-dev スキルに記載)。

### G3: スパンポリシーの明文化 (ドキュメント対象)

計測で確認できた事実をポリシーとして `.claude/skills/proc-macro-dev/SKILL.md` に
固定する:

- 生成する識別子は必ず「由来するユーザートークンのスパン」を持たせる。
  型名・フィールド名 → `decl.name` 系、エッジ派生名 → `edge.label`。
- `format_ident!` は最初に補間された `Ident` のスパンを継承する (実測で確認済み)。
  補間元が `String` や `&str` になる場合はこの継承が働かないので、`span = ..` を
  明示すること (例: `to_pascal_case` した文字列から作る `{Label}Attrs` 型名)。
- 新しいコード生成を足したら、rust-analyzer の definition provider で
  `targetSelectionRange` がユーザートークンに着地することを確認する。

### G4: エラー回復展開 (根本課題・次フェーズの実装対象)

現状、DSL 入力のどこか 1 箇所でもパースに失敗すると `syn::Error` →
`compile_error!` だけが展開され、**生成型が全て消える**。利用側のコードが
一斉に赤くなり、rust-analyzer の補完 (カーソル位置に仮識別子を入れて
speculative expansion する方式) も、仮識別子入りの入力をパーサが拒否する限り
機能しない。「編集途中はほぼ常にパース不能」なので、これは IDE 体験の
根本問題である。

方針: **宣言単位の回復型パーサ** に変える。

- `dynamic_graph_schema!`: `schema { .. }` ボディを宣言 (node/edge) 単位で読み、壊れた
  宣言はその宣言のスパンで `compile_error!` を蓄積しつつ次の宣言境界
  (`;` / ブロック終端) までスキップする。パースできた宣言だけで通常のコード
  生成を行い、`compile_error!` 群を併記する。
  - 壊れたノードを参照するエッジは、そのエッジも生成対象から外す
    (未知ノード参照エラーの二次噴出を避ける。ただし compile_error! は
    元の壊れた宣言の 1 件だけを出す)。
- `graph!`: 項目 (ノード宣言 / エッジ) 単位で同じ回復を行う。
- 期待効果: (1) 編集中も既存の生成型・アクセサが生き続け、利用側が全滅しない。
  (2) 補完の speculative expansion が「壊れた 1 宣言を捨てて残りを展開」できる
  ようになり、graph! 内のフィールド名補完・型名補完が機能する余地が生まれる。
- 検証: 実装後に (a) trybuild でエラー併記+部分生成のスナップショット、
  (b) vscode-lsp-mcp で「schema の 1 宣言を壊した状態でも利用側の別宣言由来の
  診断が出ない」ことを実測する。

**2026-08-26 更新 (schemaの回復展開の担当替え)**: schemaの公開APIを通常の
Rustファイルへ生成する形へ移したため、`dynamic_graph_schema!` 自体はコードを展開せず、
検証と指紋照合だけを行う。壊れた宣言があれば蓄積した診断を全件返し、生成は
行わない。編集途中でも利用側が生き続ける性質は、生成ファイルが前回の生成内容の
まま残ることで保たれる。宣言単位の回復展開そのものは
`graphite_codegen::expand_inline_for_test` に残り、`#[doc(hidden)]` の
`graphite::__dynamic_graph_schema_inline_for_test!` を通じて `tests/ui/*.rs` の
compile-fail テストが検査する。この入口は診断テスト専用であり、利用者向けの
経路ではない。`graph!` 側の回復は変更していない。

### G5: `graph!` ↔ `dynamic_graph_schema!` 同一ファイル制約 (v3 で解消済み)

**2026-07-14/15 更新: `docs/history/graph_literal_v3.md` の実装により、この制約自体が
構造的に消滅した。** 以下は制約が存在していた当時 (構文 v0〜v2) の記録として
残す。

当時の状況: 未知エッジラベル診断のハンドシェイク
(`__graphite_check_edge_{Schema}!`) は `macro_rules!` のテキストスコープに
依存するため、schema と graph! が同一ファイル (正確には同一スコープで schema
が先) にないと機能しなかった。検討した選択肢:

- (a) `#[macro_export]` を付ける — マクロがクレートルートに強制輸出される。
  `graph!` 側は schema がどのクレートにあるか知らないためパス解決できず、
  別クレートの schema には結局効かない。名前空間汚染も原則6に反する。
- (b) ハンドシェイクを廃止し、`b.{label}` の method-not-found に任せる —
  クロスファイルで動くが、診断品質が大きく落ちる (利用可能エッジ一覧が出ない)。
- (c) 現状維持 + 制約の文書化 — 当時採用。同一ファイルでない場合も
  **method-not-found という正しいコンパイルエラー自体は出る** (ハンドシェイク
  は診断の上乗せであり健全性には関与しない) ため、実害は診断品質に限られる。

**v3 での解消**: `docs/history/graph_literal_v3.md` でエッジ属性ペイロードを
`-[label = 式]->` という式渡しに変えたことで、ハンドシェイクマクロ
(`__graphite_edge_{Schema}!`) 自体が完全に不要になった (属性の struct
リテラル構築はユーザーの式そのものであり、マクロが介在する必要が無くなった)。
これは実質的に上記選択肢 (b) を採用したことに相当する: 未知ラベルの検出は
`b.{label}` の method-not-found (E0599) のみに委ね、「利用可能なエッジ一覧」
付きの親切な診断は失うが、これは意図した trade-off (ユーザー決定) である。

副産物として、ハンドシェイクマクロが担っていたテキストスコープ依存が消える
ため、**`dynamic_graph_schema!` と `graph!` はもはや同一ファイルである必要がない**。
`graph!` が参照するのは (1) スキーマ struct の `create`、(2) builder の総称
`insert`、(3) builder の型名付きエッジメソッド (`b.{label}(..)`) という
普通の Rust の型・メソッドだけになったため、別モジュールから `use` すれば
足りる。これを実証するテストを追加した:
`crates/graphite/tests/graph_cross_module.rs` (schema を専用モジュールに
隔離し、別モジュールから `use super::schema::*;` した上で `graph!` を呼ぶ)。

将来 Rust の `macro` (宣言マクロ 2.0、パスベーススコープ) が安定化しても、
もはやハンドシェイクマクロ自体が存在しないため再訪の必要はない。

### G6: 補完の実測 (G4 後の検証項目)

G4 実装後に vscode-lsp-mcp の completion プロバイダで実測する:
- `graph!` ノード宣言のフィールド名位置で、ノード struct のフィールドが補完されるか
- エッジラベル位置 (`-[` の後) の補完は原理的に難しい (トークン木の中の自由識別子)。
  効かない場合は G5 同様「制約の記録」とする。

### G7: rename のケース変換取り残しへの緩和策 (検討・未着手)

§1.5「rename カスケードの境界条件」の通り、ケース変換を挟む派生名
(`{Label}Attrs` 等) は RA の rename に追従できない。検討した緩和策:

- (a) 積み荷型名の明示構文 — `edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee) where each subordinate: 0..1;`
  に対し、属性型名をユーザーが書ける構文 (例: `{ since: i32 } as BossAttrs`) を
  足す。`BossAttrs` がユーザー自身のトークンになるため、ラベル rename と独立に
  なり (ラベルを変えても型名は変わらない = 取り残し自体が起きない)、型名の
  rename は型名のトークンで直接 F2 できる。原則1 (明示) とも整合。
- (b) 現状維持 + 文書化 — 取り残しは正確なコンパイルエラーとして現れるため
  静かな破壊はない。rename 後の手修正 2〜3 箇所を許容する。
- 判断: 当面 (b)。(a) は構文追加のコストに対して「rename の後始末が消える」
  だけの利得なので、実利用で取り残しが頻出するようなら再訪する。

## 3. 実装順序

1. G1 (graph! let 束縛化) + G2 (.vscode/settings.json) + G3 (スキル追記) — 本セッション
2. G1/G2 の効果を vscode-lsp-mcp で再計測し、マトリクスを更新
3. G4 (エラー回復) — 仕様は上記、実装は規模が大きいので独立フェーズ
4. G6 (補完実測) — G4 の後
