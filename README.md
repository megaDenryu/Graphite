# Graphite

型付きの図式グラフ (ノード種別・ラベル付きエッジ・多重度) を Rust の型システムに
乗せる proc-macro DSL + ランタイムです。

> **実験的プロジェクトです (v0)。API は予告なく変わります。**

自作言語 Vertex のグラフ機能の設計検討から派生した、独立した Rust プロジェクトです
(Vertex 本体とは切り離されており、Vertex 言語処理系のコードには一切依存しません)。
Vertex 側では「グラフ指向」を独立言語の構文・型システムとして実装する道を選び
ましたが、その設計を壁打ちする過程で「グラフはあくまで既存言語 (Rust) の型
システムと所有権に乗るデータ構造として実装でき、DSL 部分だけを proc マクロ
+ ライブラリとして切り出せるのではないか」という仮説が生まれました。Graphite
はその仮説を検証するプロジェクトです。設計の系譜 (Vertex 側リポジトリの
ドキュメントであり、このリポジトリには含まれません):

- `graph_design_sketches.md` — グラフ型そのものの設計決定
  (ノード同一性、可変性、矢印記法、多重度検査、可視性、型推論)
- `rust_graph_extension_sketch.md` — 上記の決定を Rust の
  proc マクロ + ライブラリとしてどう実現するかの一次資料。`graph_schema!`/
  `graph!` の展開イメージはここで最初に書かれた

v1〜v4.2 の全過程 (何を検討し、何を採用/棄却したか) を通読したい場合は
`docs/history/design_journal.html` (このリポジトリ内のドキュメント) をローカルで
ブラウザで開いてください。

「ある値をグラフの要素 (ノード/エッジ) として書くべきか、それとも普通の
構造体のフィールドとして書くべきか」の判断基準は `docs/modeling_guide.md`
(このリポジトリ内のドキュメント) にまとめています。

**[脱糖リファレンス (`docs/desugaring_reference.md`)](docs/desugaring_reference.md)**
が仕様の正本です。「この構文を書くと、どの普通の Rust の型・値・関数になるのか」を
構文ごとに8段組 (Graphite構文 / 利用者定義 / 公開生成物 / private生成物 /
構築時の処理 / 完成済み Graph の内部保存 / 公開 API / 計算量) で示し、掲載コードは
すべて実在する生成ファイルからの引用に出典の行を併記しています。以下の README の
説明で足りない場合はそちらを参照してください。

## 最小の例

`examples/hello-graph` から抜粋した最小の例です (ノード型2種・属性なしエッジ・
属性ありエッジを1本ずつ)。

```rust
// ノード型・エッジ属性型は普通の Rust struct として宣言する。
// graph_schema! はこれらの型を生成せず、参照するだけ。
pub struct Person { pub name: String }

pub struct Team { pub name: String }

pub struct BossEdge { pub since: i32 }

graphite::graph_schema! {
    generated = "generated/main_org.rs";
    schema Org {
        node Person;
        node Team;

        edge BelongsTo = (member: Person) -> (team: Team) where each member: 1;
        edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod Org {
    include!("generated/main_org.rs");
}

#[rustfmt::skip]
let g = graphite::graph!(Org {
    alice = Person { name: "Alice".into() },
    bob   = Person { name: "Bob".into() },
    eng   = Team { name: "Engineering".into() },

    alice_eng = BelongsTo(alice -> eng),
    bob_eng   = BelongsTo(bob -> eng),
    bob_boss  = Boss(bob -[BossEdge { since: 2021 }]-> alice),
})?;

let alice = g.person_by_id(&Org::PersonId("alice".to_string())).unwrap();
let bob = g.person_by_id(&Org::PersonId("bob".to_string())).unwrap();
let team = alice.belongs_to_as_member().team();
let boss_edge = bob.boss_as_subordinate().unwrap();
let (boss, attrs) = (boss_edge.superior(), boss_edge.payload());
```

`cargo xtask generate` が何を生成するか (newtype キー・builder・辺の第一級型・
違反 enum)、`where` 制約ごとにアクセサが何を返すかは下記「使用例」節で
詳しく説明します。「`edge Kind = ...` とは何を定義しているのか、何ができて
何ができないのか」を実際のコンパイルエラー付きで1つずつ確認したい場合は、
まず `examples/hello-graph` を読んでみてください (下記「実践例」節参照)。

利用者は `graphite` だけに依存します。クレートの分け方とその理由は
`docs/development/crate_architecture.md` に記録しています。

