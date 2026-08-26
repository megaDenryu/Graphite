//! hello-graph — Graphite (`graph_schema!`/`graph!`) の意味論を確認する
//! 入門用example。
//!
//! **これは教材です。** アプリとしての面白さは無く、「1個ずつ意味論を
//! 確かめる」ことだけが目的です。実践的な使用例は他の3本
//! (`examples/build-pipeline`・`examples/org-analyzer`・
//! `examples/dialogue-engine`) を参照してください。
//!
//! 節ごとにファイルを分けています:
//! - §1 ノード型・エッジ属性型の宣言 (普通の struct) — このファイル
//! - §2 `graph_schema!` によるスキーマ宣言 (v4: `edge Kind = ...;` は
//!   新しい nominal 型の定義、`where` は制約) — このファイル
//! - §2.5 脱糖の実像 — 全要素キー・`KeyedTable` 格納・辺は名前付きフィールドの構造体
//!   として第一級、という v4 の実装を実測して解説する — このファイル
//! - §4 「できないこと」— コンパイルエラーになる例と、実際のエラー引用 — このファイル
//!   (§2 の宣言そのものが弾く書き方を並べているため、宣言と同じファイルに置く)
//! - §3 クックブック — `graph_schema!`/`graph!` が生成する公開APIの全列挙 — `cookbook.rs`
//!   とその配下 (構築・ノードを読む・エッジを辿る・一覧する・検証エラーを受ける)
//! - §5 `flow!` — 関数の辺 (`graph!` の宣言される辺との対比) — `flow_demo.rs`
//!
//! アサーションによる確認は `tests.rs` とその配下にあります。
//!
//! `cargo run` すると §3・§5 の内容が順に表示されます。

// ============================================================
// §1 型宣言 — ノード型・エッジ属性型は普通の Rust struct
// ============================================================
//
// `graph_schema!` はこれらの型を**生成せず、参照するだけ**です
// (`docs/schema_v4.md` §1)。derive・可視性・追加のメソッドは全部ふつうの
// Rust の話であり、Graphite 固有のルールはありません。

