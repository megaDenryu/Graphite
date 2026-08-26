//! `reorg` の再配置計画と、再構築の成功パス・violationパスの両方。

use org_analyzer::schema::DepartmentId;
use org_analyzer::{dataset, reorg};

const TEST_SEED: u64 = 7;

#[test]
fn reorgは廃止部署の全社員を他部署へ再配置する() {
    let generated = dataset::generate(TEST_SEED, false);
    let target = DepartmentId("D01".to_string());

    let before_count = generated
        .chart
        .belongs_to_iter()
        .filter(|edge| edge.department().id() == &target)
        .count();
    assert!(before_count > 0, "テスト対象部署には元々社員がいるはず");

    let report =
        reorg::simulate_reorg(&generated.chart, &target).expect("D01は実在する部署キーのはず");

    assert_eq!(report.reassigned.len(), before_count);
    // 再配置先はすべて対象部署以外
    assert!(report.reassigned.iter().all(|(_, d)| *d != target));

    match &report.outcome {
        reorg::ReorgOutcome::Success(new_org) => {
            // 廃止部署はもう存在しない
            assert!(new_org.department_by_id(&target).is_none());
            // 再配置された社員は新部署に所属している
            for (emp_id, new_dept) in &report.reassigned {
                let actual = new_org
                    .employee_by_id(emp_id)
                    .map(|employee| employee.belongs_to_as_employee().department());
                assert_eq!(
                    actual.map(|d| d.name.clone()),
                    new_org.department_by_id(new_dept).map(|d| d.name.clone())
                );
            }
            // 社員総数・プロジェクト総数は変化しない
            assert_eq!(
                new_org.employee_ids().count(),
                generated.chart.employee_ids().count()
            );
            assert_eq!(
                new_org.project_ids().count(),
                generated.chart.project_ids().count()
            );
            assert_eq!(
                new_org.department_ids().count(),
                generated.chart.department_ids().count() - 1
            );
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
    // sponsors().iter()を持つ部署を1つ探す (スポンサー辺を発している側)。
    let sponsor_dept = generated
        .chart
        .sponsors_iter()
        .map(|edge| edge.department().id().clone())
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
