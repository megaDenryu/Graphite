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

「ある値をグラフの要素 (ノード/エッジ) として書くべきか、それとも普通の
構造体のフィールドとして書くべきか」の判断基準は `docs/modeling_guide.md`
(このリポジトリ内のドキュメント) にまとめています。

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
    schema Org {
        node Person;
        node Team;

        edge BelongsTo = Person -> Team              where each Person: 1;
        edge Boss      = Person -[BossEdge]-> Person where each Person: 0..1;
    }
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

let team: &Team = BelongsTo::of(&g, &PersonId("alice".to_string()));
let (boss, attrs) = Boss::of(&g, &PersonId("bob".to_string())).unwrap(); // (&Person, &BossEdge)
```

`graph_schema!` が何を生成するか (newtype キー・builder・辺の第一級型・
違反 enum)、`where` 制約ごとにアクセサが何を返すかは下記「使用例」節で
詳しく説明します。「`edge Kind = ...` とは何を定義しているのか、何ができて
何ができないのか」を実際のコンパイルエラー付きで1つずつ確認したい場合は、
まず `examples/hello-graph` を読んでみてください (下記「実践例」節参照)。

## 2 クレート構成

```
crates/graphite/         # ランタイムクレート。利用者が唯一 depend するクレート
crates/graphite-macros/  # proc-macro クレート (graph_schema!, graph! を実装する)
```

proc-macro クレート (`proc-macro = true`) は手続き型マクロ = コンパイラ
プラグインの一種であり、生成する側 (マクロ) と生成されたコードが依存する側
(ランタイム型) を同じクレートに置けないという Rust の技術的制約のため 2 分割
している (serde/serde_derive、diesel、sqlx と同型)。利用者は `graphite` だけ
に依存し、マクロは `graphite::graph_schema!` / `graphite::graph!` /
`graphite::flow!` として re-export されたものを使います。

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
    schema OrgChart {
        node Employee;
        node Department;

        edge BelongsTo = Employee -> Department              where each Employee: 1;
        edge Boss      = Employee -[BossEdge]-> Employee     where each Employee: 0..1;
        edge Reports   = Employee -> Employee                where unique pair;
    }
}
```

ノード宣言 `node 型名;` は「マクロの外で宣言済みの struct をこのノード種別
として使う」という参照です。フィールド列を書く場所はありません (値の型は
生成しないので)。省略可能な `node 型名(複数形);` で内部ストレージの複数形
フィールド名を明示指定できます (後述)。

Graphite の基盤は**多重グラフ**です。辺は独立した要素であり、辺種別
(`Kind`) は**新しい nominal 型として生成されます** (透過的な別名ではない
— 同じ形の `Boss` と仮に `Mentor` という別のエッジ種別を宣言したら、両者は
別の型になります)。辺宣言は `edge Kind = From -> To;` (属性なし) または
`edge Kind = From -[型パス]-> To;` (属性あり、例: `Boss = Employee
-[BossEdge]-> Employee`) の形です。**規則は3つだけ**
(`docs/schema_v4.md` §0):

1. **`名前 = 定義`** — 名前が要る定義は schema もリテラルも全部この形
2. **矢印の中は積み荷だけ** — `-[X]->` の `X` は積み荷の型 (schema) /
   値 (リテラル) だけ。属性なしエッジは矢印の中に何も書かない素の `->`
   になります (「何も運ばない」ことが見た目にそのまま出ます)
3. **`where` は制約** — 制約があるときだけ書く

`where` 節 (省略可、カンマ区切りで複数書ける) が持つ語彙は2つです:

- **`each <FromType>: 1`** — 各始点ノードにつきちょうど1本 (数学的には
  全域関数)。`<FromType>` は宣言の `From` と一致している必要があります
  (始点と終点が同型の自己参照エッジでも「each = 始点側の出次数」と読みます)。
- **`each <FromType>: 0..1`** — 各始点につき高々1本 (部分関数)。
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

