# Graphite

Graphite は、自作言語 [Vertex](../Bullet) のグラフ機能の設計検討から派生した、
独立した Rust プロジェクトです。Vertex 本体とは切り離されており、Vertex 言語
処理系のコードには一切依存しません。

Vertex 側では「グラフ指向」を独立言語の構文・型システムとして実装する道を
選びましたが、その設計を壁打ちする過程で「グラフはあくまで既存言語 (Rust) の
型システムと所有権に乗るデータ構造として実装でき、DSL 部分だけを proc マクロ
+ ライブラリとして切り出せるのではないか」という仮説が生まれました。Graphite
はその仮説を検証するプロジェクトです。設計の系譜:

- `../Bullet/docs/graph_design_sketches.md` — グラフ型そのものの設計決定
  (ノード同一性、可変性、矢印記法、多重度検査、可視性、型推論)
- `../Bullet/docs/rust_graph_extension_sketch.md` — 上記の決定を Rust の
  proc マクロ + ライブラリとしてどう実現するかの一次資料。`graph_schema!`/
  `graph!` の展開イメージはここで最初に書かれた

## 2 クレート構成

```
crates/graphite/         # ランタイムクレート。利用者が唯一 depend するクレート
crates/graphite-macros/  # proc-macro クレート (graph_schema!, graph! を実装する)
```

proc-macro クレート (`proc-macro = true`) は手続き型マクロ = コンパイラ
プラグインの一種であり、生成する側 (マクロ) と生成されたコードが依存する側
(ランタイム型) を同じクレートに置けないという Rust の技術的制約のため 2 分割
している (serde/serde_derive、diesel、sqlx と同型)。利用者は `graphite` だけ
に依存し、マクロは `graphite::graph_schema!` / `graphite::graph!` として
re-export されたものを使います。

## 使用例

### 1. `graph_schema!` でスキーマを宣言する

```rust
graphite::graph_schema! {
    schema OrgChart {
        node Employee { name: String, id: u32 }
        node Department { name: String }

        edge belongs_to: Employee -> Department (1);
        edge boss:       Employee -> Employee   (0..1) { since: i32 };
        edge reports:    Employee -> Employee   (0..*);
    }
}
```

これでノード struct (`Employee`/`Department`)・newtype キー
(`EmployeeId`/`DepartmentId`)・辺属性 struct (`BossAttrs`)・スキーマ struct
(`OrgChart`, フィールドは非公開)・builder (`OrgChartBuilder`)・違反 enum
(`OrgChartViolation`) が一式生成されます。

多重度→戻り値の対応:

| 多重度     | 格納                        | アクセサの戻り値                  |
|-----------|----------------------------|-----------------------------------|
| `(1)`     | 必須 1 本 (freeze で検査)    | `&T` (または属性付きなら `(&T, &Attrs)`) |
| `(0..1)`  | 高々 1 本                   | `Option<&T>` (属性付きは `Option<(&T, &Attrs)>`) |
| `(0..*)`  | 0 本以上                     | `Vec<&T>` (属性付きは `Vec<(&T, &Attrs)>`) |

多重度 `(1)` のアクセサ (`{label}(&SrcId) -> &T`) は未知キーを渡すとパニック
します (`Vec` の `v[i]` と同じ「呼び出し規約違反」の扱い)。パニックしない
版として `try_{label}(&SrcId) -> Option<&T>` (属性付きなら
`Option<(&T, &Attrs)>`) も併せて生成されます (`v.get(i)` に相当)。

さらに、`match` パターンでのグラフクエリの代替として、各エッジ種別ごとに
ペアイテレータ `{label}_pairs() -> impl Iterator<Item = (&SrcId, &DstId)>`
(属性付きなら `(&SrcId, &DstId, &Attrs)`。多重度 `(0..*)` は全ペアへ展開)
と、各ノード種別ごとにキー列挙 `{node_snake}_ids() -> impl Iterator<Item = &NodeId>`
も生成されます。使用例は「3. アクセサ・アルゴリズムを使う」節を参照。

**ID版アクセサ (フェーズ5)**: `{label}` 系アクセサは相手ノードの*値*
(`&T`) を返しますが、指揮系統チェーンのように「次のノードのキーへ辿って
またそこから辿る」処理には値ではなくキーが要ります。多重度ごとに以下の
ID版が併せて生成されます (属性は既存の値アクセサで取得できるため、ID版
には含めません):

