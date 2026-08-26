//! 通し番号から要素キーの綴りを作る。生成器とテストが同じ綴りを前提にする
//! ため、綴りの決め方をこの1箇所へ閉じる。

use crate::schema::{DepartmentId, EmployeeId, ProjectId};

pub(super) fn employee_id(index: usize) -> EmployeeId {
    EmployeeId(format!("E{:03}", index + 1))
}

pub(super) fn department_id(index: usize) -> DepartmentId {
    DepartmentId(format!("D{:02}", index + 1))
}

pub(super) fn project_id(index: usize) -> ProjectId {
    ProjectId(format!("P{:02}", index + 1))
}