これでノード種別ごとの newtype キー (`EmployeeId`/`DepartmentId`)・
辺種別ごとの newtype キー (`BelongsToId`/`BossId`/`ReportsId`) と
タプル struct (`pub struct Boss(pub EmployeeId, pub EmployeeId,
pub BossEdge);` — 属性なしは2要素)・スキーマ struct (`OrgChart`,
フィールドは非公開)・builder (`OrgChartBuilder`)・違反 enum
(`OrgChartViolation`) が一式生成されます。ノード値の型 (`Employee`/
`Department`) とエッジ属性型 (`BossEdge`) はいずれもユーザーが宣言した型を
そのまま参照するだけで、`graph_schema!` は一切生成しません。

辺のタプル struct は**マクロの外でも普通に構築できます**
(`Boss(from_id, to_id, payload)`。原則6: 消去可能な拡張のみ)。読み取りは
位置 (`.0`/`.1`/`.2`) を人間に晒さず、固定語彙のメソッドを生成します:
`fn from(&self) -> &EmployeeId` / `fn to(&self) -> &EmployeeId` /
`fn payload(&self) -> &BossEdge` (積み荷ありのみ)。

**`{Schema}Node` トレイトと総称 `insert`**: builder には型名付きの
挿入メソッド (`b.employee(id, value)` など、上記の各 `node` 宣言から1つずつ
生成) に加えて、総称メソッド `b.insert<N: OrgChartNode>(key: impl Into<String>, value: N) -> N::Id`
も生成されます。これは `graph!` が値の型名を一切パースしないために必要で、
`OrgChartNode` トレイト (各ノード型に `impl OrgChartNode for Employee { type Id = EmployeeId; .. }`
が生成される) を介して、値の型から正しい内部ストレージへの振り分けを
rustc の型推論に任せます。実行時のリフレクション・型判別は一切無く
(原則5: ゼロコスト志向)、`b.employee(id, value)` を明示的に呼ぶプログラム的
構築 (examples の合成データ生成など) では従来通り型名付きメソッドを使えます。
同じトレイトはノードの**読み取り**API (`Person::get`/`ids`/`iter`、後述) も
提供します。エッジの書き込み側も対称に `{Schema}Edge` トレイト経由の総称
`b.add(key, value)` を持ちますが、エッジの**読み取り**API (`of`/`get`/
`between`/`iter`/`ids`/`len`) はマクロが生成した `Kind` 型そのものへの
固有 impl (`impl Boss { .. }`) として提供されるため、トレイトの `use` は
不要です (詳しくは次節「アクセサ・アルゴリズムを使う」参照)。

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

**アクセスは型名前空間の関連関数 (`g.メソッド` は廃止)**: v3 にあった
「ラベルごとに1個のビューを返すメソッド `{label}()`」という間接層は v4 では
無くなりました。辺の読み取りAPI (`of`/`get`/`between`/`iter`/`ids`/`len`) は
`graph_schema!` が生成した `Kind` 型そのものへの固有 impl として直接生えます
(`Boss::of(&g, ..)` のように、型名を主語にして呼びます)。ノードの読み取り
API (`get`/`ids`/`iter`) は `{Schema}Node` トレイトの関連関数として提供され、
使う前にそのトレイトを `use` でスコープに入れる必要があります
(`use crate::schema::OrgChartNode;` のように。ユーザー struct
(`Employee` 等) への固有 impl にはしていません — 複数 schema が同じ struct
を `node` として共有したときにメソッドが衝突しないようにするためです)。

- **`Kind::of(&g, &SrcId)`** — そのエッジ種別の自然な戻り値。
  **`where` 制約が戻り型を決めます**:

  | 制約             | `of` の戻り値                                              |
  |------------------|-------------------------------------------------------------|
  | `each X: 1`      | `&T` (属性付きは `(&T, &Attrs)`)。未知キーはパニック (非パニック版 `get_of` あり) |
  | `each X: 0..1`   | `Option<&T>` (属性付きは `Option<(&T, &Attrs)>`)             |
  | 制約なし          | `Vec<&T>` (属性付きは `Vec<(&T, &Attrs)>`)                   |

  `each X: 1` の `of` は未知キーを渡すとパニックします (`Vec` の `v[i]` と
  同じ「呼び出し規約違反」の扱い。非パニック版 `get_of` も対で提供されます)。
