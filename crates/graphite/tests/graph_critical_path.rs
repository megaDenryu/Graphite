//! 汎用Graphのノード重み付き最長経路 (クリティカルパス) を検査する。

mod common;

use common::{sample_people, Person};
use graphite::Graph;

#[test]
fn critical_path_by_ノード重み付き最長経路を返す() {
    // 田中(30) -> 佐藤(25) -> 鈴木(40)。年齢をノード重みとして使う。
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
        ],
    )
    .unwrap();

    let (path, total) = g
        .critical_path_by(|_key, person| person.age)
        .expect("循環がないので成功するはず");

    assert_eq!(
        path,
        vec![
            &"田中".to_string(),
            &"佐藤".to_string(),
            &"鈴木".to_string()
        ]
    );
    assert_eq!(total, 30 + 25 + 40);
}

#[test]
fn critical_path_by_空グラフはvecと初期値を返す() {
    let g: Graph<Person> = Graph::build(vec![], vec![]).unwrap();
    let (path, total): (Vec<&String>, u32) = g
        .critical_path_by(|_key, person| person.age)
        .expect("空グラフは循環なしとして成功するはず");
    assert!(path.is_empty());
    assert_eq!(total, 0);
}