`graph_schema!`/`graph!` の辺は**宣言**(構築時に検証されるデータの繋がり)
ですが、`graphite::flow!` の矢印 `-[関数式]->` は**実行**です — `x -[f]->
y` は `let y = (f)(x);` に即時脱糖するだけの糖衣で、`x -[f]-> y -[g]-> z`
というチェーン形、`(a, b) -[f]-> y` という fan-in (多引数呼び出し) も書けます
(詳細は `docs/flow_macro.md`、動く例は `examples/hello-graph` §5)。

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
    generated = "generated/org_chart.rs";
    schema OrgChart {
        node Employee;
        node Department;

        edge BelongsTo = (employee: Employee) -> (department: Department) where each employee: 1;
        edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee) where each subordinate: 0..1;
        edge Reports = (reporter: Employee) -> (recipient: Employee) where unique pair;
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod OrgChart {
    include!("generated/org_chart.rs");
}
```

`graph_schema!` は宣言を検証し、通常のRust生成ファイルとの指紋一致を検査するため、モジュール直下に書いてください。
参照するノード値型・明示ID型・積み荷型も関数の外に宣言します。関数本体の
ローカル型は生成moduleから参照できません。生成ファイルは、リポジトリルートで`cargo xtask generate`を実行して更新します。
生成規約の正本は`docs/code_generation.md`です。

ノード宣言 `node 型名;` は「マクロの外で宣言済みの struct をこのノード種別
として使う」という参照です。フィールド列を書く場所はありません (値の型は
生成しないので)。

Graphite の基盤は**多重グラフ**です。辺は独立した要素であり、辺種別
(`Kind`) は**新しい nominal 型として生成されます** (透過的な別名ではない
— 同じ形の `Boss` と仮に `Mentor` という別のエッジ種別を宣言したら、両者は
別の型になります)。有向辺宣言は `edge Kind = (始点role: From) -> (終点role: To);`
(積み荷なし) または `edge Kind = (始点role: From) -[積み荷role: 型パス]-> (終点role: To);`
(積み荷あり) の形です。**規則は3つだけ**
(`docs/schema_v4.md` §0):

1. **`名前 = 定義`** — 名前が要る定義は schema もリテラルも全部この形
2. **矢印の中は積み荷だけ** — `-[X]->` の `X` は積み荷の型 (schema) /
   値 (リテラル) だけ。属性なしエッジは矢印の中に何も書かない素の `->`
   になります (「何も運ばない」ことが見た目にそのまま出ます)
3. **`where` は制約** — 制約があるときだけ書く

`where` 節 (省略可、カンマ区切りで複数書ける) が持つ語彙は2つです:

- **`each <役割名>: N | N..M | N..*`** — 端点の役割名ごとの多重度制約。
  始点・終点の両方へ独立に指定できます。無向辺には役割名がないため
  `each` は使えず、制約は `unique pair` のみです。
- **`unique pair`** — 同じ (始点, 終点) の対に2本目の辺を張ることを禁止
  (「関係」らしさ、平行辺の禁止)。`each` の制約と両立させても構いません
  (実装は単純さを優先し、冗長な組み合わせでも警告なく受け付けます)。

制約を何も書かなければ「平行辺を含め自由な多重グラフ」です (旧多重度
`(0..*)` に相当する状態で、専用の字面は無く単に `where` 節を省略します)。

属性型は `edges::BossEdge` のようなモジュール修飾付きパスも書けますが、
**ノード型名は単純な識別子のみ**です (`node Employee;` の `Employee` に
モジュール修飾は書けません)。理由は用途の違いです — ノード型名はエッジの
`from`/`to` 端点の型名と文字列として照合される (`Employee` という同じ
トークンが `node` 宣言と `edge` 宣言の両方に現れて初めて同一ノード種別だと
判定できる) ため、`crate::Employee` のようなパスにすると単純トークン
`Employee` と同一視できず照合が破綻します。モジュール修飾したい場合は
`use` でこのスコープに名前を持ち込んでください。

これで`generated/org_chart.rs`に`OrgChart`のRust module本文が生成されます。その中にノード・辺種別ごとの
newtype キー (`OrgChart::EmployeeId`/`DepartmentId`/`BelongsToId`/`BossId`/`ReportsId`) と構築用の辺値
(`OrgChart::Boss`)・完成済みグラフを読む参照型 (`OrgChart::EmployeeRef<'graph>`/`BossRef<'graph>`)・グラフ本体 (`OrgChart::Graph`、フィールドは非公開)・
builder (`OrgChart::Builder`)・違反 enum (`OrgChart::Violation`) が置かれます。
ノード値の型 (`Employee`/
`Department`) とエッジ属性型 (`BossEdge`) はいずれもユーザーが宣言した型を
そのまま参照するだけで、Graphiteは値型を生成しません。ID型は宣言ごとに選べます。`node Employee;` と `edge Boss = ...;` は schema module 内に型付き文字列IDを生成し、`node Employee(id: EmployeeNumber);` と `edge Boss(id: RelationNumber) = ...;` は既存型を使います。

構築用の辺値は**マクロの外でも普通に構築できます**
(`OrgChart::Boss { subordinate, superior, appointment }`)。端点と積み荷の
公開フィールド名はスキーマの役割名そのものです。完成済みグラフから得る
`BossRef<'graph>` では `edge.subordinate()` / `edge.superior()` が
`EmployeeRef<'graph>` を返し、積み荷は `edge.payload()` で読みます。

