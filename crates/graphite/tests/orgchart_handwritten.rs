//! これは `graph_schema!` が生成する想定のコードを手で書いたもの。
//! マクロ実装のテンプレート。
//!
//! **フェーズ5での注記**: フェーズ5でマクロ生成側の違反 enum の形が進化し
//! (項目k: `MultiplicityViolation { employee: EmployeeId, .. }` という単一の
//! 共通バリアントから、エッジ単位の型付きバリアント
//! `{Label}Multiplicity` / `{Label}UnknownSource` / `{Label}UnknownTarget`
//! へ置き換え)、この手書きテンプレートとの間に差異が生まれている。
//! このファイルは歴史的テンプレートとしてそのまま残しているので、
//! 最新の生成コードの形は `crates/graphite/tests/orgchart_macro.rs` 側を
//! 参照すること。
//!
//! 元になるスキーマ (`docs/rust_graph_extension_sketch.md` の「水準2相当」節):
//!
//! ```text
//! graphite::graph_schema! {
//!     schema OrgChart {
//!         node Employee { name: String, id: u32 }
//!         node Department { name: String }
//!
//!         edge belongs_to: Employee -> Department (1);
//!         edge boss:       Employee -> Employee   (0..1) { since: i32 };
//!         edge reports:    Employee -> Employee   (0..*);
//!     }
//! }
//! ```
//!
//! このファイルはそれを一切マクロを使わず手で展開したもの。フェーズ3で
//! `graph_schema!` を実装するときに「展開後の目標コード」として突き合わせる
//! 対象になる。
//!
//! ## 実装方針: パート A の `Graph<N, E, K>` は使わない (独立実装)
//!
//! `graphite::Graph<N, E, K>` は水準1相当 (ノードの型が `N` 1 種類に固定)
//! であり、`OrgChart` は `Employee`/`Department` という 2 種のノード型と
//! `belongs_to`/`boss`/`reports` という 3 種のエッジ型 (それぞれ多重度も
//! 属性の有無も違う) を持つ、水準2相当の**異種混在**スキーマである。
//! `docs/graph_design_sketches.md` の「水準1の限界」節がまさにこの形で
//! 「`Graph<T>` はノードは T のどれかとしか言えず、辺ごとに違う多重度・
//! 端点種別の制約を表現できない」と指摘している通り、1 つの同種
//! `Graph<N, E, K>` インスタンスにこれを押し込もうとすると、ノード型を
//! enum で包む・辺属性型をすべて 1 つの enum に押し込む、といった不自然な
//! 変換が必要になり、かえって `graph_schema!` の展開規則が複雑になる。
//!
//! それよりも「1 スキーマ宣言 = 1 独立 struct、1 エッジ種別 = 1
//! `HashMap` フィールド」という対応の方が、`graph_schema!` マクロが
//! ノード宣言・エッジ宣言をそれぞれ機械的に 1 つの構造体フィールドへ
//! 変換するだけで済み、生成規則がシンプルになる。よってこのテンプレートは
//! パート A のジェネリック `Graph` を使わず、独立実装として書く。
//! (マクロが内部で `Graph<N,E,K>` を使う設計を選ぶ余地は残るが、
//! それは「複数の同種グラフを 1 つの構造体に埋め込む」形になり、
//! 結局このテンプレートと同じ量のフィールド宣言が必要になるため、
//! 独立実装より簡単になるわけではない、というのがここでの判断)。

use std::collections::HashMap;
use std::error::Error as StdError;
use std::fmt;

// ---------------------------------------------------------------------
// ノード種別 (`node Employee { .. }` / `node Department { .. }` に対応)
// ---------------------------------------------------------------------

/// 社員ノードのキー。ユーザーキー方式 (決定1) だが、`Employee` と
/// `Department` を同じ文字列キー空間で扱うと取り違えが起きるため、
/// ノード種別ごとに newtype でキー型を分ける。これも `graph_schema!` が
/// 各 `node` 宣言ごとに機械的に生成できる形。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EmployeeId(pub String);

/// 部署ノードのキー。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DepartmentId(pub String);

/// `node Employee { name: String, id: u32 }` に対応する値。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Employee {
    pub name: String,
    pub id: u32,
}

/// `node Department { name: String }` に対応する値。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Department {
    pub name: String,
}

/// `edge boss: .. (0..1) { since: i32 }` の属性。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BossAttrs {
    pub since: i32,
}

// ---------------------------------------------------------------------
// スキーマ適合エラー
// ---------------------------------------------------------------------

