//! 構造異常検出コマンド (`anomalies`) が拾うべき異常を、生成済みの辺集合へ
//! 意図的に埋め込む。何を埋め込んだかは `AnomalyPlan` に記録し、テストが
//! 検出結果と突き合わせられるようにする。

use super::element_id::{employee_id, project_id};
use super::generated_org::AnomalyPlan;
use crate::schema::{AssignedEdge, BossEdge, DepartmentId, EmployeeId, ProjectId};

pub(super) fn inject_anomalies(
    boss_edges: &mut Vec<(EmployeeId, EmployeeId, BossEdge)>,
    assigned_edges: &mut Vec<(EmployeeId, ProjectId, AssignedEdge)>,
    sponsors_edges: &mut Vec<(DepartmentId, ProjectId)>,
) -> AnomalyPlan {
    // 1. 相互上司ペア: E001 <-> E002 (両者の既存 boss 辺を上書き)
    let mutual_a = employee_id(0);
    let mutual_b = employee_id(1);
    boss_edges.retain(|(from, _, _)| *from != mutual_a && *from != mutual_b);
    boss_edges.push((mutual_a.clone(), mutual_b.clone(), BossEdge { since: 2021 }));
    boss_edges.push((mutual_b.clone(), mutual_a.clone(), BossEdge { since: 2020 }));

    // 2. 上司循環: E003 -> E004 -> E005 -> E003
    let cycle: Vec<EmployeeId> = vec![employee_id(2), employee_id(3), employee_id(4)];
    boss_edges.retain(|(from, _, _)| !cycle.contains(from));
    for k in 0..cycle.len() {
        let next = cycle[(k + 1) % cycle.len()].clone();
        boss_edges.push((
            cycle[k].clone(),
            next,
            BossEdge {
                since: 2019 + k as i32,
            },
        ));
    }

    // 3. スポンサー無しプロジェクト強制: P01 を指す sponsors 辺を全て除去
    let sponsorless_project = project_id(0);
    sponsors_edges.retain(|(_, p)| *p != sponsorless_project);

    // 4. 無人プロジェクト強制: P02 を指す assigned 辺を全て除去
    let unstaffed_project = project_id(1);
    assigned_edges.retain(|(_, p, _)| *p != unstaffed_project);

    AnomalyPlan {
        mutual_pair: (mutual_a, mutual_b),
        cycle,
        sponsorless_project,
        unstaffed_project,
    }
}