- **`Kind::get(&g, &{Kind}Id)`** — 辺そのものをキー (`{Kind}Id`) で1本検索
  します。見つかれば `Some(&Kind)` (from/to/payload を持つタプル struct)。
- **`Kind::between(&g, &SrcId, &DstId)`** — (始点, 終点) の対で検索します。
  `where unique pair` が付いていれば `Option<&Kind>`、無ければ平行辺を
  許すため `Vec<&Kind>` を返します。
- **`Kind::iter(&g)`** — 表全体を `(&{Kind}Id, &Kind)` で走査します。`match`
  パターンでのグラフクエリの代替として使えます。
- **`Kind::ids(&g)`/`Kind::len(&g)`** — 全キー列挙 / 本数。

ノード種別ごとのキー列挙は `{Type}::ids(&g)` (`{Schema}Node` トレイト経由)
です。

**`create_collecting`**: `create` は最初の1件の違反で `Err`
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
    tanaka = Employee { name: "田中".into(), id: 1 },
    sato   = Employee { name: "佐藤".into(), id: 2 },
    sales  = Department { name: "営業".into() },

    tanaka_dept = BelongsTo(tanaka -> sales),
    sato_dept   = BelongsTo(sato -> sales),
    tanaka_boss = Boss(tanaka -[BossEdge { since: 2020 }]-> sato),
})?; // Result<OrgChart, OrgChartViolation>
```

**全行が `名前 = 値`** です (`docs/schema_v4.md` §0 規則1)。ノードの名前は
ノードキー、辺の名前は辺キーの束縛であり、**ノードキー・辺キーは1つの
`graph!` 呼び出しの中で単一の平坦な名前空間を共有します** (同じ識別子を
2回使うとコンパイルエラー。詳細は後述「名前空間に関する制約」節)。辺の
コンストラクタはタプル struct の顔 `Kind(from -> to)` /
`Kind(from -[積み荷式]-> to)` で、`from`/`to` はその `graph!` 呼び出し内で
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

`graph!` は `OrgChart::create(|__graphite_b| { ... })` の呼び出し列へ脱糖する
だけで、スキーマの中身 (どのエッジが存在するか等) は一切知りません。値の型も
一切パースせず、`graph_schema!` が生成した総称 `insert`/`add` メソッド
(下記) へユーザーの式トークンをそのまま渡すだけです (型推論は rustc に
任せる。ゼロコスト志向、原則5)。

ノードキー・辺キーはその場で文字列化するのではなく、キーごとに `let` 束縛を
1つ作り、以後はその識別子への参照として運びます (IDE サポート項目G1、
`docs/ide_support_spec.md` 参照)。展開結果はおおよそ次の形になります:

```rust
OrgChart::create(|__graphite_b| {
    // (1) 全ノード宣言 (記述順)
    let tanaka = __graphite_b.insert("tanaka", Employee { .. });
    let sales = __graphite_b.insert("sales", Department { .. });
    // (2) 全エッジ (記述順)
    let tanaka_dept = __graphite_b.add("tanaka_dept", BelongsTo(tanaka.clone(), sales.clone()));
    let tanaka_boss = __graphite_b.add("tanaka_boss", Boss(tanaka.clone(), sato.clone(), BossEdge { since: 2020 }));
})
```

`insert`/`add` は `graph_schema!` が各スキーマごとに生成する総称メソッドで、
`{Schema}Node`/`{Schema}Edge` トレイト境界を介して値の型から正しい内部
ストレージへ振り分けます (詳細は上記「1. `graph_schema!` でスキーマを
宣言する」節)。`N::Id`/`E::Id` の型は rustc がこの trait 境界から単相化して
決めるため、`let tanaka = ...` の型は `graph!` 自身は一切知りません。

これにより rust-analyzer 上でノードキー・辺キー識別子への定義ジャンプ・
rename・参照検索・hover が「普通のローカル変数」として機能します。`graph!`
はエッジをノード宣言より前に書くこともできますが (キー→宣言の対応表は
全項目を先に走査して作るため)、`let` 束縛は使用より前に必要なので、
展開そのものは記述順によらず「全ノード → 全エッジ」の2段に並べ替えます
(builder の検証は freeze 時に行われるため意味論は変わりません)。builder の
クロージャ引数名が `b` ではなく `__graphite_b` なのは、ノードキーに `b` を
使ったときに生成される `let b = ..;` が builder 変数を隠してしまう衝突を
避けるためです。

未知の Kind 名は `#kind(..)` というタプル struct 構築式がそのまま rustc の
cannot-find-type / no-such-function に落ちることで検出されます (「利用可能な
エッジ一覧」付きの親切な `compile_error!` は無いという意図的なトレードオフ)。
これにより `graph_schema!` と `graph!` を同一ファイルに置く制約も無く、
`graph!` が参照するのは通常の型・メソッドだけです (別モジュールから `use`
すれば足ります。実証は `crates/graphite/tests/graph_cross_module.rs`)。

