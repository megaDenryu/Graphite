//! 各サブコマンドの表示整形。`analysis.rs` / `reorg.rs` が返す構造化データを
//! 受け取り、人間が読みやすいテキストレポートを標準出力へ書く。

use crate::analysis::{AnomalyReport, ChainResult, SummaryReport};
use crate::reorg::{ReorgOutcome, ReorgReport};
use crate::schema::{Department, Employee, EmployeeId, OrgChart, OrgChartNode, Project};

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
    println!("  管理職(grade3以上)平均: {:.2}人", report.span_of_control.average);
    match &report.span_of_control.max_manager {
        Some((id, name)) => {
            println!("  最大: {}人 ({} / {})", report.span_of_control.max, name, id.0)
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
        println!("  {:<16} ({}) : {:>3}人{}", p.name, p.project.0, p.count, marker);
    }
}

pub fn print_chain(result: &ChainResult) {
    println!("=== 管理チェーン ===");
    for entry in &result.entries {
        let indent = "  ".repeat(entry.depth);
        match entry.since {
            Some(since) => println!(
                "{}└─ {} ({} / {}) [在任 {}年〜, 深さ{}]",
                indent, entry.name, entry.title, entry.employee.0, since, entry.depth
            ),
            None => println!(
                "{}{} ({} / {}) [起点, 深さ{}]",
                indent, entry.name, entry.title, entry.employee.0, entry.depth
            ),
        }
    }
    if let Some(back_to) = &result.cycle_back_to {
        println!(
            "\n[警告] 循環を検出したため打ち切りました (社員 {} まで戻っています)",
            back_to.0
        );
    } else {
        println!("\nトップ層に到達しました (これ以上の上司なし)");
    }
}

pub fn print_anomalies(org: &OrgChart, report: &AnomalyReport) {
    println!("=== 構造異常レポート ===\n");

    println!("--- 相互上司ペア ---");
    if report.mutual_boss_pairs.is_empty() {
        println!("  なし");
    } else {
        for (a, b) in &report.mutual_boss_pairs {
            let name_a = Employee::get(org, a).map(|e| e.name.as_str()).unwrap_or("?");
            let name_b = Employee::get(org, b).map(|e| e.name.as_str()).unwrap_or("?");
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
                    let name = Employee::get(org, id).map(|e| e.name.as_str()).unwrap_or("?");
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

fn print_project_list(org: &OrgChart, ids: &[crate::schema::ProjectId]) {
    if ids.is_empty() {
        println!("  なし");
        return;
    }
    for id in ids {
        let name = Project::get(org, id).map(|p| p.name.as_str()).unwrap_or("?");
        println!("  {} ({})", name, id.0);
    }
}

pub fn print_reorg(org: &OrgChart, report: &ReorgReport) {
    println!("=== 組織改編シミュレーション ===");
    println!(
        "廃止対象部署: {} ({})",
        report.removed_department_name, report.removed_department.0
    );
    println!("再配置対象: {}人\n", report.reassigned.len());

    println!("--- 再配置先 (社員キー順、ラウンドロビン) ---");
    for (emp_id, new_dept) in report.reassigned.iter().take(10) {
        let name = Employee::get(org, emp_id).map(|e| e.name.as_str()).unwrap_or("?");
        let dept_name = Department::get(org, new_dept).map(|d| d.name.as_str()).unwrap_or("?");
        println!("  {} ({}) -> {} ({})", name, emp_id.0, dept_name, new_dept.0);
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
                Employee::ids(new_org).count(),
                Department::ids(new_org).count(),
                Project::ids(new_org).count()
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

/// `main.rs` から使う小ヘルパー: 社員キーが存在するかどうかの案内。
pub fn print_unknown_employee(key: &EmployeeId) {
    eprintln!("エラー: 社員キー '{}' は存在しません", key.0);
}
