//! span of control (管理職1人あたりの直属部下数) の統計。

use crate::dataset::MANAGER_GRADE_THRESHOLD;
use crate::schema::{EmployeeId, OrgChart};

// span of control (直属部下数) の統計。
//
// `average` は、管理職 (grade >= `MANAGER_GRADE_THRESHOLD`) 全員を母数にした
// 直属部下数の平均である (部下0人の管理職も含めて平均する)。
//
// `zero_report_managers` は、部下が1人もいない管理職の一覧である。要素は
// `(id, name, title)` の3つ組である。
#[derive(Debug, Clone, PartialEq)]
pub struct SpanOfControlStats {
    pub average: f64,
    pub max: usize,
    pub max_manager: Option<(EmployeeId, String)>,
    pub zero_report_managers: Vec<(EmployeeId, String, String)>,
}
// 管理職を終点とする `Boss` 辺の本数を、`superior.boss_as_superior()` で
// 直接引いて集計する (直属部下の一覧を全辺から事前に作る必要はない)。
pub fn span_of_control(org: &OrgChart::Graph) -> SpanOfControlStats {
    let managers: Vec<EmployeeId> = org
        .employee_ids()
        .filter(|id| org.employee_by_id(id).unwrap().grade >= MANAGER_GRADE_THRESHOLD)
        .cloned()
        .collect();

    let mut max: usize = 0;
    let mut max_manager: Option<(EmployeeId, String)> = None;
    let mut zero_report_managers: Vec<(EmployeeId, String, String)> = Vec::new();
    let mut sum: usize = 0;
    for id in &managers {
        let count = org
            .employee_by_id(id)
            .expect("列挙した社員は存在する")
            .boss_as_superior()
            .count();
        sum += count;
        let emp = org.employee_by_id(id).unwrap();
        if count > max {
            max = count;
            max_manager = Some((id.clone(), emp.name.clone()));
        }
        if count == 0 {
            zero_report_managers.push((id.clone(), emp.name.clone(), emp.title.clone()));
        }
    }
    zero_report_managers.sort_by(|a, b| a.0.cmp(&b.0));
    let average = if managers.is_empty() {
        0.0
    } else {
        sum as f64 / managers.len() as f64
    };

    SpanOfControlStats {
        average,
        max,
        max_manager,
        zero_report_managers,
    }
}