// `node Person;` と `node Team;` は、`Org` module 内に `PersonId` と
// `TeamId` を生成します。既存ID型を使う場合は `node Person(id: 型);` と
// 明示します (`docs/node_id_v4_2.md`)。

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
// 1. **`名前 = 定義`** — `edge Kind = (役割名: From) -> (役割名: To) ...;` は **`Kind` という
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
//    - `each <役割名>: N | N..M | N..*` — 端点の役割名ごとの多重度制約
//    - `unique pair` — 同じ (始点, 終点) の対に2本目を張ることを禁止
//      (=平行辺の禁止)
//
// 以下4本のエッジは、この "each 1 / each 0..1 / unique pair / 制約なし"
// という4パターンを一通りカバーするように選んであります:
//
// | エッジ         | 制約                | 積み荷        | 読み方 |
// |----------------|---------------------|----------------|--------|
// | `BelongsTo`    | `each member: 1`    | なし           | 全域関数。全社員は必ずどこか1つのチームに所属する |
// | `Boss`         | `each subordinate: 0..1` | `BossEdge` | 部分関数。上司がいない社員がいてもよいが、いるなら1人だけ |
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
// 方向を示すアクセサではなく `endpoints() -> (PersonRef,
// PersonRef)` になります。両端は同じノード型でなければならず、役割名も
// 書けません (対称性を型にも及ぼす設計、詳細は §3 の実行例参照)。

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod Org {
    include!("generated/main_org.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/main_org.rs";
    schema Org {
        node Person;
        node Team;

        edge BelongsTo = (member: Person) -> (team: Team) where each member: 1;
        edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1;
        edge Reports = (reporter: Person) -> (recipient: Person) where unique pair;
        edge ReviewedBy = (reviewee: Person) -[review: ReviewEdge]-> (reviewer: Person);
        edge Friends = Person -- Person where unique pair;
    }
}

mod cookbook;
mod flow_demo;
#[cfg(test)]
mod tests;

fn main() {
    cookbook::section3();
    flow_demo::section5();
}

// ============================================================
// §2.5 脱糖の実像 — 全要素キー・KeyedTable格納・辺の第一級化
// ============================================================
//
// 以下は `cargo expand` で実際に確認した生成物 (`cargo install cargo-expand`
// して `cargo expand --bin hello-graph 2>&1 | Select-String -Context 5
// "struct Boss"` のように確認できます) を元に、要点だけ抜き出して整理した
// ものです。要約であって書き下ろしではありません — 生成ロジックそのものは
// `crates/graphite-codegen/src/schema/codegen/` を正としてください。
//
// ## 1. 全要素がキー化される — ノードもエッジも
//
// v3 までは「エッジは HashMap のエントリ」でしたが、v4 では
// **辺そのものが、ノードと同じ資格を持つ第一級の要素**になりました。
// `graph_schema!` は `Boss` エッジ宣言から、`§1` で宣言した `PersonId` と
// 全く同じ形の newtype キーを生成します:
//
// ```rust
// pub struct BossId(pub String);
// ```
//
// `BossId` と `PersonId` は、どちらも `Org` schema module 内へ生成されます。
// そのため、同じ綴りのIDでも別schemaの生成型とは混ざりません。
//
// `graph!` リテラルの各行 `名前 = 値` の「名前」は、構築中にはノード/辺の
// 公開ID束縛、完成後には静的アクセサ名です (`docs/schema_v4.md` §0 規則1)。これは
// `instance_codegen.rs` の脱糖を見ると直接分かります — 例えば
//
// ```rust
// tanaka_boss = Boss(bob -[promo]-> alice),
// ```
//
// は次のように展開されます (実装が生成する呼び出し形。`__graphite_b` が
// builder、`__graphite_permit` は名前付き位置を積む操作の許可証
// (`crates/graphite/src/schema_runtime/named_construction.rs` の
// `NamedInsertPermit` 参照)、`clone()` は
// 端点のキーを渡すため):
//
// ```rust
// #[allow(unused_variables)]
// let (tanaka_boss, __graphite_named_tanaka_boss) = __graphite_b.add_named(
//     "tanaka_boss",
//     <Org::Boss as graphite::DirectedEdgeLiteral<_, _, _>>::from_graph_literal(
//         bob.clone(),
//         alice.clone(),
//         promo,
//     ),
//     __graphite_permit,
// );
// ```
//
// `tanaka_boss` はここで `Boss` の値そのものではなく **`BossId`** に束縛され、
// `__graphite_named_tanaka_boss` は凍結後の静的アクセサへ運ばれます。ノード行
// (`alice = Person { .. }`) も同じ形で
// `__graphite_b.insert_named("alice", Person { .. }, __graphite_permit)` に
// 展開され、構築中の `alice` は `PersonId`、完成後の `g.alice()` は
// `PersonRef` を返します。
//
// ## 2. 辺は名前付きフィールドの構造体として実在する
//
// `edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person) where each subordinate: 0..1;` から
// `graph_schema!` が生成する実際の型は次の通りです
// (`graphite_codegen::schema::codegen::edge_value::gen_edge_value_structs`):
//
// ```rust
// #[derive(Debug, Clone, PartialEq)]
// pub struct Boss {
//     pub subordinate: PersonId,
//     pub superior: PersonId,
//     pub appointment: BossEdge,
// }
//
// impl Boss {
//     pub fn new(from: PersonId, to: PersonId, payload: BossEdge) -> Self { .. }
// }
// ```
//
// 積み荷なしの辺 (`BelongsTo`) も役割名の公開フィールドだけを持ちます。
// **この辺値型はマクロの内部表現ではなく、公開structとして実在します** —
// マクロの外で `Boss { subordinate, superior, appointment }` と普通に構築
// できることは、`crates/graphite/tests/orgchart_macro.rs` の
// `名前付きフィールドの辺値はマクロ外でも普通に構築できる` が実例です。
//
// ## 3. 完成済みグラフは内部位置で端点を結ぶ
//
// builder が受け取る `Boss` 値は公開IDを持ちますが、freeze はIDを種別専用の
// 非公開位置へ変換します。完成済みグラフの概念的な格納形は次の通りです。
//
// ```rust
// pub struct Graph {
//     persons: graphite::KeyedTable<PersonId, Person>,
//     teams: graphite::KeyedTable<TeamId, Team>,
//     boss: graphite::KeyedTable<BossId, BossRecord>,
//     boss_from_index: std::collections::HashMap<PersonPosition, Vec<BossPosition>>,
// }
// ```
//
// `BossRecord` は端点位置と積み荷を持ちます。公開の `PersonRef<'graph>` と
// `BossRef<'graph>` は `&Graph` と位置だけを持つ `Copy + Clone` の値です。
// 公開IDから参照を得る最初の検索後は、端点を辿るたびにIDのハッシュ検索を
// 繰り返しません。
//
// ## 4. メンタルモデル: 「ラベル=表」から「辺=第一級の行」へ
//
// v3 の比喩は「ラベルはリレーショナルDBの表名、辺はその1行」でしたが、
// v4 ではさらに一歩進み、**辺という「行」自体が独立したキーを持つ実体**
// になりました。`bob_ref.boss_as_subordinate()` はNodeRef内の位置から、
// 位置索引から `BossPosition` を取得して `BossRef` を返します。`BossRef` の
// `superior()` は記録済みの端点位置から `PersonRef` を直接組み立てます。

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
// `Boss` はスキーマ宣言で定義された名前付きフィールドの構造体型です (§2.5)。
// `since` は `BossEdge` のfieldなので、辺値からは `boss.appointment.since`
// または `boss.payload().since` と辿ります。型名 `Boss` は辺の実体ではないため、
// `Boss.since` とは書けません。
//
// fn section4_1() {
//     let _ = Boss.since;
// }
//
// コンパイラは型名を値として使ったことを報告します。

// --- 4.2 フィールドに直接アクセスしようとする (非公開フィールドで弾かれる) ---
//
// `Org::Graph` の各フィールド (`boss` 等) は非公開の内部ストレージであり、
// 格納値は構築用の `Boss` 値そのものではなく非公開レコード型
// `KeyedTable<BossId, __BossRecord>` です (§2.5 参照)。`graph_schema!` は
// schema の中身全体を `pub mod Org { .. }` へ生成するため
// (`crates/graphite-codegen/src/schema/codegen/mod.rs` の `generate` 参照)、この
// ファイルの `fn section4_2` はマクロ呼び出しと同じソースファイルにあっても
// `Org` module の**外側**にいます。したがって `g.boss` は型不一致以前に
// 非公開フィールドへのアクセスとして弾かれます。
//
// fn section4_2(g: &Org::Graph) -> Person {
//     g.boss
// }
//
// 実際のエラー (コメントを外して `cargo build` した際に採取):
//
//   error[E0616]: field `boss` of struct `Org::Graph` is private
//       --> src\main.rs:1310:7
//        |
//   1310 |     g.boss
//        |       ^^^^ private field
//
// (`Org` module の内側であれば `g.boss` 自体は評価できますが、その型は
// `Boss` ではなく非公開レコード型 `KeyedTable<BossId, __BossRecord>` その
// ものです。つまり「boss というフィールドで社員そのものが手に入る」という
// 誤解は、まず可視性で、次に型不一致で二重に正されます。)

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
//   error[E0433]: failed to resolve: use of undeclared type `NoSuchKind`
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
//   note: `BelongsTo::new` requires a `TeamId` for the `team` field
//      --> src\main.rs:94:14
//       |
//    94 |         edge BelongsTo = (member: Person) -> (team: Team) where each member: 1;
//       |              ^^^^^^^^^
//       = note: this error originates in the macro `graphite::graph` (in Nightly builds, run with -Z macro-backtrace for more info)
