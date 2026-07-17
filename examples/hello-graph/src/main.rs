//! hello-graph — Graphite (`graph_schema!`/`graph!`) の意味論を確認する
//! 入門用example。
//!
//! **これは教材です。** アプリとしての面白さは無く、「1個ずつ意味論を
//! 確かめる」ことだけが目的です。実践的な使用例は他の3本
//! (`examples/build-pipeline`・`examples/org-analyzer`・
//! `examples/dialogue-engine`) を参照してください。
//!
//! 上から順に読める構成にしています:
//! - §1 ノード型・エッジ属性型の宣言 (普通の struct)
//! - §2 `graph_schema!` によるスキーマ宣言 (v4: `edge Kind = ...;` は
//!   新しい nominal 型の定義、`where` は制約)
//! - §2.5 脱糖の実像 — 全要素キー・`KeyedTable` 格納・辺はタプル struct
//!   として第一級、という v4 の実装を実測して解説する
//! - §3 クックブック — `graph_schema!`/`graph!` が生成する公開APIの全列挙
//! - §4 「できないこと」— コンパイルエラーになる例と、実際のエラー引用
//! - §5 `flow!` — 関数の辺 (`graph!` の宣言される辺との対比)
//!
//! `cargo run` すると §3・§5 の内容が順に表示されます。

// ============================================================
// §1 型宣言 — ノード型・エッジ属性型は普通の Rust struct
// ============================================================
//
// `graph_schema!` はこれらの型を**生成せず、参照するだけ**です
// (`docs/schema_v4.md` §1)。derive・可視性・追加のメソッドは全部ふつうの
// Rust の話であり、Graphite 固有のルールはありません。

/// ノード型その1: 社員。
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

/// ノード型その2: チーム。
#[derive(Debug, Clone, PartialEq)]
pub struct Team {
    pub name: String,
}

/// `Boss` エッジが辺1本ごとに運ぶペイロード (積み荷)。
#[derive(Debug, Clone, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}

/// `ReviewedBy` エッジが辺1本ごとに運ぶペイロード (積み荷)。
#[derive(Debug, Clone, PartialEq)]
pub struct ReviewEdge {
    pub year: i32,
}

// ============================================================
// §2 schema 宣言
// ============================================================
//
// v4 (`docs/schema_v4.md` §0) の骨格は3規則だけです:
//
// 1. **`名前 = 定義`** — `edge Kind = From -> To ...;` は **`Kind` という
//    新しい nominal 型 (名前で区別される型) を定義する宣言**です。透過的な
//    別名ではありません。`Person -> Person` という同じ形の辺を2つ宣言
//    しても (下記の `Boss` と `Reports` の関係と終点の型は同じですが)、
//    それぞれ独立した別の型になります (取り違えてもコンパイルエラーに
//    なる、という利点はここから来ます)。
// 2. **矢印の中は積み荷だけ** — `-[X]->` の `X` は、その辺1本が運ぶ
//    積み荷の**型**です。積み荷が無い辺 (`Person -> Team` のように矢印の
//    中に何も書かない) は素の `->` になります。ラベルや関係の名前を
//    矢印の中に書くことはありません — `Kind` という名前は既に
//    `edge Kind = ..` の左辺で言い切っているからです。
// 3. **`where` は制約** — 制約があるときだけ書きます。省略時は「制約なし」
//    (=平行辺も含めて自由) を意味します。
//    - `each <FromType>: 1` — 各始点ノードにつきちょうど1本
//      (数学的には: この辺は始点の型から終点の型への**全域関数**)
//    - `each <FromType>: 0..1` — 各始点ノードにつき高々1本
//      (数学的には: **部分関数**)
//    - `unique pair` — 同じ (始点, 終点) の対に2本目を張ることを禁止
//      (=平行辺の禁止)
//
// 以下4本のエッジは、この "each 1 / each 0..1 / unique pair / 制約なし"
// という4パターンを一通りカバーするように選んであります:
//
// | エッジ         | 制約                | 積み荷        | 読み方 |
// |----------------|---------------------|----------------|--------|
// | `BelongsTo`    | `each Person: 1`    | なし           | 全域関数。全社員は必ずどこか1つのチームに所属する |
// | `Boss`         | `each Person: 0..1` | `BossEdge`     | 部分関数。上司がいない社員がいてもよいが、いるなら1人だけ |
// | `Reports`      | `unique pair`       | なし           | 同じ (上司, 部下) の対を2回宣言できない (平行辺の禁止) |
// | `ReviewedBy`   | 制約なし            | `ReviewEdge`   | 平行辺OK。同じ2人の間で複数年度の考課が積み重なってよい |
//
// `Friends` は上記4本とは別の軸 (向きの意味論) を確認するために追加した
// **無向辺** です (`docs/edge_endpoints_v4_1.md` §2)。矢印 (`->`/`-[X]->`)
// には必ず「向き」の意味が伴いますが、「友人関係」のように対称な (向きに
// 意味が無い) 関係を無理に矢印で書くと、どちらが from でどちらが to かに
// 嘘の意味が生まれてしまいます。無向の柄 `--` (積み荷ありなら `-[X]-`、
// 有向の柄から矢尻を落とした形) はこれを解消し、端点を「位置0/位置1」
// ではなく順序なし対として扱います。`Friends(alice -- bob)` と
// `Friends(bob -- alice)` は同じ辺であり、生成されるアクセサも
// `from()`/`to()` という嘘の語彙ではなく `endpoints() -> (&PersonId,
// &PersonId)` になります。両端は同じノード型でなければならず、役割名も
// 書けません (対称性を型にも及ぼす設計、詳細は §3 の実行例参照)。

#[rustfmt::skip]
graphite::graph_schema! {
    schema Org {
        node Person;
        node Team;

        edge BelongsTo  = Person -> Team              where each Person: 1;
        edge Boss       = Person -[BossEdge]-> Person where each Person: 0..1;
        edge Reports    = Person -> Person             where unique pair;
        edge ReviewedBy = Person -[ReviewEdge]-> Person; // 制約なし (平行辺も自由)
        edge Friends    = Person -- Person             where unique pair; // 無向辺 (v4.1)
    }
}

fn main() {
    section3();
    section5();
}