/// `OrgChart::create` の freeze 検査が返しうる違反。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaViolation {
    /// 社員キーが重複した。
    DuplicateEmployee(EmployeeId),
    /// 部署キーが重複した。
    DuplicateDepartment(DepartmentId),
    /// 辺が未知の社員キーを参照している。
    UnknownEmployee(EmployeeId),
    /// 辺が未知の部署キーを参照している。
    UnknownDepartment(DepartmentId),
    /// 多重度違反。`edge` はエッジ種別名、`expected` は期待した多重度の説明。
    MultiplicityViolation {
        edge: &'static str,
        employee: EmployeeId,
        expected: &'static str,
        actual: usize,
    },
}

impl fmt::Display for SchemaViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchemaViolation::DuplicateEmployee(id) => {
                write!(f, "社員キーが重複しています: {id:?}")
            }
            SchemaViolation::DuplicateDepartment(id) => {
                write!(f, "部署キーが重複しています: {id:?}")
            }
            SchemaViolation::UnknownEmployee(id) => {
                write!(f, "未知の社員キーが参照されています: {id:?}")
            }
            SchemaViolation::UnknownDepartment(id) => {
                write!(f, "未知の部署キーが参照されています: {id:?}")
            }
            SchemaViolation::MultiplicityViolation {
                edge,
                employee,
                expected,
                actual,
            } => write!(
                f,
                "多重度違反: エッジ種別 `{edge}` は社員 {employee:?} について多重度 {expected} を期待しますが実際は {actual} 本です"
            ),
        }
    }
}

impl StdError for SchemaViolation {}

// ---------------------------------------------------------------------
// OrgChart 本体 — フィールドは非公開、公開は create と各アクセサのみ
// (スマートコンストラクタ。docs/graph_design_sketches.md 決定5)
// ---------------------------------------------------------------------

/// `schema OrgChart { .. }` に対応する凍結済み図式グラフ。
///
/// 構築後は不変。可変 API は一切公開しない (決定2)。
pub struct OrgChart {
    employees: HashMap<EmployeeId, Employee>,
    departments: HashMap<DepartmentId, Department>,
    /// 多重度 (1): 社員 → ちょうど1つの部署
    belongs_to: HashMap<EmployeeId, DepartmentId>,
    /// 多重度 (0..1): 社員 → 0または1人の上司 + 属性
    boss: HashMap<EmployeeId, (EmployeeId, BossAttrs)>,
    /// 多重度 (0..*): 社員(上司側) → 0人以上の直属の部下
    reports: HashMap<EmployeeId, Vec<EmployeeId>>,
}

impl OrgChart {
    /// builder をクロージャに貸し出し、戻ったら凍結して図式適合
    /// (端点種別・多重度) を一括検査する。
    ///
    /// `for<'b> FnOnce(&'b mut OrgChartBuilder)` により、builder への
    /// 参照をクロージャの外に持ち出すことは借用検査器が静的に拒否する
    /// (`docs/rust_graph_extension_sketch.md` の builder→freeze節、
    /// `std::thread::scope` と同型の保証)。
    pub fn create<F>(f: F) -> Result<Self, SchemaViolation>
    where
        F: for<'b> FnOnce(&'b mut OrgChartBuilder),
    {
        let mut builder = OrgChartBuilder::new();
        f(&mut builder);
        builder.freeze()
    }

    /// 社員ノードを引く。
    pub fn employee(&self, id: &EmployeeId) -> Option<&Employee> {
        self.employees.get(id)
    }

    /// 部署ノードを引く。
    pub fn department(&self, id: &DepartmentId) -> Option<&Department> {
        self.departments.get(id)
    }

    /// `belongs_to` — 多重度 (1) → 戻り値は `&Department` そのもの。
    ///
    /// freeze 検査 (`OrgChartBuilder::freeze`) で「生存する全社員は
    /// ちょうど1つの部署に所属する」ことを保証済みなので、呼び出し側に
    /// `Option`/`Result` で「無いかもしれない」を見せる必要が無い —
    /// これが多重度→戻り型写像の核心 (決定2c)。
    ///
    /// # Panics
    /// `id` がこの `OrgChart` に存在しない社員キーの場合パニックする。
    /// これは入力検証の欠如ではなく呼び出し規約 (この `OrgChart` から
    /// 得たキーだけを渡す) の違反であり、`petgraph::graph::NodeIndex` や
    /// `slotmap` のキーと同じ考え方 (キーを外部に持ち出しても意味は
    /// 保証されない)。
    pub fn belongs_to(&self, id: &EmployeeId) -> &Department {
        let dept_id = self.belongs_to.get(id).unwrap_or_else(|| {
            panic!("belongs_to: 未知の EmployeeId です (このOrgChartが発行したキーではありません): {id:?}")
        });
        &self.departments[dept_id]
    }

    /// `boss` — 多重度 (0..1) → 戻り値は `Option<(&Employee, &BossAttrs)>`。
    /// 未知のキーを渡した場合も `None` を返す (上司なし社員と区別しない
    /// ―― こちらは「無い」ことが正常なドメイン状態なので、パニックさせない)。
    pub fn boss(&self, id: &EmployeeId) -> Option<(&Employee, &BossAttrs)> {
        let (boss_id, attrs) = self.boss.get(id)?;
        Some((&self.employees[boss_id], attrs))
    }

    /// `reports` — 多重度 (0..*) → 戻り値は `Vec<&Employee>`。
    /// 部下がいない・未知キーのどちらも空 `Vec` に落ちる (0..* は
    /// 「無い」ことがそもそも制約の内側なので区別を持たない)。
    pub fn reports(&self, id: &EmployeeId) -> Vec<&Employee> {
        match self.reports.get(id) {
            Some(ids) => ids.iter().map(|rid| &self.employees[rid]).collect(),
            None => Vec::new(),
        }
    }

    /// 導出エッジの例: 同じ部署の他の社員一覧。
    /// 保存されない (フィールドを持たない) 計算結果であり、使う側からは
    /// 保存エッジの `reports`/`boss` と区別が付かない普通のメソッド
    /// (決定2d: 保存エッジ=フィールド、導出エッジ=getter)。
    pub fn colleagues(&self, id: &EmployeeId) -> Vec<&Employee> {
        let Some(dept_id) = self.belongs_to.get(id) else {
            return Vec::new();
        };
        self.employees
            .keys()
            .filter(|other| *other != id && self.belongs_to.get(*other) == Some(dept_id))
            .map(|other| &self.employees[other])
            .collect()
    }
}

