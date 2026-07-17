# IDE サポート仕様 (VSCode / rust-analyzer)

Graphite の DSL (`graph_schema!` / `graph!`) を書くとき、VSCode 上で参照ジャンプ・
型追跡・rename などが「普通の Rust コードと同じように」機能することを目標とする。

計測方法: vscode-lsp-mcp 経由で rust-analyzer の definition/references プロバイダを
直接叩き、`targetSelectionRange` (F12 でカーソルが着地する正確な位置) まで確認した。
計測対象: `crates/graphite/tests/orgchart_macro.rs` (2026-07-14 計測)。

## 1. 現状マトリクス

| 操作 | 結果 | 備考 |
|---|---|---|
| 使用側の型名 (`Employee {..}`) → 定義 | ✅ 精密 | schema 内 `node Employee` の `Employee` トークンに着地 |
| 使用側のアクセサ (`g.belongs_to(..)`) → 定義 | ✅ 精密 | schema 内 `edge belongs_to` の `belongs_to` に着地 |
| 派生名 (`try_belongs_to`, `*_id`, `*_ids` 等) → 定義 | ✅ 精密 | `format_ident!` が最初に補間した Ident のスパンを継承するため |
| 属性フィールド (`attrs.since`) → 定義 | ✅ 精密 | schema 内 `{ since: i32 }` の `since` に着地 |
| 違反 enum (`OrgChartViolation::..`) → 定義 | ✅ | schema 名トークンに着地 |
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
| examples/* の解析 | ✅ `Scene`/`SceneId` 等がワークスペースシンボルとして引ける。graph_schema! 内トークンへのスパンも機能 |
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

構文 v2 (`docs/edge_syntax_v2.md`: ノード型・エッジ属性型を外部 struct 参照化)
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

`docs/graph_literal_v3.md` (`-[label = 式]->` への変更、ハンドシェイクマクロ
全廃) の実装後、`cargo expand -p graphite --test orgchart_macro` で実際の
展開結果を確認した。§1.7 で「二段マクロ展開を rust-analyzer が追跡できない」
と記録した制約は、**展開そのものが単段になったことで構造的に解消した**
(rust-analyzer 側の実測は今回未実施だが、展開結果に中間マクロ呼び出しが
一切残っていないことをコード上で確認済み):

```rust
// cargo expand の実際の出力 (抜粋)
let g = OrgChart::create(|__graphite_b| {
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

v4 (`docs/schema_v4.md`: 辺の第一級化・where 制約・型名前空間アクセス) 実装後、
`crates/graphite/tests/orgchart_macro.rs` で実測:

| 操作 | 結果 |
|---|---|
| schema `-[BossEdge]->` の積み荷型 → 定義 | ✅ ユーザー struct へ精密 |
| `Boss::of(&g,..)` / マクロ外の `Boss(from,to,payload)` 構築 / リテラルの `Boss(..)` → 定義 | ✅ いずれも schema の `edge Boss` トークンへ精密着地 (辺種別 = 生成タプル struct の解決が全文脈で機能) |
| リテラルのノードキー (`tanaka`)・積み荷フィールド (`since`)・辺キー束縛 (`tanaka_boss`) | ✅ v3 同様に精密 (let 束縛・式素通しの機構は v4 でも維持) |
| schema `Boss` → 参照検索 | ✅ 宣言 + 全使用 15 件 (アクセス・リテラル・素の構築・型注釈) |
| `where each Employee` の `Employee` → 定義 | ✅ `2dce96a` で修正し実測確認済み: ユーザーの `struct Employee` 宣言へ精密着地。`EdgeInfo::each_from_token` にトークンを保持し、freeze 検証コード内にゼロコストの型検査文 (`let _: fn(&Type) = \|_\| {};`) として補間することで、このトークンが実在の型参照になった |
| `Boss::of` の `of` 等、生成関連関数のメソッド名トークン → 定義 | ✅ `2dce96a` で修正し実測確認済み: schema の `edge Boss` トークンへ精密着地 (修正前はマクロブロックに着地)。生成 fn ident に由来する Kind/ノード型トークンのスパンを付与 (G3 ポリシー適用) |

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

## 2. 仕様項目

### G1: `graph!` ノードキーの let 束縛化 (実装対象)

現行の展開はキー識別子をその場で文字列化する:

```rust
b.employee(EmployeeId("tanaka".to_string()), Employee { .. });
b.belongs_to(EmployeeId("tanaka".to_string()), DepartmentId("sales".to_string()));
```

これを、ノードキーごとに 1 つの `let` 束縛を作り、以後は識別子参照で運ぶ形に変える:

```rust
OrgChart::create(|__graphite_b| {
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

- `graph_schema!`: `schema { .. }` ボディを宣言 (node/edge) 単位で読み、壊れた
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

### G5: `graph!` ↔ `graph_schema!` 同一ファイル制約 (v3 で解消済み)

**2026-07-14/15 更新: `docs/graph_literal_v3.md` の実装により、この制約自体が
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

**v3 での解消**: `docs/graph_literal_v3.md` でエッジ属性ペイロードを
`-[label = 式]->` という式渡しに変えたことで、ハンドシェイクマクロ
(`__graphite_edge_{Schema}!`) 自体が完全に不要になった (属性の struct
リテラル構築はユーザーの式そのものであり、マクロが介在する必要が無くなった)。
これは実質的に上記選択肢 (b) を採用したことに相当する: 未知ラベルの検出は
`b.{label}` の method-not-found (E0599) のみに委ね、「利用可能なエッジ一覧」
付きの親切な診断は失うが、これは意図した trade-off (ユーザー決定) である。

副産物として、ハンドシェイクマクロが担っていたテキストスコープ依存が消える
ため、**`graph_schema!` と `graph!` はもはや同一ファイルである必要がない**。
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

- (a) 属性型名の明示構文 — `edge boss: Employee -> Employee (0..1) { since: i32 }`
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
