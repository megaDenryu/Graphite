# org-analyzer

Graphite (`graphite::graph_schema!`) を使った、組織データ (人事グラフ) 分析
CLI ツールの実用example。

社員・部署・プロジェクトという3種類のノードと、それらを結ぶ4種類の型付き
エッジを `graph_schema!` で宣言し、多重度制約 (「全社員は必ずちょうど1つの
部署に所属する」など) と、Graphite の「不変 + 再構築」パターンによる構造
検査の実演を目的にしている。

## スキーマ

```rust
pub struct Employee { pub name: String, pub title: String, pub grade: u8 }
pub struct Department { pub name: String }
pub struct Project { pub name: String, pub priority: u8 }
pub struct BossEdge { pub since: i32 }
pub struct AssignedEdge { pub role: String }

graphite::graph_schema! {
    schema OrgChart {
        node Employee;
        node Department;
        node Project;

        edge BelongsTo = Employee -> Department              where each Employee: 1;
        edge Boss      = Employee -[BossEdge]-> Employee     where each Employee: 0..1;
        edge Assigned  = Employee -[AssignedEdge]-> Project;
        edge Sponsors  = Department -> Project                where each Department: 0..1;
    }
}
```

| エッジ (Kind) | where 制約 | 意味 |
|---|---|---|
| `BelongsTo` | `where each Employee: 1` | 全社員は必ずちょうど1つの部署に所属する |
| `Boss` | `where each Employee: 0..1` | 上司は高々1人 (トップ層は0人) |
| `Assigned` | 制約なし | プロジェクトへの割当は0件以上 (兼務・未アサイン可)。同じ社員が同じプロジェクトに異なる役割で複数アサインされるケースを排除しないため、あえて `unique pair` を付けない |
| `Sponsors` | `where each Department: 0..1` | 部署がスポンサーするプロジェクトは高々1件 |

## データ

外部クレートに一切依存しない、決定的な合成データ生成器 (`src/dataset.rs`)。
線形合同法 (LCG) による自前の擬似乱数を使い、**同じシードなら常に同じ組織が
再現される**。既定では以下の規模で生成する。

- 社員 120人 (日本語の姓名プールから合成、grade 1〜5、役職は grade に対応)
- 部署 8 (営業部・開発部・人事部・経理部・マーケティング部・総務部・法務部・
  カスタマーサポート部)
- プロジェクト 15件

通常モードでは、上司関係は「部署内で自分より grade が厳密に高い人からランダム
に選ぶ」というルールで構築するため、部署ごとに見ると森 (forest) 構造になり、
相互上司や循環は原理的に発生しない。

`--seed N --inject-anomalies` を付けると、以下の4種の異常を意図的に埋め込む
(`anomalies` コマンドが検出すべき「正解データ」として `AnomalyPlan` に記録され、
統合テストで突き合わせている)。

1. 社員 E001/E002 を相互上司にする (お互いがお互いのboss)
2. 社員 E003→E004→E005→E003 の上司循環 (3人) を作る
3. プロジェクト P01 をどの部署からもスポンサーされない状態にする
4. プロジェクト P02 に誰もアサインされない状態にする

## 使い方

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
cargo run -- <サブコマンド> [引数] [--seed N] [--inject-anomalies]
```

共通オプション:

- `--seed N` : 生成シード (既定 `42`)
- `--inject-anomalies` : 上記4種の異常を注入する

### 1. `summary` — 部署別人数・grade分布・span of control・プロジェクト別アサイン人数

```
cargo run -- summary
```

出力例 (抜粋):

```
=== 組織サマリ ===
社員総数: 120人

--- 部署別人数 ---
  営業部          (D01) :  11人
  開発部          (D02) :  18人
  ...

--- grade分布 ---
  grade1 :  43人
  grade2 :  34人
  ...

--- span of control (直属部下数) ---
  管理職(grade3以上)平均: 2.07人
  最大: 6人 (伊藤隆 / E061)
  部下ゼロの管理職: 9人
    - 石川花子 (係長 / E027)
    ...

--- プロジェクト別アサイン人数 ---
  次世代基幹システム刷新      (P01) :   9人
  ...
```

「span of control」は `grade >= 3` (係長以上) を管理職とみなし、`Boss::iter(&g)`
を上司キーで集計した直属部下数から平均・最大・部下ゼロの管理職一覧を出す。

### 2. `chain <社員キー>` — 管理チェーンを根まで辿る

```
cargo run -- chain E003 --seed 7 --inject-anomalies
```

```
=== 管理チェーン ===
森雄大 (係長 / E003) [起点, 深さ0]
  └─ 後藤恵子 (課長 / E004) [在任 2019年〜, 深さ1]
    └─ 後藤陽菜 (主任 / E005) [在任 2020年〜, 深さ2]