// ---------------------------------------------------------------------
// Builder — 構築中は多重度検査を一切行わない (決定4: freezeで一括検査)
// ---------------------------------------------------------------------

/// [`OrgChart::create`] に貸し出される構築用 builder。
pub struct OrgChartBuilder {
    employees: Vec<(EmployeeId, Employee)>,
    departments: Vec<(DepartmentId, Department)>,
    belongs_to: Vec<(EmployeeId, DepartmentId)>,
    boss: Vec<(EmployeeId, EmployeeId, BossAttrs)>,
    reports: Vec<(EmployeeId, EmployeeId)>,
}

impl OrgChartBuilder {
    fn new() -> Self {
        Self {
            employees: Vec::new(),
            departments: Vec::new(),
            belongs_to: Vec::new(),
            boss: Vec::new(),
            reports: Vec::new(),
        }
    }

    /// 社員ノードを積む。
    pub fn employee(&mut self, id: EmployeeId, value: Employee) -> &mut Self {
        self.employees.push((id, value));
        self
    }

    /// 部署ノードを積む。
    pub fn department(&mut self, id: DepartmentId, value: Department) -> &mut Self {
        self.departments.push((id, value));
        self
    }

    /// `belongs_to` 辺を積む (多重度 (1) は freeze で検査)。
    pub fn belongs_to(&mut self, employee: EmployeeId, department: DepartmentId) -> &mut Self {
        self.belongs_to.push((employee, department));
        self
    }

    /// `boss` 辺を積む (多重度 (0..1) は freeze で検査)。
    pub fn boss(&mut self, employee: EmployeeId, boss: EmployeeId, attrs: BossAttrs) -> &mut Self {
        self.boss.push((employee, boss, attrs));
        self
    }

    /// `reports` 辺を積む (多重度 (0..*) なので freeze では端点検査のみ)。
    pub fn reports(&mut self, manager: EmployeeId, report: EmployeeId) -> &mut Self {
        self.reports.push((manager, report));
        self
    }