| 多重度     | ID版アクセサ                                        |
|-----------|-----------------------------------------------------|
| `(1)`     | `{label}_id(&SrcId) -> &DstId` (未知キーはパニック。`# Panics` 明記) + `try_{label}_id(&SrcId) -> Option<&DstId>` |
| `(0..1)`  | `{label}_id(&SrcId) -> Option<&DstId>`               |
| `(0..*)`  | `{label}_ids(&SrcId) -> Vec<&DstId>` (格納順を保持。後述「`(0..*)` エッジの順序保証」節参照) |

**`create_collecting` (フェーズ5)**: `create` は最初の1件の違反で `Err`
になりますが、組織図の全違反を一覧表示するような検証系ユースケースでは
複数違反をまとめて見たいことがあります。`{Schema}::create_collecting(|b| { ... }) -> Result<Self, Vec<{Schema}Violation>>`
が同じ builder クロージャを受け取り、freeze 検査を打ち切らず全違反を
`Vec` に集めて返します。`create` はこの収集版に委譲し先頭の1件を返す
薄いラッパーとして実装されています (検証ロジックの二重実装を避けるため)。

ノード struct・エッジ属性 struct は `#[derive(Debug, Clone, PartialEq)]` のみ
(`Eq` は付けません)。`f64` のように `Eq` を実装できないフィールド型も使える
ようにするための設計判断です (newtype キー型は内部で `HashMap` のキーに
使うため `Hash + Eq` を維持します)。

### 2. `graph!` でインスタンスを組み立てる

```rust
let g = graphite::graph!(OrgChart {
    tanaka: Employee { name: "田中".into(), id: 1 },
    sato:   Employee { name: "佐藤".into(), id: 2 },
    sales:  Department { name: "営業".into() },

    tanaka -[belongs_to]-> sales,
    sato   -[belongs_to]-> sales,
    tanaka -[boss { since: 2020 }]-> sato,
})?; // Result<OrgChart, OrgChartViolation>
```

`graph!` は `OrgChart::create(|b| { ... })` の呼び出し列へ脱糖するだけで、
スキーマの中身 (どのエッジが存在するか等) は一切知りません。ノード宣言行
`key: Type { .. }` の型名から builder メソッド名・newtype キー型名を
`graph_schema!` と同じ命名規則 (snake_case / `{Type}Id`) で機械的に導出し、
辺の端点キーの型はその場で作った「識別子 → 宣言時の型名」対応表から逆引き
します。

`-[label]->` の向きは「`from` = 辺ラベルの builder 引数の 1 番目、`to` = 2
番目」に対応します。上の例の `edge boss: Employee -> Employee` は手書き
テンプレートの `boss(employee, boss, attrs)` という引数順を踏襲しているため、
`tanaka -[boss]-> sato` は「田中の上司は佐藤」を意味します (向きを取り違え
やすい点なので、独自スキーマを書くときは意識してください)。

マクロ呼び出しの中の `-[label]->` は `-`, `[`, ident, `]`, `-`, `>` という
独自トークン列のため、rustfmt を混乱させないよう呼び出しには
`#[rustfmt::skip]` を付けることを推奨します。

### 3. アクセサ・アルゴリズムを使う

