//! 統合テスト: 手計算できる小さな既知データでの波数・クリティカルパス長。

use build_pipeline::schema::BuildPipeline;
use build_pipeline::{analysis, builder, parser};

fn build(input: &str) -> BuildPipeline::Graph {
    let parsed = parser::parse(input).expect("パースに成功するはず");
    builder::build_graph(&parsed).expect("構築に成功するはず")
}

// 手計算できる小さな既知データ (fetch -> {build_a, build_b} -> link) での
// 波数・クリティカルパス長の一致を確認する (analysis.rs 内のユニットテストと
// 同じ題材を、モジュール外からの統合テストとしても固定しておく)。
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
