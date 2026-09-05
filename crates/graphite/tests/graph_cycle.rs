//! 汎用Graphの循環検出と、循環時に返る閉路の内容を検査する。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 再設計待ち)。この
//! ファイルは検証対象1つ (循環検出と閉路の内容) に対するテスト関数の列であ
//! る。このファイルはスキーマを `mod common` へ外へ出しているため、この
//! ファイルの分割は宣言を複製しない。テスト関数は互いに独立している。統合による
//! 超過の根拠が成り立たないため、テストの分け方の判定は issue #28 のやるこ
//! と4 が行う。超過を許す根拠の台帳は
//! `docs/development/line_count_ledger.md` にある。

mod common;

use common::{sample_people, Person};
use graphite::Graph;

#[test]
fn has_cycle_循環なし() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
        ],
    )
    .unwrap();
    assert!(!g.has_cycle());
}

#[test]
fn has_cycle_循環あり() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
            ("鈴木".to_string(), "田中".to_string(), ()),
        ],
    )
    .unwrap();
    assert!(g.has_cycle());
}

#[test]
fn topological_sort_循環ありならエラー() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
            ("鈴木".to_string(), "田中".to_string(), ()),
        ],
    )
    .unwrap();

    assert!(g.topological_sort().is_err());
}

#[test]
fn topological_levels_循環ありならエラー() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
            ("鈴木".to_string(), "田中".to_string(), ()),
        ],
    )
    .unwrap();

    assert!(g.topological_levels().is_err());
}

#[test]
fn critical_path_by_循環ありならエラー() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
            ("鈴木".to_string(), "田中".to_string(), ()),
        ],
    )
    .unwrap();

    assert!(g.critical_path_by(|_key, person| person.age).is_err());
}

#[test]
fn cycle_error_循環を構成するノード列全体を返す() {
    let g: Graph<Person> = Graph::build(
        sample_people(),
        vec![
            ("田中".to_string(), "佐藤".to_string(), ()),
            ("佐藤".to_string(), "鈴木".to_string(), ()),
            ("鈴木".to_string(), "田中".to_string(), ()),
        ],
    )
    .unwrap();

    let err = g.topological_sort().unwrap_err();
    assert_eq!(err.cycle.len(), 3);

    let mut sorted = err.cycle.clone();
    sorted.sort();
    assert_eq!(
        sorted,
        vec!["佐藤".to_string(), "田中".to_string(), "鈴木".to_string()]
    );

    // cycle[0] から辿って cycle[0] に戻る閉路になっていることを検証する。
    for i in 0..err.cycle.len() {
        let from = &err.cycle[i];
        let to = &err.cycle[(i + 1) % err.cycle.len()];
        assert!(
            g.edge_weight(from, to).is_some(),
            "{from:?} -> {to:?} の辺が無い"
        );
    }
}

#[test]
fn cycle_error_自己ループも循環として検出する() {
    let g: Graph<(), (), &str> = Graph::from_edges(vec!["a"], vec![("a", "a")]).unwrap();
    let err = g.topological_sort().unwrap_err();
    assert_eq!(err.cycle, vec!["a"]);
}
