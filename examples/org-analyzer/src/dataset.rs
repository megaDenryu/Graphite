//! 決定的な合成組織データ生成器。
//!
//! 外部の乱数クレートに頼らず、線形合同法 (LCG) による自前の擬似乱数を使う。
//! 同じ `seed` を渡せば常に同じ組織データが再現される (テスト・デモの再現性の
//! ため)。`inject_anomalies` を立てると、構造異常検出コマンド (`anomalies`)
//! が拾うべき既知の異常を意図的に埋め込む。
//!
//! 注意: `generate` は1つのシードから1つの組織データを合成しきる1本の流れで
//! あり、コード行で100行を超える。途中で切ると、社員一覧・部署割当・4種の辺の
//! 下書きという「生成途中の中間状態」を外へ晒す断片になる。そのため語彙表
//! (`name_pool`)・擬似乱数 (`lcg`)・分布 (`distribution`)・キーの綴り
//! (`element_id`)・結果の型 (`generated_org`)・異常の注入
//! (`anomaly_injection`) を分けたうえで、生成の流れ自体は1つに統合している。

mod anomaly_injection;
mod distribution;
mod element_id;
mod generated_org;
mod lcg;
mod name_pool;

use distribution::{weighted_assignment_count, weighted_grade};
use element_id::{department_id, employee_id, project_id};
use lcg::Lcg;
use name_pool::{DEPARTMENT_NAMES, GIVEN_NAMES, PROJECT_NAMES, ROLES, SURNAMES, TITLES_BY_GRADE};

pub use generated_org::{AnomalyPlan, GeneratedOrg};

use crate::schema::{
    Assigned, AssignedEdge, BelongsTo, Boss, BossEdge, Department, DepartmentId, Employee,
    EmployeeId, OrgChart, Project, ProjectId, Sponsors,
};

pub const EMPLOYEE_COUNT: usize = 120; // 社員数。
pub const DEPARTMENT_COUNT: usize = 8; // 部署数 (`DEPARTMENT_NAMES` の要素数と一致させる)。
pub const PROJECT_COUNT: usize = 15; // プロジェクト数 (`PROJECT_NAMES` の要素数と一致させる)。

// 管理職とみなす最低 grade (係長相当以上)。`analysis/span_of_control.rs` からも参照する。
pub const MANAGER_GRADE_THRESHOLD: u8 = 3;