// ============================================================
// §2.5 脱糖の実像 — 全要素キー・KeyedTable格納・辺の第一級化
// ============================================================
//
// 以下は `cargo expand` で実際に確認した生成物 (`cargo install cargo-expand`
// して `cargo expand --bin hello-graph 2>&1 | Select-String -Context 5
// "struct Boss"` のように確認できます) を元に、要点だけ抜き出して整理した
// ものです。要約であって書き下ろしではありません — 生成ロジックそのものは
// `crates/graphite-macros/src/schema_codegen.rs` を正としてください。
//
// ## 1. 全要素がキー化される — ノードもエッジも
//
// v3 までは「エッジは HashMap のエントリ」でしたが、v4 では
// **辺そのものが、ノードと同じ資格を持つ第一級の要素**になりました。
// `graph_schema!` は `Boss` エッジ宣言から、ノードの `PersonId` と全く
// 同じ形の newtype キーを生成します:
//
// ```rust
// pub struct BossId(pub String);
// ```
//
// `graph!` リテラルの各行 `名前 = 値` の「名前」は、ノード行でもエッジ行
// でも常に**キーの束縛**です (`docs/schema_v4.md` §0 規則1)。これは
// `instance_codegen.rs` の脱糖を見ると直接分かります — 例えば
//
// ```rust
// tanaka_boss = Boss(bob -[promo]-> alice),
// ```
//
// は次のように展開されます (実際の展開形そのまま。`__graphite_b` が
// builder、`clone()` は from/to のキーを渡すため):
//
// ```rust
// let tanaka_boss = __graphite_b.add(
//     "tanaka_boss",
//     Boss(bob.clone(), alice.clone(), promo),
// );
// ```
//
// `tanaka_boss` はここで `Boss` の値そのものではなく、`add` が返す
// **`BossId`** に束縛されます。ノード行 (`alice = Person { .. }`) も
// 同じ形で `__graphite_b.insert("alice", Person { .. })` に展開され、
// `alice` は `PersonId` です。「名前 = 値」の名前は常にキー、という規則が
// ノード・エッジ双方に一貫して効いています。
//
// ## 2. 辺はタプル struct として実在する
//
// `edge Boss = Person -[BossEdge]-> Person where each Person: 0..1;` から
// `graph_schema!` が生成する実際の型は次の通りです
// (`schema_codegen.rs::gen_edge_tuple_structs`):
//
// ```rust
// #[derive(Debug, Clone, PartialEq)]
// pub struct Boss(pub PersonId, pub PersonId, pub BossEdge);
//
// impl Boss {
//     pub fn from(&self) -> &PersonId { &self.0 }
//     pub fn to(&self) -> &PersonId { &self.1 }
//     pub fn payload(&self) -> &BossEdge { &self.2 }
// }
// ```
//
// 積み荷なしエッジ (`BelongsTo`) は3要素目が無いだけの2要素タプル struct
// `pub struct BelongsTo(pub PersonId, pub TeamId);` になり、`payload()` は
// 生成されません。**このタプル struct はマクロの内部表現ではなく、公開
// struct として実在します** (`docs/schema_v4.md` §3.1 原則6) — マクロの
// 外で `Boss(bob_id, tanaka_id, BossEdge { since: 2020 })` と普通に構築
// できることは、`crates/graphite/tests/orgchart_macro.rs` の
// `タプルstructはマクロ外でも普通に構築できる`/`タプルstructを直接構築して
// addできる` が実例です。読み取りは `.0`/`.1`/`.2` という位置アクセスを
// 人間に晒さず、`from()`/`to()`/`payload()` という固定語彙のメソッドに
// 統一されています。
//
// ## 3. 格納先は KeyedTable — HashMap 直書きではない
//
// `Org` struct 本体は次の形で生成されます
// (`schema_codegen.rs::gen_schema_struct`。フィールド名はノード種別名の
// 複数形・エッジ種別名の snake_case):
//
// ```rust
// pub struct Org {
//     persons: graphite::KeyedTable<PersonId, Person>,
//     teams: graphite::KeyedTable<TeamId, Team>,
//
//     belongs_to: graphite::KeyedTable<BelongsToId, BelongsTo>,
//     belongs_to_from_index: std::collections::HashMap<PersonId, Vec<BelongsToId>>,
//
//     boss: graphite::KeyedTable<BossId, Boss>,
//     boss_from_index: std::collections::HashMap<PersonId, Vec<BossId>>,
//
//     reports: graphite::KeyedTable<ReportsId, Reports>,
//     reports_from_index: std::collections::HashMap<PersonId, Vec<ReportsId>>,
//
//     reviewed_by: graphite::KeyedTable<ReviewedById, ReviewedBy>,
//     reviewed_by_from_index: std::collections::HashMap<PersonId, Vec<ReviewedById>>,
// }
// ```
//
// ノード表もエッジ表も同じ `KeyedTable<Key, Value>` (`crates/graphite/src/
// keyed_table.rs`) というジェネリック機構を共有しています — v3 では
// ノード用の素朴な `HashMap` とエッジ用のビュー6型が別々の機構でしたが、
// v4 は「キー付き要素表」という1つの機構にノードと辺の両方を載せています。
// `{accessor}_from_index` (始点キー -> そこから出る辺キーの一覧) は
// `freeze` 時に構築される索引で、`Kind::of`/`between` の実装が使います。
//
// ## 4. メンタルモデル: 「ラベル=表」から「辺=第一級の行」へ
//
// v3 の比喩は「ラベルはリレーショナルDBの表名、辺はその1行」でしたが、
// v4 ではさらに一歩進み、**辺という「行」自体が独立したキーを持つ実体**
// になりました。`Boss::of(&g, &bob)` は「`boss` 表を `bob` で引く」ので
// はなく、正確には「`boss_from_index` で `bob` から出る `BossId` の一覧を
// 引き、`boss` 表からその `BossId` の `Boss` を取り、その `to()` で
// `persons` 表を引く」という3段の索引です。§4.2 で実際に `g.boss` へ
// 直接アクセスしようとした際の型不一致から、この `KeyedTable<BossId,
// Boss>` という実際の格納型がそのまま見えます。