以下、ノード種別ごとの `{Node}Ref<'graph>` を NodeRef、辺種別ごとの
`{Kind}Ref<'graph>` を EdgeRef と総称します。

完成済みグラフは、公開IDを種別ごとの非公開な内部位置へ一度だけ変換します。完成済みの辺記録はIDではなく端点位置を保持し、`NodeRef` と `EdgeRef` は `&Graph` と位置だけを持つ `Copy + Clone` の値です。このため、参照を得た後のID・値・端点・積み荷の取得は公開IDのハッシュ検索を繰り返しません。`NodeRef` は `Deref<Target = NodeValue>` を実装しますが、構築用の辺値と完成済みの `EdgeRef` は役割が異なるため、`EdgeRef` は辺値へ `Deref` しません。

`NodeRef` は ID型・値型がどちらも `Debug` を実装するかにかかわらず常に
`Debug` を実装しますが、値を安全に表示できるのはmacro展開時に判定できる
場合 (IDが省略記法による自動生成型のとき) に限られ、それ以外は型名だけを
表示します (ID型・値型へ `Debug` を無条件要求しない契約を守るため)。`EdgeRef`
も同じ方針で、自動生成IDのときだけ `id` を表示します。ノード値型が
`id`/`value` という名のメソッドを持つ場合、`NodeRef` の同名の固有メソッドが
優先されます。値側を呼ぶには `(*node_ref).id()` のように `Deref` させます。

`graph!` の左辺名は、完成後も名前付き要素として残ります。ノードと辺の
どちらも、同じ呼び出し箇所で生成される名前付きラッパーから `Graph` の借用に
束縛された参照値 (NodeRef/EdgeRef) を直接取得できます (用語の定義は
`docs/schema_v4.md` §3.1.1 参照)。

```rust
let graph = graphite::graph!(OrgChart {
    alice @ EmployeeId("external-alice".into()) = Employee { /* ... */ },
    sales = Department { /* ... */ },
    assignment = BelongsTo(alice -> sales),
})?;

graph.alice();      // EmployeeRef<'_>
graph.assignment(); // BelongsToRef<'_>
```

`alice` というRust名、`EmployeeId("external-alice")` という公開ID、凍結後の
内部位置は別概念です。名前付きアクセサはbuilderが挿入時に記録した内部位置を
使うため公開IDのハッシュ表での検索を行いません。実行時にIDしか分からない
場合は `graph.employee_by_id(&id)` を使います。名前付き
ラッパーは `Deref<Target = OrgChart::Graph>` / `DerefMut` を実装するため
既存の借用APIへそのまま渡せます。

名前付きラッパーの型名は呼び出し箇所ローカルであり、外部から書けません。
そのためこの型を関数の引数・戻り値の型として書くことができず、名前付き
ラッパーのまま関数境界を越えることはできません (選択ではなく制約)。関数
境界では `graph.into_graph()` で素の `OrgChart::Graph` へ戻します。
`..items` でスプライスした要素は公開IDを保ちますが、呼び出し箇所に左辺名が
無いため静的アクセサを暗黙生成しません。

**ノード挿入トレイトと総称 `insert`**: builder には型名付きの
挿入メソッド (`b.employee(id, value)` など、上記の各 `node` 宣言から1つずつ
生成) に加えて、総称メソッド `b.insert<N: OrgChart::OrgChartNode>(key: impl Into<String>, value: N) -> N::Id`
も生成されます。これは `graph!` が値の型名を一切パースしないために必要で、
`OrgChart::OrgChartNode` トレイト (各ノード型に `impl OrgChart::OrgChartNode for Employee { .. }`
が生成される) を介して、値の型から正しい内部ストレージへの振り分けを
rustc の型推論に任せます。実行時のリフレクション・型判別は一切無く
(原則5: ゼロコスト志向)、`b.employee(id, value)` を明示的に呼ぶプログラム的
構築 (examples の合成データ生成など) では従来通り型名付きメソッドを使えます。
ノードの**読み取り**API は `Graph` のメソッド
(`graph.employee_by_id`/`employee_ids`/`employee_iter`、後述) が提供します。
エッジの書き込み側も対称に `{Schema}::{Schema}Edge` トレイト経由の総称
`b.add(key, value)` を持ちますが、エッジの**読み取り**API も同じく `Graph` の
メソッド (`graph.boss_by_id`/`boss_iter` 等) と `NodeRef` のメソッド
(`person.boss_as_subordinate()` 等) なので、トレイトの `use` は不要です
(詳しくは次節「アクセサ・アルゴリズムを使う」参照)。

