//! `graph!` のスプライス項 (`..式`) を検証する統合テスト
//! (`docs/graph_splice.md` §1)。
//!
//! `OrgChart` (`orgchart_macro.rs`) は `each Employee: 1` のようなノードごとの
//! 制約を多く持つため、スプライスの挙動 (ノードのみ/辺のみ/混在/空/挿入順) を
//! 単体で確かめるにはノイズが多い。ここでは制約なしの小さな専用スキーマ
//! `SpliceDemo` を使う。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema SpliceDemo {
        node Person;

        // 制約なし (`where` 節を省略): 平行辺・自己ループを許す多重グラフ。
        // スプライスの挿入順保証をそのまま観測できる。
        edge Knows = Person -> Person;
    }
}

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
        Person::get(&g, &PersonId("鈴木".to_string())).unwrap().name,
        "鈴木"
    );
    assert_eq!(
        Person::get(&g, &PersonId("田中".to_string())).unwrap().name,
        "田中"
    );
    assert_eq!(
        Person::get(&g, &PersonId("佐藤".to_string())).unwrap().name,
        "佐藤"
    );
    assert_eq!(Person::ids(&g).count(), 3);
}

#[test]
#[rustfmt::skip]
fn スプライスで辺のみを追加できる() {
    let deps: Vec<(String, Knows)> = vec![
        ("k1".to_string(), Knows(PersonId("alice".to_string()), PersonId("bob".to_string()))),
        ("k2".to_string(), Knows(PersonId("bob".to_string()), PersonId("carol".to_string()))),
    ];

    let g = graphite::graph!(SpliceDemo {
        alice = Person { name: "Alice".into() },
        bob   = Person { name: "Bob".into() },
        carol = Person { name: "Carol".into() },
        ..deps,
    })
    .expect("辺のみのスプライスは構築に成功するはず");

    assert_eq!(Knows::len(&g), 2);
    let k1 = Knows::get(&g, &KnowsId("k1".to_string())).expect("k1が存在するはず");
    assert_eq!(k1.from(), &PersonId("alice".to_string()));
    assert_eq!(k1.to(), &PersonId("bob".to_string()));
}

#[test]
#[rustfmt::skip]
fn 静的項とスプライスを混在できる() {
    let staff: Vec<(String, Person)> = vec![("dave".to_string(), Person { name: "Dave".into() })];
    let extra_edges: Vec<(String, Knows)> = vec![(
        "k_extra".to_string(),
        Knows(PersonId("dave".to_string()), PersonId("alice".to_string())),
    )];

    let g = graphite::graph!(SpliceDemo {
        alice = Person { name: "Alice".into() },
        ..staff,
        k1 = Knows(alice -> alice),
        ..extra_edges,
    })
    .expect("静的項とスプライスの混在は構築に成功するはず");

    assert_eq!(Person::ids(&g).count(), 2);
    assert_eq!(Knows::len(&g), 2);
    assert!(Knows::get(&g, &KnowsId("k_extra".to_string())).is_some());
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

    assert_eq!(Person::ids(&g).count(), 1);
    assert_eq!(Knows::len(&g), 0);
}

#[test]
#[rustfmt::skip]
fn 静的項とスプライスが混在する場合_挿入順は記述順になる() {
    // `docs/graph_splice.md` §1: 実行順は「静的ノードのlet列 → 静的エッジと
    // スプライスを記述順」。ここでは 静的辺 → スプライス → 静的辺 の順で
    // 書き、`Knows::ids` (挿入順を保持する `KeyedTable` 経由) がその記述順
    // どおりに列挙することを確認する。
    let middle: Vec<(String, Knows)> = vec![
        (
            "k_mid1".to_string(),
            Knows(PersonId("p_alice".to_string()), PersonId("p_bob".to_string())),
        ),
        (
            "k_mid2".to_string(),
            Knows(PersonId("p_bob".to_string()), PersonId("p_carol".to_string())),
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

    let ids: Vec<String> = Knows::ids(&g).map(|id| id.0.clone()).collect();
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