// ============================================================
// §3 クックブック — 生成される公開APIの全列挙
// ============================================================
//
// `graph_schema!` が `schema Org { .. }` から生成する公開API を、
// 1関数=1つの「やりたいこと」に分けて全部並べています。
// カテゴリ順: 構築 → ノードを読む → エッジを辿る → 一覧する →
// 検証エラーを受ける。
//
// v4 (`docs/schema_v4.md` §3.2) では「すべて型名前空間の関連関数」です。
// `g.メソッド()` は一切生成されません:
// - ノード: `{Schema}Node` トレイト経由 (`Person::get(&g, &id)` 等。この
//   トレイトを `use` でスコープに入れておく必要があります — 本ファイルは
//   `graph_schema!` 呼び出しと同じモジュールなので暗黙にスコープ内です)。
// - エッジ: 各 `Kind` への固有 impl (`Boss::of`/`get`/`between`/`iter`/
//   `ids`/`len`)。`of`/`between` の戻り型は宣言した `where` 制約が決めます
//   (`each 1` → 直接参照、`each 0..1` → `Option`、制約なし → `Vec`、
//   `unique pair` → `between` が `Option`)。
//
// スタイル: イテレータ連鎖 (`map`/`filter`/`collect`) やクロージャによる
// データ加工は使わず、素の `for`/`if let`/`match` だけで書いています
// (`Org::create(|b| { .. })` の `|b| { .. }` は API が要求する引数であって
// データ加工のクロージャではないので例外です)。

fn section3() {
    println!("=== §3 クックブック: graph_schema!/graph! が生成する公開APIの全列挙 ===\n");

    // --- 構築 (3通りの書き方) ---
    println!("--- 構築 ---");
    let g: Org = インライン式でgraphリテラルを組み立てる();
    外部変数を渡してgraphリテラルを組み立てる();
    外部で作ったエッジ属性をgraphリテラルに渡す();
    builderの型名メソッドで組み立てる();
    builderの総称insertとaddで組み立てる();

    // --- ノードを読む ---
    println!("\n--- ノードを読む ---");
    人ノードを1件読む(&g);
    チームノードを1件読む(&g);
    personidの作り方とgraphのキーの対応を確認する(&g);

    // --- エッジを辿る (Kind::of/get/between) ---
    println!("\n--- エッジを辿る (Kind::of/get/between) ---");
    each_1のofは直接参照を返す(&g);
    each_0か1のofはoptionを返す(&g);
    unique_pairのbetweenはoptionを返す(&g);
    制約なしのofはvecを返す(&g);
    無向辺のendpointsアクセサで両端を読む(&g);
    無向辺のofとbetweenは対称に辿れる(&g);

    // --- 一覧する (iter/ids/len) ---
    println!("\n--- 一覧する (iter/ids/len) ---");
    person_idsで全ノードキーを列挙する(&g);
    team_idsで全ノードキーを列挙する(&g);
    belongs_toのiterで制約ありエッジを列挙する(&g);
    bossのiterで積み荷ありエッジを列挙する(&g);
    lenで表の辺の本数を確認する(&g);

    // --- 検証エラーを受ける ---
    println!("\n--- 検証エラーを受ける ---");
    重複ノードキーの違反を受け取る();
    辺キー重複の違反を受け取る();
    未知の始点キーの違反を受け取る();
    未知の終点キーの違反を受け取る();
    each違反を受け取る();
    unique_pair違反を受け取る();
    createは最初の1件で違反を止める();
    create_collectingで全違反を集める();
}

// --- 構築 ---

// やりたいこと: graph! にノード式・エッジをそのまま書いて組み立てる (最も基本の書き方)。
// この g を以降の「ノードを読む」「エッジを辿る」「一覧する」節で使い回す。
fn インライン式でgraphリテラルを組み立てる() -> Org {
    #[rustfmt::skip]
    let g: Org = graphite::graph!(Org {
        alice = Person { name: "Alice".into() },
        bob   = Person { name: "Bob".into() },
        carol = Person { name: "Carol".into() },
        eng   = Team { name: "Engineering".into() },

        alice_dept = BelongsTo(alice -> eng),
        bob_dept   = BelongsTo(bob -> eng),
        carol_dept = BelongsTo(carol -> eng),
        bob_boss   = Boss(bob -[BossEdge { since: 2021 }]-> alice),
        alice_reports_bob   = Reports(alice -> bob),
        alice_reports_carol = Reports(alice -> carol),
        review_2023 = ReviewedBy(bob -[ReviewEdge { year: 2023 }]-> alice),
        review_2024 = ReviewedBy(bob -[ReviewEdge { year: 2024 }]-> carol),
        alice_bob_friends = Friends(alice -- bob),
    })
    .expect("正常なグラフは構築に成功するはず");
    let alice_person: &Person = Person::get(&g, &PersonId("alice".to_string())).unwrap();
    println!("(構築1: インライン式) alice = {}", alice_person.name);
    g
}

// やりたいこと: グラフの外で作った値を graph! にそのまま渡す (`alice = alice_value` の形)。
fn 外部変数を渡してgraphリテラルを組み立てる() {
    let alice_value: Person = Person { name: "Alice".to_string() };
    let eng_value: Team = Team { name: "Engineering".to_string() };
    #[rustfmt::skip]
    let g: Org = graphite::graph!(Org {
        alice = alice_value,
        eng   = eng_value,
        alice_dept = BelongsTo(alice -> eng),
    })
    .expect("外部変数を渡した graph! も構築に成功するはず");
    let alice_person: &Person = Person::get(&g, &PersonId("alice".to_string())).unwrap();
    println!("(構築2: 外部変数渡し) alice = {}", alice_person.name);
}

// やりたいこと: エッジの積み荷 (`BossEdge`) もグラフの外で作った値を渡せることを確認する。
fn 外部で作ったエッジ属性をgraphリテラルに渡す() {
    let promotion: BossEdge = BossEdge { since: 2019 };
    #[rustfmt::skip]
    let g: Org = graphite::graph!(Org {
        alice = Person { name: "Alice".into() },
        bob   = Person { name: "Bob".into() },
        eng   = Team { name: "Engineering".into() },
        alice_dept = BelongsTo(alice -> eng),
        bob_dept   = BelongsTo(bob -> eng),
        bob_boss   = Boss(bob -[promotion]-> alice),
    })
    .expect("外部エッジ属性を渡した graph! も構築に成功するはず");
    let boss_pair: (&Person, &BossEdge) = Boss::of(&g, &PersonId("bob".to_string())).unwrap();
    println!("(構築3: 外部エッジ属性渡し) bob の上司就任年 = {}", boss_pair.1.since);
}