`Kind(from -> to)` の向きは「`from` = タプル struct の1番目、`to` = 2番目」に
対応します。上の例の `edge Boss = Employee -[BossEdge]-> Employee` は
`Boss(from, to, attrs)` という構築順のため、`Boss(tanaka -> sato)` は
「田中の上司は佐藤」を意味します (向きを取り違えやすい点なので、独自
スキーマを書くときは意識してください)。

マクロ呼び出しの中の `-[式]->` は `-`, `[`, .., `]`, `-`, `>` という独自
トークン列のため、rustfmt を混乱させないよう呼び出しには `#[rustfmt::skip]`
を付けることを推奨します。

### 3. アクセサ・アルゴリズムを使う

```rust
let dept = BelongsTo::of(&g, &EmployeeId("tanaka".to_string())); // &Department (each 1)
let (boss, attrs) = Boss::of(&g, &EmployeeId("tanaka".to_string())).unwrap(); // Option<(&Employee, &BossEdge)>
let reports = Reports::of(&g, &EmployeeId("tanaka".to_string())); // Vec<&Employee> (制約なし)

// get_of: each 1 の非パニック版。未知キーは None に落ちる。
let dept_opt = BelongsTo::get_of(&g, &EmployeeId("no-such-id".to_string())); // None

// iter(): match パターンの代替。イテレータチェーンでクエリを書く。
// 例: 相互に上司であるペア (A の boss が B かつ B の boss が A) を検出する。
let all: Vec<(&EmployeeId, &EmployeeId)> = Boss::iter(&g)
    .map(|(_id, edge)| (edge.from(), edge.to()))
    .collect();
let mutual_bosses: Vec<(&EmployeeId, &EmployeeId)> = all
    .iter()
    .copied()
    .filter(|(a, b)| all.contains(&(b, a)))
    .collect();

// {Type}::ids(&g): ノード種別ごとの全キー列挙 ({Schema}Node トレイトの関連関数)。
let all_employee_ids: Vec<&EmployeeId> = Employee::ids(&g).collect();

// Kind::get: 辺キー (newtype) そのもので1本検索する。
let edge: Option<&BelongsTo> = BelongsTo::get(&g, &BelongsToId("tanaka_dept".to_string()));

// create_collecting: 最初の1件で打ち切らず全違反を収集する。
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
の DSLには含めていません。生成された struct は同一モジュール内であれば
私有フィールドに直接アクセスできる (通常の Rust 可視性規則) ので、
`impl OrgChart { pub fn colleagues(&self, ...) -> Vec<&Employee> { ... } }`
のように後から普通のメソッドとして追記してください
(`crates/graphite/tests/orgchart_macro.rs` に実例あり)。

### 4. 制約なしエッジの順序保証

ノード表・辺表 (`graph_schema!` が生成する `graphite::KeyedTable<K, V>`) は
内部的に `Vec<(K, V)>` (挿入順の本体) + `HashMap<K, usize>` (キー→添字の
索引) という構成になっており、**`ids()`/`iter()` は挿入順 (`insert` を
呼んだ順) を保持することを仕様として保証します**
(`crates/graphite/src/keyed_table.rs` 参照)。これにより、制約なしエッジ
(`where` 節を省略した種別) の `Kind::of`/`between`/`iter` が返す `Vec` も、
同一始点キーに対する複数終点の相対順序が構築時の追加順 (builder の呼び出し
順。`graph!` の場合はソース中の記述順) をそのまま保持します。分岐ノベルの
選択肢表示順のように、順序そのものが意味を持つ場面で安心して依存できます
(`crates/graphite/tests/keyed_table_insertion_order.rs` に回帰テストあり)。

この保証はランタイム移行の初期実装では抜け落ちており (`KeyedTable` が素の
`HashMap` ラッパーで反復順序が未規定だったため)、dialogue-engine の v4 移行
中に「制約なし辺の `of()` の並びがプロセスごとに変わる」flaky なテストとして
発覚し、`KeyedTable` の内部構造を挿入順保持に変更する形で修正・仕様化された
経緯がある (`docs/dev_history_2026-07-14_session2.md` §3.10 参照)。

ただし、これは「同一始点キー内での順序」の保証であり、`iter()` が異なる
始点キーをまたいで列挙する順序までは保証しません (始点キーの集合は内部の
`HashMap` 索引で管理されているため)。

### 名前空間に関する制約 (`graph!`)

`graph!` 内の識別子 (`tanaka = Employee { .. }` の `tanaka`、`tanaka_dept =
BelongsTo(..)` の `tanaka_dept` の部分) は**ノード・エッジの種別を跨いで
単一の平坦な名前空間**です (`docs/schema_v4.md` §0 規則1: 全項目が
`名前 = 値` であり、名前は常にキーの束縛であるため)。異なる種別 (例:
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
  種別**・決定性=`where each OrderState: 0..1` として再定式化する。到達
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
  (未宣言ノード型を端点に指定 / 不正な `where each` 指定 / `graph!` で
  存在しないエッジ種別 / ノードキー重複 / 宣言単位のエラー回復)。stderr の
  再生成は `TRYBUILD=overwrite cargo test --test compile_fail`

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
   (`{Kind}EachViolation` / `{Kind}UniquePairViolation` /
   `{Kind}DuplicateKey` / `{Kind}UnknownSource` / `{Kind}UnknownTarget`)**。
   手書き版は `MultiplicityViolation { employee: EmployeeId, .. }` という
   スキーマ共通の 1 バリアントでしたが、一般のスキーマではエッジごとに
   始点/終点ノード型が異なりうる (例: `A -> B` と `C -> D` が両方 each
   違反を起こしうる) ため、エッジごとに専用バリアントを生成することで型を
   `String` に落とさず固定できるようにしています (「型の strictness」
   原則。`docs/design_principles.md` 原則1 参照)。例:
   `edge BelongsTo = Employee -> Department where each Employee: 1;` からは
   `BelongsToEachViolation { source: EmployeeId, count: usize }` /
   `BelongsToUnknownSource { edge: BelongsToId, source: EmployeeId }` /
   `BelongsToUnknownTarget { edge: BelongsToId, target: DepartmentId }` /
   `BelongsToDuplicateKey(BelongsToId)` が生成されます (v4 で辺キー重複・
   `unique pair` 違反が追加された。`docs/schema_v4.md` §3.1 参照)。
3. **builder のエッジ追加メソッドの引数は `({Kind}Id, {Kind})`**。手書き版
   は `boss(employee, boss, attrs)`・`reports(manager, report)` のように
   端点を直接引数に取っていましたが、v4 では辺そのものが第一級のキー付き
   要素になったため、builder のエッジメソッドは常に「辺キー + 辺値
   (タプル struct)」のペアを取ります (`b.boss(BossId("b1".into()),
   Boss(employee_id, boss_id, attrs))` のように)。
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
   (`node Employee;` / `edge Boss = Employee -[BossEdge]-> Employee where
   each Employee: 0..1;`) に書かれた型をそのまま参照します。派生する
   trait 要求も無い (上記
   「ノード値の型・エッジ属性型に対する trait 要求」参照) ため、derive する
   かどうかも含めて完全に利用者の自由です。

## 未決事項 / フェーズ4があるとしたら

以下はフェーズ3終了時点での未決事項一覧でした。フェーズ4で 5 項目中 4 項目
(残り 1 項目は設計判断として据え置き) に着手し、対応関係は以下の通りです。

- **多重度 `(1)` アクセサへ未知キーを渡した場合は v0 ではパニックとする —
  解決済み (フェーズ4、その後ビュー方式へ移行)**。多重度 `(1)` のビューの
  `of(&SrcId) -> &T` (パニック版) は「このスキーマが発行したキーだけを
  渡すことが呼び出し側の責務」という設計のまま残しつつ、非パニック版
  `get(&SrcId) -> Option<&T>` (属性付きは `Option<(&T, &Attrs)>`) を対で
  持ちます。`Vec` の `v[i]` (パニック) と `v.get(i)` (`Option`) の対と
  同じ関係です (フェーズ4では `{label}`/`try_{label}` という導出名の
  メソッド対でしたが、`docs/edge_view_api.md` でビュー1個の `of`/`get`
  に統合されました)。
- **`match` パターンでのクエリは非対応 — 一部緩和 (フェーズ4)、
  `match` 構文そのものは引き続き非対応**。Vertex 側で検討していた
  `match g { @{ a -[boss]-> b, b -[boss]-> a } => ... }` のような辺ラベル
  付きパターンマッチは、Rust の安定版では `match` アーム位置に任意の
  カスタム構文を注入できないため今後も実装しません。代わりに、各エッジ
  種別ごとのビューの `iter() -> impl Iterator<Item = (&SrcId, &DstId[, &Attrs])>`
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
- **`graph!` のエラーメッセージ品質 (未知エッジラベル) — フェーズ4で
  ハンドシェイクマクロにより一時解決、v3 (`docs/graph_literal_v3.md` §4) で
  全廃**。フェーズ4では `graph_schema!` が `__graphite_edge_{Schema}!` という
  宣言的マクロを追加生成し、未知のラベルに「利用可能なエッジ一覧」付きの
  `compile_error!` を出していましたが、この方式には `macro_rules!` の
  テキストスコープに起因する同一ファイル制約があり (別モジュールから
  `graph!` を使うには `#[macro_export]` 等の追加対応が必要)、かつ
  `graph!` (proc-macro) → ハンドシェイク (macro_rules) という二段展開が
  rust-analyzer の定義ジャンプを妨げる副作用もありました
  (`docs/ide_support_spec.md` §1.7)。v3 でリテラルの属性ペイロードを
  `-[label = 式]->` という式渡しに変えたことでハンドシェイク自体が不要になり、
  完全に廃止しました。未知ラベルは `graph!` が生成する
  `__graphite_b.{label}(..)` 呼び出しが素の rustc method-not-found (E0599) に
  落ちることで検出されます。「利用可能なエッジ一覧」付きの親切なメッセージは
  失いますが、健全性 (コンパイルエラーになること自体) には関与しないため
  許容する、というユーザー決定です (`tests/ui/graph_unknown_edge_label.stderr`
  参照)。この全廃により、同一ファイル制約自体も構造的に消滅しました
  (`graph!` が参照するのは通常の型・メソッドだけになったため。実証は
  `crates/graphite/tests/graph_cross_module.rs`)。

## ライセンス

ライセンス未定 (TBD)。
