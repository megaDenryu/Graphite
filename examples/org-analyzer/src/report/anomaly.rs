//! `anomalies` サブコマンドの表示。

use crate::analysis::AnomalyReport;
use crate::schema::{OrgChart, ProjectId};

pub fn print_anomalies(org: &OrgChart::Graph, report: &AnomalyReport) {
    println!("=== 構造異常レポート ===\n");

    println!("--- 相互上司ペア ---");
    if report.mutual_boss_pairs.is_empty() {
        println!("  なし");
    } else {
        for (a, b) in &report.mutual_boss_pairs {
            let name_a = org
                .employee_by_id(a)
                .map(|e| e.value().name.as_str())
                .unwrap_or("?");
            let name_b = org
                .employee_by_id(b)
                .map(|e| e.value().name.as_str())
                .unwrap_or("?");
            println!("  {} ({}) <-> {} ({})", name_a, a.0, name_b, b.0);
        }
    }

    println!("\n--- 上司関係の循環 ---");
    if report.boss_cycles.is_empty() {
        println!("  なし");
    } else {
        for (i, cycle) in report.boss_cycles.iter().enumerate() {
            let names: Vec<String> = cycle
                .iter()
                .map(|id| {
                    let name = org
                        .employee_by_id(id)
                        .map(|e| e.value().name.as_str())
                        .unwrap_or("?");
                    format!("{}({})", name, id.0)
                })
                .collect();
            println!("  循環{}: {} -> (先頭に戻る)", i + 1, names.join(" -> "));
        }
    }

    println!("\n--- 部署跨ぎ上司 ---");
    if report.cross_department_bosses.is_empty() {
        println!("  なし");
    } else {
        for c in &report.cross_department_bosses {
            println!(
                "  {} ({}, 所属:{}) の上司は {} ({}, 所属:{})",
                c.employee_name,
                c.employee.0,
                c.employee_dept.0,
                c.boss_name,
                c.boss.0,
                c.boss_dept.0
            );
        }
    }

    println!("\n--- 無人プロジェクト ---");
    print_project_list(org, &report.unstaffed_projects);

    println!("\n--- スポンサー無しプロジェクト ---");
    print_project_list(org, &report.sponsorless_projects);
}

fn print_project_list(org: &OrgChart::Graph, ids: &[ProjectId]) {
    if ids.is_empty() {
        println!("  なし");
        return;
    }
    for id in ids {
        let name = org
            .project_by_id(id)
            .map(|p| p.value().name.as_str())
            .unwrap_or("?");
        println!("  {} ({})", name, id.0);
    }
}
