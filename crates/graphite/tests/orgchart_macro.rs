//! `graph_schema!` で `OrgChart` を宣言し直し、`orgchart_handwritten.rs`
//! (フェーズ2の手書きテンプレート) と同等のテストを実行する。
//!
//! 手書き版との既知の差異 (README.md「手書きテンプレートとの差異」節も参照):
//! - 違反 enum 名は `OrgChartViolation` (手書き版は共通の `SchemaViolation`)。
//! - 多重度違反・未知キー参照はエッジ単位の型付きバリアント
//!   (`BelongsToMultiplicity { source: EmployeeId, count: usize }` /
//!   `BelongsToUnknownSource { key: EmployeeId }` 等) として生成される。
//!   手書き版は `MultiplicityViolation { employee: EmployeeId, .. }` という
//!   1つの共通バリアントだったが、一般のスキーマではエッジごとに始点/終点
//!   ノード型が異なりうるため、エッジごとに専用バリアントを生成することで
//!   型を固定できるようにしている。
//! - builder のエッジ追加メソッドの引数名は汎用的に `from`/`to`
//!   (手書き版は `employee`/`boss`、`manager`/`report` のようにドメイン語)。
//! - 導出エッジ (`colleagues`) はマクロが生成しない。生成された `OrgChart`
//!   に同一モジュール内で `impl` を追記すれば手書き版と同じことができる
//!   (下記参照。私有フィールドへのアクセスは同一モジュールツリー内なら可能、
//!   という通常の Rust 可視性規則をそのまま使っている)。
//! - ノード型 (`Employee`/`Department`) とエッジ属性型 (`BossEdge`) は
//!   いずれも `graph_schema!` の外で普通の struct として宣言し、schema には
//!   参照させるだけ (下記参照。`docs/edge_syntax_v3.md` 参照)。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub name: String,
    pub id: u32,
}

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Department {
    pub name: String,
}