```rust
let dept = g.belongs_to(&EmployeeId("tanaka".to_string())); // &Department (多重度 (1))
let (boss, attrs) = g.boss(&EmployeeId("tanaka".to_string())).unwrap(); // Option<(&Employee, &BossAttrs)>
let reports = g.reports(&EmployeeId("tanaka".to_string())); // Vec<&Employee>

// try_{label}: 多重度 (1) の非パニック版。未知キーは None に落ちる。
let dept_opt = g.try_belongs_to(&EmployeeId("no-such-id".to_string())); // None

// {label}_pairs(): match パターンの代替。イテレータチェーンでクエリを書く。
// 例: 相互に上司であるペア (A の boss が B かつ B の boss が A) を検出する。
let all: Vec<(&EmployeeId, &EmployeeId)> =
    g.boss_pairs().map(|(a, b, _attrs)| (a, b)).collect();
let mutual_bosses: Vec<(&EmployeeId, &EmployeeId)> = all
    .iter()
    .copied()
    .filter(|(a, b)| all.contains(&(b, a)))
    .collect();

// {node_snake}_ids(): ノード種別ごとの全キー列挙。
let all_employee_ids: Vec<&EmployeeId> = g.employee_ids().collect();

// {label}_id / {label}_ids: 値ではなくキーを返すID版アクセサ (フェーズ5)。
// 指揮系統チェーンのように「キーのまま次のノードへ辿る」処理に使う。
let dept_id: &DepartmentId = g.belongs_to_id(&EmployeeId("tanaka".to_string()));
let boss_id: Option<&EmployeeId> = g.boss_id(&EmployeeId("sato".to_string()));
let report_ids: Vec<&EmployeeId> = g.reports_ids(&EmployeeId("tanaka".to_string()));

// create_collecting: 最初の1件で打ち切らず全違反を収集する (フェーズ5)。
let result: Result<OrgChart, Vec<OrgChartViolation>> = OrgChart::create_collecting(|b| {
    // ... 複数の違反を含みうる構築 ...
});
```

図式グラフ (`graph_schema!`) とは別に、ノード型が 1 種類の同種グラフ用に
ジェネリックな `graphite::Graph<N, E, K>` (水準1相当、petgraph の薄い
ラッパー) も用意しています。`has_cycle`/`topological_sort`/
`topological_levels`/`critical_path_by`/`reachable_from`/`path`/
`out_neighbors`/`in_neighbors`/`map_nodes`/`map_nodes_with_key`/
`filter_nodes`/`filter_nodes_with_key`/`from_edges` などのアルゴリズム・
ヘルパーはこちらに実装されており、`graph_schema!` が生成する図式グラフ
とは独立した別 API です (`crates/graphite/src/graph.rs`)。

- `in_neighbors(&K) -> Vec<&K>` — `out_neighbors` と対称 (入ってくる辺の
  始点キー一覧)。
- `Graph::<(), (), K>::from_edges(nodes, edges) -> Result<Self, GraphError<K>>` —
  ノードキー集合と `(from, to)` の列から値なしの構造グラフを作る射影用
  ヘルパー。`{label}_pairs()` から汎用アルゴリズムへ射影する定型操作向け。
- `topological_levels() -> Result<Vec<Vec<&K>>, CycleError<K>>` — 依存の
  ないノードから順にレベル (波) 分割したトポロジカルソート (レベル内は
  挿入順で決定的)。
- `critical_path_by(node_weight) -> Result<(Vec<&K>, W), CycleError<K>>` —
  ノード重み付き最長経路 (クリティカルパス)。空グラフは
  `(vec![], W::default())`。
- `filter_nodes_with_key`/`map_nodes_with_key` — 既存の `filter_nodes`/
  `map_nodes` のキー付き版。述語/変換関数がノード値だけでなくキーも
  参照できる。
- `CycleError<K>` は循環検出時、循環を構成するノードキー列全体
  (`cycle: Vec<K>`。`cycle[0]` から辿って `cycle[0]` に戻る閉路) を返す
  (フェーズ5で `node: K` (代表ノード1つ) から拡張した破壊的変更)。

導出エッジ (保存されない計算結果、例: 同じ部署の同僚一覧) は `graph_schema!`
の DSLには含めていません。生成された struct は同一モジュール内であれば
私有フィールドに直接アクセスできる (通常の Rust 可視性規則) ので、
`impl OrgChart { pub fn colleagues(&self, ...) -> Vec<&Employee> { ... } }`
のように後から普通のメソッドとして追記してください
(`crates/graphite/tests/orgchart_macro.rs` に実例あり)。

## テスト

```powershell
cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50
cargo test
```

- `crates/graphite/tests/orgchart_handwritten.rs` — フェーズ2で手書きした
  `OrgChart` (`graph_schema!` が生成すべきコードの目標形。テンプレートとして
  残置)
- `crates/graphite/tests/orgchart_macro.rs` — `graph_schema!` で `OrgChart`
  を宣言し直した同等テスト、および `graph!` リテラルのテスト
