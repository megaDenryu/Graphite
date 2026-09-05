//! `reorg` コマンドの結果を表す型。

use crate::schema::{DepartmentId, EmployeeId, OrgChart};

// `reorg` コマンドの結果。
//
// `OrgChart` は `Debug` を派生していない (schema struct は素の Rust 可視性
// 規則のためマクロが derive を付けていない) ので、この struct 自体にも
// `#[derive(Debug)]` は付けられない。表示は `report.rs::print_reorg` が
// 個別に行う。
//
// `reassigned` は再配置された社員 `(社員キー, 移動先部署キー)` の一覧である
// (決定的な順序: 元の所属を社員キー順にソートしてラウンドロビンで割当)。
pub struct ReorgReport {
    pub removed_department: DepartmentId,
    pub removed_department_name: String,
    pub reassigned: Vec<(EmployeeId, DepartmentId)>,
    pub outcome: ReorgOutcome,
}

pub enum ReorgOutcome {
    Success(Box<OrgChart::Graph>), // 再構築に成功した新しい組織図。
    Violated(OrgChart::Violation), // `freeze` 検証が検出した違反。
}
