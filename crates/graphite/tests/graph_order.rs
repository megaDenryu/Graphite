//! 汎用Graphのトポロジカル順序と、依存レベルへの分割の順序保証を検査する。

mod common;

use common::{sample_people, Person};
use graphite::Graph;

#[test]
fn topological_sort_循環なしなら順序を返す() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
        ],
    )
    .unwrap();

    let order = g.topological_sort().expect("循環がないので成功するはず");
    let pos = |k: &str| order.iter().position(|&x| x == k).unwrap();
    assert!(pos("田中") < pos("佐藤"));
    assert!(pos("佐藤") < pos("鈴木"));
}

#[test]
fn topological_levels_依存のないノードから順にレベル分割する() {
    let g: Graph<()> = Graph::build(
        vec![
            ("fetch".to_string(), ()),
            ("build_a".to_string(), ()),
            ("build_b".to_string(), ()),
            ("link".to_string(), ()),
        ],
        vec![
            ("fetch".to_string(), "build_a".to_string(), ()),
            ("fetch".to_string(), "build_b".to_string(), ()),
            ("build_a".to_string(), "link".to_string(), ()),
            ("build_b".to_string(), "link".to_string(), ()),
        ],
    )
    .unwrap();

    let levels = g.topological_levels().expect("循環がないので成功するはず");
    assert_eq!(levels.len(), 3);
    assert_eq!(levels[0], vec![&"fetch".to_string()]);
    // レベル内の順序は挿入順 (build_a が build_b より先に宣言されている)。
    assert_eq!(
        levels[1],
        vec![&"build_a".to_string(), &"build_b".to_string()]
    );
    assert_eq!(levels[2], vec![&"link".to_string()]);
}