- `crates/graphite/tests/compile_fail.rs` + `tests/ui/*.rs` —
  [`trybuild`](https://docs.rs/trybuild) によるコンパイルエラー系テスト
  (未宣言ノード型を端点に指定 / 不正な多重度 / `graph!` で存在しないエッジ
  種別)。stderr の再生成は `TRYBUILD=overwrite cargo test --test compile_fail`

## 実践例 (`examples/`)

`graphite` を実際のアプリケーションから使う例として、`examples/` 配下に
3 本のスタンドアロンクレートを用意しています。いずれも `Cargo.toml` 先頭に
空の `[workspace]` テーブルを置いてルートの Cargo workspace から独立させた、
`graphite` のみに依存する単体アプリです (ルート `cargo test` の対象には
含まれないため、個別に `cd` してビルド・実行します)。

- **`examples/build-pipeline/`** — ビルドパイプライン・オーケストレータ。
  `pipeline.txt` (23 タスク) をパースして `Graph` に取り込み、循環検出・
  クリティカルパス計算・波 (wave) 分割・Mermaid 図出力を行う。
  ```powershell
  cd examples/build-pipeline
  cargo run -- plan
  ```
- **`examples/org-analyzer/`** — 組織分析ツール。LCG で合成した社員 120 人分の
  組織データから、サマリ統計・指揮系統チェーン・異常検知・組織再編シミュレー
  ションを行う。
  ```powershell
  cd examples/org-analyzer
  cargo run -- summary
  ```
- **`examples/dialogue-engine/`** — 分岐ノベルエンジン。`graph!` リテラルで
  30 シーン・4 エンディング・56 選択肢のシナリオを組み立て、プレイ・検証・
  マップ表示・最短ルート探索・統計を行う。
  ```powershell
  cd examples/dialogue-engine
  cargo run -- validate
  ```

各ディレクトリの詳細な使い方・サブコマンド一覧は、それぞれの `README.md` を
参照してください。

## 手書きテンプレートとの差異

`graph_schema!` は基本的に `orgchart_handwritten.rs` と同じ形を生成します
が、「任意のノード型・エッジ型の組み合わせ」に一般化する過程でいくつか
手書き版と異なる設計判断をしています。

1. **違反 enum は 1 スキーマにつき 1 つ生成** (`{Schema}Violation`)。手書き
   版は `SchemaViolation` という固定名でしたが、複数のスキーマを同じモジュール
   に宣言したときに型名が衝突しないよう、スキーマ名をプレフィックスにして
   います。
2. **違反 enum のバリアントはエッジ単位で型付き生成される
   (`{Label}Multiplicity` / `{Label}UnknownSource` / `{Label}UnknownTarget`)**。
   手書き版は `MultiplicityViolation { employee: EmployeeId, .. }` という
   スキーマ共通の 1 バリアントでしたが、一般のスキーマではエッジごとに
   始点/終点ノード型が異なりうる (例: `A -> B` と `C -> D` が両方多重度
   違反を起こしうる) ため、エッジごとに専用バリアントを生成することで型を
   `String` に落とさず固定できるようにしています (フェーズ5、「型の
   strictness」原則。`docs/design_principles.md` 原則1 参照)。例:
   `edge belongs_to: Employee -> Department (1)` からは
   `BelongsToMultiplicity { source: EmployeeId, count: usize }` /
   `BelongsToUnknownSource { key: EmployeeId }` /
   `BelongsToUnknownTarget { key: DepartmentId }` が生成されます。
3. **builder のエッジ追加メソッドの引数名は汎用的に `from`/`to`**。手書き版
   は `boss(employee, boss, attrs)`・`reports(manager, report)` のように
   ドメイン語で命名されていましたが、マクロはノード型名だけから引数名を
   導出する必要があり、自己参照エッジ (`Employee -> Employee`) では同名
   引数の衝突を避けられないため、常に `from`/`to` にしています。
4. **内部ストレージの複数形フィールド名は既定では素朴な英語複数形
   (`+ "s"`)**。不規則複数形 (`Category` → `Categorys` になってしまう等)
   には自動対応していません。この名前は非公開フィールドで利用者から見えない
   ため機能上の問題はありませんが、生成コードを `cargo expand` 等で目視する
   際は注意してください。**フェーズ4で明示指定構文を追加済み**
   (`node Category(categories) { name: String }` のように `node` 宣言に
   `(識別子)` を付けると内部フィールド名を上書きできる)。詳細は後述の
   「未決事項」節を参照。
5. **導出エッジ (`colleagues` 等) はマクロが生成しない**。上記「使用例 3」
   参照。

## 未決事項 / フェーズ4があるとしたら

以下はフェーズ3終了時点での未決事項一覧でした。フェーズ4で 5 項目中 4 項目
(残り 1 項目は設計判断として据え置き) に着手し、対応関係は以下の通りです。

- **多重度 `(1)` アクセサへ未知キーを渡した場合は v0 ではパニックとする —
  解決済み (フェーズ4)**。既存の `{label}(&SrcId) -> &T` (パニック版) は
  「このスキーマが発行したキーだけを渡すことが呼び出し側の責務」という
  設計のまま残しつつ、非パニック版 `try_{label}(&SrcId) -> Option<&T>`
  (属性付きは `Option<(&T, &Attrs)>`) を追加生成しました。`Vec` の `v[i]`
  (パニック) と `v.get(i)` (`Option`) の対と同じ関係です。
- **`match` パターンでのクエリは非対応 — 一部緩和 (フェーズ4)、
  `match` 構文そのものは引き続き非対応**。Vertex 側で検討していた
  `match g { @{ a -[boss]-> b, b -[boss]-> a } => ... }` のような辺ラベル
  付きパターンマッチは、Rust の安定版では `match` アーム位置に任意の
  カスタム構文を注入できないため今後も実装しません。代わりに、各エッジ
  種別ごとのペアイテレータ `{label}_pairs() -> impl Iterator<Item = (&SrcId, &DstId[, &Attrs])>`
  と、各ノード種別ごとのキー列挙 `{node_snake}_ids()` を追加し、メソッド
  チェーンで同種のクエリ (例: 相互上司の検出) を書けるようにしました
  (独自パーサ・コンビネータ DSL は「独自パーサの再演になる」という
  `../Bullet/docs/rust_graph_extension_sketch.md` の警告に従い採用して
  いません)。
- **エッジ属性のフィールド型に対する derive 制約 (`f64` 等が使えない問題)
  — 解決済み (フェーズ4)**。生成するノード struct・エッジ属性 struct の
  derive から `Eq` を外し `PartialEq` のみにしました。これにより `f64` の
  ような `Eq` を実装できない型もフィールドに使えます (newtype キー型は
  `HashMap` キーとして使うため `Hash + Eq` を維持しています)。
- **`plural_field_name` の素朴な複数形化 — 解決済み (フェーズ4)**。
  `node Category(categories) { .. }` のように `node` 宣言に省略可能な
  `(識別子)` を付けると、内部ストレージのフィールド名をその識別子で
  明示指定できるようにしました。省略時は従来通り素朴な `+ "s"` に
  フォールバックします。
- **`graph!` のエラーメッセージ品質 (未知エッジラベル) — 解決済み
  (フェーズ4、ただしスコープ制約あり)**。`graph_schema!` が
  `__graphite_check_edge_{Schema}!` という宣言的マクロを追加生成し、
  既知のエッジラベルなら何もせず、未知のラベルには「利用可能なエッジ一覧」
  付きの `compile_error!` を出すようにしました。`graph!` は各エッジ行の
  脱糖時にスキーマ名からマクロ名を機械的に導出して呼ぶだけで、スキーマの
  中身 (エッジ一覧) を知る必要はありません。ただし `macro_rules!` は既定で
  テキストスコープ (定義箇所より後、同一クレート内でのみ利用可能) のため、
  **`graph_schema!` と `graph!` を同一モジュール (同一ファイル) 内で使う
  ケースが主な対象**です。別モジュール・別クレートから使う場合は
  `#[macro_export]` や `pub(crate) use` によるスコープ調整が別途必要になり
  ますが、そこまでは対応していません。また、親切なメッセージが出た後も
  ビルダーに対する通常の Rust メソッド解決は引き続き走るため、rustc 標準の
  「メソッドが見つからない」エラーも重ねて出ます (`tests/ui/graph_unknown_edge_label.stderr`
  参照)。