    /// 凍結: ノード・辺の一括検証を行い `OrgChart` を返す。
    fn freeze(self) -> Result<OrgChart, SchemaViolation> {
        let mut employees: HashMap<EmployeeId, Employee> = HashMap::new();
        for (id, value) in self.employees {
            if employees.contains_key(&id) {
                return Err(SchemaViolation::DuplicateEmployee(id));
            }
            employees.insert(id, value);
        }

        let mut departments: HashMap<DepartmentId, Department> = HashMap::new();
        for (id, value) in self.departments {
            if departments.contains_key(&id) {
                return Err(SchemaViolation::DuplicateDepartment(id));
            }
            departments.insert(id, value);
        }

        // belongs_to: 端点検査 + 多重度(1) = 「生存する全社員がちょうど1本」
        let mut belongs_to: HashMap<EmployeeId, DepartmentId> = HashMap::new();
        let mut belongs_to_count: HashMap<EmployeeId, usize> = HashMap::new();
        for (emp, dept) in self.belongs_to {
            if !employees.contains_key(&emp) {
                return Err(SchemaViolation::UnknownEmployee(emp));
            }
            if !departments.contains_key(&dept) {
                return Err(SchemaViolation::UnknownDepartment(dept));
            }
            *belongs_to_count.entry(emp.clone()).or_insert(0) += 1;
            belongs_to.insert(emp, dept);
        }
        for emp in employees.keys() {
            let count = belongs_to_count.get(emp).copied().unwrap_or(0);
            if count != 1 {
                return Err(SchemaViolation::MultiplicityViolation {
                    edge: "belongs_to",
                    employee: emp.clone(),
                    expected: "ちょうど1",
                    actual: count,
                });
            }
        }

        // boss: 端点検査 + 多重度(0..1) = 「高々1本」
        let mut boss: HashMap<EmployeeId, (EmployeeId, BossAttrs)> = HashMap::new();
        let mut boss_count: HashMap<EmployeeId, usize> = HashMap::new();
        for (emp, boss_id, attrs) in self.boss {
            if !employees.contains_key(&emp) {
                return Err(SchemaViolation::UnknownEmployee(emp));
            }
            if !employees.contains_key(&boss_id) {
                return Err(SchemaViolation::UnknownEmployee(boss_id));
            }
            *boss_count.entry(emp.clone()).or_insert(0) += 1;
            boss.insert(emp, (boss_id, attrs));
        }
        for (emp, count) in &boss_count {
            if *count > 1 {
                return Err(SchemaViolation::MultiplicityViolation {
                    edge: "boss",
                    employee: emp.clone(),
                    expected: "0または1",
                    actual: *count,
                });
            }
        }

        // reports: 多重度(0..*) は常に合法。端点検査のみ行う。
        let mut reports: HashMap<EmployeeId, Vec<EmployeeId>> = HashMap::new();
        for (manager, report) in self.reports {
            if !employees.contains_key(&manager) {
                return Err(SchemaViolation::UnknownEmployee(manager));
            }
            if !employees.contains_key(&report) {
                return Err(SchemaViolation::UnknownEmployee(report));
            }
            reports.entry(manager).or_default().push(report);
        }

        Ok(OrgChart {
            employees,
            departments,
            belongs_to,
            boss,
            reports,
        })
    }
}

