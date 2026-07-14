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
/// ノード型。`graph_schema!` の外で普通の struct として宣言する。
/// `graph_schema!` はこの型を生成せず、参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub name: String,
    pub id: u32,
}

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Department {
    pub name: String,
}

/// `boss` エッジの属性。同様に `graph_schema!` の外で宣言する。
#[derive(Debug, Clone, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}

graphite::graph_schema! {
    schema OrgChart {
        node Employee;
        node Department;

        edge Employee -[belongs_to]-> Department (1);
        edge Employee -[boss: BossEdge]-> Employee (0..1);
        edge Employee -[reports]-> Employee (0..*);
    }
}
```

ノード宣言 `node 型名;` は「マクロの外で宣言済みの struct をこのノード種別
として使う」という参照です。フィールド列を書く場所はありません (値の型は
生成しないので)。省略可能な `node 型名(複数形);` で内部ストレージの複数形
フィールド名を明示指定できます (後述)。

エッジ宣言の矢印の中はラベル (属性なし、例: `belongs_to`) または
`ラベル: 型パス` (属性あり、例: `boss: BossEdge`)。属性型は `edges::BossEdge`
のようなモジュール修飾付きパスも書けますが、**ノード型名は単純な識別子のみ**
です (`node Employee;` の `Employee` にモジュール修飾は書けません)。理由は
用途の違いです — ノード型名はエッジの `from`/`to` 端点の型名と文字列として
照合される (`Employee` という同じトークンが `node` 宣言と `edge` 宣言の両方に
現れて初めて同一ノード種別だと判定できる) ため、`crate::Employee` のような
パスにすると単純トークン `Employee` と同一視できず照合が破綻します。
モジュール修飾したい場合は `use` でこのスコープに名前を持ち込んでください。
多重度は矢印の外側 (辺そのものではなく制約なので) に置きます。

これでノード種別ごとの newtype キー (`EmployeeId`/`DepartmentId`)・
スキーマ struct (`OrgChart`, フィールドは非公開)・builder
(`OrgChartBuilder`)・違反 enum (`OrgChartViolation`) が一式生成されます。
ノード値の型 (`Employee`/`Department`) とエッジ属性型 (`BossEdge`) は
いずれもユーザーが宣言した型をそのまま参照するだけで、`graph_schema!` は
一切生成しません。

**ノード値の型・エッジ属性型に対する trait 要求**: `graph_schema!`/`graph!`
の生成コードはこれらの値を builder → freeze → アクセサへ move/参照で受け
渡すだけなので、`Clone`/`Debug`/`PartialEq` などの trait を一切要求しません
(newtype キー型は内部で `HashMap` のキーに使うため `Hash + Eq` を要求します
が、これはキー型の話でノード値の型とは別物です)。テストでの比較・表示の
ために `#[derive(Debug, Clone, PartialEq)]` を付けるかどうかは利用者の
自由です (上記の例は付けている例)。

**同一モジュール内で複数 schema がノード型を共有する場合の制約**: 同じ
struct を複数の schema が `node` として参照すると、両方の schema が同じ
`{Node}Id` newtype を生成しようとして名前衝突します。schema ごとにモジュール
を分けて運用してください。

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

ノード値の型・エッジ属性型はいずれもユーザー宣言なので、`f64` のように `Eq`
を実装できないフィールド型を持たせるかどうかも含めて derive 方針は完全に
利用者の自由です (上記の「ノード値の型・エッジ属性型に対する trait 要求」
参照。newtype キー型だけは内部で `HashMap` のキーに使うため `Hash + Eq` を
要求します)。

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

`graph!` は `OrgChart::create(|__graphite_b| { ... })` の呼び出し列へ脱糖する
だけで、スキーマの中身 (どのエッジが存在するか等) は一切知りません。ノード
宣言行 `key: Type { .. }` の型名から builder メソッド名・newtype キー型名を
`graph_schema!` と同じ命名規則 (snake_case / `{Type}Id`) で機械的に導出し、
辺の端点キーの型はその場で作った「識別子 → 宣言時の型名」対応表から逆引き
します。

ノードキーはその場で文字列化するのではなく、キーごとに `let` 束縛を1つ作り、
以後はその識別子への参照として運びます (IDE サポート項目G1、
`docs/ide_support_spec.md` 参照)。展開結果はおおよそ次の形になります:

```rust
OrgChart::create(|__graphite_b| {
    // (1) 全ノード宣言 (記述順)
    let tanaka = EmployeeId("tanaka".to_string());
    __graphite_b.employee(tanaka.clone(), Employee { .. });
    let sales = DepartmentId("sales".to_string());
    __graphite_b.department(sales.clone(), Department { .. });
    // (2) 全エッジ (記述順)
    __graphite_edge_OrgChart!(check belongs_to);
    __graphite_b.belongs_to(tanaka.clone(), sales.clone());
})
```

属性付きエッジ (`tanaka -[boss { since: 2020 }]-> sato`) の場合、属性の
struct リテラルへの展開もハンドシェイクマクロが担います:

```rust
__graphite_edge_OrgChart!(check boss);
__graphite_b.boss(
    tanaka.clone(),
    sato.clone(),
    __graphite_edge_OrgChart!(attrs boss { since: 2020 })
);
```

これにより rust-analyzer 上でノードキー識別子への定義ジャンプ・rename・
参照検索・hover が「普通のローカル変数」として機能します。`graph!` は
従来エッジをノード宣言より前に書くこともできますが (キー→型の逆引き表は
全項目を先に走査して作るため)、`let` 束縛は使用より前に必要なので、
展開そのものは記述順によらず「全ノード → 全エッジ」の2段に並べ替えます
(builder の検証は freeze 時に行われるため意味論は変わりません)。builder の
クロージャ引数名が `b` ではなく `__graphite_b` なのは、ノードキーに `b` を
使ったときに生成される `let b = ..;` が builder 変数を隠してしまう衝突を
避けるためです。

`-[label]->` の向きは「`from` = 辺ラベルの builder 引数の 1 番目、`to` = 2
番目」に対応します。上の例の `edge Employee -[boss: BossEdge]-> Employee`
は手書きテンプレートの `boss(employee, boss, attrs)` という引数順を踏襲
しているため、
`tanaka -[boss]-> sato` は「田中の上司は佐藤」を意味します (向きを取り違え
やすい点なので、独自スキーマを書くときは意識してください)。

マクロ呼び出しの中の `-[label]->` は `-`, `[`, ident, `]`, `-`, `>` という
独自トークン列のため、rustfmt を混乱させないよう呼び出しには
`#[rustfmt::skip]` を付けることを推奨します。

### 3. アクセサ・アルゴリズムを使う

