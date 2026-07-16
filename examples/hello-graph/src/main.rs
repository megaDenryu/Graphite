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
//! - §2 `graph_schema!` によるスキーマ宣言 (ラベルとは何なのかの説明)
//! - §2.5 脱糖の実像 — `-[label = 式]->` は誰が何を持つ形に展開されるのか
//! - §3 クックブック — `graph_schema!`/`graph!` が生成する公開APIの全列挙
//! - §4 「できないこと」— コンパイルエラーになる例と、実際のエラー引用
//!
//! `cargo run` すると §3 の内容が順に表示されます。

// ============================================================
// §1 型宣言 — ノード型・エッジ属性型は普通の Rust struct
// ============================================================
//
// `graph_schema!` はこれらの型を**生成せず、参照するだけ**です
// (README「使用例」節、`docs/edge_syntax_v2.md` 参照)。derive・可視性・
// 追加のメソッドは全部ふつうの Rust の話であり、Graphite 固有のルールは
// ありません。

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

/// `boss` エッジが辺1本ごとに運ぶペイロード (属性)。
#[derive(Debug, Clone, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}

/// `reviewed_by` エッジが辺1本ごとに運ぶペイロード (属性)。
#[derive(Debug, Clone, PartialEq)]
pub struct ReviewEdge {
    pub year: i32,
}

// ============================================================
// §2 schema 宣言
// ============================================================
//
// `edge From -[label]-> To (多重度);` の読み方:
//
// - `label` は「エッジ種別の宣言」です。struct のフィールド名に相当する
//   1トークンで、ここから `{label}`/`try_{label}`/`{label}_id(s)`/
//   `{label}_pairs` といったアクセサ・builder メソッド・違反 enum の
//   バリアントが機械的に命名・生成されます。つまり `label` は「値」では
//   なく「これから生成される一群のメソッド名の元になる識別子」です。
//   値のように読み書きできる変数ではありません (§4 で実際に確認します)。
// - `label: Type` の `Type` は「辺1本ごとが運ぶペイロードの型」です。
//   属性なしエッジ (`belongs_to`・`reports`) は何も運ばず、属性ありエッジ
//   (`boss: BossEdge`・`reviewed_by: ReviewEdge`) はその型の値を辺1本ごとに
//   1つ持ちます。(この `label: Type` という書き方は schema 宣言だけの話
//   です。schema は常に `:` — 型付け — を使います。次の §3 で見る `graph!`
//   リテラルはこれとは対照的に常に `=` — 代入 — を使います。)
// - 多重度 `(1)`/`(0..1)`/`(0..*)` は矢印の外側に書きます (辺そのものの
//   属性ではなく「本数の制約」だからです)。
//
// 4本のエッジは「多重度 × 属性の有無」の組み合わせを一通り確かめられる
// ように選んであります:
//
// | ラベル        | 多重度   | 属性         |
// |---------------|----------|--------------|
// | `belongs_to`  | `(1)`    | なし         |
// | `boss`        | `(0..1)` | `BossEdge`   |
// | `reports`     | `(0..*)` | なし         |
// | `reviewed_by` | `(0..*)` | `ReviewEdge` |

#[rustfmt::skip]
graphite::graph_schema! {
    schema Org {
        node Person;
        node Team;

        edge Person -[belongs_to]-> Team (1);
        edge Person -[boss: BossEdge]-> Person (0..1);
        edge Person -[reports]-> Person (0..*);
        edge Person -[reviewed_by: ReviewEdge]-> Person (0..*);
    }
}

fn main() {
    section3();
}

