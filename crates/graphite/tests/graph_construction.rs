//! 汎用Graphの構築経路と、構築時に検出する失敗を検査する。

mod common;

use common::{sample_people, Person};
use graphite::{Graph, GraphError};

#[test]
fn build_正常系() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
        ],
    )
    .expect("構築に成功するはず");

    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);
    assert_eq!(g.node(&"田中".to_string()).unwrap().age, 30);
    assert!(g.node(&"存在しない".to_string()).is_none());
}

#[test]
fn build_重複キーはエラー() {
    let err = Graph::<Person>::build(
        vec![
            (
                "田中".to_string(),
                Person {
                    name: "田中".to_string(),
                    age: 30,
                },
            ),
            (
                "田中".to_string(),
                Person {
                    name: "田中2".to_string(),
                    age: 31,
                },
            ),
        ],
        vec![],
    )
    .unwrap_err();

    assert_eq!(err, GraphError::DuplicateKey("田中".to_string()));
}

#[test]
fn build_未知キーへの辺はエラー() {
    let err = Graph::<Person>::build(
        sample_people(),
        vec![("田中".to_string(), "存在しない".to_string(), ())],
    )
    .unwrap_err();

    assert_eq!(
        err,
        GraphError::UnknownEndpoint {
            from: "田中".to_string(),
            to: "存在しない".to_string(),
            missing: "存在しない".to_string(),
        }
    );
}

#[test]
fn create_builderパターンで構築できる() {
    let g: Graph<Person> = Graph::create(|b| {
        for (k, v) in sample_people() {
            b.node(k, v);
        }
        b.edge("田中".to_string(), "佐藤".to_string(), ());
    })
    .expect("構築に成功するはず");

    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 1);
    assert_eq!(
        g.out_neighbors(&"田中".to_string()),
        vec![&"佐藤".to_string()]
    );
}

#[test]
fn create_builder内のエラーも_resultで返る() {
    let result: Result<Graph<Person>, _> = Graph::create(|b| {
        b.node(
            "田中".to_string(),
            Person {
                name: "田中".to_string(),
                age: 30,
            },
        );
        b.edge("田中".to_string(), "存在しない".to_string(), ());
    });

    assert!(result.is_err());
}

#[test]
fn edge_weight_辺属性にアクセスできる() {
    #[derive(Debug, Clone, PartialEq)]
    struct Friendship {
        since: u32,
    }

    let g: Graph<Person, Friendship> = Graph::build(
        sample_people(),
        vec![(
            "田中".to_string(),
            "佐藤".to_string(),
            Friendship { since: 2015 },
        )],
    )
    .unwrap();

    assert_eq!(
        g.edge_weight(&"田中".to_string(), &"佐藤".to_string()),
        Some(&Friendship { since: 2015 })
    );
    assert_eq!(
        g.edge_weight(&"佐藤".to_string(), &"田中".to_string()),
        None
    );
}

#[test]
fn from_edges_pairsイテレータから射影してhas_cycleが動く() {
    let ids = ["a".to_string(), "b".to_string(), "c".to_string()];
    // `{label}_pairs()` のような `(&K, &K)` を yield するイテレータを模す。
    let pairs: Vec<(&String, &String)> = vec![(&ids[0], &ids[1]), (&ids[1], &ids[2])];

    let g: Graph<(), (), String> = Graph::from_edges(
        ids.iter().cloned(),
        pairs.into_iter().map(|(a, b)| (a.clone(), b.clone())),
    )
    .unwrap();

    assert_eq!(g.node_count(), 3);
    assert_eq!(g.edge_count(), 2);
    assert!(!g.has_cycle());

    // 循環にもなる。
    let cyclic: Graph<(), (), &str> =
        Graph::from_edges(vec!["a", "b"], vec![("a", "b"), ("b", "a")]).unwrap();
    assert!(cyclic.has_cycle());
}

#[test]
fn from_edges_未知キーへの辺はエラー() {
    let err = Graph::<(), (), &str>::from_edges(vec!["a", "b"], vec![("a", "c")]).unwrap_err();
    assert_eq!(
        err,
        GraphError::UnknownEndpoint {
            from: "a",
            to: "c",
            missing: "c",
        }
    );
}
