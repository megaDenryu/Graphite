//! `anomalies` サブコマンドの報告一式。上司側とプロジェクト側の検出結果を
//! 1つにまとめる。

use super::boss_anomaly::{
    detect_boss_cycles, detect_cross_department_bosses, detect_mutual_boss_pairs,
};
use super::project_anomaly::{detect_sponsorless_projects, detect_unstaffed_projects};
use crate::schema::{EmployeeId, OrgChart, ProjectId};

pub use super::boss_anomaly::CrossDepartmentBoss;

#[derive(Debug, Clone, PartialEq)]
pub struct AnomalyReport {
    pub mutual_boss_pairs: Vec<(EmployeeId, EmployeeId)>, // 相互上司ペア (正規化済み: 同じペアが2回出ないよう `(小さい方, 大きい方)` に統一)。
    pub boss_cycles: Vec<Vec<EmployeeId>>, // 上司関係の循環。各要素は循環に含まれる社員キーの並び。
    pub cross_department_bosses: Vec<CrossDepartmentBoss>,
    pub unstaffed_projects: Vec<ProjectId>,
    pub sponsorless_projects: Vec<ProjectId>,
}

pub fn detect_anomalies(org: &OrgChart::Graph) -> AnomalyReport {
    AnomalyReport {
        mutual_boss_pairs: detect_mutual_boss_pairs(org),
        boss_cycles: detect_boss_cycles(org),
        cross_department_bosses: detect_cross_department_bosses(org),
        unstaffed_projects: detect_unstaffed_projects(org),
        sponsorless_projects: detect_sponsorless_projects(org),
    }
}
