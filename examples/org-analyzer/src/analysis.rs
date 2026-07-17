//! 集計 (`summary`)・管理チェーン追跡 (`chain`)・構造異常検出 (`anomalies`)
//! のロジック。
//!
//! CLI からの呼び出しと表示整形 (`report.rs`) を分離し、この module は
//! 「`OrgChart` を読んで構造化データを返す」ことだけに専念する。

use std::collections::{HashMap, HashSet};

use graphite::{CycleError, Graph};

use crate::dataset::MANAGER_GRADE_THRESHOLD;
use crate::schema::{
    Assigned, BelongsTo, Boss, Department, DepartmentId, Employee, EmployeeId, OrgChart,
    OrgChartNode, Project, ProjectId, Sponsors,
};

// ============================================================
// summary
// ============================================================

/// 部署別の在籍人数。
#[derive(Debug, Clone, PartialEq)]
pub struct DeptCount {
    pub department: DepartmentId,
    pub name: String,
    pub count: usize,
}

/// grade 別の人数分布。
#[derive(Debug, Clone, PartialEq)]
pub struct GradeCount {
    pub grade: u8,
    pub count: usize,
}

/// span of control (直属部下数) の統計。
#[derive(Debug, Clone, PartialEq)]
pub struct SpanOfControlStats {
    /// 管理職 (grade >= `MANAGER_GRADE_THRESHOLD`) 全員を母数にした
    /// 直属部下数の平均 (部下0人の管理職も含めて平均する)。
    pub average: f64,
    pub max: usize,
    pub max_manager: Option<(EmployeeId, String)>,
    /// 部下が1人もいない管理職一覧 (`(id, name, title)`)。
    pub zero_report_managers: Vec<(EmployeeId, String, String)>,
}

/// プロジェクト別のアサイン人数。
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

