//! 上司関係そのものの異常 — 相互上司ペア・上司関係の循環・部署跨ぎの上司。

use std::collections::{HashMap, HashSet};

use graphite::{CycleError, Graph};

use crate::schema::{DepartmentId, EmployeeId, OrgChart};

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
/// 相互上司ペアの検出。README に載っている手法そのもの:
/// 全ペアを集めておき、`(a, b)` かつ `(b, a)` が両方存在するものを拾う。
///
/// 判定には `EmployeeId` の対そのものが要るため、EdgeRefの両端からIDを集める。
pub(super) fn detect_mutual_boss_pairs(org: &OrgChart::Graph) -> Vec<(EmployeeId, EmployeeId)> {
    let all: Vec<(&EmployeeId, &EmployeeId)> = org
        .boss_iter()
        .map(|edge| (edge.subordinate().id(), edge.superior().id()))
        .collect();

    let mut result: Vec<(EmployeeId, EmployeeId)> = Vec::new();
    for &(a, b) in &all {
        if a < b && all.contains(&(b, a)) {
            result.push((a.clone(), b.clone()));
        }
    }
    result.sort();
    result
}

/// 上司関係の循環検出。
///
/// `Boss` エッジ (`(subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee)`, each subordinate: 0..1) を
/// 汎用 `graphite::Graph<(), (), EmployeeId>` に射影する (`Graph::from_edges`
/// が `{kind}_iter` からの定型的な射影をまとめてくれる)。`topological_sort`
/// が返す `CycleError::cycle` はフェーズ5から循環メンバー全体を返すように
/// なったため、以前のような「boss辺を手で辿って復元する」処理は不要になった。
/// 1つの循環を見つけたら `filter_nodes_with_key` でそのメンバーを取り除いた
/// 部分グラフに対して再度検出し、複数の循環があっても全て拾えるようにして
/// いる。
pub(super) fn detect_boss_cycles(org: &OrgChart::Graph) -> Vec<Vec<EmployeeId>> {
    let mut graph: Graph<(), (), EmployeeId> = Graph::from_edges(
        org.employee_ids().cloned(),
        org.boss_iter().map(|edge| {
            (
                edge.subordinate().id().clone(),
                edge.superior().id().clone(),
            )
        }),
    )
    .expect("employee_idsは重複せず、boss_iterの端点は全てemployee_idsに含まれるはず");

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
pub(super) fn detect_cross_department_bosses(org: &OrgChart::Graph) -> Vec<CrossDepartmentBoss> {
    let dept_of: HashMap<&EmployeeId, &DepartmentId> = org
        .belongs_to_iter()
        .map(|edge| (edge.employee().id(), edge.department().id()))
        .collect();

    let mut result: Vec<CrossDepartmentBoss> = org
        .boss_iter()
        .filter_map(|edge| {
            let emp_id = edge.subordinate().id();
            let boss_id = edge.superior().id();
            let emp_dept: &DepartmentId = *dept_of.get(emp_id)?;
            let boss_dept: &DepartmentId = *dept_of.get(boss_id)?;
            if emp_dept == boss_dept {
                return None;
            }
            Some(CrossDepartmentBoss {
                employee: emp_id.clone(),
                employee_name: org.employee_by_id(emp_id).unwrap().name.clone(),
                employee_dept: emp_dept.clone(),
                boss: boss_id.clone(),
                boss_name: org.employee_by_id(boss_id).unwrap().name.clone(),
                boss_dept: boss_dept.clone(),
            })
        })
        .collect();
    result.sort_by(|a, b| a.employee.cmp(&b.employee));
    result
}
