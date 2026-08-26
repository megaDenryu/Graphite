//! 汎用Graphの変換 (ノード値の写像と、述語によるノードの絞り込み) を検査する。

mod common;

use std::collections::HashSet;

use common::{sample_people, Person};
use graphite::Graph;

#[test]
fn map_nodes_構造を保ったまま値を変換する() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![("田中".to_string(), "佐藤".to_string(), ())],
    )
    .unwrap();

    let ages: Graph<u32> = g.map_nodes(|p| p.age);

    assert_eq!(ages.node_count(), 3);
    assert_eq!(ages.edge_count(), 1);
    assert_eq!(*ages.node(&"田中".to_string()).unwrap(), 30);
    assert_eq!(
        ages.out_neighbors(&"田中".to_string()),
        vec![&"佐藤".to_string()]
    );
}

#[test]
fn filter_nodes_述語を満たすノードと両端が生き残った辺だけ残す() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
        ],
    )
    .unwrap();

    // 30 歳以上: 田中(30), 鈴木(40) が残り、佐藤(25) は落ちる
    // → 田中-佐藤, 佐藤-鈴木 の辺は両方とも片方の端点を失うので消える
    let adults = g.filter_nodes(|p| p.age >= 30);

    assert_eq!(adults.node_count(), 2);
    assert_eq!(adults.edge_count(), 0);
    assert!(adults.node(&"田中".to_string()).is_some());
    assert!(adults.node(&"鈴木".to_string()).is_some());
    assert!(adults.node(&"佐藤".to_string()).is_none());
}

#[test]
fn filter_nodes_with_key_キーに依存するフィルタができる() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
        ],
    )
    .unwrap();

    // 特定のID集合に含まれるノードだけ抽出する (値ではなくキーで判定)。
    let allowed: HashSet<String> = ["田中".to_string(), "鈴木".to_string()]
        .into_iter()
        .collect();
    let filtered = g.filter_nodes_with_key(|key, _person| allowed.contains(key));

    assert_eq!(filtered.node_count(), 2);
    assert!(filtered.node(&"田中".to_string()).is_some());
    assert!(filtered.node(&"鈴木".to_string()).is_some());
    assert!(filtered.node(&"佐藤".to_string()).is_none());
    // 両端が生き残っていない辺は消える。
    assert_eq!(filtered.edge_count(), 0);
}

#[test]
fn map_nodes_with_key_キーも見て変換できる() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![("田中".to_string(), "佐藤".to_string(), ())],
    )
    .unwrap();

    let labeled: Graph<String> =
        g.map_nodes_with_key(|key, person| format!("{key}:{}", person.age));

    assert_eq!(labeled.node(&"田中".to_string()).unwrap(), "田中:30");
    assert_eq!(labeled.node(&"佐藤".to_string()).unwrap(), "佐藤:25");
}