// ============================================================
// §2.5 脱糖の実像 — エッジは誰が持っているのか
// ============================================================
//
// 「`-[boss = BossEdge { .. }]->` は脱糖されたとき、構造体は誰が
// どういうプロパティとして持つのか? `boss.attr = 構造体` のような形
// なのか?」という疑問への回答です。答えは **No** です。`boss` は
// オブジェクトではなく、以下で見るように「表 (テーブル) の名前」です。
//
// ## 1. `graph!` の脱糖はただのメソッド呼び出し
//
// `crates/graphite-macros/src/instance_codegen.rs` (`generate` 関数) を
// 読むと、
//
// ```rust
// bob -[boss = BossEdge { since: 2021 }]-> alice,
// ```
//
// は次のコードへ展開されることが分かります (実際の生成コードそのまま。
// 引数は3つ: 始点キー・終点キー・ペイロード式):
//
// ```rust
// __graphite_b.boss(bob.clone(), alice.clone(), BossEdge { since: 2021 });
// ```
//
// `boss` はここでは `OrgBuilder` の**メソッド名**であって、`.attr` で
// たどる先のフィールドではありません。
//
// ## 2. 格納先はグラフ本体の「ラベル名の非公開フィールド」
//
// `b.boss(..)` が最終的に格納する先は、`graph_schema!` が生成する `Org`
// struct の `boss` という名前のフィールドです
// (`crates/graphite-macros/src/schema_codegen.rs` の `gen_schema_struct`/
// `edge_stored_value_type` 参照。以下は実際に生成される型そのもの、
// 多重度・属性の有無で形が変わることを1行ずつ示します):
//
// ```rust
// // belongs_to: (1) + 属性なし  -> 終点キーを直接値に持つ
// belongs_to: std::collections::HashMap<PersonId, TeamId>,
// // boss:       (0..1) + 属性あり -> (終点キー, ペイロード) のタプル
// boss: std::collections::HashMap<PersonId, (PersonId, BossEdge)>,
// // reports:    (0..*) + 属性なし  -> 終点キーの Vec
// reports: std::collections::HashMap<PersonId, Vec<PersonId>>,
// // reviewed_by: (0..*) + 属性あり -> (終点キー, ペイロード) タプルの Vec
// reviewed_by: std::collections::HashMap<PersonId, Vec<(PersonId, ReviewEdge)>>,
// ```
//
// つまり `boss` エッジ1本 (`bob -[boss = BossEdge{since:2021}]-> alice`) は
// `boss` という `HashMap` の中の1エントリ `bob -> (alice, BossEdge{since:2021})`
// として格納されるだけです。`BossEdge` の値は「`boss` というオブジェクトの
// プロパティ」ではなく、「`boss` という表の、キー `bob` の行に載っている
// ペイロード列」です。
//
// ## 3. メンタルモデル: リレーショナル DB の比喩
//
// - **ラベル = テーブル名**。`boss`/`belongs_to`/`reports`/`reviewed_by` は
//   それぞれ独立した1つの表の名前です。
// - **辺1本 = 1行** = `(from, to[, ペイロード])`。
// - `-[boss = 式]->` は「`boss` 表に `(bob, alice, 式の値)` という1行を
//   INSERT する」ことに相当します (実際 `OrgBuilder::boss` は
//   `self.boss.push((from, to, attrs))` で `Vec` に積むだけ。検査は
//   `freeze` 時にまとめて行われます)。
// - `g.boss(&bob)` は「`boss` 表を `from = bob` で引き、`to` (=alice) を
//   ノード実体 (`Person`) に解決してから返す」ことに相当します。
//
// `boss.since` のように書けない (§4.1) のは、`boss` がオブジェクトでは
// なく表の名前だから、というのがこの比喩の結論です。`attrs.since` の形で
// アクセスできるのは、`g.boss(&id)` という**クエリの戻り値**
// (`(&Person, &BossEdge)`) の2番目の要素だからであり、`boss` という値
// そのものが `since` を持っているわけではありません。
//
// ## 4. §4.2 との対応
//
// 後述 §4.2 で `g.boss` を `Person` として直接使おうとすると
// `mismatched types` になり、実際の型が
// `HashMap<PersonId, (PersonId, BossEdge)>` であることが分かります。
// これはまさに上記2節の内部テーブルそのものです — `g.boss` という式を
// 素朴に書いた瞬間、隠していたはずの内部実装 (表そのもの) がそのまま型
// エラーに露出する、というのが §4.2 の正体です。

