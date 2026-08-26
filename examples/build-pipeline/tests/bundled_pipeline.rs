//! 統合テスト: 同梱 `pipeline.txt` がボリューム要件を満たし、ドメイン違反
//! ゼロで波計画とクリティカルパスを計算できること。

use std::collections::BTreeSet;

use build_pipeline::{analysis, builder, parser};

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
