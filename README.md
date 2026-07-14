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
```

図式グラフ (`graph_schema!`) とは別に、ノード型が 1 種類の同種グラフ用に
ジェネリックな `graphite::Graph<N, E, K>` (水準1相当、petgraph の薄い
ラッパー) も用意しています。`has_cycle`/`topological_sort`/`reachable_from`/
`path`/`map_nodes`/`filter_nodes` などのアルゴリズムはこちらに実装されて
おり、`graph_schema!` が生成する図式グラフとは独立した別 API です
(`crates/graphite/src/graph.rs`)。

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

## 手書きテンプレートとの差異

`graph_schema!` は基本的に `orgchart_handwritten.rs` と同じ形を生成します
が、「任意のノード型・エッジ型の組み合わせ」に一般化する過程でいくつか
手書き版と異なる設計判断をしています。

1. **違反 enum は 1 スキーマにつき 1 つ生成** (`{Schema}Violation`)。手書き
   版は `SchemaViolation` という固定名でしたが、複数のスキーマを同じモジュール
   に宣言したときに型名が衝突しないよう、スキーマ名をプレフィックスにして
   います。
2. **`MultiplicityViolation` の違反キーは `source: String` (`Debug` 表現)**。
   手書き版は `employee: EmployeeId` と型付きでしたが、一般のスキーマでは
   エッジごとに始点ノード型が異なりうる (例: `A -> B` と `C -> D` が両方
   `MultiplicityViolation` を返しうる) ため、型を 1 つに固定できません。
   `format!("{:?}", key)` で妥協しています。型安全性は下がりますが、
   `edge` フィールド (`&'static str`) で「どのエッジ種別の違反か」は判別
   できます。
3. **builder のエッジ追加メソッドの引数名は汎用的に `from`/`to`**。手書き版
   は `boss(employee, boss, attrs)`・`reports(manager, report)` のように
   ドメイン語で命名されていましたが、マクロはノード型名だけから引数名を
   導出する必要があり、自己参照エッジ (`Employee -> Employee`) では同名
   引数の衝突を避けられないため、常に `from`/`to` にしています。
4. **内部ストレージの複数形フィールド名は素朴な英語複数形 (`+ "s"`)**。
   不規則複数形 (`Category` → `Categorys` になってしまう等) には対応して
   いません。この名前は非公開フィールドで利用者から見えないため機能上の
   問題はありませんが、生成コードを `cargo expand` 等で目視する際は
   注意してください。
5. **導出エッジ (`colleagues` 等) はマクロが生成しない**。上記「使用例 3」
   参照。

## 未決事項 / フェーズ4があるとしたら

- **多重度 `(1)` アクセサへ未知キーを渡した場合は v0 ではパニックとする**。
  `Vec` の範囲外添字アクセスと同じ「呼び出し規約違反」として扱っています
  (このスキーマが発行したキーだけを渡すことが呼び出し側の責務)。将来
  `Result`/`Option` を返す設計に変える余地はありますが、多重度 `(1)` の
  「必ず存在する」という保証と矛盾するため、現状は据え置いています。
- **`match` パターンでのクエリは非対応**。Vertex 側で検討していた
  `match g { @{ a -[boss]-> b, b -[boss]-> a } => ... }` のような辺ラベル
  付きパターンマッチは、Rust の安定版では `match` アーム位置に任意の
  カスタム構文を注入できないため実装していません。メソッドチェーン
  (`g.boss(id)` 等) と `if let` の組み合わせで妥協しています
  (`../Bullet/docs/rust_graph_extension_sketch.md` の該当節を参照)。
- **エッジの重複宣言・ノード型の重複宣言はエラーにするが、エッジ属性の
  フィールド型に対する制約は特に検査していない**。例えば `Eq` を実装
  できない型 (`f64` 等) をフィールドに使うと、生成された struct の
  `#[derive(PartialEq, Eq)]` がコンパイルエラーになります。これは
  Rust の通常の derive エラーとして表面化するため、マクロ側で追加の
  検査は行っていません。
- **`plural_field_name` の素朴な複数形化**。上記「差異」の 4 番を参照。
  複数形化のルールをユーザーが上書きできる構文 (`node Employee(employees) { .. }`
  のような明示的複数形指定) を足すのがフェーズ4の候補です。
- **`graph!` のエラーメッセージ品質**。未宣言ノードキーの参照は独自の
  `syn::Error` で親切なメッセージを出しますが、存在しないエッジ種別の
  参照は rustc 標準の「メソッドが見つからない」に委ねています
  (`tests/ui/graph_unknown_edge_label.stderr` 参照)。`graph!` にスキーマの
  エッジ一覧を伝える仕組み (別マクロでの登録・トレイトオブジェクト経由の
  イントロスペクション等) を作れば改善できますが、複雑さとのトレードオフ
  です。