// ============================================================
// §3 クックブック — 生成される公開APIの全列挙
// ============================================================
//
// `graph_schema!` が `schema Org { .. }` から生成する公開API を、
// 1関数=1つの「やりたいこと」に分けて全部並べています。
// カテゴリ順: 構築 → ノードを読む → エッジを辿る (多重度別) →
// 一覧する (pairs/ids) → 検証エラーを受ける。
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
    builderの総称insertで組み立てる();

    // --- ノードを読む ---
    println!("\n--- ノードを読む ---");
    人ノードを1件読む(&g);
    チームノードを1件読む(&g);
    personidの作り方とgraphのキーの対応を確認する(&g);

    // --- エッジを辿る (多重度別) ---
    println!("\n--- エッジを辿る (多重度別) ---");
    多重度1のエッジを辿る(&g);
    多重度1のエッジを安全に辿る(&g);
    多重度1のエッジをidだけで辿る(&g);
    多重度0か1のエッジを辿る(&g);
    多重度0か1のエッジをidだけで辿る(&g);
    多重度0以上のエッジを辿る(&g);
    多重度0以上のエッジをidだけで辿る(&g);
    多重度0以上属性ありのエッジを辿る(&g);
    多重度0以上属性ありのエッジをidだけで辿る(&g);

    // --- 一覧する (pairs / ids) ---
    println!("\n--- 一覧する (pairs / ids) ---");
    person_idsで全ノードキーを列挙する(&g);
    team_idsで全ノードキーを列挙する(&g);
    belongs_to_pairsで属性なしエッジを列挙する(&g);
    boss_pairsで属性ありエッジを列挙する(&g);
    reports_pairsで多重度0以上のエッジを列挙する(&g);
    reviewed_by_pairsで属性あり多重度0以上のエッジを列挙する(&g);

    // --- 検証エラーを受ける ---
    println!("\n--- 検証エラーを受ける ---");
    重複ノードキーの違反を受け取る();
    未知の始点キーの違反を受け取る();
    未知の終点キーの違反を受け取る();
    多重度違反を受け取る();
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

        alice -[belongs_to]-> eng,
        bob   -[belongs_to]-> eng,
        carol -[belongs_to]-> eng,
        bob   -[boss = BossEdge { since: 2021 }]-> alice,
        alice -[reports]-> bob,
        alice -[reports]-> carol,
        bob   -[reviewed_by = ReviewEdge { year: 2023 }]-> alice,
        bob   -[reviewed_by = ReviewEdge { year: 2024 }]-> carol,
    })
    .expect("正常なグラフは構築に成功するはず");
    let alice_person: &Person = g.person(&PersonId("alice".to_string())).unwrap();
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
        alice -[belongs_to]-> eng,
    })
    .expect("外部変数を渡した graph! も構築に成功するはず");
    let alice_person: &Person = g.person(&PersonId("alice".to_string())).unwrap();
    println!("(構築2: 外部変数渡し) alice = {}", alice_person.name);
}

// やりたいこと: エッジの属性 (`BossEdge`) もグラフの外で作った値を渡せることを確認する。
fn 外部で作ったエッジ属性をgraphリテラルに渡す() {
    let promotion: BossEdge = BossEdge { since: 2019 };
    #[rustfmt::skip]
    let g: Org = graphite::graph!(Org {
        alice = Person { name: "Alice".into() },
        bob   = Person { name: "Bob".into() },
        eng   = Team { name: "Engineering".into() },
        alice -[belongs_to]-> eng,
        bob   -[belongs_to]-> eng,
        bob   -[boss = promotion]-> alice,
    })
    .expect("外部エッジ属性を渡した graph! も構築に成功するはず");
    let boss_pair: (&Person, &BossEdge) = g.boss(&PersonId("bob".to_string())).unwrap();
    println!("(構築3: 外部エッジ属性渡し) bob の上司就任年 = {}", boss_pair.1.since);
}

// やりたいこと: graph! を使わず、builder の型名つきメソッド (`b.person(id, value)`) で組み立てる。
fn builderの型名メソッドで組み立てる() {
    let g: Org = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("dave".to_string()), Person { name: "Dave".to_string() });
        b.team(TeamId("sales".to_string()), Team { name: "Sales".to_string() });
        b.belongs_to(PersonId("dave".to_string()), TeamId("sales".to_string()));
    })
    .expect("builder の型名メソッドでも構築に成功するはず");
    let dave: &Person = g.person(&PersonId("dave".to_string())).unwrap();
    println!("(構築4: builderの型名メソッド) dave = {}", dave.name);
}