**一括構築 (`extend`)**: 実行時データ (合成データ生成器・CSV 等) から構築する
場合、要素単位の `insert`/`add` に加えて単一の総称メソッド
`b.extend(iter of (key, value))` も生成されます。値の型がノード型か辺種別かは
(他の総称メソッドと同様) rustc の型推論が決めるため、ノード用・辺用の呼び分けは
不要です (旧 `extend_nodes`/`extend_edges` は v4 で `extend` に統一され廃止
されました)。戻り値はキー列 (`Vec<N::Id>`/`Vec<E::Id>`、挿入順)。意味論は
要素単位 API の反復と完全に同一 (重複キー検証・挿入順保持も従来通り) で、
ループを「データを作る部分」だけに留められます (詳細: `docs/bulk_construction.md`、
`docs/graph_splice.md`)。`graph!` リテラル内でも同じ `extend` へ脱糖する
スプライス構文 (`..式`) が使えます (次節「2. `graph!` でインスタンスを組み立てる」
参照)。

**ノード値の型・エッジ属性型に対する trait 要求**: Graphiteの生成コードは
これらの値を builder → 凍結 → アクセサへ move/参照で受け
渡すだけなので、`Clone`/`Debug`/`PartialEq` などの trait を一切要求しません
(自動生成IDには `Debug + Clone + PartialEq + Eq + Hash` を導出します。明示ID型に必要なのは `Clone + Eq + Hash` だけで、`Debug`・`Display`・文字列変換は要求しません。これはID型の話でノード値の型とは別物です。詳細は後述「キーの設計」参照)。
テストでの比較・表示のために `#[derive(Debug, Clone, PartialEq)]` を
付けるかどうかは利用者の自由です (上記の例は付けている例)。
公開される `NodeRef` の `Deref::Target` にノード値型が現れるため、公開schemaで使うノード値型には到達可能な可視性が必要です。通常は例のように `pub struct Employee` と宣言します。

**複数 schema でノード型を共有する場合**: 既定IDは schema module の中に生成されるため、同じ `Person` を参照しても `Org::PersonId` と `Approval::PersonId` は別型です。同じIDを共有したい場合は、両方で `node Person(id: PersonId);` と明示します (`crates/graphite/tests/node_id_shared_across_schemas.rs`)。

schema ごとの生成物は `OrgChart`/`ApprovalFlow` のように別々の Rust module
へ置かれます。同じ `Person` 値型と同じ辺名を共有しても生成型は衝突せず、
問い合わせ先はどちらの `Graph` の `person_by_id(..)` を呼ぶかで一意に
決まります。

**種別APIの主語は `Graph`、探索の主語は `Ref`**: 種別APIとは、ある種別に属する
個体の全体を対象にする読み取り・可変操作 (`graph.boss_by_id`・`graph.boss_iter`
等) のことである (`docs/schema_v4.md` §3.2 と同文)。完成済みの `Graph` が
個体と索引の所有者なので、公開IDからの検索と種別全体への操作は `Graph` の
メソッドとして生えます (`graph.boss_by_id(&id)`、`graph.boss_iter()`)。一度
`NodeRef`/`EdgeRef` を得た後の関係の探索は、その参照が親 `Graph` と内部位置を
保持しているので参照自身のメソッドで辿ります (`person.boss_as_subordinate()`)。
親 `Graph` を引数で渡し直す形は作りません。種別APIは、`graph!` 左辺名から
呼び出し箇所ごとに生成される `graph.alice()` のような名前付き静的アクセサとは
別物です。

種別APIの名前はすべて `{種別名}_{固定接尾辞}` の機械的連結です。同じ種別の
操作が同じ接頭辞で並ぶため、補完で `graph.boss` と打てば辺 `Boss` に対する
操作が一覧に出ます。接尾辞は英語のまま固定で、`bosses()` のような自然言語の
複数形は生成しません。

- **`node.<kind>_as_<role>()`** — 指定した役割で接続する辺を検索します。
  **その役割の `where each` 制約が戻り型を決めます**。

  | 制約             | 戻り値 |
  |------------------|--------|
  | `each X: 1`      | `KindRef<'graph>` |
  | `each X: 0..1`   | `Option<KindRef<'graph>>` |
  | 制約なし          | `impl Iterator<Item = KindRef<'graph>>` |

  戻り値は常に辺参照なので、相手端点・積み荷・辺IDを失いません。複数件は
  問い合わせ時の `Vec` 確保なしで挿入順に走査できます。無向辺は役割名を
  持たないため `node.<kind>_incident()` になり、常にiteratorを返します。
- **`graph.<kind>_by_id(&{Kind}Id)`** — 辺そのものをキー (`{Kind}Id`) で1本
  検索します。見つかれば `Some(KindRef<'graph>)` を返します。ノードも同じく
  `graph.<type>_by_id(&{Type}Id)` です。
- **`a.<kind>_between(b)`** — 2つのNodeRefの対で検索します。主語は位置0側
  (有向辺は始点側、無向辺は唯一の端点型) のNodeRefです。
  `where unique pair` が付いていれば `Option<KindRef<'graph>>`、無ければ平行辺を
  許すためiteratorを返します。異なるGraph由来を `Result` で扱う
  `a.<kind>_try_between(b)` もあります。有向は順序付き、無向は順序なしです。