// やりたいこと: graph! を使わず、builder の型名つきメソッド (`b.person(id, value)`) で組み立てる。
fn builderの型名メソッドで組み立てる() {
    let g: Org = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("dave".to_string()), Person { name: "Dave".to_string() });
        b.team(TeamId("sales".to_string()), Team { name: "Sales".to_string() });
        b.belongs_to(
            BelongsToId("dave_dept".to_string()),
            BelongsTo(PersonId("dave".to_string()), TeamId("sales".to_string())),
        );
    })
    .expect("builder の型名メソッドでも構築に成功するはず");
    let dave: &Person = Person::get(&g, &PersonId("dave".to_string())).unwrap();
    println!("(構築4: builderの型名メソッド) dave = {}", dave.name);
}

// やりたいこと: builder の総称メソッド `insert`/`add` に値を渡し、値の型から自動で
// 振り分けさせる (`insert` の型境界 `N: OrgNode`、`add` の型境界 `E: OrgEdge` は
// graph_schema! が生成したトレイトで満たされる。利用者がこのトレイトを直接呼ぶことは無い)。
fn builderの総称insertとaddで組み立てる() {
    let g: Org = Org::create(|b: &mut OrgBuilder| {
        let eve_id: PersonId = b.insert("eve", Person { name: "Eve".to_string() });
        let sales_id: TeamId = b.insert("sales", Team { name: "Sales".to_string() });
        let _dept_id: BelongsToId = b.add("eve_dept", BelongsTo(eve_id.clone(), sales_id.clone()));
    })
    .expect("insert/add 経由の構築も成功するはず");
    let eve: &Person = Person::get(&g, &PersonId("eve".to_string())).unwrap();
    println!("(構築5: builderの総称insert/add) eve = {}", eve.name);
}

// --- ノードを読む ---

// やりたいこと: `{Type}::get(&g, &id)` で1件読む (無ければ None)。
fn 人ノードを1件読む(g: &Org) {
    let alice: Option<&Person> = Person::get(g, &PersonId("alice".to_string()));
    if let Some(person) = alice {
        println!("(ノード) Person::get(&g, &alice) = {}", person.name);
    }
    let unknown: Option<&Person> = Person::get(g, &PersonId("dave".to_string()));
    println!("(ノード) Person::get(&g, &dave)  = {unknown:?} (この g には居ない)");
}

// やりたいこと: `Team::get` も同じ形。ノード型が違っても命名規則は共通。
fn チームノードを1件読む(g: &Org) {
    let eng: Option<&Team> = Team::get(g, &TeamId("eng".to_string()));
    if let Some(team) = eng {
        println!("(ノード) Team::get(&g, &eng) = {}", team.name);
    }
}

// やりたいこと: `PersonId` はただの newtype なので手で組み立てられる。graph! の
// キー (`alice = ..`) はこの `PersonId("alice".to_string())` と同一視される。
fn personidの作り方とgraphのキーの対応を確認する(g: &Org) {
    let hand_built_id: PersonId = PersonId("alice".to_string());
    let alice: &Person = Person::get(g, &hand_built_id)
        .expect("graph!のキーaliceがPersonId(\"alice\")と一致するはず");
    println!("(型) PersonId(\"alice\".to_string()) で graph! の alice = {} が引ける", alice.name);
}

// --- エッジを辿る (Kind::of/get/between) ---

// やりたいこと: `each Person: 1` のエッジは `of` が参照そのものを返す
// (未知キーはパニックする契約。非パニック版は `get_of`)。
fn each_1のofは直接参照を返す(g: &Org) {
    let team: &Team = BelongsTo::of(g, &PersonId("alice".to_string()));
    println!("(each 1) BelongsTo::of(&g, &alice) = {}", team.name);

    let safe: Option<&Team> = BelongsTo::get_of(g, &PersonId("alice".to_string()));
    println!("(each 1) BelongsTo::get_of(&g, &alice) = {:?}", safe.map(|t| &t.name));
    let unknown: Option<&Team> = BelongsTo::get_of(g, &PersonId("dave".to_string()));
    println!("(each 1) BelongsTo::get_of(&g, &dave) = {unknown:?} (未知キーはNone)");
}

// やりたいこと: `each Person: 0..1` のエッジは `of` が `Option` を返す。
// 積み荷ありなので `Option<(&Node, &Attrs)>` になり、積み荷へは "ふつうの
// フィールドアクセス" で辿れる (`attrs.since`)。
fn each_0か1のofはoptionを返す(g: &Org) {
    let boss: Option<(&Person, &BossEdge)> = Boss::of(g, &PersonId("bob".to_string()));
    if let Some((boss_person, attrs)) = boss {
        println!("(each 0..1) Boss::of(&g, &bob) = {} (就任年: {})", boss_person.name, attrs.since);
    }
    let no_boss: Option<(&Person, &BossEdge)> = Boss::of(g, &PersonId("alice".to_string()));
    println!("(each 0..1) Boss::of(&g, &alice) = {no_boss:?} (aliceには上司がいない)");
}

// やりたいこと: `unique pair` のエッジは `between` が `Option` を返す
// (同じ対に2本目を張れないので「あるかないか」で十分)。
fn unique_pairのbetweenはoptionを返す(g: &Org) {
    let r: Option<&Reports> = Reports::between(
        g,
        &PersonId("alice".to_string()),
        &PersonId("bob".to_string()),
    );
    println!("(unique pair) Reports::between(&g, &alice, &bob) = {}", r.is_some());
    let none = Reports::between(
        g,
        &PersonId("bob".to_string()),
        &PersonId("alice".to_string()),
    );
    println!("(unique pair) Reports::between(&g, &bob, &alice) = {} (逆向きは無い)", none.is_some());
}

// やりたいこと: 制約なしのエッジは `of` が `Vec` を返す (平行辺を許すため)。
// 積み荷ありなので `Vec<(&Node, &Attrs)>`。
fn 制約なしのofはvecを返す(g: &Org) {
    let reviewers: Vec<(&Person, &ReviewEdge)> = ReviewedBy::of(g, &PersonId("bob".to_string()));
    for (reviewer, attrs) in &reviewers {
        println!(
            "(制約なし) ReviewedBy::of(&g, &bob) に {} ({}年度) が含まれる",
            reviewer.name, attrs.year
        );
    }
}

