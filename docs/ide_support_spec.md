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
| rename | ⚠️ 判定保留 — この環境では rust-analyzer の rename provider が**マクロと無関係な普通の関数でも**「Unexpected type」例外を投げるため、マクロ起因かどうか切り分け不能 (環境問題)。参照検索は全出現を正しく検出しているので、rename の土台となる名前解決は機能している |

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

### G5: `graph!` ↔ `graph_schema!` 同一ファイル制約 (言語側制約の記録)

未知エッジラベル診断のハンドシェイク (`__graphite_check_edge_{Schema}!`) は
`macro_rules!` のテキストスコープに依存するため、schema と graph! が同一
ファイル (正確には同一スコープで schema が先) にないと機能しない。検討した選択肢:

- (a) `#[macro_export]` を付ける — マクロがクレートルートに強制輸出される。
  `graph!` 側は schema がどのクレートにあるか知らないためパス解決できず、
  別クレートの schema には結局効かない。名前空間汚染も原則6に反する。
- (b) ハンドシェイクを廃止し、`b.{label}` の method-not-found に任せる —
  クロスファイルで動くが、診断品質が大きく落ちる (利用可能エッジ一覧が出ない)。
- (c) 現状維持 + 制約の文書化 — 採用。同一ファイルでない場合も **method-not-found
  という正しいコンパイルエラー自体は出る** (ハンドシェイクは診断の上乗せであり
  健全性には関与しない) ため、実害は診断品質に限られる。

将来 Rust の `macro` (宣言マクロ 2.0、パスベーススコープ) が安定化したら (a) の
問題が解消するので再訪する。

### G6: 補完の実測 (G4 後の検証項目)

G4 実装後に vscode-lsp-mcp の completion プロバイダで実測する:
- `graph!` ノード宣言のフィールド名位置で、ノード struct のフィールドが補完されるか
- エッジラベル位置 (`-[` の後) の補完は原理的に難しい (トークン木の中の自由識別子)。
  効かない場合は G5 同様「制約の記録」とする。

## 3. 実装順序

1. G1 (graph! let 束縛化) + G2 (.vscode/settings.json) + G3 (スキル追記) — 本セッション
2. G1/G2 の効果を vscode-lsp-mcp で再計測し、マトリクスを更新
3. G4 (エラー回復) — 仕様は上記、実装は規模が大きいので独立フェーズ
4. G6 (補完実測) — G4 の後