- **`graph.<kind>_iter()`** — 表全体を `KindRef<'graph>` で走査します。`match`
  パターンでのグラフクエリの代替として使えます。
- **`graph.<kind>_ids()`/`graph.<kind>_len()`** — 全キー列挙 / 本数。
  ノードも同じく `graph.<type>_ids()`/`graph.<type>_len()` です。
- **`graph.<type>_value_mut(&{Type}Id)`/`graph.<kind>_payload_mut(&{Kind}Id)`** — ノード値または辺の積み荷だけを可変参照として取得します。端点と内部位置は変更できません。

可変APIの主語は `&mut Graph` だけです。`NodeRef`/`EdgeRef` は共有借用の
ハンドルなのでそこから可変借用は作れず、引数も公開IDのままにしています
(可変借用中は `Ref` を生かせないため、内部位置をキーにできません)。

**`create_collecting`**: `create` は最初の1件の違反で `Err`
になりますが、組織図の全違反を一覧表示するような検証系ユースケースでは
複数違反をまとめて見たいことがあります。`{Schema}::Graph::create_collecting(|b| { ... }) -> Result<{Schema}::Graph, Vec<{Schema}::Violation>>`
が同じ builder クロージャを受け取り、凍結検査を打ち切らず全違反を
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
    tanaka = Employee { name: "田中".into(), id: 1 },
    sato   = Employee { name: "佐藤".into(), id: 2 },
    sales  = Department { name: "営業".into() },

    tanaka_dept = BelongsTo(tanaka -> sales),
    sato_dept   = BelongsTo(sato -> sales),
    tanaka_boss = Boss(tanaka -[BossEdge { since: 2020 }]-> sato),
})?; // 呼び出し箇所固有の名前付きラッパー (Deref<Target = OrgChart::Graph>)
```

静的項目は `名前 = 値`、またはID値を渡す `名前 @ ID式 = 値` です (`docs/schema_v4.md` §0 規則1)。ノードの名前は
ノード名、辺名は構築中の公開ID束縛であると同時に完成後の静的アクセサ名で、**ノード・辺は1つの
`graph!` 呼び出しの中で単一の平坦な名前空間を共有します** (同じ識別子を
2回使うとコンパイルエラー。詳細は後述「名前空間に関する制約」節)。辺の
リテラル構文は `Kind(from -> to)` / `Kind(from -[積み荷式]-> to)` で、
内部では柄に対応する辺リテラルトレイトの構築関数へ脱糖されます。有向・無向の
柄がスキーマ宣言と一致しなければコンパイルエラーになります。端点はその `graph!` 呼び出し内で
既にノードとして宣言済みのキー識別子でなければなりません。`alice =
alice_value` のように外部で構築済みの値をそのまま渡すこともできます
(ノード項の値・エッジの積み荷はいずれも任意の Rust の式で、値の型は
マクロではなく rustc が推論します)。

```rust
let tanaka_value = Employee { name: "田中".to_string(), id: 1 };
let promotion = BossEdge { since: 2021 };
let g = graphite::graph!(OrgChart {
    tanaka = tanaka_value, // 外で作った値を move
    sato   = Employee { name: "佐藤".into(), id: 2 },
    sales  = Department { name: "営業".into() },

    tanaka_dept = BelongsTo(tanaka -> sales),
    sato_dept   = BelongsTo(sato -> sales),
    sato_boss   = Boss(sato -[promotion]-> tanaka), // 外で作った値を move
})?;
```

`graph!` は `OrgChart::Graph::create_named(|__graphite_b, __graphite_permit| { ... })` と
呼び出し箇所ローカルの名前付きラッパーへ脱糖します。スキーマの中身
(どのエッジが存在するか等) は一切知りません。値の型も一切パースせず、
schema生成コードの総称 `insert_named`/`add_named` メソッド
(下記) へユーザーの式トークンをそのまま渡すだけです (型推論は rustc に
任せる。ゼロコスト志向、原則5)。

ノードキー・辺キーはその場で文字列化するのではなく、キーごとに `let` 束縛を
1つ作り、以後はその識別子への参照として運びます (IDE サポート項目G1、
`docs/development/ide_support_spec.md` 参照)。展開結果はおおよそ次の形になります:

```rust
OrgChart::Graph::create_named(|__graphite_b, __graphite_permit| {
    // (1) 全ノード宣言 (記述順)
    let (tanaka, tanaka_position) =
        __graphite_b.insert_named("tanaka", Employee { .. }, __graphite_permit);
    let (sales, sales_position) =
        __graphite_b.insert_named("sales", Department { .. }, __graphite_permit);
    // (2) 全エッジ (記述順)
    let (tanaka_dept, tanaka_dept_position) = __graphite_b.add_named(
        "tanaka_dept",
        <OrgChart::BelongsTo as graphite::DirectedEdgeLiteral<_, _, _>>::from_graph_literal(
            tanaka.clone(), sales.clone(), (),
        ),
        __graphite_permit,
    );
    let (tanaka_boss, tanaka_boss_position) = __graphite_b.add_named(
        "tanaka_boss",
        <OrgChart::Boss as graphite::DirectedEdgeLiteral<_, _, _>>::from_graph_literal(
            tanaka.clone(), sato.clone(), BossEdge { since: 2020 },
        ),
        __graphite_permit,
    );
    (tanaka_position, sales_position, tanaka_dept_position, tanaka_boss_position)
})
```

成功時の `(Graph, positions)` はローカルの名前付きラッパーへ移され、`g.tanaka()`
等は型付き名前付き位置から参照値を直接作ります。上の展開図では名前付き
ラッパーのstruct/implを
読みやすさのため省略しています。

`insert`/`add` はschema生成コードが各スキーマごとに作る総称メソッドで、
`{Schema}::{Schema}Node`/`{Schema}::{Schema}Edge` トレイト境界を介して値の型から正しい内部
ストレージへ振り分けます (詳細は上記「1. `graph_schema!` でスキーマを
宣言する」節)。`N::Id`/`E::Id` の型は rustc がこの trait 境界から単相化して
決めるため、`let tanaka = ...` の型は `graph!` 自身は一切知りません。

これにより rust-analyzer 上でノードキー・辺キー識別子への定義ジャンプ・
rename・参照検索・hover が「普通のローカル変数」として機能します。`graph!`
はエッジをノード宣言より前に書くこともできますが (キー→宣言の対応表は
全項目を先に走査して作るため)、`let` 束縛は使用より前に必要なので、
展開そのものは記述順によらず「全ノード → 全エッジ」の2段に並べ替えます
(builder の検証は凍結時に行われるため意味論は変わりません)。builder の
クロージャ引数名が `b` ではなく `__graphite_b` なのは、ノードキーに `b` を
使ったときに生成される `let b = ..;` が builder 変数を隠してしまう衝突を
避けるためです。

**スプライス (`..式`)**: 実行時コレクションを静的な項と合成してノード・辺を
追加するには、項の先頭を `..` にします (Rust の struct update 構文
`..rest` の借用):

```rust
let staff: Vec<(String, Employee)> = load_staff();
let deps: Vec<(String, BelongsTo)> = load_belongs_to();