// やりたいこと: builder の総称メソッド `insert` に値を渡し、値の型から自動で振り分けさせる
// (`insert` の型境界 `N: OrgNode` は graph_schema! が生成した `OrgNode` トレイトで
// 満たされる。利用者がこのトレイトを直接呼ぶことは無い)。
fn builderの総称insertで組み立てる() {
    let g: Org = Org::create(|b: &mut OrgBuilder| {
        let eve_id: PersonId = b.insert("eve", Person { name: "Eve".to_string() });
        let sales_id: TeamId = b.insert("sales", Team { name: "Sales".to_string() });
        b.belongs_to(eve_id, sales_id);
    })
    .expect("insert 経由の構築も成功するはず");
    let eve: &Person = g.person(&PersonId("eve".to_string())).unwrap();
    println!("(構築5: builderの総称insert) eve = {}", eve.name);
}

// --- ノードを読む ---

// やりたいこと: ノード種別ごとのアクセサ `{node}(&id)` で1件読む (無ければ None)。
fn 人ノードを1件読む(g: &Org) {
    let alice: Option<&Person> = g.person(&PersonId("alice".to_string()));
    if let Some(person) = alice {
        println!("(ノード) person(&alice) = {}", person.name);
    }
    let unknown: Option<&Person> = g.person(&PersonId("dave".to_string()));
    println!("(ノード) person(&dave)  = {unknown:?} (この g には居ない)");
}

// やりたいこと: `team(&id)` も同じ形。ノード型が違っても命名規則は共通。
fn チームノードを1件読む(g: &Org) {
    let eng: Option<&Team> = g.team(&TeamId("eng".to_string()));
    if let Some(team) = eng {
        println!("(ノード) team(&eng) = {}", team.name);
    }
}

// やりたいこと: `PersonId` はただの newtype なので手で組み立てられる。graph! の
// キー (`alice = ..`) はこの `PersonId("alice".to_string())` と同一視される。
fn personidの作り方とgraphのキーの対応を確認する(g: &Org) {
    let hand_built_id: PersonId = PersonId("alice".to_string());
    let alice: &Person = g
        .person(&hand_built_id)
        .expect("graph!のキーaliceがPersonId(\"alice\")と一致するはず");
    println!("(型) PersonId(\"alice\".to_string()) で graph! の alice = {} が引ける", alice.name);
}

// --- エッジを辿る (多重度別) ---

// やりたいこと: 多重度(1)のアクセサ `{label}(&id)` は参照そのものを返す (未知キーはパニック)。
fn 多重度1のエッジを辿る(g: &Org) {
    let team: &Team = g.belongs_to(&PersonId("alice".to_string()));
    println!("(1) belongs_to(&alice) = {}", team.name);
}

// やりたいこと: パニックしない版 `try_{label}` は Option を返す (多重度(1)にのみ存在)。
fn 多重度1のエッジを安全に辿る(g: &Org) {
    let known: Option<&Team> = g.try_belongs_to(&PersonId("alice".to_string()));
    if let Some(team) = known {
        println!("(1) try_belongs_to(&alice) = {}", team.name);
    }
    let unknown: Option<&Team> = g.try_belongs_to(&PersonId("dave".to_string()));
    println!("(1) try_belongs_to(&dave)  = {unknown:?} (未知キーはNone)");
}

// やりたいこと: 相手ノードの値ではなくキーだけが欲しい場合は `{label}_id`/`try_{label}_id`。
fn 多重度1のエッジをidだけで辿る(g: &Org) {
    let team_id: &TeamId = g.belongs_to_id(&PersonId("alice".to_string()));
    println!("(1) belongs_to_id(&alice) = {team_id:?}");
    let unknown_id: Option<&TeamId> = g.try_belongs_to_id(&PersonId("dave".to_string()));
    println!("(1) try_belongs_to_id(&dave) = {unknown_id:?}");
}