// =======================================================================
// テスト
// =======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn emp(id: &str) -> EmployeeId {
        EmployeeId(id.to_string())
    }

    fn dept(id: &str) -> DepartmentId {
        DepartmentId(id.to_string())
    }

    /// 正常な組織図: 田中(営業部, 上司なし) - 佐藤(営業部, 上司=田中)
    fn build_healthy_chart() -> OrgChart {
        OrgChart::create(|b| {
            b.employee(
                emp("田中"),
                Employee {
                    name: "田中".to_string(),
                    id: 1,
                },
            );
            b.employee(
                emp("佐藤"),
                Employee {
                    name: "佐藤".to_string(),
                    id: 2,
                },
            );
            b.department(
                dept("営業部"),
                Department {
                    name: "営業".to_string(),
                },
            );

            b.belongs_to(emp("田中"), dept("営業部"));
            b.belongs_to(emp("佐藤"), dept("営業部"));
            b.boss(emp("佐藤"), emp("田中"), BossAttrs { since: 2020 });
            b.reports(emp("田中"), emp("佐藤"));
        })
        .expect("正常な組織図は構築に成功するはず")
    }

    #[test]
    fn 正常構築できる() {
        let g = build_healthy_chart();
        assert_eq!(g.employee(&emp("田中")).unwrap().name, "田中");
        assert_eq!(g.department(&dept("営業部")).unwrap().name, "営業");
    }

    #[test]
    fn belongs_to_は参照そのものを返す() {
        let g = build_healthy_chart();
        // 多重度 (1) → &Department。Option/Result で包まれていないことを
        // 型注釈で明示的に確認する。
        let d: &Department = g.belongs_to(&emp("田中"));
        assert_eq!(d.name, "営業");
    }

    #[test]
    fn boss_は_option_を返す() {
        let g = build_healthy_chart();

        // 多重度 (0..1) → Option<(&Employee, &BossAttrs)>
        let b: Option<(&Employee, &BossAttrs)> = g.boss(&emp("佐藤"));
        let (boss_emp, attrs) = b.expect("佐藤の上司は田中のはず");
        assert_eq!(boss_emp.name, "田中");
        assert_eq!(attrs.since, 2020);

        // 上司なしの社員は None
        let no_boss: Option<(&Employee, &BossAttrs)> = g.boss(&emp("田中"));
        assert!(no_boss.is_none());
    }

    #[test]
    fn reports_は_vec_を返す() {
        let g = build_healthy_chart();

        // 多重度 (0..*) → Vec<&Employee>
        let subordinates: Vec<&Employee> = g.reports(&emp("田中"));
        assert_eq!(subordinates.len(), 1);
        assert_eq!(subordinates[0].name, "佐藤");

        // 部下がいない社員は空 Vec
        let none: Vec<&Employee> = g.reports(&emp("佐藤"));
        assert!(none.is_empty());
    }

    #[test]
    fn belongs_toが0本の社員は多重度違反になる() {
        let result = OrgChart::create(|b| {
            b.employee(
                emp("鈴木"),
                Employee {
                    name: "鈴木".to_string(),
                    id: 3,
                },
            );
            b.department(
                dept("営業部"),
                Department {
                    name: "営業".to_string(),
                },
            );
            // 鈴木を意図的にどの部署にも所属させない
        });

        match result {
            Err(violation) => assert_eq!(
                violation,
                SchemaViolation::MultiplicityViolation {
                    edge: "belongs_to",
                    employee: emp("鈴木"),
                    expected: "ちょうど1",
                    actual: 0,
                }
            ),
            Ok(_) => panic!("多重度違反が検出されるはず"),
        }
    }

    #[test]
    fn bossが2本ある社員は多重度違反になる() {
        let result = OrgChart::create(|b| {
            b.employee(
                emp("田中"),
                Employee {
                    name: "田中".to_string(),
                    id: 1,
                },
            );
            b.employee(
                emp("佐藤"),
                Employee {
                    name: "佐藤".to_string(),
                    id: 2,
                },
            );
            b.employee(
                emp("鈴木"),
                Employee {
                    name: "鈴木".to_string(),
                    id: 3,
                },
            );
            b.department(
                dept("営業部"),
                Department {
                    name: "営業".to_string(),
                },
            );
            b.belongs_to(emp("田中"), dept("営業部"));
            b.belongs_to(emp("佐藤"), dept("営業部"));
            b.belongs_to(emp("鈴木"), dept("営業部"));
            // 田中に上司を2人つける (多重度 0..1 違反)
            b.boss(emp("田中"), emp("佐藤"), BossAttrs { since: 2018 });
            b.boss(emp("田中"), emp("鈴木"), BossAttrs { since: 2019 });
        });

        assert!(matches!(
            result,
            Err(SchemaViolation::MultiplicityViolation { edge: "boss", .. })
        ));
    }

    #[test]
    fn 未知の部署への所属はエラーになる() {
        let result = OrgChart::create(|b| {
            b.employee(
                emp("田中"),
                Employee {
                    name: "田中".to_string(),
                    id: 1,
                },
            );
            b.belongs_to(emp("田中"), dept("存在しない部署"));
        });

        match result {
            Err(violation) => assert_eq!(
                violation,
                SchemaViolation::UnknownDepartment(dept("存在しない部署"))
            ),
            Ok(_) => panic!("未知の部署参照はエラーになるはず"),
        }
    }

    #[test]
    fn colleagues_は同じ部署の他の社員を返す() {
        let g = OrgChart::create(|b| {
            b.employee(
                emp("田中"),
                Employee {
                    name: "田中".to_string(),
                    id: 1,
                },
            );
            b.employee(
                emp("佐藤"),
                Employee {
                    name: "佐藤".to_string(),
                    id: 2,
                },
            );
            b.employee(
                emp("鈴木"),
                Employee {
                    name: "鈴木".to_string(),
                    id: 3,
                },
            );
            b.department(
                dept("営業部"),
                Department {
                    name: "営業".to_string(),
                },
            );
            b.department(
                dept("開発部"),
                Department {
                    name: "開発".to_string(),
                },
            );
            b.belongs_to(emp("田中"), dept("営業部"));
            b.belongs_to(emp("佐藤"), dept("営業部"));
            b.belongs_to(emp("鈴木"), dept("開発部"));
        })
        .unwrap();

        let mut colleagues: Vec<String> = g
            .colleagues(&emp("田中"))
            .into_iter()
            .map(|e| e.name.clone())
            .collect();
        colleagues.sort();
        assert_eq!(colleagues, vec!["佐藤".to_string()]);

        // 開発部の鈴木は営業部の田中の同僚ではない (念のため)
        assert!(!colleagues.contains(&"鈴木".to_string()));
    }
}
