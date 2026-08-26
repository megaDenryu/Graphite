//! `chain` が循環で打ち切ること・トップ層で停止すること・未知キーを弾くこと。

use org_analyzer::schema::EmployeeId;
use org_analyzer::{analysis, dataset};

const TEST_SEED: u64 = 7;

#[test]
fn chainは循環を検出して打ち切る() {
    let generated = dataset::generate(TEST_SEED, true);
    let plan = generated.anomaly_plan.as_ref().unwrap();
    let start = plan.cycle[0].clone();

    let result = analysis::management_chain(&generated.chart, &start)
        .expect("存在する社員キーなのでSomeのはず");

    assert!(
        result.cycle_back_to.is_some(),
        "循環に突入するチェーンは打ち切られるはず"
    );
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
            let emp = generated.chart.employee_by_id(id).unwrap();
            emp.grade == 5
                && generated
                    .chart
                    .employee_by_id(id)
                    .unwrap()
                    .boss_as_subordinate()
                    .is_none()
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