let g = graphite::graph!(OrgChart {
    sales = Department { name: "営業".into() },
    ..staff,
    ..deps,
})?;
```

- 式の型は `IntoIterator<Item = (K, T)>` (`K: Into<String>`)。`T` がノード型か
  辺種別かは (静的な項と同様) rustc の型推論が決めます。
- 脱糖は `__graphite_b.extend(式);` (上記「一括構築」の統一 `extend`)。
  スプライスの要素は静的な項と異なり名前を持たないため、戻り値のキー列は
  捨てます。
- 実行順は「全ノードの `let` 束縛列 → 以降、静的な辺の項とスプライスを記述順」
  です。検証は凍結時に一括で行われるため意味論は順序に依存しませんが、
  制約なしの辺の挿入順保証には記述順がそのまま現れます。
- 詳細は `docs/graph_splice.md` §1 参照。

未知の Kind 名は辺値型の解決をrustcへ委ねることで検出されます (「利用可能な
エッジ一覧」付きの親切な `compile_error!` は無いという意図的なトレードオフ)。
これにより `graph_schema!` と `graph!` を同一ファイルに置く制約も無く、
`graph!` が参照するのは通常の型・メソッドだけです (別モジュールから `use`
すれば足ります。実証は `crates/graphite/tests/graph_cross_module.rs`)。

`Kind(from -> to)` の向きはスキーマの始点と終点の役割名に対応します。上の例の
`edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee)` は
有向辺リテラルの構築関数へ `(subordinate, superior, appointment)` の順で渡されるため、`Boss(tanaka -> sato)` は
「田中の上司は佐藤」を意味します (向きを取り違えやすい点なので、独自
スキーマを書くときは意識してください)。

マクロ呼び出しの中の `-[式]->` は `-`, `[`, .., `]`, `-`, `>` という独自
トークン列のため、rustfmt を混乱させないよう呼び出しには `#[rustfmt::skip]`
を付けることを推奨します。

### 3. アクセサ・アルゴリズムを使う