// シードから組織データを合成する。
//
// `inject_anomalies` が `true` の場合、以下を強制的に埋め込む
// (`AnomalyPlan` に記録して返す。`anomalies` コマンドの検出結果とテストで
// 突き合わせる):
//
// 1. 社員 E001/E002 を相互上司 (お互いがお互いの boss) にする
// 2. 社員 E003→E004→E005→E003 の上司循環 (3人) を作る
// 3. プロジェクト P01 をどの部署からもスポンサーされない状態にする
// 4. プロジェクト P02 に誰もアサインされない状態にする
pub fn generate(seed: u64, inject_anomalies: bool) -> GeneratedOrg {
    let mut rng = Lcg::new(seed);

    // --- ノード生成 -------------------------------------------------
    let departments: Vec<(DepartmentId, Department)> = (0..DEPARTMENT_COUNT)
        .map(|i| {
            (
                department_id(i),
                Department {
                    name: DEPARTMENT_NAMES[i].to_string(),
                },
            )
        })
        .collect();

    let projects: Vec<(ProjectId, Project)> = (0..PROJECT_COUNT)
        .map(|i| {
            (
                project_id(i),
                Project {
                    name: PROJECT_NAMES[i].to_string(),
                    priority: rng.next_range_inclusive(1, 5) as u8,
                },
            )
        })
        .collect();

    // 社員ごとの部署所属 (インデックス) を先に確定させ、後段のボス階層生成で
    // 部署内グルーピングに使う。
    let mut employees: Vec<(EmployeeId, Employee)> = Vec::with_capacity(EMPLOYEE_COUNT);
    let mut dept_of_employee: Vec<usize> = Vec::with_capacity(EMPLOYEE_COUNT);

    for i in 0..EMPLOYEE_COUNT {
        let surname = SURNAMES[rng.next_range(SURNAMES.len())];
        let given = GIVEN_NAMES[rng.next_range(GIVEN_NAMES.len())];
        let grade = weighted_grade(&mut rng);
        let dept_idx = rng.next_range(DEPARTMENT_COUNT);

        employees.push((
            employee_id(i),
            Employee {
                name: format!("{surname}{given}"),
                title: TITLES_BY_GRADE[(grade - 1) as usize].to_string(),
                grade,
            },
        ));
        dept_of_employee.push(dept_idx);
    }

    // --- belongs_to 辺 (全社員ちょうど1本) ---------------------------
    let belongs_to_edges: Vec<(EmployeeId, DepartmentId)> = (0..EMPLOYEE_COUNT)
        .map(|i| (employee_id(i), department_id(dept_of_employee[i])))
        .collect();

    // --- boss 辺: 部署内で grade の高い人を上司候補としてランダムに選ぶ。
    // 「自分より厳密に grade が高い人だけを候補にする」ため、部署ごとに見ると
    // 森 (forest) 構造になり、通常運転では循環も相互上司も原理的に発生しない。
    let mut boss_edges: Vec<(EmployeeId, EmployeeId, BossEdge)> = Vec::new();
    for dept_idx in 0..DEPARTMENT_COUNT {
        let members: Vec<usize> = (0..EMPLOYEE_COUNT)
            .filter(|&i| dept_of_employee[i] == dept_idx)
            .collect();

        for &i in &members {
            let my_grade = employees[i].1.grade;
            let candidates: Vec<usize> = members
                .iter()
                .copied()
                .filter(|&j| employees[j].1.grade > my_grade)
                .collect();
            if candidates.is_empty() {
                continue; // この部署でのトップ層 (上司なし)
            }
            let chosen = candidates[rng.next_range(candidates.len())];
            let since = rng.next_range_inclusive(2014, 2023) as i32;
            boss_edges.push((employee_id(i), employee_id(chosen), BossEdge { since }));
        }
    }

    // --- assigned 辺 (社員 -> プロジェクト、0〜3件の兼務) --------------
    let mut assigned_edges: Vec<(EmployeeId, ProjectId, AssignedEdge)> = Vec::new();
    let mut seen_assignment: std::collections::HashSet<(usize, usize)> =
        std::collections::HashSet::new();
    for i in 0..EMPLOYEE_COUNT {
        let count = weighted_assignment_count(&mut rng);
        for _ in 0..count {
            let proj_idx = rng.next_range(PROJECT_COUNT);
            if !seen_assignment.insert((i, proj_idx)) {
                continue; // 同じプロジェクトへの重複アサインは避ける
            }
            let role = ROLES[rng.next_range(ROLES.len())].to_string();
            assigned_edges.push((employee_id(i), project_id(proj_idx), AssignedEdge { role }));
        }
    }

    // --- sponsors 辺 (部署 -> プロジェクト、部署ごとに高々1件) ---------
    let mut sponsors_edges: Vec<(DepartmentId, ProjectId)> = Vec::new();
    for dept_idx in 0..DEPARTMENT_COUNT {
        if rng.chance(6, 10) {
            let proj_idx = rng.next_range(PROJECT_COUNT);
            sponsors_edges.push((department_id(dept_idx), project_id(proj_idx)));
        }
    }

    // --- 異常注入 (--inject-anomalies) --------------------------------
    let anomaly_plan = if inject_anomalies {
        Some(anomaly_injection::inject_anomalies(
            &mut boss_edges,
            &mut assigned_edges,
            &mut sponsors_edges,
        ))
    } else {
        None
    };

    // --- 構築 -----------------------------------------------------
    // ここまでの生成ロジックは各多重度制約 (belongs_to はちょうど1、boss/
    // sponsors は高々1) を常に満たすように組んでいるので、合成データの
    // 構築自体が失敗することはない想定 (失敗したら生成ロジックのバグ)。
    //
    // 構築コード自体は統一 `extend` (`docs/bulk_construction.md`、
    // `docs/graph_splice.md` §2) に集約し、for ループは上記の「データを
    // 生成する」部分だけに残す。ノード用・エッジ用の呼び分けは無く、値の型
    // (`{Schema}Insertable` を満たすか) から rustc が振り分ける。
    let chart = OrgChart::Graph::create(|b| {
        b.extend(employees.into_iter().map(|(id, e)| (id.0, e)));
        b.extend(departments.into_iter().map(|(id, d)| (id.0, d)));
        b.extend(projects.into_iter().map(|(id, p)| (id.0, p)));
        b.extend(
            belongs_to_edges
                .into_iter()
                .map(|(e, d)| (format!("bt_{}", e.0), BelongsTo::new(e, d))),
        );
        b.extend(
            boss_edges
                .into_iter()
                .map(|(from, to, attrs)| (format!("boss_{}", from.0), Boss::new(from, to, attrs))),
        );
        b.extend(
            assigned_edges
                .into_iter()
                .map(|(e, p, attrs)| (format!("asn_{}_{}", e.0, p.0), Assigned::new(e, p, attrs))),
        );
        b.extend(
            sponsors_edges
                .into_iter()
                .map(|(d, p)| (format!("spon_{}", d.0), Sponsors::new(d, p))),
        );
    })
    .expect("合成データ生成器は常に多重度制約を満たすよう組んでいるはず");

    GeneratedOrg {
        chart,
        anomaly_plan,
    }
}
