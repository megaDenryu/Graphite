//! 決定的な合成組織データ生成器。
//!
//! 外部の乱数クレートに頼らず、線形合同法 (LCG) による自前の擬似乱数を使う。
//! 同じ `seed` を渡せば常に同じ組織データが再現される (テスト・デモの再現性の
//! ため)。`inject_anomalies` を立てると、構造異常検出コマンド (`anomalies`)
//! が拾うべき既知の異常を意図的に埋め込む。

use crate::schema::{
    Assigned, AssignedEdge, BelongsTo, Boss, BossEdge, Department, DepartmentId, Employee,
    EmployeeId, OrgChart, Project, ProjectId, Sponsors,
};

/// 社員数。
pub const EMPLOYEE_COUNT: usize = 120;
/// 部署数 (`DEPARTMENT_NAMES` の要素数と一致させる)。
pub const DEPARTMENT_COUNT: usize = 8;
/// プロジェクト数 (`PROJECT_NAMES` の要素数と一致させる)。
pub const PROJECT_COUNT: usize = 15;

/// 管理職とみなす最低 grade (係長相当以上)。`analysis.rs` からも参照する。
pub const MANAGER_GRADE_THRESHOLD: u8 = 3;

const SURNAMES: &[&str] = &[
    "佐藤", "鈴木", "高橋", "田中", "伊藤", "渡辺", "山本", "中村", "小林", "加藤", "吉田", "山田",
    "佐々木", "山口", "松本", "井上", "木村", "林", "斎藤", "清水", "山崎", "森", "池田", "橋本",
    "阿部", "石川", "前田", "藤田", "後藤", "岡田",
];

const GIVEN_NAMES: &[&str] = &[
    "翔太", "陽菜", "大輝", "結衣", "健太", "美咲", "拓也", "彩", "亮", "真央", "悠斗", "沙織",
    "直樹", "花子", "健一", "誠", "由美", "浩二", "麻衣", "隆", "恵子", "淳", "千尋", "康平",
    "夏美", "雄大", "里奈", "俊介", "和也", "泰輔",
];

/// 8 要素固定 (`DEPARTMENT_COUNT` と一致)。
const DEPARTMENT_NAMES: [&str; DEPARTMENT_COUNT] = [
    "営業部",
    "開発部",
    "人事部",
    "経理部",
    "マーケティング部",
    "総務部",
    "法務部",
    "カスタマーサポート部",
];

/// 15 要素固定 (`PROJECT_COUNT` と一致)。
const PROJECT_NAMES: [&str; PROJECT_COUNT] = [
    "次世代基幹システム刷新",
    "モバイルアプリリニューアル",
    "顧客管理システム移行",
    "海外市場拡販",
    "新卒採用強化",
    "経費精算自動化",
    "ブランド刷新キャンペーン",
    "オフィス移転",
    "コンプライアンス体制整備",
    "サポート窓口AI化",
    "サプライチェーン最適化",
    "社内データ基盤構築",
    "新製品ローンチ",
    "働き方改革推進",
    "セキュリティ監査対応",
];

const TITLES_BY_GRADE: [&str; 5] = ["一般社員", "主任", "係長", "課長", "部長"];

const ROLES: &[&str] = &[
    "開発", "設計", "PM", "QA", "要件定義", "運用", "企画", "デザイン", "調整", "レビュー",
];

/// Numerical Recipes 系の定数を使った線形合同法 (LCG)。
/// `state_{n+1} = state_n * A + C (mod 2^64)`。外部乱数クレート禁止という
/// 制約のもとで「同じ seed なら同じ組織になる」再現性だけを目的にした最小実装
/// であり、暗号用途などの品質は求めていない。
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        // seed=0 だと初期状態が単調になりやすいので撹拌しておく。
        Self {
            state: seed ^ 0x9E37_79B9_7F4A_7C15,
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        self.state
    }

    /// `[0, n)` の一様乱数。上位ビットを使うことで LCG 下位ビットの周期性の
    /// 影響を避ける。
    fn next_range(&mut self, n: usize) -> usize {
        debug_assert!(n > 0);
        ((self.next_u64() >> 33) % n as u64) as usize
    }

    /// `[lo, hi]` (両端含む) の一様乱数。
    fn next_range_inclusive(&mut self, lo: i64, hi: i64) -> i64 {
        lo + self.next_range((hi - lo + 1) as usize) as i64
    }

    /// `numerator / denominator` の確率で `true`。
    fn chance(&mut self, numerator: usize, denominator: usize) -> bool {
        self.next_range(denominator) < numerator
    }
}

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
    pub chart: OrgChart,
    /// `inject_anomalies` が有効なときだけ `Some`。
    pub anomaly_plan: Option<AnomalyPlan>,
}

fn employee_id(index: usize) -> EmployeeId {
    EmployeeId(format!("E{:03}", index + 1))
}

