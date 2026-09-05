//! `summary` サブコマンドの集計。部署別人数・grade分布・プロジェクト別
//! アサイン人数を数え、span of control の統計と併せて1つの報告にまとめる。

use std::collections::HashMap;

use super::span_of_control::{span_of_control, SpanOfControlStats};
use crate::schema::{DepartmentId, OrgChart, ProjectId};

// 部署別の在籍人数。
#[derive(Debug, Clone, PartialEq)]
pub struct DeptCount {
    pub department: DepartmentId,
    pub name: String,
    pub count: usize,
}

// grade 別の人数分布。
#[derive(Debug, Clone, PartialEq)]
pub struct GradeCount {
    pub grade: u8,
    pub count: usize,
}
// プロジェクト別のアサイン人数。
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectAssignmentCount {
    pub project: ProjectId,
    pub name: String,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SummaryReport {
    pub total_employees: usize,
    pub dept_counts: Vec<DeptCount>,
    pub grade_counts: Vec<GradeCount>,
    pub span_of_control: SpanOfControlStats,
    pub project_assignments: Vec<ProjectAssignmentCount>,
}
pub fn summarize(org: &OrgChart::Graph) -> SummaryReport {
    let total_employees = org.employee_ids().count();

    // 部署別人数: 部署を終点とする BelongsTo エッジの本数。`docs/reverse_query.md`
    // の役割探索 `department.belongs_to_as_department()` を使うと、
    // 全エッジを走査して HashMap に集計する前段が不要になる (freeze 時に
    // 構築済みの終点索引を `id` ごとに引くだけで済む)。
    let mut dept_counts: Vec<DeptCount> = org
        .department_ids()
        .map(|id| DeptCount {
            department: id.clone(),
            name: org
                .department_by_id(id)
                .expect("department_idsから得たキーは必ず存在する")
                .name
                .clone(),
            count: org
                .department_by_id(id)
                .expect("列挙した部署は存在する")
                .belongs_to_as_department()
                .count(),
        })
        .collect();
    dept_counts.sort_by(|a, b| a.department.cmp(&b.department));

    // grade 分布
    let mut grade_counter: HashMap<u8, usize> = HashMap::new();
    for id in org.employee_ids() {
        let grade = org
            .employee_by_id(id)
            .expect("employee_idsから得たキーは必ず存在する")
            .grade;
        *grade_counter.entry(grade).or_insert(0) += 1;
    }
    let mut grade_counts: Vec<GradeCount> = grade_counter
        .into_iter()
        .map(|(grade, count)| GradeCount { grade, count })
        .collect();
    grade_counts.sort_by_key(|g| g.grade);

    // プロジェクト別アサイン人数。同じ理由で `project.assigned_as_project()` を使う。
    let mut project_assignments: Vec<ProjectAssignmentCount> = org
        .project_ids()
        .map(|id| ProjectAssignmentCount {
            project: id.clone(),
            name: org.project_by_id(id).unwrap().name.clone(),
            count: org
                .project_by_id(id)
                .expect("列挙したプロジェクトは存在する")
                .assigned_as_project()
                .count(),
        })
        .collect();
    project_assignments.sort_by(|a, b| a.project.cmp(&b.project));
    SummaryReport {
        total_employees,
        dept_counts,
        grade_counts,
        span_of_control: span_of_control(org),
        project_assignments,
    }
}