```rust
let dept = g.belongs_to(&EmployeeId("tanaka".to_string())); // &Department (多重度 (1))
let (boss, attrs) = g.boss(&EmployeeId("tanaka".to_string())).unwrap(); // Option<(&Employee, &BossEdge)>
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

### 4. `(0..*)` エッジの順序保証 (項目i、フェーズ5)

多重度 `(0..*)` のエッジは内部で `HashMap<SrcId, Vec<DstId>>` (属性付きは
`Vec<(DstId, Attrs)>`) として格納されます。**同一始点キーに対する複数
終点の相対順序は、構築時の追加順 (builder の呼び出し順。`graph!` の場合は
ソース中の記述順) をそのまま保持することを仕様として保証します。** これは
実装詳細ではなく、`{label}`/`{label}_ids`/`{label}_pairs()` いずれの
アクセサでも成り立つ保証です (`crates/graphite/tests/orgchart_macro.rs`
の `reports_ids` 順序テスト参照)。分岐ノベルの選択肢表示順のように、
順序そのものが意味を持つ場面で安心して依存できます。

ただし、これは「同一始点キー内での順序」の保証であり、`{label}_pairs()`
が異なる始点キーをまたいで列挙する順序までは保証しません (始点キーの
集合は `HashMap` で管理されているため)。

### 名前空間に関する制約 (`graph!`)

`graph!` 内のノード識別子 (`tanaka: Employee { .. }` の `tanaka` の部分) は
**ノード型を跨いで単一の平坦な名前空間**です。異なるノード型 (例:
`Scene` と `Ending`) であっても同じ識別子を2回使うと衝突するため、
命名規約 (プレフィックス等) で回避する必要があります。これは設計上の
既知の制約であり、型ごとに名前空間を分ける再設計はフェーズ5では見送り
ました (`docs/phase5_candidates.md` 項目h)。代わりに、同じ識別子を2回
ノード宣言した場合は `syn::Error` (「識別子 `X` は既に宣言されています」)
がその場でコンパイルエラーとして報告されます
(`crates/graphite/tests/ui/graph_duplicate_node_key.rs` 参照)。

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
4 本のスタンドアロンクレートを用意しています。いずれも `Cargo.toml` 先頭に
空の `[workspace]` テーブルを置いてルートの Cargo workspace から独立させた、
`graphite` のみに依存する単体アプリです (ルート `cargo test` の対象には
含まれないため、個別に `cd` してビルド・実行します)。

- **`examples/hello-graph/`** — **まずこれ**。入門用の教材example。
  アプリとしての面白さは無く、「ラベルとは何なのか (変数か関数か)」
  「多重度ごとにアクセサは何を返すのか」「何ができて何ができないのか
  (実際のコンパイルエラー付き)」を最小の題材で1つずつ確認する。
  ```powershell
  cd examples/hello-graph
  cargo run
  ```
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

## IDE サポート (rust-analyzer)

`examples/` 配下はルート Cargo workspace から独立したスタンドアロンクレート
ですが、`.vscode/settings.json` の `rust-analyzer.linkedProjects` で明示的に
リンクしているため、VSCode で開けば通常のクレートと同様に rust-analyzer の
解析対象になります。今後 example を追加したときは `linkedProjects` に 1 行
足すことを運用ルールとします。詳細は `docs/ide_support_spec.md` を参照して
ください。

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
   `edge Employee -[belongs_to]-> Department (1)` からは
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
   際は注意してください。**明示指定構文を追加済み** (`node Category(categories);`
   のように `node` 宣言に `(識別子)` を付けると内部フィールド名を上書き
   できる)。
5. **導出エッジ (`colleagues` 等) はマクロが生成しない**。上記「使用例 3」
   参照。
6. **ノード値の型・エッジ属性型はいずれもユーザーが `graph_schema!` の外で
   宣言し、マクロは参照するだけ**。手書き版は `pub struct Employee { .. }` /
   `pub struct BossAttrs { pub since: i32 }` をテンプレート内に直接書いて
   いましたが、マクロはこれらの型を一切生成せず、スキーマ宣言
   (`node Employee;` / `edge Employee -[boss: BossEdge]-> Employee (0..1);`)
   に書かれた型をそのまま参照します。派生する trait 要求も無い (上記
   「ノード値の型・エッジ属性型に対する trait 要求」参照) ため、derive する
   かどうかも含めて完全に利用者の自由です。

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
- **エッジ属性・ノード値のフィールド型に対する derive 制約 (`f64` 等が使えない
  問題) — 解決済み**。ノード値の型・エッジ属性型はいずれもユーザー宣言なので、
  マクロからの derive 制約はそもそも存在しません。`f64` のような `Eq` を
  実装できない型を使うかどうかも含めて derive 方針は利用者の自由です
  (newtype キー型だけは `HashMap` キーとして使うため `Hash + Eq` を要求します)。
- **`plural_field_name` の素朴な複数形化 — 解決済み**。
  `node Category(categories);` のように `node` 宣言に省略可能な
  `(識別子)` を付けると、内部ストレージのフィールド名をその識別子で
  明示指定できるようにしました。省略時は従来通り素朴な `+ "s"` に
  フォールバックします。
- **`graph!` のエラーメッセージ品質 (未知エッジラベル) — 解決済み
  (ただしスコープ制約あり)**。`graph_schema!` が `__graphite_edge_{Schema}!`
  という宣言的マクロを追加生成します。このマクロは2つの役割を持つ
  ハンドシェイクマクロです: `check` アームは既知のエッジラベルなら何もせず、
  未知のラベルには「利用可能なエッジ一覧」付きの `compile_error!` を出す。
  `attrs` アームは属性付きエッジの struct リテラルへの展開を担う
  (`docs/edge_syntax_v2.md` 3.1 参照)。`graph!` は各エッジ行の脱糖時に
  スキーマ名からマクロ名を機械的に導出して呼ぶだけで、スキーマの中身
  (エッジ一覧・属性型) を知る必要はありません。ただし `macro_rules!` は既定
  でテキストスコープ (定義箇所より後、同一クレート内でのみ利用可能) のため、
  **`graph_schema!` と `graph!` を同一モジュール (同一ファイル) 内で使う
  ケースが主な対象**です。別モジュール・別クレートから使う場合は
  `#[macro_export]` や `pub(crate) use` によるスコープ調整が別途必要になり
  ますが、そこまでは対応していません。また、親切なメッセージが出た後も
  ビルダーに対する通常の Rust メソッド解決は引き続き走るため、rustc 標準の
  「メソッドが見つからない」エラーも重ねて出ます (`tests/ui/graph_unknown_edge_label.stderr`
  参照)。