pub fn summarize(org: &OrgChart) -> SummaryReport {
    let total_employees = Employee::ids(org).count();

    // 部署別人数: BelongsTo::iter (each Employee: 1、社員ごとにちょうど1本) を
    // 部署キーで集計する。
    let mut dept_counter: HashMap<DepartmentId, usize> = HashMap::new();
    for (_id, edge) in BelongsTo::iter(org) {
        *dept_counter.entry(edge.to().clone()).or_insert(0) += 1;
    }
    let mut dept_counts: Vec<DeptCount> = Department::ids(org)
        .map(|id| DeptCount {
            department: id.clone(),
            name: Department::get(org, id)
                .expect("Department::idsから得たキーは必ず存在する")
                .name
                .clone(),
            count: dept_counter.get(id).copied().unwrap_or(0),
        })
        .collect();
    dept_counts.sort_by(|a, b| a.department.cmp(&b.department));

    // grade 分布
    let mut grade_counter: HashMap<u8, usize> = HashMap::new();
    for id in Employee::ids(org) {
        let grade = Employee::get(org, id)
            .expect("Employee::idsから得たキーは必ず存在する")
            .grade;
        *grade_counter.entry(grade).or_insert(0) += 1;
    }
    let mut grade_counts: Vec<GradeCount> = grade_counter
        .into_iter()
        .map(|(grade, count)| GradeCount { grade, count })
        .collect();
    grade_counts.sort_by_key(|g| g.grade);

    // span of control: Boss::iter から「boss -> 直属部下数」を集計する。
    let mut direct_reports: HashMap<EmployeeId, usize> = HashMap::new();
    for (_id, edge) in Boss::iter(org) {
        *direct_reports.entry(edge.to().clone()).or_insert(0) += 1;
    }

    let managers: Vec<EmployeeId> = Employee::ids(org)
        .filter(|id| Employee::get(org, id).unwrap().grade >= MANAGER_GRADE_THRESHOLD)
        .cloned()
        .collect();

    let mut max: usize = 0;
    let mut max_manager: Option<(EmployeeId, String)> = None;
    let mut zero_report_managers: Vec<(EmployeeId, String, String)> = Vec::new();
    let mut sum: usize = 0;
    for id in &managers {
        let count = direct_reports.get(id).copied().unwrap_or(0);
        sum += count;
        let emp = Employee::get(org, id).unwrap();
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

    // プロジェクト別アサイン人数
    let mut project_counter: HashMap<ProjectId, usize> = HashMap::new();
    for (_id, edge) in Assigned::iter(org) {
        *project_counter.entry(edge.to().clone()).or_insert(0) += 1;
    }
    let mut project_assignments: Vec<ProjectAssignmentCount> = Project::ids(org)
        .map(|id| ProjectAssignmentCount {
            project: id.clone(),
            name: Project::get(org, id).unwrap().name.clone(),
            count: project_counter.get(id).copied().unwrap_or(0),
        })
        .collect();
    project_assignments.sort_by(|a, b| a.project.cmp(&b.project));

    SummaryReport {
        total_employees,
        dept_counts,
        grade_counts,
        span_of_control: SpanOfControlStats {
            average,
            max,
            max_manager,
            zero_report_managers,
        },
        project_assignments,
    }
}

// ============================================================
// chain
// ============================================================

/// 管理チェーン中の 1 エントリ。
#[derive(Debug, Clone, PartialEq)]
pub struct ChainEntry {
    /// 起点からの深さ (起点自身は0)。
    pub depth: usize,
    pub employee: EmployeeId,
    pub name: String,
    pub title: String,
    /// このエントリの上司との在任年 (起点自身は `None`)。
    pub since: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChainResult {
    pub entries: Vec<ChainEntry>,
    /// 途中で循環を検出して打ち切った場合 `Some(戻り先のキー)`。
    pub cycle_back_to: Option<EmployeeId>,
}

/// 指定した社員から `Boss` 辺を根 (トップ層) まで辿る。
///
/// `Boss::of` (each Employee: 0..1) は `Option<(&Employee, &BossEdge)>` を
/// 返すだけで上司の `EmployeeId` そのものは含まないため、辿るには
/// `Boss::iter` から `EmployeeId -> (EmployeeId, since)` の索引を先に
/// 作っておく必要がある。
///
/// 訪問済み集合を持ちながら辿ることで循環を検出する。循環に突入したら
/// そこで打ち切り、`cycle_back_to` にループの戻り先キーを記録する
/// (`anomalies` コマンドの循環検出とは独立した、チェーン単体での安全対策)。
pub fn management_chain(org: &OrgChart, start: &EmployeeId) -> Option<ChainResult> {
    let start_employee = Employee::get(org, start)?;

    let boss_of: HashMap<EmployeeId, (EmployeeId, i32)> = Boss::iter(org)
        .map(|(_id, edge)| (edge.from().clone(), (edge.to().clone(), edge.payload().since)))
        .collect();

    let mut entries = vec![ChainEntry {
        depth: 0,
        employee: start.clone(),
        name: start_employee.name.clone(),
        title: start_employee.title.clone(),
        since: None,
    }];
    let mut visited: HashSet<EmployeeId> = HashSet::new();
    visited.insert(start.clone());

    let mut current = start.clone();
    let mut depth = 1usize;
    let mut cycle_back_to = None;

    loop {
        match boss_of.get(&current) {
            None => break, // トップ層に到達 (これ以上の上司なし)
            Some((boss_id, since)) => {
                if visited.contains(boss_id) {
                    cycle_back_to = Some(boss_id.clone());
                    break;
                }
                let boss_employee = Employee::get(org, boss_id)
                    .expect("Boss::iterの終点は必ずemployeeに存在するはず");
                entries.push(ChainEntry {
                    depth,
                    employee: boss_id.clone(),
                    name: boss_employee.name.clone(),
                    title: boss_employee.title.clone(),
                    since: Some(*since),
                });
                visited.insert(boss_id.clone());
                current = boss_id.clone();
                depth += 1;
            }
        }
    }

    Some(ChainResult {
        entries,
        cycle_back_to,
    })
}

// ============================================================
// anomalies
// ============================================================

/// 部署を跨いだ上司関係 (上司と部下が異なる部署に所属している)。
#[derive(Debug, Clone, PartialEq)]
pub struct CrossDepartmentBoss {
    pub employee: EmployeeId,
    pub employee_name: String,
    pub employee_dept: DepartmentId,
    pub boss: EmployeeId,
    pub boss_name: String,
    pub boss_dept: DepartmentId,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnomalyReport {
    /// 相互上司ペア (正規化済み: 同じペアが2回出ないよう `(小さい方, 大きい方)` に統一)。
    pub mutual_boss_pairs: Vec<(EmployeeId, EmployeeId)>,
    /// 上司関係の循環。各要素は循環に含まれる社員キーの並び。
    pub boss_cycles: Vec<Vec<EmployeeId>>,
    pub cross_department_bosses: Vec<CrossDepartmentBoss>,
    pub unstaffed_projects: Vec<ProjectId>,
    pub sponsorless_projects: Vec<ProjectId>,
}

pub fn detect_anomalies(org: &OrgChart) -> AnomalyReport {
    AnomalyReport {
        mutual_boss_pairs: detect_mutual_boss_pairs(org),
        boss_cycles: detect_boss_cycles(org),
        cross_department_bosses: detect_cross_department_bosses(org),
        unstaffed_projects: detect_unstaffed_projects(org),
        sponsorless_projects: detect_sponsorless_projects(org),
    }
}

/// 相互上司ペアの検出。README に載っている手法そのもの:
/// 全ペアを集めておき、`(a, b)` かつ `(b, a)` が両方存在するものを拾う。
fn detect_mutual_boss_pairs(org: &OrgChart) -> Vec<(EmployeeId, EmployeeId)> {
    let all: Vec<(&EmployeeId, &EmployeeId)> =
        Boss::iter(org).map(|(_id, edge)| (edge.from(), edge.to())).collect();

    let mut result: Vec<(EmployeeId, EmployeeId)> = Vec::new();
    for (a, b) in &all {
        if a < b && all.contains(&(b, a)) {
            result.push(((*a).clone(), (*b).clone()));
        }
    }
    result.sort();
    result
}

/// 上司関係の循環検出。
///
/// `Boss` エッジ (Employee -[BossEdge]-> Employee, each Employee: 0..1) を
/// 汎用 `graphite::Graph<(), (), EmployeeId>` に射影する (`Graph::from_edges`
/// が `Kind::iter` からの定型的な射影をまとめてくれる)。`topological_sort`
/// が返す `CycleError::cycle` はフェーズ5から循環メンバー全体を返すように
/// なったため、以前のような「boss辺を手で辿って復元する」処理は不要になった。
/// 1つの循環を見つけたら `filter_nodes_with_key` でそのメンバーを取り除いた
/// 部分グラフに対して再度検出し、複数の循環があっても全て拾えるようにして
/// いる。
fn detect_boss_cycles(org: &OrgChart) -> Vec<Vec<EmployeeId>> {
    let mut graph: Graph<(), (), EmployeeId> = Graph::from_edges(
        Employee::ids(org).cloned(),
        Boss::iter(org).map(|(_id, edge)| (edge.from().clone(), edge.to().clone())),
    )
    .expect("Employee::idsは重複せず、Boss::iterの端点は全てEmployee::idsに含まれるはず");

    let mut cycles: Vec<Vec<EmployeeId>> = Vec::new();

    while let Err(CycleError { cycle }) = graph.topological_sort() {
        let members_set: HashSet<EmployeeId> = cycle.iter().cloned().collect();
        // 長さ2の循環 (相互上司) は「相互上司ペア」で別途報告済みなので
        // ここには含めない (2つのレポート項目が同じ事実を重複して指す
        // のを避ける)。ここでの関心は「3人以上」の循環。
        if cycle.len() >= 3 {
            cycles.push(cycle);
        }

        // 見つけた循環のメンバーを除いた部分グラフで再検出する
        // (残りに別の独立した循環があるケースに備える)。
        graph = graph.filter_nodes_with_key(|k, _| !members_set.contains(k));
    }

    cycles
}

/// 部署跨ぎの上司関係 (上司と部下が異なる部署)。
fn detect_cross_department_bosses(org: &OrgChart) -> Vec<CrossDepartmentBoss> {
    let dept_of: HashMap<&EmployeeId, &DepartmentId> = BelongsTo::iter(org)
        .map(|(_id, edge)| (edge.from(), edge.to()))
        .collect();

    let mut result: Vec<CrossDepartmentBoss> = Boss::iter(org)
        .filter_map(|(_id, edge)| {
            let emp_id = edge.from();
            let boss_id = edge.to();
            let emp_dept = *dept_of.get(emp_id)?;
            let boss_dept = *dept_of.get(boss_id)?;
            if emp_dept == boss_dept {
                return None;
            }
            Some(CrossDepartmentBoss {
                employee: emp_id.clone(),
                employee_name: Employee::get(org, emp_id).unwrap().name.clone(),
                employee_dept: emp_dept.clone(),
                boss: boss_id.clone(),
                boss_name: Employee::get(org, boss_id).unwrap().name.clone(),
                boss_dept: boss_dept.clone(),
            })
        })
        .collect();
    result.sort_by(|a, b| a.employee.cmp(&b.employee));
    result
}

/// 誰もアサインされていないプロジェクト。
fn detect_unstaffed_projects(org: &OrgChart) -> Vec<ProjectId> {
    let staffed: HashSet<&ProjectId> = Assigned::iter(org).map(|(_id, edge)| edge.to()).collect();
    let mut result: Vec<ProjectId> = Project::ids(org)
        .filter(|p| !staffed.contains(p))
        .cloned()
        .collect();
    result.sort();
    result
}

/// どの部署からもスポンサーされていないプロジェクト。
fn detect_sponsorless_projects(org: &OrgChart) -> Vec<ProjectId> {
    let sponsored: HashSet<&ProjectId> = Sponsors::iter(org).map(|(_id, edge)| edge.to()).collect();
    let mut result: Vec<ProjectId> = Project::ids(org)
        .filter(|p| !sponsored.contains(p))
        .cloned()
        .collect();
    result.sort();
    result
}
