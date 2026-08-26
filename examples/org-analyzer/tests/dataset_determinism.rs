//! 合成データ生成器が、同じシードなら同じ組織を、違うシードなら違う組織を
//! 返すこと。

use org_analyzer::dataset;

#[test]
fn 同じシードなら生成結果は決定的である() {
    let a = dataset::generate(123, false);
    let b = dataset::generate(123, false);

    let names_a: Vec<String> = a
        .chart
        .employee_ids()
        .map(|id| a.chart.employee_by_id(id).unwrap().name.clone())
        .collect();
    let names_b: Vec<String> = b
        .chart
        .employee_ids()
        .map(|id| b.chart.employee_by_id(id).unwrap().name.clone())
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
        .map(|d| {
            a.chart
                .belongs_to_iter()
                .filter(|edge| edge.department().id() == d)
                .count()
        })
        .collect();
    let dept_counts_b: Vec<usize> = b
        .chart
        .department_ids()
        .map(|d| {
            b.chart
                .belongs_to_iter()
                .filter(|edge| edge.department().id() == d)
                .count()
        })
        .collect();

    assert_ne!(
        dept_counts_a, dept_counts_b,
        "seedが違えば部署別人数分布は変わるはず"
    );
}
