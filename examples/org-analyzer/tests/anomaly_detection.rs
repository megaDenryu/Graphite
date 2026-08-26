//! 固定シードで注入した異常が、`anomalies` の検出結果と一致すること。

use org_analyzer::schema::EmployeeId;
use org_analyzer::{analysis, dataset};

const TEST_SEED: u64 = 7;

#[test]
fn anomalies検出結果が既知の注入異常と一致する() {
    let generated = dataset::generate(TEST_SEED, true);
    let plan = generated
        .anomaly_plan
        .as_ref()
        .expect("inject_anomalies=trueならAnomalyPlanが返るはず");
    let report = analysis::detect_anomalies(&generated.chart);

    // 1. 相互上司ペア: 注入した (a, b) が正規化された形で含まれる。
    let (a, b) = plan.mutual_pair.clone();
    let normalized = if a < b { (a, b) } else { (b, a) };
    assert!(
        report.mutual_boss_pairs.contains(&normalized),
        "注入した相互上司ペア {normalized:?} が検出されるはず (実際: {:?})",
        report.mutual_boss_pairs
    );
    // 相互上司ペアは他に紛れ込みが無いこと (合成データは基本forest構造なので
    // 注入した1組だけのはず)。
    assert_eq!(report.mutual_boss_pairs.len(), 1);

    // 2. 上司循環 (3人): 注入したメンバー集合と一致する循環が1つ見つかる。
    assert_eq!(report.boss_cycles.len(), 1, "循環は注入した1件のみのはず");
    let detected_cycle = &report.boss_cycles[0];
    let expected_set: std::collections::HashSet<&EmployeeId> = plan.cycle.iter().collect();
    let detected_set: std::collections::HashSet<&EmployeeId> = detected_cycle.iter().collect();
    assert_eq!(
        expected_set, detected_set,
        "検出された循環メンバーが注入したメンバーと一致するはず"
    );

    // 3. スポンサー無しプロジェクト: 注入したプロジェクトが含まれる。
    assert!(
        report
            .sponsorless_projects
            .contains(&plan.sponsorless_project),
        "注入したスポンサー無しプロジェクトが検出されるはず"
    );

    // 4. 無人プロジェクト: 注入したプロジェクトが含まれる。
    assert!(
        report.unstaffed_projects.contains(&plan.unstaffed_project),
        "注入した無人プロジェクトが検出されるはず"
    );
}

#[test]
fn デフォルト生成では異常が注入されない() {
    let generated = dataset::generate(TEST_SEED, false);
    assert!(generated.anomaly_plan.is_none());

    let report = analysis::detect_anomalies(&generated.chart);
    // 通常運転 (grade厳密不等号による森構造) では相互上司も循環も
    // 原理的に発生しない。
    assert!(report.mutual_boss_pairs.is_empty());
    assert!(report.boss_cycles.is_empty());
}
