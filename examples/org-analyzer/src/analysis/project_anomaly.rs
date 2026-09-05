//! プロジェクト側の異常 — 誰もアサインされていない・どの部署もスポンサー
//! していないプロジェクト。

use crate::schema::{OrgChart, ProjectId};

// 誰もアサインされていないプロジェクト。
//
// 「このプロジェクトを終点とする Assigned エッジが1本もない」を判定するには
// 「全エッジから staffed 集合を事前に作る」必要はなく、`project.assigned_as_project()`
// をプロジェクトごとに直接引いて空かどうかを見れば
// 十分 (freeze 時に構築済みの終点索引を引くだけ)。
pub(super) fn detect_unstaffed_projects(org: &OrgChart::Graph) -> Vec<ProjectId> {
    let mut result: Vec<ProjectId> = org
        .project_ids()
        .filter(|p| {
            org.project_by_id(p)
                .unwrap()
                .assigned_as_project()
                .next()
                .is_none()
        })
        .cloned()
        .collect();
    result.sort();
    result
}

// どの部署からもスポンサーされていないプロジェクト。同じ理由で
// `project.sponsors_as_project()` を使う。
pub(super) fn detect_sponsorless_projects(org: &OrgChart::Graph) -> Vec<ProjectId> {
    let mut result: Vec<ProjectId> = org
        .project_ids()
        .filter(|p| {
            org.project_by_id(p)
                .unwrap()
                .sponsors_as_project()
                .next()
                .is_none()
        })
        .cloned()
        .collect();
    result.sort();
    result
}
