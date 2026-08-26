//! `summary` の統計値が健全な範囲に収まること。

use org_analyzer::{analysis, dataset};

const TEST_SEED: u64 = 7;

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