```rust
let tanaka = g.employee_by_id(&OrgChart::EmployeeId("tanaka".to_string())).unwrap();
let dept_edge = tanaka.belongs_to_as_employee(); // BelongsToRef (each employee: 1)
let boss_edge = tanaka.boss_as_subordinate();    // Option<BossRef>
let reports = tanaka.reports_as_reporter();      // iterator<Item = ReportsRef>

// {kind}_iter(): match パターンの代替。イテレータチェーンでクエリを書く。
// 例: 相互に上司であるペア (A の boss が B かつ B の boss が A) を検出する。
let all: Vec<(&OrgChart::EmployeeId, &OrgChart::EmployeeId)> = g.boss_iter()
    .map(|edge| (edge.subordinate().id(), edge.superior().id()))
    .collect();
let mutual_bosses: Vec<(&OrgChart::EmployeeId, &OrgChart::EmployeeId)> = all
    .iter()
    .copied()
    .filter(|(a, b)| all.contains(&(b, a)))
    .collect();

// graph.{type}_ids(): ノード種別ごとの全キー列挙。
let all_employee_ids: Vec<&EmployeeId> = g.employee_ids().collect();

// graph.{kind}_by_id: 辺キー (newtype) そのもので1本検索する。
let edge: Option<OrgChart::BelongsToRef<'_>> =
    g.belongs_to_by_id(&OrgChart::BelongsToId("tanaka_dept".to_string()));

// create_collecting: 最初の1件で打ち切らず全違反を収集する。
let result: Result<OrgChart::Graph, Vec<OrgChart::Violation>> = OrgChart::Graph::create_collecting(|b| {
    // ... 複数の違反を含みうる構築 ...
});
```

図式グラフ (`graph_schema!`) とは別に、ノード型が 1 種類の同種グラフ用に
ジェネリックな `graphite::Graph<N, E, K>` (水準1相当、petgraph の薄い
ラッパー) も用意しています。`has_cycle`/`topological_sort`/
`topological_levels`/`critical_path_by`/`reachable_from`/`path`/
`out_neighbors`/`in_neighbors`/`map_nodes`/`map_nodes_with_key`/
`filter_nodes`/`filter_nodes_with_key`/`from_edges` などのアルゴリズム・
ヘルパーはこちらに実装されており、通常のRustファイルに生成する図式グラフ
とは独立した別 API です (`crates/graphite/src/graph/`)。

- `in_neighbors(&K) -> Vec<&K>` — `out_neighbors` と対称 (入ってくる辺の
  始点キー一覧)。
- `Graph::<(), (), K>::from_edges(nodes, edges) -> Result<Self, GraphError<K>>` —
  ノードキー集合と `(from, to)` の列から値なしの構造グラフを作る射影用
  ヘルパー。`{label}().iter()` から汎用アルゴリズムへ射影する定型操作向け。
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
の DSLには含めていません。生成した`OrgChart` moduleの私有ストレージ・索引へは
親moduleからアクセスできませんが、`graph.{type}_by_id`/`graph.{kind}_iter`
のような公開クエリAPIだけで導出クエリを書けます。
`impl OrgChart::Graph { pub fn colleagues(&self, ...) -> Vec<&Employee> { ... } }`
のように後から普通のメソッドとして追記してください
(`crates/graphite/tests/orgchart_macro.rs` に実例あり)。

### 4. 制約なしエッジの順序保証

ノード表・辺表 (schema生成コードが使う `graphite::KeyedTable<K, V>`) は
内部的に `Vec<(K, V)>` (挿入順の本体) + `HashMap<K, usize>` (キー→添字の
索引) という構成になっており、**`ids()`/`iter()` は挿入順 (`insert` を
呼んだ順) を保持することを仕様として保証します**
(`crates/graphite/src/keyed_table.rs` 参照)。これにより、制約なしエッジ
(`where` 節を省略した種別) の役割クエリ/`between`/`iter` が返すiteratorも、
同一始点キーに対する複数終点の相対順序が構築時の追加順 (builder の呼び出し
順。`graph!` の場合はソース中の記述順) をそのまま保持します。分岐ノベルの
選択肢表示順のように、順序そのものが意味を持つ場面で安心して依存できます
(`crates/graphite/tests/keyed_table_insertion_order.rs` に回帰テストあり)。

この保証はランタイム移行の初期実装では抜け落ちており (`KeyedTable` が素の
`HashMap` ラッパーで反復順序が未規定だったため)、dialogue-engine の v4 移行
中に「制約なし辺の `of()` の並びがプロセスごとに変わる」flaky なテストとして
発覚し、`KeyedTable` の内部構造を挿入順保持に変更する形で修正・仕様化された
経緯がある (`docs/history/dev_history_2026-07-14_session2.md` §3.10 参照)。

ただし、これは「同一始点キー内での順序」の保証であり、`iter()` が異なる
始点キーをまたいで列挙する順序までは保証しません (始点キーの集合は内部の
`HashMap` 索引で管理されているため)。

### キーの設計 (ノード・エッジの同一性)

グラフ上のノード・エッジの同一性は、宣言ごとに既定IDまたは明示IDで表します。