[警告] 循環を検出したため打ち切りました (社員 E003 まで戻っています)
```

`Boss::of` (`where each Employee: 0..1`) は `Option<(&Employee, &BossEdge)>` を
返すだけで上司の ID そのものは含まないため、`Boss::iter(&g)` から
`EmployeeId -> (EmployeeId, since)` の索引を作ってから辿っている。訪問済み集合を
持ちながら辿ることで、途中で循環に入った場合も無限ループせず検出・打ち切りできる。

存在しない社員キーを渡すとエラー終了する:

```
$ cargo run -- chain E999
エラー: 社員キー 'E999' は存在しません
```

### 3. `anomalies` — 構造異常検出レポート

```
cargo run -- anomalies --seed 7 --inject-anomalies
```

```
=== 構造異常レポート ===

--- 相互上司ペア ---
  吉田康平 (E001) <-> 鈴木陽菜 (E002)

--- 上司関係の循環 ---
  循環1: 後藤陽菜(E005) -> 森雄大(E003) -> 後藤恵子(E004) -> (先頭に戻る)

--- 部署跨ぎ上司 ---
  森雄大 (E003, 所属:D04) の上司は 後藤恵子 (E004, 所属:D05)
  後藤陽菜 (E005, 所属:D05) の上司は 森雄大 (E003, 所属:D04)

--- 無人プロジェクト ---
  モバイルアプリリニューアル (P02)

--- スポンサー無しプロジェクト ---
  次世代基幹システム刷新 (P01)
  ...
```

検出手法:

- **相互上司ペア**: `Boss::iter(&g)` で全ペアを集めておき、`(a, b)` かつ
  `(b, a)` が両方存在するものを拾う (README (Graphite本体) に載っている手法
  そのもの)。
- **上司循環**: `Boss` エッジを `Graph::from_edges` で汎用
  `graphite::Graph<(), (), EmployeeId>` に射影し、`topological_sort()` で
  検出する。`CycleError::cycle` (循環メンバー全体を返す) をそのまま使えるので
  「boss辺を手で辿って復元する」処理は不要。長さ2の循環 (相互上司) は上の
  項目と重複するのでここには含めない。
- **部署跨ぎ上司**: `BelongsTo::iter(&g)` で作った所属索引と `Boss::iter(&g)`
  を突き合わせ、上司と部下の部署が異なるものを拾う。
- **無人プロジェクト / スポンサー無しプロジェクト**: `Assigned::iter(&g)` /
  `Sponsors::iter(&g)` に現れないプロジェクトキーを `Project::ids(&g)` との
  差分で求める。

### 4. `reorg <部署キー>` — 組織改編シミュレーション

```
cargo run -- reorg D01
```

```
=== 組織改編シミュレーション ===
廃止対象部署: 営業部 (D01)
再配置対象: 11人

--- 再配置先 (社員キー順、ラウンドロビン) ---
  加藤花子 (E014) -> 開発部 (D02)
  木村雄大 (E016) -> 人事部 (D03)
  ...
  ... 他 1人

[OK] 再構築に成功しました (freeze検証をパス)
  新組織: 社員120人 / 部署7人 / プロジェクト15件
```

指定した部署を廃止し、その部署に所属していた社員を残り部署へ社員キー順の
ラウンドロビンで機械的に再配置した新しい `OrgChart` を、`OrgChart::create`
で **丸ごと再構築** する。可変 API 経由の「部署を消す」操作は存在しない
(Graphite は構築後不変) ので、これが唯一の編集手段になる。

この再構築ロジックは意図的に「素朴」なままにしている箇所が1つある: 廃止部署
が発していた `Sponsors` 辺 (Department -> Project) をカスケード削除せず、
そのまま新しいノード集合に持ち込む。廃止対象の部署がどのプロジェクトもスポン
サーしていなければ何も起こらず成功するが、スポンサーしていた場合は
存在しない部署キーを参照する辺が残ったまま `create` に渡り、`freeze` 検証が
`OrgChartViolation::SponsorsUnknownSource` を検出してエラーになる:

```
$ cargo run -- reorg D03
=== 組織改編シミュレーション ===
廃止対象部署: 人事部 (D03)
再配置対象: 9人
...