/// `boss` エッジの属性。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema OrgChart {
        node Employee;
        node Department;

        edge belongs_to: Employee -> Department (1);
        edge boss:       Employee -[BossEdge]-> Employee (0..1);
        edge reports:    Employee -> Employee (0..*);
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
            b.boss(emp("佐藤"), emp("田中"), BossEdge { since: 2020 });
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
        let d: &Department = g.belongs_to().of(&emp("田中"));
        assert_eq!(d.name, "営業");
    }

    #[test]
    fn boss_は_option_を返す() {
        let g = build_healthy_chart();

        let b: Option<(&Employee, &BossEdge)> = g.boss().of(&emp("佐藤"));
        let (boss_emp, attrs) = b.expect("佐藤の上司は田中のはず");
        assert_eq!(boss_emp.name, "田中");
        assert_eq!(attrs.since, 2020);

        let no_boss: Option<(&Employee, &BossEdge)> = g.boss().of(&emp("田中"));
        assert!(no_boss.is_none());
    }

    #[test]
    fn reports_は_vec_を返す() {
        let g = build_healthy_chart();

        let subordinates: Vec<&Employee> = g.reports().of(&emp("田中"));
        assert_eq!(subordinates.len(), 1);
        assert_eq!(subordinates[0].name, "佐藤");

        let none: Vec<&Employee> = g.reports().of(&emp("佐藤"));
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
                OrgChartViolation::BelongsToMultiplicity {
                    source: emp("鈴木"),
                    count: 0,
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
            b.boss(emp("田中"), emp("佐藤"), BossEdge { since: 2018 });
            b.boss(emp("田中"), emp("鈴木"), BossEdge { since: 2019 });
        });

        assert!(matches!(
            result,
            Err(OrgChartViolation::BossMultiplicity { .. })
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
                OrgChartViolation::BelongsToUnknownTarget {
                    key: dept("存在しない部署")
                }
            ),
            Ok(_) => panic!("未知の部署参照はエラーになるはず"),
        }
    }

    #[test]
    fn getは未知キーでnoneを返す() {
        let g = build_healthy_chart();
        assert!(g.belongs_to().get(&emp("存在しない社員")).is_none());
        let d = g
            .belongs_to()
            .get(&emp("田中"))
            .expect("田中は営業部に所属しているはず");
        assert_eq!(d.name, "営業");
    }

    // 項目d (フェーズ5)・ビュー方式移行後 (docs/edge_view_api.md): id_of/get_id/ids_of。

    #[test]
    fn id_ofは多重度1でキーそのものを返す() {
        let g = build_healthy_chart();
        let dept_id: &DepartmentId = g.belongs_to().id_of(&emp("田中"));
        assert_eq!(*dept_id, dept("営業部"));
    }

    #[test]
    #[should_panic(expected = "belongs_to().id_of")]
    fn id_ofは未知キーでパニックする() {
        let g = build_healthy_chart();
        let _ = g.belongs_to().id_of(&emp("存在しない社員"));
    }

    #[test]
    fn get_idは未知キーでnoneを返す() {
        let g = build_healthy_chart();
        assert!(g.belongs_to().get_id(&emp("存在しない社員")).is_none());
        let dept_id = g
            .belongs_to()
            .get_id(&emp("田中"))
            .expect("田中は営業部に所属しているはず");
        assert_eq!(*dept_id, dept("営業部"));
    }

    #[test]
    fn boss_のid_ofは多重度0か1でoptionのキーを返す() {
        let g = build_healthy_chart();
        let boss_id: Option<&EmployeeId> = g.boss().id_of(&emp("佐藤"));
        assert_eq!(boss_id, Some(&emp("田中")));
        assert_eq!(g.boss().id_of(&emp("田中")), None);
    }

    #[test]
    fn reportsのids_ofは多重度0以上でキーのvecを返す_かつ追加順を保持する() {
        let g = OrgChart::create(|b| {
            b.employee(emp("部長"), Employee { name: "部長".to_string(), id: 1 });
            b.employee(emp("C"), Employee { name: "C".to_string(), id: 2 });
            b.employee(emp("A"), Employee { name: "A".to_string(), id: 3 });
            b.employee(emp("B"), Employee { name: "B".to_string(), id: 4 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            b.belongs_to(emp("部長"), dept("営業部"));
            b.belongs_to(emp("C"), dept("営業部"));
            b.belongs_to(emp("A"), dept("営業部"));
            b.belongs_to(emp("B"), dept("営業部"));
            // 追加順は C, A, B (アルファベット順でもキー文字列順でもない)。
            b.reports(emp("部長"), emp("C"));
            b.reports(emp("部長"), emp("A"));
            b.reports(emp("部長"), emp("B"));
        })
        .unwrap();

        let ids: Vec<&EmployeeId> = g.reports().ids_of(&emp("部長"));
        assert_eq!(ids, vec![&emp("C"), &emp("A"), &emp("B")]);

        let none: Vec<&EmployeeId> = g.reports().ids_of(&emp("A"));
        assert!(none.is_empty());

        // 項目i (フェーズ5): `of` も同じ順序保証を持つ
        // (README「`(0..*)` エッジの順序保証」節)。
        let names: Vec<&str> = g
            .reports()
            .of(&emp("部長"))
            .into_iter()
            .map(|e| e.name.as_str())
            .collect();
        assert_eq!(names, vec!["C", "A", "B"]);
    }

    // 項目g (フェーズ5): create_collecting は全違反を収集して返す。

    #[test]
    fn create_collectingは複数の違反を全件収集する() {
        let result = OrgChart::create_collecting(|b| {
            b.employee(emp("鈴木"), Employee { name: "鈴木".to_string(), id: 3 });
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.employee(emp("佐藤"), Employee { name: "佐藤".to_string(), id: 2 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            // 鈴木をどの部署にも所属させない (belongs_to 多重度違反その1)。
            b.belongs_to(emp("田中"), dept("営業部"));
            b.belongs_to(emp("佐藤"), dept("営業部"));
            // 田中に上司を2人つける (boss 多重度違反その2、belongs_toとは
            // 独立したエッジ種別)。
            b.boss(emp("田中"), emp("佐藤"), BossEdge { since: 2018 });
            b.boss(emp("田中"), emp("鈴木"), BossEdge { since: 2019 });
        });

        let violations = match result {
            Err(violations) => violations,
            Ok(_) => panic!("2件の違反が収集されるはず"),
        };
        assert_eq!(violations.len(), 2);
        assert!(violations
            .iter()
            .any(|v| matches!(v, OrgChartViolation::BelongsToMultiplicity { .. })));
        assert!(violations
            .iter()
            .any(|v| matches!(v, OrgChartViolation::BossMultiplicity { .. })));
    }

    #[test]
    fn create_collectingは違反がなければokを返す() {
        let result = OrgChart::create_collecting(|b| {
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            b.belongs_to(emp("田中"), dept("営業部"));
        });
        assert!(result.is_ok());
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
            b.boss(emp("田中"), emp("佐藤"), BossEdge { since: 2020 });
            b.boss(emp("佐藤"), emp("田中"), BossEdge { since: 2019 });
            // 鈴木は上司なし (相互ペアには現れない)。
        })
        .unwrap();

        // match パターン (`match g { @{ a -[boss]-> b, b -[boss]-> a } => ... }`)
        // の代替として、`iter()` + メソッドチェーンで同じクエリを書ける
        // ことを実証する。
        let all: Vec<(&EmployeeId, &EmployeeId)> =
            g.boss().iter().map(|(a, b, _attrs)| (a, b)).collect();
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
            tanaka = Employee { name: "田中".into(), id: 1 },
            sato = Employee { name: "佐藤".into(), id: 2 },
            sales = Department { name: "営業".into() },

            tanaka -[belongs_to]-> sales,
            sato -[belongs_to]-> sales,
            tanaka -[boss = BossEdge { since: 2020 }]-> sato,
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

        let d: &Department = g.belongs_to().of(&EmployeeId("tanaka".to_string()));
        assert_eq!(d.name, "営業");

        // `edge boss: Employee -> Employee` の方向は from=部下, to=上司
        // (手書きテンプレートの builder 引数順 `boss(employee, boss, attrs)`
        // に合わせた規約)。よって `tanaka -[boss]-> sato` は
        // 「田中の上司は佐藤」を意味する。
        let (boss_emp, attrs) = g
            .boss()
            .of(&EmployeeId("tanaka".to_string()))
            .expect("田中の上司は佐藤のはず");
        assert_eq!(boss_emp.name, "佐藤");
        assert_eq!(attrs.since, 2020);
    }

    #[test]
    #[rustfmt::skip]
    fn graphリテラルの多重度違反はresultのerrになる() {
        let result = graphite::graph!(OrgChart {
            suzuki = Employee { name: "鈴木".into(), id: 3 },
            sales = Department { name: "営業".into() },
            // 鈴木を意図的にどの部署にも所属させない (belongs_to 多重度(1)違反)
        });

        assert!(matches!(
            result,
            Err(OrgChartViolation::BelongsToMultiplicity { .. })
        ));
    }

    // 項目G1 (`docs/ide_support_spec.md`): `graph!` の展開をノードキーごとの
    // `let` 束縛方式に変更した際の回帰テスト2件。

    #[test]
    #[rustfmt::skip]
    fn ノードキーにbを使っても_builder変数と衝突しない() {
        // 展開後のクロージャ引数は `__graphite_b` (`b` ではない) なので、
        // ノードキー `b` から生成される `let b = ..;` が builder 変数を
        // 隠してしまうことはない。もし引数名が `b` のままなら、この
        // graph! はビルダーメソッド呼び出しが `String` に対する呼び出し
        // として解釈されてコンパイルエラーになっていたはず。
        let g = graphite::graph!(OrgChart {
            b = Employee { name: "B社員".into(), id: 10 },
            sales = Department { name: "営業".into() },

            b -[belongs_to]-> sales,
        })
        .expect("ノードキー b を使った graph! も構築に成功するはず");

        assert_eq!(
            g.employee(&EmployeeId("b".to_string())).unwrap().name,
            "B社員"
        );
        let d: &Department = g.belongs_to().of(&EmployeeId("b".to_string()));
        assert_eq!(d.name, "営業");
    }

    #[test]
    #[rustfmt::skip]
    fn エッジをノード宣言より先に書いても構築できる() {
        // let束縛は使用より前に必要なため、展開時に「全ノード宣言(記述順)
        // → 全エッジ(記述順)」の2段に並べ替えている。この並べ替えが正しく
        // 機能し、記述順に依存せず構築できることを確認する。
        let g = graphite::graph!(OrgChart {
            tanaka -[belongs_to]-> sales,
            sato -[belongs_to]-> sales,
            tanaka -[boss = BossEdge { since: 2020 }]-> sato,

            tanaka = Employee { name: "田中".into(), id: 1 },
            sato = Employee { name: "佐藤".into(), id: 2 },
            sales = Department { name: "営業".into() },
        })
        .expect("エッジをノード宣言より先に書いても構築できるはず");

        let d: &Department = g.belongs_to().of(&EmployeeId("tanaka".to_string()));
        assert_eq!(d.name, "営業");

        let (boss_emp, attrs) = g
            .boss()
            .of(&EmployeeId("tanaka".to_string()))
            .expect("田中の上司は佐藤のはず");
        assert_eq!(boss_emp.name, "佐藤");
        assert_eq!(attrs.since, 2020);
    }

    // v3 (`docs/graph_literal_v3.md`): ノード項・エッジ属性はいずれも任意の
    // 式なので、graph! の外で構築済みの値をそのまま move で渡せる。
    #[test]
    #[rustfmt::skip]
    fn 外で構築した値をそのままgraphリテラルに渡せる() {
        let tanaka_value = Employee { name: "田中".to_string(), id: 1 };
        let promotion = BossEdge { since: 2021 };

        let g = graphite::graph!(OrgChart {
            tanaka = tanaka_value,
            sato = Employee { name: "佐藤".into(), id: 2 },
            sales = Department { name: "営業".into() },

            tanaka -[belongs_to]-> sales,
            sato -[belongs_to]-> sales,
            sato -[boss = promotion]-> tanaka,
        })
        .expect("外で構築した値を渡した graph! も構築に成功するはず");

        assert_eq!(
            g.employee(&EmployeeId("tanaka".to_string())).unwrap().name,
            "田中"
        );
        let (boss_emp, attrs) = g
            .boss()
            .of(&EmployeeId("sato".to_string()))
            .expect("佐藤の上司は田中のはず");
        assert_eq!(boss_emp.name, "田中");
        assert_eq!(attrs.since, 2021);
    }
}
