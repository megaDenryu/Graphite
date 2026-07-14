//! 実装バージョン非依存の期待値テスト (`shared_tests/tests/` の思想を踏襲)。
//!
//! - 固定シード + `--inject-anomalies` での `anomalies` 検出結果が、
//!   `dataset::generate` が返す `AnomalyPlan` (既知の注入異常) と一致すること
//! - `chain` の循環検出
//! - `reorg` 後の再検証 (成功パス・violationパスの両方)
//! - `summary` の統計値の健全性

use org_analyzer::{analysis, dataset, reorg};
use org_analyzer::schema::{DepartmentId, EmployeeId};

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
        report.sponsorless_projects.contains(&plan.sponsorless_project),
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

#[test]
fn 同じシードなら生成結果は決定的である() {
    let a = dataset::generate(123, false);
    let b = dataset::generate(123, false);

    let names_a: Vec<String> = a
        .chart
        .employee_ids()
        .map(|id| a.chart.employee(id).unwrap().name.clone())
        .collect();
    let names_b: Vec<String> = b
        .chart
        .employee_ids()
        .map(|id| b.chart.employee(id).unwrap().name.clone())
        .collect();

    let mut sorted_a = names_a.clone();
    let mut sorted_b = names_b.clone();
    sorted_a.sort();
    sorted_b.sort();
    assert_eq!(sorted_a, sorted_b);
}

#[test]
fn 異なるシードなら生成結果が変わる() {
    let a = dataset::generate(1, false);
    let b = dataset::generate(2, false);

    let dept_counts_a: Vec<usize> = a
        .chart
        .department_ids()
        .map(|d| a.chart.belongs_to_pairs().filter(|(_, dep)| *dep == d).count())
        .collect();
    let dept_counts_b: Vec<usize> = b
        .chart
        .department_ids()
        .map(|d| b.chart.belongs_to_pairs().filter(|(_, dep)| *dep == d).count())
        .collect();

    assert_ne!(dept_counts_a, dept_counts_b, "seedが違えば部署別人数分布は変わるはず");
}

#[test]
fn chainは循環を検出して打ち切る() {
    let generated = dataset::generate(TEST_SEED, true);
    let plan = generated.anomaly_plan.as_ref().unwrap();
    let start = plan.cycle[0].clone();

    let result = analysis::management_chain(&generated.chart, &start)
        .expect("存在する社員キーなのでSomeのはず");

    assert!(result.cycle_back_to.is_some(), "循環に突入するチェーンは打ち切られるはず");
    // 打ち切られるまでに訪れたエントリ数は循環の長さ以下であるはず
    // (無限ループせず必ず停止することの確認)。
    assert!(result.entries.len() <= plan.cycle.len());
}

#[test]
fn chainはトップ層まで辿ると停止する() {
    let generated = dataset::generate(TEST_SEED, false);
    // grade5 (部長) の誰か1人はトップ層 (boss無し) のはず。トップ層から
    // 辿ると即座にentries=1件・循環無しで停止する。
    let top_id = generated
        .chart
        .employee_ids()
        .find(|id| {
            let emp = generated.chart.employee(id).unwrap();
            emp.grade == 5 && generated.chart.boss(id).is_none()
        })
        .cloned();

    if let Some(id) = top_id {
        let result = analysis::management_chain(&generated.chart, &id).unwrap();
        assert_eq!(result.entries.len(), 1);
        assert!(result.cycle_back_to.is_none());
    }
}

#[test]
fn chainは未知の社員キーでnoneを返す() {
    let generated = dataset::generate(TEST_SEED, false);
    let unknown = EmployeeId("E999".to_string());
    assert!(analysis::management_chain(&generated.chart, &unknown).is_none());
}

