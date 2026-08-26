//! 手計算できる小さな題材で、最長経路・合計時間・循環時の失敗を固定する。

use crate::analysis::*;
use crate::schema::{BuildPipeline, TaskId};
use crate::analysis::plan;
use crate::builder::build_graph;
use crate::parser::parse;

fn graph_from(input: &str) -> BuildPipeline::Graph {
    let parsed = parse(input).unwrap();
    build_graph(&parsed).unwrap()
}

#[test]
fn critical_pathは最長経路と合計時間を返す() {
    let g = graph_from(
        "\
task fetch: cargo fetch (10s)
fetch produces target/idx
task build_a: cargo build a (20s)
build_a consumes target/idx
build_a produces target/a
task build_b: cargo build b (30s)
build_b consumes target/idx
build_b produces target/b
task link: cargo link (5s)
link consumes target/a
link consumes target/b
",
    );
    let cp = critical_path(&g).expect("循環がないので成功するはず");
    assert_eq!(cp.total_secs, 45); // fetch(10) + build_b(30) + link(5)
    assert_eq!(
        cp.path,
        vec![
            TaskId("fetch".to_string()),
            TaskId("build_b".to_string()),
            TaskId("link".to_string()),
        ]
    );
    assert_eq!(cp.total_work_secs, 65); // 10+20+30+5
}

#[test]
fn 循環があるとplanもcritical_pathもエラーになる() {
    let g = graph_from(
        "\
task a: cmd a (10s)
a consumes target/from_b
a produces target/from_a
task b: cmd b (10s)
b consumes target/from_a
b produces target/from_b
",
    );
    assert!(plan(&g).is_err());
    assert!(critical_path(&g).is_err());
}
