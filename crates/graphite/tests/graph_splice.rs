//! `graph!` のスプライス項 (`..式`) を検証する統合テスト
//! (`docs/graph_splice.md` §1)。
//!
//! `OrgChart` (`orgchart_macro.rs`) は `each employee: 1` のようなroleごとの
//! 制約を多く持つため、スプライスの挙動 (ノードのみ/辺のみ/混在/空/挿入順) を
//! 単体で確かめるにはノイズが多い。ここでは制約なしの小さな専用スキーマ
//! `SpliceDemo` を使う。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 再設計待ち)。この
//! ファイルは検証対象1つ (`graph!` のスプライス項) に対するテスト用スキーマ
//! とテスト関数の列を持つ。`each_declaration_order.rs` が `#[path]` で宣言
//! を親に残したままテストを部分モジュールへ出す技法を実証したため、このファ
//! イルの分割が同じ宣言を各ファイルへ複製するという統合の根拠は成り立たない。
//! 検証観点ごとに部分モジュールへ分ける判定を issue #28 のやること4 で行う。
//! 超過を許す根拠の台帳は `docs/development/line_count_ledger.md` にある。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod SpliceDemo {
    include!("generated/graph_splice_splice_demo.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/graph_splice_splice_demo.rs";
    schema SpliceDemo {
        node Person;

        // 制約なし (`where` 節を省略): 平行辺・自己ループを許す多重グラフ。
        // スプライスの挿入順保証をそのまま観測できる。
        edge Knows = (knower: Person) -> (known: Person);
    }
}

use SpliceDemo::{Knows, KnowsId, PersonId};

#[test]
#[rustfmt::skip]
fn スプライスでノードのみを追加できる() {
    let staff: Vec<(String, Person)> = vec![
        ("田中".to_string(), Person { name: "田中".to_string() }),
        ("佐藤".to_string(), Person { name: "佐藤".to_string() }),
    ];

    let g = graphite::graph!(SpliceDemo {
        鈴木 = Person { name: "鈴木".into() },
        ..staff,
    })
    .expect("ノードのみのスプライスは構築に成功するはず");

    assert_eq!(
        g.person_by_id(&PersonId("鈴木".to_string())).unwrap().name,
        "鈴木"
    );
    assert_eq!(
        g.person_by_id(&PersonId("田中".to_string())).unwrap().name,
        "田中"
    );
    assert_eq!(
        g.person_by_id(&PersonId("佐藤".to_string())).unwrap().name,
        "佐藤"
    );
    assert_eq!(g.person_ids().count(), 3);
}

#[test]
#[rustfmt::skip]
fn スプライスで辺のみを追加できる() {
    let deps: Vec<(String, Knows)> = vec![
        ("k1".to_string(), Knows::new(PersonId("alice".to_string()), PersonId("bob".to_string()))),
        ("k2".to_string(), Knows::new(PersonId("bob".to_string()), PersonId("carol".to_string()))),
    ];

    let g = graphite::graph!(SpliceDemo {
        alice = Person { name: "Alice".into() },
        bob   = Person { name: "Bob".into() },
        carol = Person { name: "Carol".into() },
        ..deps,
    })
    .expect("辺のみのスプライスは構築に成功するはず");

    assert_eq!(g.knows_len(), 2);
    let k1 = g.knows_by_id(&KnowsId("k1".to_string())).expect("k1が存在するはず");
    assert_eq!(k1.knower().id(), &PersonId("alice".to_string()));
    assert_eq!(k1.known().id(), &PersonId("bob".to_string()));
}

#[test]
#[rustfmt::skip]
fn 静的項とスプライスを混在できる() {
    let staff: Vec<(String, Person)> = vec![("dave".to_string(), Person { name: "Dave".into() })];
    let extra_edges: Vec<(String, Knows)> = vec![(
        "k_extra".to_string(),
        Knows::new(PersonId("dave".to_string()), PersonId("alice".to_string())),
    )];

    let g = graphite::graph!(SpliceDemo {
        alice = Person { name: "Alice".into() },
        ..staff,
        k1 = Knows(alice -> alice),
        ..extra_edges,
    })
    .expect("静的項とスプライスの混在は構築に成功するはず");

    assert_eq!(g.person_ids().count(), 2);
    assert_eq!(g.knows_len(), 2);
    assert!(g.knows_by_id(&KnowsId("k_extra".to_string())).is_some());
}

#[test]
#[rustfmt::skip]
fn 空コレクションのスプライスは何も追加しない() {
    let empty_nodes: Vec<(String, Person)> = Vec::new();
    let empty_edges: Vec<(String, Knows)> = Vec::new();

    let g = graphite::graph!(SpliceDemo {
        alice = Person { name: "Alice".into() },
        ..empty_nodes,
        ..empty_edges,
    })
    .expect("空コレクションのスプライスも成功するはず");

    assert_eq!(g.person_ids().count(), 1);
    assert_eq!(g.knows_len(), 0);
}

#[test]
#[rustfmt::skip]
fn 静的項とスプライスが混在する場合_挿入順は記述順になる() {
    // `docs/graph_splice.md` §1: 実行順は「静的ノードのlet列 → 静的エッジと
    // スプライスを記述順」。ここでは 静的辺 → スプライス → 静的辺 の順で
    // 書き、`graph.knows_ids()` (挿入順を保持する `KeyedTable` 経由) がその記述順
    // どおりに列挙することを確認する。
    let middle: Vec<(String, Knows)> = vec![
        (
            "k_mid1".to_string(),
            Knows::new(PersonId("p_alice".to_string()), PersonId("p_bob".to_string())),
        ),
        (
            "k_mid2".to_string(),
            Knows::new(PersonId("p_bob".to_string()), PersonId("p_carol".to_string())),
        ),
    ];

    let g = graphite::graph!(SpliceDemo {
        p_alice = Person { name: "Alice".into() },
        p_bob   = Person { name: "Bob".into() },
        p_carol = Person { name: "Carol".into() },

        k_first = Knows(p_alice -> p_alice),
        ..middle,
        k_last = Knows(p_carol -> p_carol),
    })
    .expect("構築に成功するはず");

    let ids: Vec<String> = g.knows_ids().map(|id| id.0.clone()).collect();
    assert_eq!(
        ids,
        vec![
            "k_first".to_string(),
            "k_mid1".to_string(),
            "k_mid2".to_string(),
            "k_last".to_string(),
        ]
    );
}