#[test]
fn reorgは廃止部署の全社員を他部署へ再配置する() {
    let generated = dataset::generate(TEST_SEED, false);
    let target = DepartmentId("D01".to_string());

    let before_count = generated
        .chart
        .belongs_to_pairs()
        .filter(|(_, d)| **d == target)
        .count();
    assert!(before_count > 0, "テスト対象部署には元々社員がいるはず");

    let report = reorg::simulate_reorg(&generated.chart, &target)
        .expect("D01は実在する部署キーのはず");

    assert_eq!(report.reassigned.len(), before_count);
    // 再配置先はすべて対象部署以外
    assert!(report.reassigned.iter().all(|(_, d)| *d != target));

    match &report.outcome {
        reorg::ReorgOutcome::Success(new_org) => {
            // 廃止部署はもう存在しない
            assert!(new_org.department(&target).is_none());
            // 再配置された社員は新部署に所属している
            for (emp_id, new_dept) in &report.reassigned {
                let actual = new_org.try_belongs_to(emp_id);
                assert_eq!(actual.map(|d| d.name.clone()), new_org.department(new_dept).map(|d| d.name.clone()));
            }
            // 社員総数・プロジェクト総数は変化しない
            assert_eq!(new_org.employee_ids().count(), generated.chart.employee_ids().count());
            assert_eq!(new_org.project_ids().count(), generated.chart.project_ids().count());
            assert_eq!(new_org.department_ids().count(), generated.chart.department_ids().count() - 1);
        }
        reorg::ReorgOutcome::Violated(_) => {
            // D01がスポンサー関係を持っていた場合はこちらのパスもありうる
            // (モジュールdocの「カスケード削除忘れ」ショーケース)。どちらの
            // 分岐でもテストとして許容するが、再配置計画自体は既に検証済み。
        }
    }
}

#[test]
fn reorgは存在しない部署キーでnoneを返す() {
    let generated = dataset::generate(TEST_SEED, false);
    let unknown = DepartmentId("D99".to_string());
    assert!(reorg::simulate_reorg(&generated.chart, &unknown).is_none());
}

#[test]
fn reorgでスポンサー元部署を廃止するとviolationになる() {
    let generated = dataset::generate(TEST_SEED, false);
    // sponsors_pairsを持つ部署を1つ探す (スポンサー辺を発している側)。
    let sponsor_dept = generated
        .chart
        .sponsors_pairs()
        .map(|(d, _p)| d.clone())
        .next();

    let Some(target) = sponsor_dept else {
        // このシードでスポンサー関係が1件も無ければテストの前提が崩れるので
        // スキップ相当として早期returnする (シード次第で起こりうる)。
        return;
    };

    let report = reorg::simulate_reorg(&generated.chart, &target).unwrap();
    match report.outcome {
        reorg::ReorgOutcome::Violated(violation) => {
            // UnknownDepartment系のViolationになっているはず
            let msg = violation.to_string();
            assert!(
                msg.contains("Department") || msg.contains("部署"),
                "違反メッセージが部署関連であるはず: {msg}"
            );
        }
        reorg::ReorgOutcome::Success(_) => {
            panic!("スポンサー元部署を廃止するとsponsors辺が宙に浮きviolationになるはず");
        }
    }
}

#[test]
fn summaryの統計値は健全な範囲に収まる() {
    let generated = dataset::generate(TEST_SEED, false);
    let summary = analysis::summarize(&generated.chart);

    assert_eq!(summary.total_employees, dataset::EMPLOYEE_COUNT);
    assert_eq!(summary.dept_counts.len(), dataset::DEPARTMENT_COUNT);
    assert_eq!(summary.project_assignments.len(), dataset::PROJECT_COUNT);

    // 部署別人数の合計は社員総数と一致する (belongs_to多重度1の帰結)。
    let dept_total: usize = summary.dept_counts.iter().map(|d| d.count).sum();
    assert_eq!(dept_total, summary.total_employees);

    // grade分布の合計も社員総数と一致する。
    let grade_total: usize = summary.grade_counts.iter().map(|g| g.count).sum();
    assert_eq!(grade_total, summary.total_employees);

    // 平均span of controlは0以上。
    assert!(summary.span_of_control.average >= 0.0);
}