// やりたいこと: 多重度(0..1)+属性ありのアクセサは `Option<(&Node, &Attrs)>` を返す。
// 属性の値へは "ふつうのフィールドアクセス" で辿れる (`attrs.since`)。
fn 多重度0か1のエッジを辿る(g: &Org) {
    let boss: Option<(&Person, &BossEdge)> = g.boss(&PersonId("bob".to_string()));
    if let Some((boss_person, attrs)) = boss {
        println!("(0..1)   boss(&bob) = {} (就任年: {})", boss_person.name, attrs.since);
    }
    let no_boss: Option<(&Person, &BossEdge)> = g.boss(&PersonId("alice".to_string()));
    println!("(0..1)   boss(&alice) = {no_boss:?} (aliceには上司がいない)");
}

// やりたいこと: 多重度(0..1)のID版アクセサ `{label}_id` は `Option<&Id>` を返す
// (多重度(1)と違い try_版は存在しない。無い/未知キーどちらも None に落ちるため)。
fn 多重度0か1のエッジをidだけで辿る(g: &Org) {
    let boss_id: Option<&PersonId> = g.boss_id(&PersonId("bob".to_string()));
    println!("(0..1)   boss_id(&bob) = {boss_id:?}");
}

// やりたいこと: 多重度(0..*)のアクセサは `Vec` を返す。素の for ループで受ける。
fn 多重度0以上のエッジを辿る(g: &Org) {
    let reports: Vec<&Person> = g.reports(&PersonId("alice".to_string()));
    for report in &reports {
        println!("(0..*)   reports(&alice) に {} が含まれる", report.name);
    }
}

// やりたいこと: 多重度(0..*)のID版アクセサ `{label}_ids` は `Vec<&Id>` を返す。
// 格納順 (`graph!` のソース記述順) を保持する。
fn 多重度0以上のエッジをidだけで辿る(g: &Org) {
    let report_ids: Vec<&PersonId> = g.reports_ids(&PersonId("alice".to_string()));
    for id in &report_ids {
        println!("(0..*)   reports_ids(&alice) に {id:?} が含まれる");
    }
}

// やりたいこと: 多重度(0..*)+属性ありのアクセサは `Vec<(&Node, &Attrs)>` を返す。
fn 多重度0以上属性ありのエッジを辿る(g: &Org) {
    let reviewers: Vec<(&Person, &ReviewEdge)> = g.reviewed_by(&PersonId("bob".to_string()));
    for (reviewer, attrs) in &reviewers {
        println!("(0..*属性あり) bob のレビュアー: {} ({}年度)", reviewer.name, attrs.year);
    }
}

// やりたいこと: 属性の有無に関わらず、ID版アクセサ (`{label}_ids`) は属性を含まず
// キーだけを返す (属性が欲しい場合は値アクセサ `{label}` を使う)。
fn 多重度0以上属性ありのエッジをidだけで辿る(g: &Org) {
    let reviewer_ids: Vec<&PersonId> = g.reviewed_by_ids(&PersonId("bob".to_string()));
    for id in &reviewer_ids {
        println!("(0..*属性あり) reviewed_by_ids(&bob) に {id:?} が含まれる");
    }
}

// --- 一覧する (pairs / ids) ---

// やりたいこと: `{node}_ids()` でノード種別ごとの全キーを列挙する。
fn person_idsで全ノードキーを列挙する(g: &Org) {
    for id in g.person_ids() {
        println!("(一覧) person_ids: {id:?}");
    }
}

fn team_idsで全ノードキーを列挙する(g: &Org) {
    for id in g.team_ids() {
        println!("(一覧) team_ids: {id:?}");
    }
}

// やりたいこと: 属性なしエッジの `{label}_pairs()` は (始点キー, 終点キー) の2つ組。
fn belongs_to_pairsで属性なしエッジを列挙する(g: &Org) {
    for (from, to) in g.belongs_to_pairs() {
        println!("(pairs 2つ組) belongs_to: {from:?} -> {to:?}");
    }
}

// やりたいこと: 属性ありエッジの `{label}_pairs()` は (始点キー, 終点キー, 属性) の3つ組。
fn boss_pairsで属性ありエッジを列挙する(g: &Org) {
    for (from, to, attrs) in g.boss_pairs() {
        println!("(pairs 3つ組) boss: {from:?} -> {to:?} (since={})", attrs.since);
    }
}

