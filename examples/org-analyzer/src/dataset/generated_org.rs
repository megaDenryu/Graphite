//! 生成結果の型 — 組織図そのものと、意図的に注入した異常の「正解」記録。

use crate::schema::{EmployeeId, OrgChart, ProjectId};

/// 意図的に注入した構造異常の「正解」記録。`anomalies` コマンドの検出結果と
/// 突き合わせるテスト用データ。
#[derive(Debug, Clone)]
pub struct AnomalyPlan {
    /// 相互上司ペア (A の boss が B かつ B の boss が A)。
    pub mutual_pair: (EmployeeId, EmployeeId),
    /// 上司関係の循環 (3人。`cycle[0]` の boss は `cycle[1]`、
    /// `cycle[1]` の boss は `cycle[2]`、`cycle[2]` の boss は `cycle[0]`)。
    pub cycle: Vec<EmployeeId>,
    /// どの部署からもスポンサーされないよう強制したプロジェクト。
    pub sponsorless_project: ProjectId,
    /// 誰もアサインされないよう強制したプロジェクト。
    pub unstaffed_project: ProjectId,
}

/// 生成された組織データ一式。
pub struct GeneratedOrg {
    pub chart: OrgChart::Graph,
    /// `inject_anomalies` が有効なときだけ `Some`。
    pub anomaly_plan: Option<AnomalyPlan>,
}
