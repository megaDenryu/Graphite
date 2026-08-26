//! §3 クックブック — 生成される公開APIの全列挙。
//!
//! `graph_schema!` が `schema Org { .. }` から生成する公開API を、
//! 1関数=1つの「やりたいこと」に分けて全部並べています。
//! カテゴリ順: 構築 → ノードを読む → エッジを辿る → 一覧する →
//! 検証エラーを受ける。このファイルはその**呼び出し順**を持ち、実演そのものは
//! 確かめる内容ごとのサブモジュール (`literal_construction`・
//! `builder_construction`・`node_reading`・`edge_traversal`・`listing`・
//! `duplicate_key_violation`・`unknown_endpoint_violation`・
//! `constraint_violation`・`violation_reception`) が持ちます。
//!
//! v4 (`docs/schema_v4.md` §3.2) の動的検索は `Graph` に生えた種別APIの
//! メソッドです。これとは別に、graph! の左辺名は呼び出しsite固有の
//! `g.alice()` のような静的アクセサになります:
//! - ノード: `Graph` の種別メソッド (`g.person_by_id(&id)`/`person_iter`/
//!   `person_ids`/`person_len` 等)。
//! - エッジ: `Graph` の種別メソッド (`g.boss_by_id`/`boss_iter`/`boss_ids`/
//!   `boss_len`) と `NodeRef` の役割探索メソッド (`bob.boss_as_subordinate()`/
//!   `alice.boss_between(bob)`)。役割探索/`between` の戻り型は宣言した
//!   `where` 制約が決めます (`each 1` → 直接参照、`each 0..1` → `Option`、
//!   制約なし → イテレータ、`unique pair` → `between` が `Option`)。
//! - 名前付き静的アクセス: `g.alice()` / `g.bob_boss()`。内部位置から直接
//!   NodeRef/EdgeRefを作り、公開IDのhash lookupを行いません。
//!
//! スタイル: イテレータ連鎖 (`map`/`filter`/`collect`) やクロージャによる
//! データ加工は使わず、素の `for`/`if let`/`match` だけで書いています
//! (`Org::Graph::create(|b| { .. })` の `|b| { .. }` は API が要求する引数であって
//! データ加工のクロージャではないので例外です)。

mod builder_construction;
mod constraint_violation;
mod duplicate_key_violation;
mod edge_traversal;
mod listing;
mod literal_construction;
mod node_reading;
mod unknown_endpoint_violation;
mod violation_reception;

use crate::Org;

pub fn section3() {
    println!("=== §3 クックブック: graph_schema!/graph! が生成する公開APIの全列挙 ===\n");

    // --- 構築 (3通りの書き方) ---
    println!("--- 構築 ---");
    let g: Org::Graph = literal_construction::インライン式でgraphリテラルを組み立てる();
    literal_construction::外部変数を渡してgraphリテラルを組み立てる();
    literal_construction::外部で作ったエッジ属性をgraphリテラルに渡す();
    builder_construction::builderの型名メソッドで組み立てる();
    builder_construction::builderの総称insertとaddで組み立てる();

    // --- ノードを読む ---
    println!("\n--- ノードを読む ---");
    node_reading::人ノードを1件読む(&g);
    node_reading::チームノードを1件読む(&g);
    node_reading::personidの作り方とgraphのキーの対応を確認する(&g);

    // --- エッジを辿る ({kind}_as_<role> / {kind}_by_id / {kind}_between) ---
    println!("\n--- エッジを辿る (種別名_as_役割名 / 種別名_by_id / 種別名_between) ---");
    edge_traversal::each_1の役割探索は直接参照を返す(&g);
    edge_traversal::each_0か1の役割探索はoptionを返す(&g);
    edge_traversal::unique_pairのbetweenはoptionを返す(&g);
    edge_traversal::制約なしの役割探索はvecを返す(&g);
    edge_traversal::無向辺のendpointsアクセサで両端を読む(&g);
    edge_traversal::無向辺の接続探索と端点対検索は対称に辿れる(&g);

    // --- 一覧する (iter/ids/len) ---
    println!("\n--- 一覧する (iter/ids/len) ---");
    listing::person_idsで全ノードキーを列挙する(&g);
    listing::team_idsで全ノードキーを列挙する(&g);
    listing::belongs_toのiterで制約ありエッジを列挙する(&g);
    listing::bossのiterで積み荷ありエッジを列挙する(&g);
    listing::lenで表の辺の本数を確認する(&g);

    // --- 検証エラーを受ける ---
    println!("\n--- 検証エラーを受ける ---");
    duplicate_key_violation::重複ノードキーの違反を受け取る();
    duplicate_key_violation::辺キー重複の違反を受け取る();
    unknown_endpoint_violation::未知の始点キーの違反を受け取る();
    unknown_endpoint_violation::未知の終点キーの違反を受け取る();
    constraint_violation::each違反を受け取る();
    constraint_violation::unique_pair違反を受け取る();
    violation_reception::createは最初の1件で違反を止める();
    violation_reception::create_collectingで全違反を集める();
}