// やりたいこと: 無向辺 (`Friends`) は `from()`/`to()` の代わりに
// `endpoints() -> (&PersonId, &PersonId)` を持つ (`docs/edge_endpoints_v4_1.md`
// §2)。位置0/1は `Friends(alice -- bob)` と書いた際の記述順そのままだが、
// 意味論としては順序なし対であることに注意 (次の関数で確認する)。
fn 無向辺のendpointsアクセサで両端を読む(g: &Org) {
    let friends_id = FriendsId("alice_bob_friends".to_string());
    let edge: &Friends = Friends::get(g, &friends_id).unwrap();
    let (p0, p1) = edge.endpoints();
    println!("(無向) Friends::get(&g, &alice_bob_friends).endpoints() = ({p0:?}, {p1:?})");
}

// やりたいこと: `of`/`between` はどちらの位置に置かれても対称に辿れる。
// `unique pair` の同値判定も順序を無視する (`alice -- bob` と `bob -- alice`
// は同じ対)。
fn 無向辺のofとbetweenは対称に辿れる(g: &Org) {
    let alice = PersonId("alice".to_string());
    let bob = PersonId("bob".to_string());

    let friends_of_bob: Vec<&Person> = Friends::of(g, &bob);
    for friend in &friends_of_bob {
        println!("(無向) Friends::of(&g, &bob) に {} が含まれる (aliceが位置0でも辿れる)", friend.name);
    }

    let forward: Option<&Friends> = Friends::between(g, &alice, &bob);
    let backward: Option<&Friends> = Friends::between(g, &bob, &alice);
    println!(
        "(無向) between(alice, bob) = {} / between(bob, alice) = {} (順序を無視して同じ辺)",
        forward.is_some(),
        backward.is_some()
    );
}

// --- 一覧する (iter/ids/len) ---

// やりたいこと: `{Type}::ids(&g)` でノード種別ごとの全キーを列挙する。
fn person_idsで全ノードキーを列挙する(g: &Org) {
    for id in Person::ids(g) {
        println!("(一覧) Person::ids: {id:?}");
    }
}

fn team_idsで全ノードキーを列挙する(g: &Org) {
    for id in Team::ids(g) {
        println!("(一覧) Team::ids: {id:?}");
    }
}

// やりたいこと: `Kind::iter(&g)` は `(&{Kind}Id, &Kind)` の組。積み荷なしエッジの例。
fn belongs_toのiterで制約ありエッジを列挙する(g: &Org) {
    for (id, edge) in BelongsTo::iter(g) {
        println!("(iter) BelongsTo {id:?}: {:?} -> {:?}", edge.from(), edge.to());
    }
}

// やりたいこと: 積み荷ありエッジの `iter()` も同じ形。`edge.payload()` で積み荷を読む。
fn bossのiterで積み荷ありエッジを列挙する(g: &Org) {
    for (id, edge) in Boss::iter(g) {
        println!(
            "(iter) Boss {id:?}: {:?} -> {:?} (since={})",
            edge.from(), edge.to(), edge.payload().since
        );
    }
}

// やりたいこと: `Kind::len(&g)` で表の辺の本数を確認する。
fn lenで表の辺の本数を確認する(g: &Org) {
    println!("(len) BelongsTo::len(&g) = {}", BelongsTo::len(g));
    println!("(len) ReviewedBy::len(&g) = {} (制約なしは平行辺込みの総本数)", ReviewedBy::len(g));
}

// --- 検証エラーを受ける ---

// やりたいこと: 同じキーを2回宣言すると `Duplicate{Node}` 違反になることを確認する。
fn 重複ノードキーの違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        b.person(PersonId("alice".to_string()), Person { name: "Alice2".to_string() });
    });
    match result {
        Err(OrgViolation::DuplicatePerson(id)) => println!("(違反) 重複ノードキー: {id:?}"),
        _ => panic!("重複ノードキー違反が検出されるはず"),
    }
}

// やりたいこと: v4で新規追加された「辺キーの重複」も検出できることを確認する
// (辺も第一級のキー付き要素になったため)。
fn 辺キー重複の違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        b.person(PersonId("bob".to_string()), Person { name: "Bob".to_string() });
        b.team(TeamId("eng".to_string()), Team { name: "Engineering".to_string() });
        b.belongs_to(
            BelongsToId("dup".to_string()),
            BelongsTo(PersonId("alice".to_string()), TeamId("eng".to_string())),
        );
        b.belongs_to(
            BelongsToId("dup".to_string()),
            BelongsTo(PersonId("bob".to_string()), TeamId("eng".to_string())),
        );
    });
    match result {
        Err(OrgViolation::BelongsToDuplicateKey(id)) => println!("(違反) 辺キー重複: {id:?}"),
        _ => panic!("辺キー重複違反が検出されるはず"),
    }
}

// やりたいこと: 未宣言の始点キーからエッジを張ると `{Kind}UnknownSource` 違反になる。
fn 未知の始点キーの違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.team(TeamId("eng".to_string()), Team { name: "Engineering".to_string() });
        b.belongs_to(
            BelongsToId("bt1".to_string()),
            BelongsTo(PersonId("存在しない社員".to_string()), TeamId("eng".to_string())),
        );
    });
    match result {
        Err(OrgViolation::BelongsToUnknownSource { edge, source }) => {
            println!("(違反) 未知の始点キー: 辺={edge:?} 始点={source:?}");
        }
        _ => panic!("未知の始点キー違反が検出されるはず"),
    }
}

// やりたいこと: 未宣言の終点キーへエッジを張ると `{Kind}UnknownTarget` 違反になる。
fn 未知の終点キーの違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        b.belongs_to(
            BelongsToId("bt1".to_string()),
            BelongsTo(PersonId("alice".to_string()), TeamId("存在しないチーム".to_string())),
        );
    });
    match result {
        Err(OrgViolation::BelongsToUnknownTarget { edge, target }) => {
            println!("(違反) 未知の終点キー: 辺={edge:?} 終点={target:?}");
        }
        _ => panic!("未知の終点キー違反が検出されるはず"),
    }
}