[NG] freeze検証がViolationを検出し、再構築は失敗しました:
  未知のキーが参照されています (辺 `Sponsors` SponsorsId("spon_D03") の始点, Department): DepartmentId("D03")
  詳細: SponsorsUnknownSource { edge: SponsorsId("spon_D03"), source: DepartmentId("D03") }

  解説: 廃止部署が指すsponsors辺(部署->プロジェクト)をカスケード削除
し忘れたまま再構築しようとしたため、存在しない部署キーを参照する辺が
残り、create()のfreeze検証がそれを機械的に検出しました。可変APIが
存在しないGraphiteでは「不変+再構築」しか編集手段がないため、この種の
参照切れは(見落とさない限り)必ずこの場で顕在化します。
```

「ノードを削除するときに、それを参照する辺のカスケード削除を忘れる」という
のは実務でもよくあるミスであり、可変な `HashMap` を自分で管理していたら
気づかないままサイレントに壊れたデータが残っていた可能性がある。Graphite
では編集手段が「不変な値の丸ごと再構築」しかないため、参照切れは
**再構築のたびに必ず検証され、その場で `Err` として顕在化する**。

## Graphiteを使う意味

自前で `HashMap<EmployeeId, Employee>` や `HashMap<DepartmentId, Vec<EmployeeId>>`
を手で管理する実装と対比すると、`graph_schema!` が肩代わりしてくれる点は
以下の通り具体的である。

### 1. `where each Employee: 1` による「全社員は必ず1部署」保証

生HashMap実装では「社員を登録したが部署未設定」「部署を2つ登録してしまった」
といった不整合が **実行時に静かに** 残り得る。`BelongsTo::iter(&g)` を毎回
自分で数えて検査するコードを書かない限り気づけない。

Graphiteでは `edge BelongsTo = Employee -> Department where each Employee: 1;`
と宣言した時点で、`OrgChart::create()` が全社員について「ちょうど1本」で
あることを一括検査し、満たさなければ `OrgChartViolation::BelongsToEachViolation`
で構築自体が失敗する。本アプリの合成データ生成器 (`dataset.rs`) がバグって
所属漏れの社員を作ってしまえば、`summary` を実行する前の `OrgChart::create()`
の時点で即座に検出される (`.expect(...)` で握りつぶさない限り必ず気づける)。

### 2. freezeによる一括検証 (「不変+再構築」パターン)

`reorg` コマンドがまさにその実演になっている。生HashMapで部署を「削除」する
実装なら、`departments.remove(&target)` した後にそれを参照する
`sponsors` エントリが残っていても、たまたまそのエントリを読みに行くコード
パスを通らない限り気づかない (「ダングリング参照が静かに残る」典型例)。

Graphiteには可変な削除APIが存在せず、「新しいノード集合とエッジ集合を
丸ごと `create` に渡して凍結し直す」ことでしか組織を変更できない。この
再構築のたびに `freeze` が全エッジの端点を検査するため、カスケード削除の
モレは (今回のように) 必ずその場で `Violation` として浮かび上がる。

### 3. 型付きアクセサによる誤り耐性

`BelongsTo::of(&g, &emp_id)` は `&Department` を、`Boss::of(&g, &emp_id)` は
`Option<(&Employee, &BossEdge)>` を、`Assigned::of(&g, &emp_id)` は
`Vec<(&Project, &AssignedEdge)>` を返す — `where each` 制約がそのまま戻り値
の型 (直接返却 / `Option` / `Vec`) に反映されている。生HashMap実装で
`HashMap<EmployeeId, Vec<DepartmentId>>` のように多重度を型で表現し忘れると、
「本当は1つのはずの部署が複数入っている」バグを型システムが教えてくれない。

`BelongsTo::get_of()` (非パニック版) と `BelongsTo::of()` (パニック版) の
対も、「このグラフが発行したキーだけを渡す」という呼び出し規約と、「外部
入力かもしれないキーを安全に検査する」という用途を型シグネチャで自然に
書き分けられる (`main.rs` の `chain`/`reorg` サブコマンドで未知キー入力を
扱う箇所は `get_of`、内部で確実に存在するキーを使う箇所は `of`、と使い分けて
いる)。アクセサの操作語彙 (`of`/`get_of`/`get`/`between`/`iter`/`ids`/`len`)
は `Kind` によらず共通なので、覚えることは増えない (`docs/schema_v4.md`
§3.2 参照)。

### 4. `iter()` による宣言的なクエリ

`anomalies` コマンドの相互上司検出・部署跨ぎ上司検出は、生HashMapなら
「全社員をループしてO(N)の検索を都度行う」か「逆引きインデックスを自分で
構築・保守する」必要がある。Graphiteの `Boss::iter(&g)`/`BelongsTo::iter(&g)`
は最初からその形 (`(&{Kind}Id, &{Kind})` のイテレータ、`.from()`/`.to()`/
`.payload()` で分解) で提供されるため、`filter`/`collect`/`contains` といった
通常のイテレータコンビネータだけで検出ロジックを書ける (`src/analysis.rs`
参照)。

## 構成

```
src/
  lib.rs      - モジュール公開 (mainとtestsの両方から使うためlib+bin構成)
  schema.rs   - graph_schema! によるスキーマ定義
  dataset.rs  - LCGベースの決定的合成データ生成器 (異常注入モード含む)
  analysis.rs - summary / chain / anomalies のロジック
  reorg.rs    - 組織改編シミュレーション ("不変+再構築"パターン)
  report.rs   - 各サブコマンドの表示整形
  main.rs     - CLIエントリポイント (std::env::argsで自作パース)
