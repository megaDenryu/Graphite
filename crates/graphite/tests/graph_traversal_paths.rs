//! 汎用Graphの走査 (全件・近傍・到達可能性) と経路探索を検査する。

mod common;

use common::{sample_people, Person};
use graphite::Graph;

#[test]
fn reachable_from_到達可能なノードを返す() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![("田中".to_string(), "佐藤".to_string(), ())],
    )
    .unwrap();

    let mut reachable: Vec<String> = g
        .reachable_from(&"田中".to_string())
        .into_iter()
        .cloned()
        .collect();
    reachable.sort();
    assert_eq!(reachable, vec!["佐藤".to_string(), "田中".to_string()]);

    // 辺の無い鈴木からは自分自身のみ到達可能
    assert_eq!(
        g.reachable_from(&"鈴木".to_string()),
        vec![&"鈴木".to_string()]
    );

    // 存在しないキーは空
    assert!(g.reachable_from(&"存在しない".to_string()).is_empty());
}

#[test]
fn path_経路を返す() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
        ],
    )
    .unwrap();

    let path = g
        .path(&"田中".to_string(), &"鈴木".to_string())
        .expect("経路があるはず");
    assert_eq!(
        path,
        vec![
            &"田中".to_string(),
            &"佐藤".to_string(),
            &"鈴木".to_string()
        ]
    );

    // 到達不能
    assert!(g.path(&"鈴木".to_string(), &"田中".to_string()).is_none());

    // 自分自身への経路
    assert_eq!(
        g.path(&"田中".to_string(), &"田中".to_string()),
        Some(vec![&"田中".to_string()])
    );
}

#[test]
fn keys_と_nodes_で全件走査できる() {
    let g: Graph<Person> = Graph::build(sample_people(), vec![]).unwrap();

    let mut keys: Vec<&String> = g.keys().collect();
    keys.sort();
    assert_eq!(
        keys,
        vec![
            &"佐藤".to_string(),
            &"田中".to_string(),
            &"鈴木".to_string()
        ]
    );

    assert_eq!(g.nodes().count(), 3);
}

#[test]
fn in_neighbors_out_neighborsと対称() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "鈴木".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
        ],
    )
    .unwrap();

    let mut in_neighbors: Vec<String> = g
        .in_neighbors(&"鈴木".to_string())
        .into_iter()
        .cloned()
        .collect();
    in_neighbors.sort();
    assert_eq!(in_neighbors, vec!["佐藤".to_string(), "田中".to_string()]);

    // 入る辺の無いノードは空。
    assert!(g.in_neighbors(&"田中".to_string()).is_empty());
    // 存在しないキーも空。
    assert!(g.in_neighbors(&"存在しない".to_string()).is_empty());
}