// やりたいこと: `each Person: 1` を満たさない (0本の) エッジは `{Kind}EachViolation` になる。
fn each違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        // aliceをどのチームにも所属させない (BelongsTo は each Person: 1)
    });
    match result {
        Err(OrgViolation::BelongsToEachViolation { source, count }) => {
            println!("(違反) each違反: {source:?} は {count} 本 (期待は1本)");
        }
        _ => panic!("each違反が検出されるはず"),
    }
}

// やりたいこと: `unique pair` の対に2本目を張ると `{Kind}UniquePairViolation` になる。
fn unique_pair違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        b.person(PersonId("bob".to_string()), Person { name: "Bob".to_string() });
        b.team(TeamId("eng".to_string()), Team { name: "Engineering".to_string() });
        // each Person: 1 (BelongsTo) が先に違反しないよう、両者ともチームに所属させておく。
        b.belongs_to(
            BelongsToId("bt_alice".to_string()),
            BelongsTo(PersonId("alice".to_string()), TeamId("eng".to_string())),
        );
        b.belongs_to(
            BelongsToId("bt_bob".to_string()),
            BelongsTo(PersonId("bob".to_string()), TeamId("eng".to_string())),
        );
        b.reports(
            ReportsId("r1".to_string()),
            Reports(PersonId("alice".to_string()), PersonId("bob".to_string())),
        );
        b.reports(
            ReportsId("r2".to_string()),
            Reports(PersonId("alice".to_string()), PersonId("bob".to_string())),
        );
    });
    match result {
        Err(OrgViolation::ReportsUniquePairViolation { source, target }) => {
            println!("(違反) unique pair違反: {source:?} -> {target:?} に2本目");
        }
        _ => panic!("unique pair違反が検出されるはず"),
    }
}

// やりたいこと: `create` は最初の1件の違反で `Err` になる (複数あっても1件目だけ)。
fn createは最初の1件で違反を止める() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        b.person(PersonId("bob".to_string()), Person { name: "Bob".to_string() });
        // alice, bobともどのチームにも所属させない (違反が2件あるはず)
    });
    let violation: OrgViolation = match result {
        Err(violation) => violation,
        Ok(_) => panic!("違反が検出されるはず"),
    };
    println!("(create) 最初の1件だけ: {violation}");
}

// やりたいこと: `create_collecting` は打ち切らず全違反を `Vec` に集める。
fn create_collectingで全違反を集める() {
    let result: Result<Org, Vec<OrgViolation>> = Org::create_collecting(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        b.person(PersonId("bob".to_string()), Person { name: "Bob".to_string() });
        // alice, bobともどのチームにも所属させない (2件のeach違反が集まるはず)
    });
    let violations: Vec<OrgViolation> = match result {
        Err(violations) => violations,
        Ok(_) => panic!("2件の違反が集まるはず"),
    };
    for violation in &violations {
        println!("(create_collecting) 違反: {violation}");
    }
}

// ============================================================
// §4 「できないこと」
// ============================================================
//
// 以下はすべてコメントアウトしてあります。コメントを外して
// `cargo build 2> build_errors.txt; Get-Content build_errors.txt -Head 50`
// すると、直下に引用したものと同じエラーが実際に出ることを確認できます
// (このファイルに記載のエラー文はすべて実測したもので、書き下ろしでは
// ありません)。

// --- 4.1 Kind名を積み荷のように扱おうとする (フィールドは無い) ---
//
// `Boss` はスキーマ宣言で定義された1つのタプル struct 型です (§2.5)。
// タプル struct 名は Rust 的にはそのコンストラクタ関数として式の位置に
// 書ける値でもある (`Boss` 単体は `fn(PersonId, PersonId, BossEdge) ->
// Boss` という関数) ため、「未定義」エラーにはなりません。しかし
// `.since` のような名前付きフィールドは (積み荷は `.2`/`payload()` の
// 位置アクセスでしか持たないため) 存在せず、フィールドなしエラーに
// なります — `Boss.since` と直接書けるという誤解を正すのがこの例の
// 意図です。
//
// fn section4_1() {
//     let _ = Boss.since;
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0609]: no field `since` on type `fn(PersonId, PersonId, BossEdge) -> Boss {Boss}`
//      --> src\main.rs:663:18
//       |
//   663 |     let _ = Boss.since;
//       |                  ^^^^^ unknown field

// --- 4.2 フィールドに直接アクセスしようとする (内部ストレージの型が露出する) ---
//
// `Org` の各フィールド (`boss` 等) は非公開の内部ストレージ
// (`KeyedTable<BossId, Boss>`、§2.5 参照) であり、`Person`/`Team` のような
// ノード値そのものではありません。このファイルは schema 宣言と同じ
// モジュールなので `g.boss` という式自体は private エラーにはなりません
// (Rust の可視性はモジュール単位であり、同一モジュール内では非公開
// フィールドも見えるため)。しかし中身は `Person` ではなく
// `KeyedTable<BossId, Boss>` そのものなので、`Person` として使おうとした
// 瞬間に型不一致になります。
//
// fn section4_2(g: &Org) -> Person {
//     g.boss
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0308]: mismatched types
//      --> src\main.rs:690:5
//       |
//   689 | fn section4_2(g: &Org) -> Person {
//       |                           ------ expected `Person` because of return type
//   690 |     g.boss
//       |     ^^^^^^ expected `Person`, found `KeyedTable<BossId, Boss>`
//       |
//       = note: expected struct `Person`
//                  found struct `KeyedTable<BossId, Boss>`
//
// (`g.boss` という式そのものは同一モジュール内なので private エラーには
// ならず素朴に評価できてしまいますが、その型は `Person` ではなく内部
// ストレージの `KeyedTable` そのものであることがこの型不一致から分かります。
// つまり「boss というフィールドで社員そのものが手に入る」という誤解は
// この型不一致で正されます。§2.5 で見た内部テーブルの型そのものです。)

// --- 4.3 存在しないエッジ種別を graph! に書く ---
//
// 未知の `Kind` は素の rustc 型解決 (cannot find type/function) だけで
// 検出されます (ハンドシェイクマクロは無い。意図した trade-off です)。
//
// fn section4_3() {
//     #[rustfmt::skip]
//     let _ = graphite::graph!(Org {
//         alice = Person { name: "Alice".into() },
//         eng = Team { name: "Engineering".into() },
//         no_such = NoSuchKind(alice -> eng),
//     });
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0425]: cannot find function, tuple struct or tuple variant `NoSuchKind` in this scope
//      --> src\main.rs:722:19
//       |
//   722 |         no_such = NoSuchKind(alice -> eng),
//       |                   ^^^^^^^^^^ not found in this scope

