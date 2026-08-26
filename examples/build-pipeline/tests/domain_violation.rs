//! 統合テスト: 仕込んだドメイン違反 (循環依存・孤児成果物・produce競合) が
//! それぞれ独立に検出されること。

use build_pipeline::analysis::DomainIssue;
use build_pipeline::schema::BuildPipeline;
use build_pipeline::{analysis, builder, parser};

fn build(input: &str) -> BuildPipeline::Graph {
    let parsed = parser::parse(input).expect("パースに成功するはず");
    builder::build_graph(&parsed).expect("構築に成功するはず")
}

#[test]
fn 循環依存を仕込むと検出されplanとcritical_pathもエラーになる() {
    let g = build(
        "\
task a: cmd a (10s)
a consumes target/from_b
a produces target/from_a
task b: cmd b (10s)
b consumes target/from_a
b produces target/from_b
",
    );

    let issues = analysis::validate(&g);
    assert!(
        issues
            .iter()
            .any(|i| matches!(i, DomainIssue::CyclicDependency { .. })),
        "循環依存が検出されるはず: {issues:?}"
    );
    assert!(
        analysis::plan(&g).is_err(),
        "循環があるのでplanは失敗するはず"
    );
    assert!(
        analysis::critical_path(&g).is_err(),
        "循環があるのでcritical_pathは失敗するはず"
    );
}

#[test]
fn 孤児成果物を仕込むと検出される() {
    let g = build(
        "\
task t: cargo test (5s)
t consumes target/存在しない成果物
",
    );
    let issues = analysis::validate(&g);
    assert!(
        issues.iter().any(|i| matches!(
            i,
            DomainIssue::OrphanArtifact { artifact, .. } if artifact.0 == "target/存在しない成果物"
        )),
        "孤児成果物が検出されるはず: {issues:?}"
    );
}

#[test]
fn 二重produceを仕込むと検出される() {
    let g = build(
        "\
task build_a: cargo build a (10s)
build_a produces target/out.bin
task build_b: cargo build b (10s)
build_b produces target/out.bin
",
    );
    let issues = analysis::validate(&g);
    assert!(
        issues.iter().any(|i| matches!(
            i,
            DomainIssue::ConflictingProducers { artifact, .. } if artifact.0 == "target/out.bin"
        )),
        "produce競合が検出されるはず: {issues:?}"
    );
}