- `node Person;` は `PersonId(pub String)` を、`edge Knows = ...;` は `KnowsId(pub String)` を、それぞれ schema module 内へ生成します。生成型は `Debug, Clone, PartialEq, Eq, Hash` を導出します。同じ名前でも schema が違えば別型です。
- `node Person(id: EmployeeNumber);` と `edge Knows(id: RelationNumber) = ...;` は既存型を使い、`PersonId` や `KnowsId` を生成しません。明示ID型には `Clone + Eq + Hash` だけが必要です。
- `graph!` の既定ID項は `alice = Person { ... }` と書き、束縛名 `alice` を内部文字列にします。明示ID項は `alice @ EmployeeNumber(42) = Person { ... }` と書きます。`@` の右側は普通のRust式です。明示ID宣言を `@` なしで使うとコンパイルエラーになります。
- 既定IDにも `alice @ Org::PersonId("external-name".into()) = ...` と書けば、束縛名とは別の値を渡せます。
- `insert`・`add`・`extend`・`..式` は文字列から既定IDを作る経路です。明示IDには `insert_with_id`・`add_with_id`、または `graph!` の `@` を使います。スプライス要素は動的ID経路に残り、静的アクセサを暗黙再公開しません。

IDは内部位置ではありません。`KeyedTable` はIDをハッシュキーとして扱い、挿入順は別に保持します。詳細は `docs/node_id_v4_2.md` を参照してください。

### 名前空間に関する制約 (`graph!`)

`graph!` 内の識別子 (`tanaka = Employee { .. }` の `tanaka`、`tanaka_dept =
BelongsTo(..)` の `tanaka_dept` の部分) は**ノード・エッジの種別を跨いで
単一の平坦な名前空間**です (`docs/schema_v4.md` §0 規則1: 全項目が
`名前 = 値` であり、名前は構築時のID束縛と完成後の静的アクセサを兼ねるため)。異なる種別 (例:
`Scene` ノードと `Choice` エッジ) であっても同じ識別子を2回使うと衝突する
ため、命名規約 (プレフィックス等) で回避する必要があります。これは設計上の
既知の制約です。同じ識別子を2回宣言した場合は `syn::Error` (「識別子 `X`
は既に宣言されています」)
がその場でコンパイルエラーとして報告されます
(`crates/graphite/tests/ui/graph_duplicate_node_key.rs` 参照)。

## 実践例 (`examples/`)

`graphite` を実際のアプリケーションから使う例として、`examples/` 配下に
7 本のスタンドアロンクレートを用意しています。いずれも `Cargo.toml` 先頭に
空の `[workspace]` テーブルを置いてルートの Cargo workspace から独立させた、
`graphite` のみに依存する単体アプリです (ルート `cargo test` の対象には
含まれないため、個別に `cd` してビルド・実行します)。

- **`examples/hello-graph/`** — **まずこれ**。入門用の教材example。
  アプリとしての面白さは無く、「`edge Kind = ...` とは何を定義しているのか」
  「`where` 制約ごとにアクセサは何を返すのか」「何ができて何ができないのか
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

### グラフ構文が倒す三つの敵

以下の 3 本は、いずれも同じ型の変換を行っている実証example群です:
**暗黙の制御フローで表現されていた構造を、宣言されたグラフデータに変え、
性質の検証 (循環・到達性・順序) をグラフアルゴリズムに任せる。**

- **`examples/state-machine/`** — ステートマシン地獄 (bool フラグの組合せ
  爆発、または enum + match の散在) を、状態=ノード・**イベント=エッジ
  種別**・決定性=`where each before: 0..1` として再定式化する。到達
  不能状態・行き止まり状態を `reachable_from`/`out_neighbors` で検出する。
  ```powershell
  cd examples/state-machine
  cargo run
  ```
- **`examples/async-dag/`** — 非同期オーケストレーション地獄 (`.await` の
  順序や `spawn` の配線に依存関係が暗黙に溶け込む) を、依存=`DependsOn`
  エッジとして宣言し、循環はハングではなく構築時の `CycleError` に変え、
  `topological_levels` が導く「波」を `std::thread::scope` で実際に並列
  実行する (波分割により実測 1.59 倍の高速化)。
  ```powershell
  cd examples/async-dag
  cargo run
  ```
- **`examples/reactive-cells/`** — リアクティブスパゲッティ (observer
  パターンのグリッチ・無限ループ・登録順依存の非決定性) を、依存=エッジ
  として宣言し、`topological_sort` が導く glitch-free な再計算順で解決
  する。アンチパターン実装 (`antipattern.rs`) をグラフ版と並置して問題を
  実際に再現する。
  ```powershell
  cd examples/reactive-cells
  cargo run
  ```

各ディレクトリの詳細な使い方・サブコマンド一覧は、それぞれの `README.md` を
参照してください。

## 開発者向け

Graphite 自身を実装・保守する人向けの文書は `docs/development/` にあります。
テストの構成と実行手順は `docs/development/testing.md`、生成コードと手書き
テンプレートとの差異は `docs/development/generated_vs_handwritten.md`、
クレートの分け方は `docs/development/crate_architecture.md` を参照して
ください。フェーズ3終了時点の未決事項とフェーズ4の対応関係は
`docs/history/phase4_open_questions.md` に記録しています。

## ライセンス

ライセンス未定 (TBD)。