// --- 4.4 端点の型を間違えたエッジを graph! に書く ---
//
// `BelongsTo` は `Person -> Team` として宣言されているので、from/to は
// `Person`/`Team` でなければなりません。両方を `Person` にすると型不一致に
// なります。
//
// fn section4_4() {
//     #[rustfmt::skip]
//     let _ = graphite::graph!(Org {
//         alice = Person { name: "Alice".into() },
//         bob = Person { name: "Bob".into() },
//         bad = BelongsTo(alice -> bob),
//     });
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0308]: mismatched types
//      --> src\main.rs:742:13
//       |
//   742 |       let _ = graphite::graph!(Org {
//       |  _____________^
//   743 | |         alice = Person { name: "Alice".into() },
//   744 | |         bob = Person { name: "Bob".into() },
//   745 | |         bad = BelongsTo(alice -> bob),
//       | |               --------- arguments to this struct are incorrect
//   746 | |     });
//       | |______^ expected `TeamId`, found `PersonId`
//       |
//   note: tuple struct defined here
//      --> src\main.rs:94:14
//       |
//    94 |         edge BelongsTo  = Person -> Team              where each Person: 1;
//       |              ^^^^^^^^^
//       = note: this error originates in the macro `graphite::graph` (in Nightly builds, run with -Z macro-backtrace for more info)

// ============================================================
// §5 flow! — 関数の辺 (graph! の宣言される辺との対比)
// ============================================================
//
// `graph_schema!`/`graph!` の辺 (`edge Kind = ...` / `Kind(from -> to)`) は
// **宣言**です — 構築 (`create`) 時にまとめて検証されるデータの繋がりで、
// 矢印の中の値そのもの (積み荷) はグラフの外では意味を持ちません。対して
// `graphite::flow!` (`docs/flow_macro.md`) の矢印 `-[関数式]->` は
// **実行**です — 書かれた順に `let 束縛名 = (関数式)(始点..);` という
// ただの関数呼び出しへ即時に脱糖するだけで、スキーマや builder は一切
// 関与しません。同じ矢印記法 `-[X]->` を「宣言されるデータの辺」(`graph!`)
// と「即時実行される関数の辺」(`flow!`) という対照的な2つの意味論に使い
// 分けている、という読み方が両者を統一します — どちらも「ノードは値、
// 矢印の中の `X` が辺の主役」という同じ形を共有しているのに、`X` が
// 「運ばれる積み荷の型/値」なのか「今すぐ呼ばれる関数」なのかで意味が
// 分岐する、という対応です。`flow!` は文位置マクロなので、束縛名は
// `graph!` のノード/エッジキーのように builder クロージャの中に閉じず、
// 普通の `let` 束縛としてマクロ呼び出しの後にそのまま見えます。

