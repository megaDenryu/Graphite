//! `graph_schema!` で `OrgChart` を宣言し直し、`orgchart_handwritten.rs`
//! (フェーズ2の手書きテンプレート) と同等のテストを実行する。
//!
//! 手書き版との既知の差異 (README.md「手書きテンプレートとの差異」節も参照):
//! - 違反 enum 名は `OrgChartViolation` (手書き版は共通の `SchemaViolation`)。
//! - `MultiplicityViolation` の違反キーは `source: String` (`Debug` 表現)。
//!   手書き版は `employee: EmployeeId` と型付きだったが、エッジごとに始点
//!   ノード型が異なりうる一般のスキーマに対応するため型を固定できない。
//! - builder のエッジ追加メソッドの引数名は汎用的に `from`/`to`
//!   (手書き版は `employee`/`boss`、`manager`/`report` のようにドメイン語)。
//! - 導出エッジ (`colleagues`) はマクロが生成しない。生成された `OrgChart`
//!   に同一モジュール内で `impl` を追記すれば手書き版と同じことができる
//!   (下記参照。私有フィールドへのアクセスは同一モジュールツリー内なら可能、
//!   という通常の Rust 可視性規則をそのまま使っている)。

#[rustfmt::skip]
graphite::graph_schema! {
    schema OrgChart {
        node Employee { name: String, id: u32 }
        node Department { name: String }

        edge belongs_to: Employee -> Department (1);
        edge boss:       Employee -> Employee   (0..1) { since: i32 };
        edge reports:    Employee -> Employee   (0..*);
    }
}

/// 導出エッジの例: `graph_schema!` が生成した `OrgChart` へ、保存されない
/// 計算結果を返す普通のメソッドを追記できることを示す
/// (`docs/graph_design_sketches.md` 決定「保存エッジ=フィールド、
/// 導出エッジ=getter」)。
impl OrgChart {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn emp(id: &str) -> EmployeeId {
        EmployeeId(id.to_string())
    }

    fn dept(id: &str) -> DepartmentId {
        DepartmentId(id.to_string())
    }

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
        let d: &Department = g.belongs_to(&emp("田中"));
        assert_eq!(d.name, "営業");
    }

    #[test]
    fn boss_は_option_を返す() {
        let g = build_healthy_chart();

        let b: Option<(&Employee, &BossAttrs)> = g.boss(&emp("佐藤"));
        let (boss_emp, attrs) = b.expect("佐藤の上司は田中のはず");
        assert_eq!(boss_emp.name, "田中");
        assert_eq!(attrs.since, 2020);

        let no_boss: Option<(&Employee, &BossAttrs)> = g.boss(&emp("田中"));
        assert!(no_boss.is_none());
    }

    #[test]
    fn reports_は_vec_を返す() {
        let g = build_healthy_chart();

        let subordinates: Vec<&Employee> = g.reports(&emp("田中"));
        assert_eq!(subordinates.len(), 1);
        assert_eq!(subordinates[0].name, "佐藤");

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
                OrgChartViolation::MultiplicityViolation {
                    edge: "belongs_to",
                    source: format!("{:?}", emp("鈴木")),
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
            Err(OrgChartViolation::MultiplicityViolation { edge: "boss", .. })
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
                OrgChartViolation::UnknownDepartment(dept("存在しない部署"))
            ),
            Ok(_) => panic!("未知の部署参照はエラーになるはず"),
        }
    }

    #[test]
    fn try_belongs_toは未知キーでnoneを返す() {
        let g = build_healthy_chart();
        assert!(g.try_belongs_to(&emp("存在しない社員")).is_none());
        let d = g
            .try_belongs_to(&emp("田中"))
            .expect("田中は営業部に所属しているはず");
        assert_eq!(d.name, "営業");
    }

    #[test]
    fn employee_idsで全キーを列挙できる() {
        let g = build_healthy_chart();
        let mut ids: Vec<String> = g.employee_ids().map(|id| id.0.clone()).collect();
        ids.sort();
        assert_eq!(ids, vec!["佐藤".to_string(), "田中".to_string()]);
    }

    #[test]
    fn boss_pairsで相互上司ペアを検出できる() {
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
            b.belongs_to(emp("田中"), dept("営業部"));
            b.belongs_to(emp("佐藤"), dept("営業部"));
            b.belongs_to(emp("鈴木"), dept("営業部"));
            // 田中と佐藤は相互に上司 (お互いがお互いの boss)。
            b.boss(emp("田中"), emp("佐藤"), BossAttrs { since: 2020 });
            b.boss(emp("佐藤"), emp("田中"), BossAttrs { since: 2019 });
            // 鈴木は上司なし (相互ペアには現れない)。
        })
        .unwrap();

        // match パターン (`match g { @{ a -[boss]-> b, b -[boss]-> a } => ... }`)
        // の代替として、ペアイテレータ + メソッドチェーンで同じクエリを書ける
        // ことを実証する。
        let all: Vec<(&EmployeeId, &EmployeeId)> =
            g.boss_pairs().map(|(a, b, _attrs)| (a, b)).collect();
        let mutual: Vec<(&EmployeeId, &EmployeeId)> = all
            .iter()
            .copied()
            .filter(|(a, b)| all.contains(&(b, a)))
            .collect();

        assert_eq!(mutual.len(), 2);
        assert!(mutual.contains(&(&emp("田中"), &emp("佐藤"))));
        assert!(mutual.contains(&(&emp("佐藤"), &emp("田中"))));
        assert!(!mutual.iter().any(|(a, _)| *a == &emp("鈴木")));
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
        assert!(!colleagues.contains(&"鈴木".to_string()));
    }
}