// やりたいこと: 多重度(0..*)の `{label}_pairs()` は始点キーごとの複数終点へ展開される。
fn reports_pairsで多重度0以上のエッジを列挙する(g: &Org) {
    for (from, to) in g.reports_pairs() {
        println!("(pairs 0..*展開) reports: {from:?} -> {to:?}");
    }
}

// やりたいこと: 多重度(0..*)+属性ありの `{label}_pairs()` は3つ組かつ展開される。
fn reviewed_by_pairsで属性あり多重度0以上のエッジを列挙する(g: &Org) {
    for (from, to, attrs) in g.reviewed_by_pairs() {
        println!("(pairs 0..*属性あり) reviewed_by: {from:?} -> {to:?} ({}年度)", attrs.year);
    }
}

// --- 検証エラーを受ける ---

// やりたいこと: 同じキーを2回宣言すると `Duplicate{Node}` 違反になることを確認する。
fn 重複ノードキーの違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        b.person(PersonId("alice".to_string()), Person { name: "Alice2".to_string() });
    });
    match result {
        Err(OrgViolation::DuplicatePerson(id)) => println!("(違反) 重複キー: {id:?}"),
        _ => panic!("重複キー違反が検出されるはず"),
    }
}

// やりたいこと: 未宣言の始点キーからエッジを張ると `{Label}UnknownSource` 違反になる。
fn 未知の始点キーの違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.team(TeamId("eng".to_string()), Team { name: "Engineering".to_string() });
        b.belongs_to(PersonId("存在しない社員".to_string()), TeamId("eng".to_string()));
    });
    match result {
        Err(OrgViolation::BelongsToUnknownSource { key }) => {
            println!("(違反) 未知の始点キー: {key:?}");
        }
        _ => panic!("未知の始点キー違反が検出されるはず"),
    }
}

// やりたいこと: 未宣言の終点キーへエッジを張ると `{Label}UnknownTarget` 違反になる。
fn 未知の終点キーの違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        b.belongs_to(PersonId("alice".to_string()), TeamId("存在しないチーム".to_string()));
    });
    match result {
        Err(OrgViolation::BelongsToUnknownTarget { key }) => {
            println!("(違反) 未知の終点キー: {key:?}");
        }
        _ => panic!("未知の終点キー違反が検出されるはず"),
    }
}

// やりたいこと: 多重度(1)を満たさない (0本の) エッジは `{Label}Multiplicity` 違反になる。
fn 多重度違反を受け取る() {
    let result: Result<Org, OrgViolation> = Org::create(|b: &mut OrgBuilder| {
        b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
        // aliceをどのチームにも所属させない (belongs_to は多重度(1))
    });
    match result {
        Err(OrgViolation::BelongsToMultiplicity { source, count }) => {
            println!("(違反) 多重度違反: {source:?} は {count} 本 (期待は1本)");
        }
        _ => panic!("多重度違反が検出されるはず"),
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
        // alice, bobともどのチームにも所属させない (2件の多重度違反が集まるはず)
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

// --- 4.1 ラベルを変数として使おうとする ---
//
// `boss` はスキーマ宣言の中の1トークンであり、実行時に読み書きできる
// 変数ではありません。生成されるのは `g.boss(...)` という**メソッド**
// であって、裸の `boss` という名前の値は存在しません。
//
// fn section4_1(g: &Org) {
//     let _ = boss.since;
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0425]: cannot find value `boss` in this scope
//      --> src\main.rs:580:13
//       |
//   580 |     let _ = boss.since;
//       |             ^^^^ not found in this scope

// --- 4.2 フィールドに直接アクセスしようとする (メソッド呼び出しを忘れる) ---
//
// アクセサは常に「呼び出す」もの (`g.boss(&id)`) であり、`g.boss` という
// フィールドそのものは非公開の内部ストレージ (`HashMap<PersonId, (PersonId, BossEdge)>`、
// §2.5 参照) です。このファイルは schema 宣言と同じモジュールなので `g.boss` という
// 式自体は private エラーにはなりません (Rust の可視性はモジュール単位
// であり、同一モジュール内では非公開フィールドも見えるため)。しかし
// 中身は `Person` ではなく内部ストレージそのものなので、`Person` として
// 使おうとした瞬間に型不一致になります。括弧を付け忘れて `g.boss` と
// 書くと、素朴には使えません。
//
// fn section4_2(g: &Org) -> Person {
//     g.boss
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0308]: mismatched types
//      --> src\main.rs:603:5
//       |
//   602 | fn section4_2(g: &Org) -> Person {
//       |                           ------ expected `Person` because of return type
//   603 |     g.boss
//       |     ^^^^^^ expected `Person`, found `HashMap<PersonId, (PersonId, BossEdge)>`
//       |
//       = note: expected struct `Person`
//                  found struct `HashMap<PersonId, (PersonId, BossEdge)>`
//
// (`g.boss` という式そのものは同一モジュール内なので private エラーには
// ならず素朴に評価できてしまいますが、その型は `Person` ではなく内部
// ストレージの `HashMap` そのものであることがこの型不一致から分かります。
// つまり「boss というフィールドで社員そのものが手に入る」という誤解は
// この型不一致で正されます。§2.5 で見た内部テーブルの型そのものです。)

