//! `summary` サブコマンドの表示。

use crate::analysis::SummaryReport;

pub fn print_summary(report: &SummaryReport) {
    println!("=== 組織サマリ ===");
    println!("社員総数: {}人\n", report.total_employees);

    println!("--- 部署別人数 ---");
    for d in &report.dept_counts {
        println!("  {:<12} ({}) : {:>3}人", d.name, d.department.0, d.count);
    }

    println!("\n--- grade分布 ---");
    for g in &report.grade_counts {
        println!("  grade{} : {:>3}人", g.grade, g.count);
    }

    println!("\n--- span of control (直属部下数) ---");
    println!(
        "  管理職(grade3以上)平均: {:.2}人",
        report.span_of_control.average
    );
    match &report.span_of_control.max_manager {
        Some((id, name)) => {
            println!(
                "  最大: {}人 ({} / {})",
                report.span_of_control.max, name, id.0
            )
        }
        None => println!("  最大: -"),
    }
    if report.span_of_control.zero_report_managers.is_empty() {
        println!("  部下ゼロの管理職: なし");
    } else {
        println!(
            "  部下ゼロの管理職: {}人",
            report.span_of_control.zero_report_managers.len()
        );
        for (id, name, title) in &report.span_of_control.zero_report_managers {
            println!("    - {} ({} / {})", name, title, id.0);
        }
    }

    println!("\n--- プロジェクト別アサイン人数 ---");
    for p in &report.project_assignments {
        let marker = if p.count == 0 { "  [無人]" } else { "" };
        println!(
            "  {:<16} ({}) : {:>3}人{}",
            p.name, p.project.0, p.count, marker
        );
    }
}