tests/
  integration.rs - anomalies/chain/reorg/summaryの統合テスト
```

## Graphite APIで不足を感じた点 (フェーズ5の種として)

- **`boss()` のような多重度(0..1)アクセサが相手ノードのIDを返さない**。
  `Option<(&Employee, &BossEdge)>` は値は取れるが「その上司のキーは何か」
  が分からないため、`chain` コマンドのようにチェーンを辿る用途では結局
  `boss().iter()` から自前でインデックス (`HashMap<EmployeeId, (EmployeeId,
  BossEdge)>`) を作り直す必要があった。`{label}_target(&SrcId) ->
  Option<&DstId>` のような「相手キーだけを返す」アクセサがあると、
  グラフを辿るタイプの処理 (経路探索・チェーン追跡) がもう一段書きやすい。
  → **解決 (フェーズ5で `{label}_id` として追加、その後ビュー方式
  (`docs/edge_view_api.md`) へ、さらにスキーマv4 (`docs/schema_v4.md`) で
  辺の第一級キー化へ移行)**: v4 では辺そのものがキー付き要素 (`{Kind}Id`)
  であり、`Kind::iter(&g)` が返す `(&{Kind}Id, &Kind)` の `Kind` 値に
  `.from()`/`.to()` があるため、相手キーは `iter()` から直接取れる。
  `management_chain` は `Boss::iter(&g)` から
  `HashMap<EmployeeId, (EmployeeId, i32)>` の索引を1回作って辿っており、
  これは「上司を根まで辿る」という探索自体が単発アクセサでは表現できない
  (毎回グラフに問い合わせ直すより索引を1回作る方が効率的な) ためであって、
  キー取得手段が無いことによる回避策ではない。
- **`Graph<N, E, K>::topological_sort()` の `CycleError` が循環メンバーを
  1つしか返さない**。今回の `boss` は「各ノードの出次数が高々1」という
  特殊な形 (関数グラフ) だったため自前で辿って復元できたが、一般のグラフ
  では `CycleError` から循環の全メンバーを機械的に復元する手段が無い。
  `find_cycle() -> Option<Vec<K>>` のような、循環そのものを返すAPIが
  あると `anomalies` 型のコマンドがもっと素直に書けたはず。
  → **解決 (フェーズ5)**: `CycleError<K>` が `cycle: Vec<K>` (循環メンバー
  全体、`cycle[0]` から辿って `cycle[0]` に戻る閉路) を返すように拡張された。
  `detect_boss_cycles` の自前 `boss_of` 復元コードは不要になり削除した。
- **`filter_nodes` の述語がノード「値」しか見られない**ため、ノード「キー」
  で絞り込みたいとき (今回の「見つけた循環のメンバーをキーで除外する」)は
  ノード値にキーの複製を持たせる、というやや不自然な回避策が要る。
  `filter_nodes_by_key(|k| ...)` のような、キーに対する述語版があると
  素直に書ける。
  → **解決 (フェーズ5)**: `filter_nodes_with_key(|k, v| ...)` /
  `map_nodes_with_key(|k, v| ...)` が追加された。`detect_boss_cycles` は
  ノード値をキーの複製にする回避策をやめ、`Graph<(), (), EmployeeId>` +
  `Graph::from_edges` + `filter_nodes_with_key` に書き直した。
- **`OrgChartViolation` に複数件の違反をまとめて返す手段がない**
  (`freeze` は最初に見つかった1件で即 `Err` を返す)。`reorg`
  コマンドの「違反レポート」節でも、実際には1回のシミュレーションにつき
  1件の違反しか表示できていない。人間向けレポートとしては「見つかった
  違反を全部一度に教えてほしい」需要があり、`freeze` に「全件収集モード」
  があると `reorg` のような診断系コマンドの説得力が増す。