fn department_id(index: usize) -> DepartmentId {
    DepartmentId(format!("D{:02}", index + 1))
}

fn project_id(index: usize) -> ProjectId {
    ProjectId(format!("P{:02}", index + 1))
}

/// grade 分布 (1〜5)。現場の人数が多いピラミッド型組織を模す。
fn weighted_grade(rng: &mut Lcg) -> u8 {
    let roll = rng.next_range(100);
    match roll {
        0..=39 => 1,
        40..=64 => 2,
        65..=84 => 3,
        85..=94 => 4,
        _ => 5,
    }
}

/// 1人あたりの兼務プロジェクト数 (0〜3)。
fn weighted_assignment_count(rng: &mut Lcg) -> usize {
    let roll = rng.next_range(100);
    match roll {
        0..=29 => 0,
        30..=69 => 1,
        70..=89 => 2,
        _ => 3,
    }
}

/// シードから組織データを合成する。
///
/// `inject_anomalies` が `true` の場合、以下を強制的に埋め込む
/// (`AnomalyPlan` に記録して返す。`anomalies` コマンドの検出結果とテストで
/// 突き合わせる):
///
/// 1. 社員 E001/E002 を相互上司 (お互いがお互いの boss) にする
/// 2. 社員 E003→E004→E005→E003 の上司循環 (3人) を作る
/// 3. プロジェクト P01 をどの部署からもスポンサーされない状態にする
/// 4. プロジェクト P02 に誰もアサインされない状態にする
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
        // 1. 相互上司ペア: E001 <-> E002 (両者の既存 boss 辺を上書き)
        let mutual_a = employee_id(0);
        let mutual_b = employee_id(1);
        boss_edges.retain(|(from, _, _)| *from != mutual_a && *from != mutual_b);
        boss_edges.push((mutual_a.clone(), mutual_b.clone(), BossEdge { since: 2021 }));
        boss_edges.push((mutual_b.clone(), mutual_a.clone(), BossEdge { since: 2020 }));

        // 2. 上司循環: E003 -> E004 -> E005 -> E003
        let cycle: Vec<EmployeeId> = vec![employee_id(2), employee_id(3), employee_id(4)];
        boss_edges.retain(|(from, _, _)| !cycle.contains(from));
        for k in 0..cycle.len() {
            let next = cycle[(k + 1) % cycle.len()].clone();
            boss_edges.push((cycle[k].clone(), next, BossEdge { since: 2019 + k as i32 }));
        }

        // 3. スポンサー無しプロジェクト強制: P01 を指す sponsors 辺を全て除去
        let sponsorless_project = project_id(0);
        sponsors_edges.retain(|(_, p)| *p != sponsorless_project);

        // 4. 無人プロジェクト強制: P02 を指す assigned 辺を全て除去
        let unstaffed_project = project_id(1);
        assigned_edges.retain(|(_, p, _)| *p != unstaffed_project);

        Some(AnomalyPlan {
            mutual_pair: (mutual_a, mutual_b),
            cycle,
            sponsorless_project,
            unstaffed_project,
        })
    } else {
        None
    };

    // --- 構築 -----------------------------------------------------
    // ここまでの生成ロジックは各多重度制約 (belongs_to はちょうど1、boss/
    // sponsors は高々1) を常に満たすように組んでいるので、合成データの
    // 構築自体が失敗することはない想定 (失敗したら生成ロジックのバグ)。
    //
    // 構築コード自体は `extend_nodes`/`extend_edges` (`docs/bulk_construction.md`)
    // に集約し、for ループは上記の「データを生成する」部分だけに残す。
    let chart = OrgChart::create(|b| {
        b.extend_nodes(employees.into_iter().map(|(id, e)| (id.0, e)));
        b.extend_nodes(departments.into_iter().map(|(id, d)| (id.0, d)));
        b.extend_nodes(projects.into_iter().map(|(id, p)| (id.0, p)));
        b.extend_edges(
            belongs_to_edges
                .into_iter()
                .map(|(e, d)| (format!("bt_{}", e.0), BelongsTo(e, d))),
        );
        b.extend_edges(
            boss_edges
                .into_iter()
                .map(|(from, to, attrs)| (format!("boss_{}", from.0), Boss(from, to, attrs))),
        );
        b.extend_edges(
            assigned_edges
                .into_iter()
                .map(|(e, p, attrs)| (format!("asn_{}_{}", e.0, p.0), Assigned(e, p, attrs))),
        );
        b.extend_edges(
            sponsors_edges
                .into_iter()
                .map(|(d, p)| (format!("spon_{}", d.0), Sponsors(d, p))),
        );
    })
    .expect("合成データ生成器は常に多重度制約を満たすよう組んでいるはず");

    GeneratedOrg {
        chart,
        anomaly_plan,
    }
}