/// `graph!` インスタンスリテラルのテスト。同じファイル内で宣言した
/// `OrgChart` (macro 生成) をそのまま使う。
#[cfg(test)]
mod graph_literal_tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn graphリテラルで組織図を構築できる() {
        let g = graphite::graph!(OrgChart {
            tanaka: Employee { name: "田中".into(), id: 1 },
            sato: Employee { name: "佐藤".into(), id: 2 },
            sales: Department { name: "営業".into() },

            tanaka -[belongs_to]-> sales,
            sato -[belongs_to]-> sales,
            tanaka -[boss { since: 2020 }]-> sato,
        })
        .expect("正常な graph! リテラルは構築に成功するはず");

        assert_eq!(
            g.employee(&EmployeeId("tanaka".to_string())).unwrap().name,
            "田中"
        );
        assert_eq!(
            g.department(&DepartmentId("sales".to_string())).unwrap().name,
            "営業"
        );

        let d: &Department = g.belongs_to(&EmployeeId("tanaka".to_string()));
        assert_eq!(d.name, "営業");

        // `edge boss: Employee -> Employee` の方向は from=部下, to=上司
        // (手書きテンプレートの builder 引数順 `boss(employee, boss, attrs)`
        // に合わせた規約)。よって `tanaka -[boss]-> sato` は
        // 「田中の上司は佐藤」を意味する。
        let (boss_emp, attrs) = g
            .boss(&EmployeeId("tanaka".to_string()))
            .expect("田中の上司は佐藤のはず");
        assert_eq!(boss_emp.name, "佐藤");
        assert_eq!(attrs.since, 2020);
    }

    #[test]
    #[rustfmt::skip]
    fn graphリテラルの多重度違反はresultのerrになる() {
        let result = graphite::graph!(OrgChart {
            suzuki: Employee { name: "鈴木".into(), id: 3 },
            sales: Department { name: "営業".into() },
            // 鈴木を意図的にどの部署にも所属させない (belongs_to 多重度(1)違反)
        });

        assert!(matches!(
            result,
            Err(OrgChartViolation::MultiplicityViolation { edge: "belongs_to", .. })
        ));
    }
}
