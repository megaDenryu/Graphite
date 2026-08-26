//! `reorg` サブコマンドの表示。

use crate::reorg::{ReorgOutcome, ReorgReport};
use crate::schema::OrgChart;

pub fn print_reorg(org: &OrgChart::Graph, report: &ReorgReport) {
    println!("=== 組織改編シミュレーション ===");
    println!(
        "廃止対象部署: {} ({})",
        report.removed_department_name, report.removed_department.0
    );
    println!("再配置対象: {}人\n", report.reassigned.len());

    println!("--- 再配置先 (社員キー順、ラウンドロビン) ---");
    for (emp_id, new_dept) in report.reassigned.iter().take(10) {
        let name = org
            .employee_by_id(emp_id)
            .map(|e| e.value().name.as_str())
            .unwrap_or("?");
        let dept_name = org
            .department_by_id(new_dept)
            .map(|d| d.value().name.as_str())
            .unwrap_or("?");
        println!(
            "  {} ({}) -> {} ({})",
            name, emp_id.0, dept_name, new_dept.0
        );
    }
    if report.reassigned.len() > 10 {
        println!("  ... 他 {}人", report.reassigned.len() - 10);
    }

    println!();
    match &report.outcome {
        ReorgOutcome::Success(new_org) => {
            println!("[OK] 再構築に成功しました (freeze検証をパス)");
            println!(
                "  新組織: 社員{}人 / 部署{}人 / プロジェクト{}件",
                new_org.employee_ids().count(),
                new_org.department_ids().count(),
                new_org.project_ids().count()
            );
        }
        ReorgOutcome::Violated(violation) => {
            println!("[NG] freeze検証がViolationを検出し、再構築は失敗しました:");
            println!("  {violation}");
            println!("  詳細: {violation:?}");
            println!(
                "\n  解説: 廃止部署が指すsponsors辺(部署->プロジェクト)をカスケード削除\n\
                 し忘れたまま再構築しようとしたため、存在しない部署キーを参照する辺が\n\
                 残り、create()のfreeze検証がそれを機械的に検出しました。可変APIが\n\
                 存在しないGraphiteでは「不変+再構築」しか編集手段がないため、この種の\n\
                 参照切れは(見落とさない限り)必ずこの場で顕在化します。"
            );
        }
    }
}