fn section5() {
    println!("\n=== §5 flow!: 関数の辺 (宣言ではなく即時実行) ===\n");

    fn parse(s: &str) -> i32 {
        s.parse().expect("数値のはず")
    }
    fn validate(x: i32) -> bool {
        x >= 0
    }
    fn double(x: i32) -> i32 {
        x * 2
    }
    fn merge(valid: bool, doubled: i32) -> String {
        format!("valid={valid} doubled={doubled}")
    }

    #[rustfmt::skip]
    graphite::flow! {
        "21" -[parse]-> parsed,              // 直線 (1本の矢印)
        parsed -[validate]-> valid,          // fan-out: parsed を2本の矢印に流す
        parsed -[double]-> doubled,
        (valid, doubled) -[merge]-> summary, // fan-in: タプル始点は多引数呼び出しに脱糖
    };
    // parsed/valid/doubled/summary はいずれも flow! の後で普通のローカル
    // 変数として見える (§3 の graph! のノード/エッジキーが builder クロージャ
    // の中に閉じるのとは対照的)。
    println!("(flow!) parsed={parsed} valid={valid} doubled={doubled}");
    println!("(flow!) summary = {summary}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build() -> Org {
        #[rustfmt::skip]
        let g = graphite::graph!(Org {
            alice = Person { name: "Alice".into() },
            bob   = Person { name: "Bob".into() },
            carol = Person { name: "Carol".into() },
            eng   = Team { name: "Engineering".into() },

            alice_dept = BelongsTo(alice -> eng),
            bob_dept   = BelongsTo(bob -> eng),
            carol_dept = BelongsTo(carol -> eng),
            bob_boss   = Boss(bob -[BossEdge { since: 2021 }]-> alice),
            alice_reports_bob   = Reports(alice -> bob),
            alice_reports_carol = Reports(alice -> carol),
            review_2023 = ReviewedBy(bob -[ReviewEdge { year: 2023 }]-> alice),
            review_2024 = ReviewedBy(bob -[ReviewEdge { year: 2024 }]-> carol),
        });
        g.expect("正常なグラフは構築に成功するはず")
    }

    #[test]
    fn each_1のofは参照そのものを返す() {
        let g = build();
        let team = BelongsTo::of(&g, &PersonId("alice".to_string()));
        assert_eq!(team.name, "Engineering");
    }

    #[test]
    fn each_0か1のofはoptionのタプルを返し積み荷フィールドへアクセスできる() {
        let g = build();
        let (boss, attrs) = Boss::of(&g, &PersonId("bob".to_string()))
            .expect("bobの上司はaliceのはず");
        assert_eq!(boss.name, "Alice");
        assert_eq!(attrs.since, 2021);
        assert!(Boss::of(&g, &PersonId("alice".to_string())).is_none());
    }

    #[test]
    fn 制約なしのofはvecを返す() {
        let g = build();
        let mut names: Vec<&str> = ReviewedBy::of(&g, &PersonId("bob".to_string()))
            .into_iter()
            .map(|(p, _)| p.name.as_str())
            .collect();
        names.sort();
        assert_eq!(names, vec!["Alice", "Carol"]);
    }

    #[test]
    fn each_1のget_ofは未知キーでnoneを返す() {
        let g = build();
        assert!(BelongsTo::get_of(&g, &PersonId("dave".to_string())).is_none());
    }

    #[test]
    fn iterで表全体を列挙できる() {
        let g = build();
        let boss_pairs: Vec<(&BossId, &Boss)> = Boss::iter(&g).collect();
        assert_eq!(boss_pairs.len(), 1);
        let (_, edge) = boss_pairs[0];
        assert_eq!(edge.from(), &PersonId("bob".to_string()));
        assert_eq!(edge.to(), &PersonId("alice".to_string()));
        assert_eq!(edge.payload().since, 2021);
    }

    #[test]
    fn person_getで1件読める() {
        let g = build();
        assert_eq!(Person::get(&g, &PersonId("alice".to_string())).unwrap().name, "Alice");
        assert!(Person::get(&g, &PersonId("dave".to_string())).is_none());
    }

    #[test]
    fn reports_betweenはunique_pairなのでoptionを返す() {
        let g = build();
        assert!(Reports::between(&g, &PersonId("alice".to_string()), &PersonId("bob".to_string())).is_some());
        assert!(Reports::between(&g, &PersonId("bob".to_string()), &PersonId("alice".to_string())).is_none());
    }

    #[test]
    fn review_のofは制約なしでvecのタプルを返す() {
        let g = build();
        let reviewers = ReviewedBy::of(&g, &PersonId("bob".to_string()));
        assert_eq!(reviewers.len(), 2);
        assert!(reviewers.iter().any(|(p, a)| p.name == "Alice" && a.year == 2023));
        assert!(reviewers.iter().any(|(p, a)| p.name == "Carol" && a.year == 2024));
    }

    #[test]
    fn lenで辺の本数を確認できる() {
        let g = build();
        assert_eq!(BelongsTo::len(&g), 3);
        assert_eq!(Reports::len(&g), 2);
    }

    #[test]
    fn 重複ノードキーはduplicate違反になる() {
        let result = Org::create(|b| {
            b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
            b.person(PersonId("alice".to_string()), Person { name: "Alice2".to_string() });
        });
        assert!(matches!(result, Err(OrgViolation::DuplicatePerson(_))));
    }

    #[test]
    fn 辺キー重複はduplicatekey違反になる() {
        let result = Org::create(|b| {
            b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
            b.team(TeamId("eng".to_string()), Team { name: "Engineering".to_string() });
            b.belongs_to(BelongsToId("dup".to_string()), BelongsTo(PersonId("alice".to_string()), TeamId("eng".to_string())));
            b.belongs_to(BelongsToId("dup".to_string()), BelongsTo(PersonId("alice".to_string()), TeamId("eng".to_string())));
        });
        assert!(matches!(result, Err(OrgViolation::BelongsToDuplicateKey(_))));
    }

    #[test]
    fn 未知の始点キーはunknownsource違反になる() {
        let result = Org::create(|b| {
            b.team(TeamId("eng".to_string()), Team { name: "Engineering".to_string() });
            b.belongs_to(
                BelongsToId("bt1".to_string()),
                BelongsTo(PersonId("存在しない社員".to_string()), TeamId("eng".to_string())),
            );
        });
        assert!(matches!(result, Err(OrgViolation::BelongsToUnknownSource { .. })));
    }

    #[test]
    fn unique_pair違反が検出される() {
        let result = Org::create(|b| {
            b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
            b.person(PersonId("bob".to_string()), Person { name: "Bob".to_string() });
            b.team(TeamId("eng".to_string()), Team { name: "Engineering".to_string() });
            // each Person: 1 (BelongsTo) が先に違反しないよう、両者ともチームに所属させておく。
            b.belongs_to(BelongsToId("bt_alice".to_string()), BelongsTo(PersonId("alice".to_string()), TeamId("eng".to_string())));
            b.belongs_to(BelongsToId("bt_bob".to_string()), BelongsTo(PersonId("bob".to_string()), TeamId("eng".to_string())));
            b.reports(ReportsId("r1".to_string()), Reports(PersonId("alice".to_string()), PersonId("bob".to_string())));
            b.reports(ReportsId("r2".to_string()), Reports(PersonId("alice".to_string()), PersonId("bob".to_string())));
        });
        assert!(matches!(result, Err(OrgViolation::ReportsUniquePairViolation { .. })));
    }

    #[test]
    fn create_collectingは複数の違反を集める() {
        let result = Org::create_collecting(|b| {
            b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
            b.person(PersonId("bob".to_string()), Person { name: "Bob".to_string() });
        });
        let violations = match result {
            Err(violations) => violations,
            Ok(_) => panic!("2件の違反が集まるはず"),
        };
        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn タプルstructはマクロ外でも普通に構築できる() {
        // `docs/schema_v4.md` §3.1 原則6: 生成されたタプル struct はマクロの
        // 外でも普通に構築できる。
        let e = BelongsTo(PersonId("alice".to_string()), TeamId("eng".to_string()));
        assert_eq!(e.from(), &PersonId("alice".to_string()));
        assert_eq!(e.to(), &TeamId("eng".to_string()));

        let b = Boss(
            PersonId("bob".to_string()),
            PersonId("alice".to_string()),
            BossEdge { since: 2020 },
        );
        assert_eq!(b.payload().since, 2020);
    }

    #[test]
    fn flowはfan_outとfan_inを組み合わせた関数の辺として動く() {
        // §5 のデモと同じ形。graph! の宣言される辺と対照的に、flow! は
        // その場で関数を呼ぶだけの脱糖であることをアサーションで確認する。
        fn parse(s: &str) -> i32 {
            s.parse().unwrap()
        }
        fn validate(x: i32) -> bool {
            x >= 0
        }
        fn double(x: i32) -> i32 {
            x * 2
        }
        fn merge(valid: bool, doubled: i32) -> String {
            format!("valid={valid} doubled={doubled}")
        }

        #[rustfmt::skip]
        graphite::flow! {
            "21" -[parse]-> parsed,
            parsed -[validate]-> valid,
            parsed -[double]-> doubled,
            (valid, doubled) -[merge]-> summary,
        };
        assert_eq!(parsed, 21);
        assert!(valid);
        assert_eq!(doubled, 42);
        assert_eq!(summary, "valid=true doubled=42");
    }
}
