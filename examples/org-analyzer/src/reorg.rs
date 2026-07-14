//! 組織改編シミュレーション (`reorg <部署キー>`)。
//!
//! Graphite の「不変 + 再構築」パターンのショーケース。`OrgChart` は構築後
//! 不変で、部署を1つ「削除」する編集操作は存在しない。代わりに、
//! 全ノード・全エッジをいったんスカラーデータへ展開し、対象部署を除いた
//! 集合から `OrgChart::create` で丸ごと再構築する。
//!
//! ここで意図的に「素朴な (naive) 」実装にしている点が1つある: 部署を
//! 削除する際、その部署が発していた `sponsors` 辺 (Department -> Project)
//! を明示的に取り除いていない。ノードを削除するときにそれを参照する辺の
//! カスケード削除を忘れる、というのは実務でもよくあるミスである。
//! 対象部署がどのプロジェクトもスポンサーしていなければ何も起こらず
//! `Ok` になるが、スポンサーしていた場合は `sponsors` 辺が存在しない部署
//! キーを参照したまま `OrgChart::create` に渡り、`freeze` 検証が
//! `OrgChartViolation::SponsorsUnknownSource` (フェーズ5でエッジ単位の
//! 型付きバリアントに変わった。以前は `UnknownDepartment` という
//! ノード単位の汎用バリアントだった) を返してエラーになる。
//! 「可変 API が存在しないので、参照が壊れたら気づかず放置される」のでは
//! なく「再構築 = 一括検証」なので**壊れていれば必ずその場で `Err` になる**、
//! という Graphite の設計意図を実地で確認できる。

use crate::schema::{
    AssignedAttrs, BossAttrs, Department, DepartmentId, Employee, EmployeeId, OrgChart,
    OrgChartViolation, Project, ProjectId,
};

/// `reorg` コマンドの結果。
///
/// `OrgChart` は `Debug` を派生していない (schema struct は素の Rust 可視性
/// 規則のためマクロが derive を付けていない) ので、この struct 自体にも
/// `#[derive(Debug)]` は付けられない。表示は `report.rs::print_reorg` が
/// 個別に行う。
pub struct ReorgReport {
    pub removed_department: DepartmentId,
    pub removed_department_name: String,
    /// 再配置された社員 `(社員キー, 移動先部署キー)` の一覧
    /// (決定的な順序: 元の所属を社員キー順にソートしてラウンドロビンで割当)。
    pub reassigned: Vec<(EmployeeId, DepartmentId)>,
    pub outcome: ReorgOutcome,
}

pub enum ReorgOutcome {
    /// 再構築に成功した新しい組織図。
    Success(Box<OrgChart>),
    /// `freeze` 検証が検出した違反。
    Violated(OrgChartViolation),
}

/// 指定した部署を廃止するシミュレーションを実行する。
/// 部署キーが存在しなければ `None`。
pub fn simulate_reorg(org: &OrgChart, target: &DepartmentId) -> Option<ReorgReport> {
    let removed_department_name = org.department(target)?.name.clone();

    let mut remaining_depts: Vec<DepartmentId> =
        org.department_ids().filter(|d| *d != target).cloned().collect();
    remaining_depts.sort();
    assert!(
        !remaining_depts.is_empty(),
        "部署が1つしかない組織はreorgの対象外 (現行データセットでは発生しない)"
    );

    // 元の belongs_to を社員キー順にソートし、対象部署に所属していた社員を
    // 残存部署へラウンドロビンで機械的に再配置する。
    let mut belongs_to: Vec<(EmployeeId, DepartmentId)> =
        org.belongs_to_pairs().map(|(e, d)| (e.clone(), d.clone())).collect();
    belongs_to.sort_by(|a, b| a.0.cmp(&b.0));

    let mut reassigned: Vec<(EmployeeId, DepartmentId)> = Vec::new();
    let mut new_belongs_to: Vec<(EmployeeId, DepartmentId)> = Vec::with_capacity(belongs_to.len());
    let mut round_robin = 0usize;
    for (emp_id, dept_id) in belongs_to {
        if &dept_id == target {
            let new_dept = remaining_depts[round_robin % remaining_depts.len()].clone();
            round_robin += 1;
            reassigned.push((emp_id.clone(), new_dept.clone()));
            new_belongs_to.push((emp_id, new_dept));
        } else {
            new_belongs_to.push((emp_id, dept_id));
        }
    }

    // ノード集合の再構築 (対象部署だけ除く)。
    let employees: Vec<(EmployeeId, Employee)> = org
        .employee_ids()
        .map(|id| (id.clone(), org.employee(id).unwrap().clone()))
        .collect();
    let departments: Vec<(DepartmentId, Department)> = remaining_depts
        .iter()
        .map(|id| (id.clone(), org.department(id).unwrap().clone()))
        .collect();
    let projects: Vec<(ProjectId, Project)> = org
        .project_ids()
        .map(|id| (id.clone(), org.project(id).unwrap().clone()))
        .collect();

    // boss / assigned は Employee が両端 (or 片端) なので部署削除の影響を
    // 受けない。素通しで良い。
    let boss_edges: Vec<(EmployeeId, EmployeeId, BossAttrs)> = org
        .boss_pairs()
        .map(|(a, b, attrs)| (a.clone(), b.clone(), attrs.clone()))
        .collect();
    let assigned_edges: Vec<(EmployeeId, ProjectId, AssignedAttrs)> = org
        .assigned_pairs()
        .map(|(e, p, attrs)| (e.clone(), p.clone(), attrs.clone()))
        .collect();

    // 意図的に「素朴」なまま: sponsors 辺は対象部署の分もフィルタせず
    // そのまま引き継ぐ (モジュール doc コメント参照)。
    let sponsors_edges: Vec<(DepartmentId, ProjectId)> =
        org.sponsors_pairs().map(|(d, p)| (d.clone(), p.clone())).collect();

    let result = OrgChart::create(|b| {
        for (id, e) in employees {
            b.employee(id, e);
        }
        for (id, d) in departments {
            b.department(id, d);
        }
        for (id, p) in projects {
            b.project(id, p);
        }
        for (e, d) in new_belongs_to {
            b.belongs_to(e, d);
        }
        for (from, to, attrs) in boss_edges {
            b.boss(from, to, attrs);
        }
        for (e, p, attrs) in assigned_edges {
            b.assigned(e, p, attrs);
        }
        for (d, p) in sponsors_edges {
            b.sponsors(d, p);
        }
    });

    let outcome = match result {
        Ok(new_org) => ReorgOutcome::Success(Box::new(new_org)),
        Err(violation) => ReorgOutcome::Violated(violation),
    };

    Some(ReorgReport {
        removed_department: target.clone(),
        removed_department_name,
        reassigned,
        outcome,
    })
}
