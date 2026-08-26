//! 3種のドメイン違反それぞれを、最小の壊れたパイプラインで検出できることを固定する。

use crate::analysis::*;
use crate::schema::BuildPipeline;
use crate::builder::build_graph;
use crate::parser::parse;

fn graph_from(input: &str) -> BuildPipeline::Graph {
    let parsed = parse(input).unwrap();
    build_graph(&parsed).unwrap()
}

#[test]
fn 孤児成果物を検出できる() {
    let g = graph_from(
        "\
task build: cargo build (10s)
build produces target/a
task test: cargo test (5s)
test consumes target/b
",
    );
    let issues = validate(&g);
    assert!(issues.iter().any(|i| matches!(
        i,
        DomainIssue::OrphanArtifact { artifact, .. } if artifact.0 == "target/b"
    )));
}

#[test]
fn produce競合を検出できる() {
    let g = graph_from(
        "\
task build_a: cargo build a (10s)
build_a produces target/out
task build_b: cargo build b (10s)
build_b produces target/out
",
    );
    let issues = validate(&g);
    assert!(issues.iter().any(|i| matches!(
        i,
        DomainIssue::ConflictingProducers { artifact, .. } if artifact.0 == "target/out"
    )));
}

#[test]
fn 循環依存を検出できる() {
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
    let issues = validate(&g);
    assert!(issues
        .iter()
        .any(|i| matches!(i, DomainIssue::CyclicDependency { .. })));
}

#[test]
fn 正常なパイプラインは違反ゼロ() {
    let g = graph_from(
        "\
task build: cargo build (10s)
build produces target/a
task test: cargo test (5s)
test consumes target/a
",
    );
    assert!(validate(&g).is_empty());
}