// --- 4.3 存在しないエッジラベルを graph! に書く ---
//
// v3 (`docs/graph_literal_v3.md` §4) でハンドシェイクマクロを全廃したため、
// 未知のラベルは素の rustc メソッド解決 (E0599) だけで検出されます
// (「利用可能なエッジ一覧」付きの親切な compile_error! は無くなりました。
// これは意図した trade-off です)。
//
// fn section4_3() {
//     #[rustfmt::skip]
//     let _ = graphite::graph!(Org {
//         alice = Person { name: "Alice".into() },
//         eng = Team { name: "Engineering".into() },
//         alice -[no_such_label]-> eng,
//     });
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0599]: no method named `no_such_label` found for mutable reference `&mut OrgBuilder` in the current scope
//      --> src\main.rs:637:17
//       |
//   634 |       let _ = graphite::graph!(Org {
//       |  _____________-
//   635 | |         alice = Person { name: "Alice".into() },
//   636 | |         eng = Team { name: "Engineering".into() },
//   637 | |         alice -[no_such_label]-> eng,
//       | |                -^^^^^^^^^^^^^ method not found in `&mut OrgBuilder`
//       | |________________|
//       |

// --- 4.4 端点の型を間違えたエッジを graph! に書く ---
//
// `belongs_to` は `Person -[belongs_to]-> Team` として宣言されているので、
// from/to は Person/Team でなければなりません。両方を Person にすると
// 型不一致になります。
//
// fn section4_4() {
//     #[rustfmt::skip]
//     let _ = graphite::graph!(Org {
//         alice = Person { name: "Alice".into() },
//         bob = Person { name: "Bob".into() },
//         alice -[belongs_to]-> bob,
//     });
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0308]: mismatched types
//      --> src\main.rs:663:13
//       |
//   663 |       let _ = graphite::graph!(Org {
//       |  _____________^
//   664 | |         alice = Person { name: "Alice".into() },
//   665 | |         bob = Person { name: "Bob".into() },
//   666 | |         alice -[belongs_to]-> bob,
//       | |                 ---------- arguments to this method are incorrect
//   667 | |     });
//       | |______^ expected `TeamId`, found `PersonId`
//       |
//   note: method defined here
//      --> src\main.rs:88:23
//       |
//    83 | / graphite::graph_schema! {
//    84 | |     schema Org {
//    85 | |         node Person;
//    86 | |         node Team;
//    87 | |
//    88 | |         edge Person -[belongs_to]-> Team (1);
//       | |                       ^^^^^^^^^^
//   ...   |
//    93 | | }
//       | |_-
//       = note: this error originates in the macro `graphite::graph` (in Nightly builds, run with -Z macro-backtrace for more info)

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

            alice -[belongs_to]-> eng,
            bob   -[belongs_to]-> eng,
            carol -[belongs_to]-> eng,
            bob   -[boss = BossEdge { since: 2021 }]-> alice,
            alice -[reports]-> bob,
            alice -[reports]-> carol,
            bob   -[reviewed_by = ReviewEdge { year: 2023 }]-> alice,
            bob   -[reviewed_by = ReviewEdge { year: 2024 }]-> carol,
        });
        g.expect("正常なグラフは構築に成功するはず")
    }

    #[test]
    fn 多重度1のアクセサは参照そのものを返す() {
        let g = build();
        let team = g.belongs_to(&PersonId("alice".to_string()));
        assert_eq!(team.name, "Engineering");
    }

    #[test]
    fn 多重度0か1のアクセサはoptionのタプルを返し属性フィールドへアクセスできる() {
        let g = build();
        let (boss, attrs) = g
            .boss(&PersonId("bob".to_string()))
            .expect("bobの上司はaliceのはず");
        assert_eq!(boss.name, "Alice");
        assert_eq!(attrs.since, 2021);
        assert!(g.boss(&PersonId("alice".to_string())).is_none());
    }

    #[test]
    fn 多重度0以上のアクセサはvecを返す() {
        let g = build();
        let mut names: Vec<&str> = g
            .reports(&PersonId("alice".to_string()))
            .into_iter()
            .map(|p| p.name.as_str())
            .collect();
        names.sort();
        assert_eq!(names, vec!["Bob", "Carol"]);
    }

    #[test]
    fn try_belongs_toは未知キーでnoneを返す() {
        let g = build();
        assert!(g
            .try_belongs_to(&PersonId("dave".to_string()))
            .is_none());
    }

    #[test]
    fn pairsイテレータは3つ組で列挙できる() {
        let g = build();
        let boss_pairs: Vec<(&PersonId, &PersonId, &BossEdge)> = g.boss_pairs().collect();
        assert_eq!(boss_pairs.len(), 1);
        let (from, to, attrs) = boss_pairs[0];
        assert_eq!(*from, PersonId("bob".to_string()));
        assert_eq!(*to, PersonId("alice".to_string()));
        assert_eq!(attrs.since, 2021);
    }

    #[test]
    fn person_で1件読める() {
        let g = build();
        assert_eq!(g.person(&PersonId("alice".to_string())).unwrap().name, "Alice");
        assert!(g.person(&PersonId("dave".to_string())).is_none());
    }

    #[test]
    fn belongs_to_idは多重度1でキーを返しtryは未知キーでnoneになる() {
        let g = build();
        assert_eq!(*g.belongs_to_id(&PersonId("alice".to_string())), TeamId("eng".to_string()));
        assert!(g.try_belongs_to_id(&PersonId("dave".to_string())).is_none());
    }

    #[test]
    fn boss_idは多重度0か1でoptionのキーを返す() {
        let g = build();
        assert_eq!(g.boss_id(&PersonId("bob".to_string())), Some(&PersonId("alice".to_string())));
        assert_eq!(g.boss_id(&PersonId("alice".to_string())), None);
    }

    #[test]
    fn reports_idsは追加順を保持したvecを返す() {
        let g = build();
        assert_eq!(
            g.reports_ids(&PersonId("alice".to_string())),
            vec![&PersonId("bob".to_string()), &PersonId("carol".to_string())]
        );
    }

    #[test]
    fn reviewed_byは属性あり多重度0以上でvecのタプルを返す() {
        let g = build();
        let reviewers = g.reviewed_by(&PersonId("bob".to_string()));
        assert_eq!(reviewers.len(), 2);
        assert!(reviewers.iter().any(|(p, a)| p.name == "Alice" && a.year == 2023));
        assert!(reviewers.iter().any(|(p, a)| p.name == "Carol" && a.year == 2024));
    }

    #[test]
    fn 重複キーはduplicate違反になる() {
        let result = Org::create(|b| {
            b.person(PersonId("alice".to_string()), Person { name: "Alice".to_string() });
            b.person(PersonId("alice".to_string()), Person { name: "Alice2".to_string() });
        });
        assert!(matches!(result, Err(OrgViolation::DuplicatePerson(_))));
    }

    #[test]
    fn 未知の始点キーはunknownsource違反になる() {
        let result = Org::create(|b| {
            b.team(TeamId("eng".to_string()), Team { name: "Engineering".to_string() });
            b.belongs_to(PersonId("存在しない社員".to_string()), TeamId("eng".to_string()));
        });
        assert!(matches!(result, Err(OrgViolation::BelongsToUnknownSource { .. })));
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
}
