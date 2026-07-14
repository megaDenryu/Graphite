//! 統合テスト。
//!
//! - 同梱 `pipeline.txt` (20タスク・15アーティファクト以上) が検証エラー無く
//!   波計画・クリティカルパスを計算できること
//! - 仕込みエラー (循環依存・孤児成果物・produce競合) をそれぞれ独立に検出
//!   できること
//! - パーサの異常系 (コロンなし・秒数欠落・未知キーワード) が行番号付きで
//!   報告されること
//! - 手計算できる小さな既知データでの波数・クリティカルパス長の一致

use build_pipeline::{analysis, builder, parser};
use build_pipeline::analysis::DomainIssue;
use build_pipeline::schema::BuildPipeline;
use std::collections::BTreeSet;

fn build(input: &str) -> BuildPipeline {
    let parsed = parser::parse(input).expect("パースに成功するはず");
    builder::build_graph(&parsed).expect("構築に成功するはず")
}

#[test]
fn 同梱pipeline_txtはボリューム要件を満たす() {
    let input = std::fs::read_to_string("pipeline.txt").expect("pipeline.txtを読み込めること");
    let parsed = parser::parse(&input).expect("同梱pipeline.txtはパースできるはず");

    assert!(
        parsed.tasks.len() >= 20,
        "タスクは20件以上を要求 (実際: {})",
        parsed.tasks.len()
    );

    let artifact_paths: BTreeSet<&str> = parsed.edges.iter().map(|e| e.path.as_str()).collect();
    assert!(
        artifact_paths.len() >= 15,
        "アーティファクトは15件以上を要求 (実際: {})",
        artifact_paths.len()
    );
}

#[test]
fn 同梱pipeline_txtは図式適合しドメイン違反ゼロで波とクリティカルパスを計算できる() {
    let input = std::fs::read_to_string("pipeline.txt").expect("pipeline.txtを読み込めること");
    let parsed = parser::parse(&input).expect("パースに成功するはず");
    let g = builder::build_graph(&parsed).expect("同梱pipeline.txtは図式適合するはず");

    let issues = analysis::validate(&g);
    assert!(
        issues.is_empty(),
        "同梱pipeline.txtにドメイン違反があってはいけない: {issues:?}"
    );

    let waves = analysis::plan(&g).expect("循環がないので成功するはず");
    assert!(!waves.is_empty());
    // fetch -> codegen -> build -> test/lint -> doc -> package -> deploy の
    // 8段構成に、並列可能な枝 (codegen 2本・build 複数枝・test 複数枝) を
    // 考慮すると、波の数は段数以上になる。
    assert!(
        waves.len() >= 8,
        "多段構成なので8波以上を期待 (実際: {})",
        waves.len()
    );
    // 全タスクが波のどこかに現れる (=循環なく全件スケジュールできている)。
    let scheduled: usize = waves.iter().map(|w| w.tasks.len()).sum();
    assert_eq!(scheduled, parsed.tasks.len());

    let cp = analysis::critical_path(&g).expect("循環がないので成功するはず");
    assert!(!cp.path.is_empty());
    assert!(cp.total_secs > 0);
    assert!(cp.total_work_secs >= cp.total_secs);
    // fetch_deps は全経路の起点なので、クリティカルパスの先頭は fetch_deps。
    assert_eq!(cp.path.first().unwrap().0, "fetch_deps");
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
        issues.iter().any(|i| matches!(i, DomainIssue::CyclicDependency { .. })),
        "循環依存が検出されるはず: {issues:?}"
    );
    assert!(analysis::plan(&g).is_err(), "循環があるのでplanは失敗するはず");
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

#[test]
fn パーサ異常系_コロンがない() {
    let e = parser::parse("task foo bar (1s)\n").unwrap_err();
    assert_eq!(e.line, 1);
}

#[test]
fn パーサ異常系_秒数の括弧がない() {
    let e = parser::parse("task foo: cargo build\n").unwrap_err();
    assert_eq!(e.line, 1);
}

#[test]
fn パーサ異常系_秒数の単位sがない() {
    let e = parser::parse("task foo: cargo build (10)\n").unwrap_err();
    assert_eq!(e.line, 1);
}

#[test]
fn パーサ異常系_未知キーワード() {
    let e = parser::parse("task foo: cargo build (1s)\nfoo touches target/x\n").unwrap_err();
    assert_eq!(e.line, 2);
}

#[test]
fn パーサ異常系_行番号は複数行にまたがっても正しい() {
    let input = "\
# comment
task ok: cargo build (1s)
ok produces target/a

task broken cargo test (2s)
";
    let e = parser::parse(input).unwrap_err();
    assert_eq!(e.line, 5);
}

/// 手計算できる小さな既知データ (fetch -> {build_a, build_b} -> link) での
/// 波数・クリティカルパス長の一致を確認する (analysis.rs 内のユニットテストと
/// 同じ題材を、モジュール外からの統合テストとしても固定しておく)。
#[test]
fn 既知データでの波数とクリティカルパス長が一致する() {
    let g = build(
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

    let waves = analysis::plan(&g).unwrap();
    assert_eq!(waves.len(), 3, "fetch / {{build_a,build_b}} / link の3波");
    assert_eq!(waves[1].duration_secs, 30, "波2の所要時間はmax(20,30)=30");

    let cp = analysis::critical_path(&g).unwrap();
    assert_eq!(cp.total_secs, 45, "fetch(10)+build_b(30)+link(5)=45");
    assert_eq!(cp.total_work_secs, 65, "10+20+30+5=65");
}
