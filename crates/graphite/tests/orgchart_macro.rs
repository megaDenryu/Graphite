//! `graph_schema!` で `OrgChart` を v4 構文 (`docs/schema_v4.md`) で宣言し、
//! ノード・辺の読み書き一式を検証する統合テスト。
//!
//! v4 の要点 (このファイルで確認する項目):
//! - `edge Kind = From -> To where ...;` 宣言と `where each ../unique pair` 制約
//! - 辺は第一級キー付き要素 (`{Kind}Id`) であり、タプル struct
//!   `Kind(from, to, payload?)` として実在する
//! - ノードアクセスは `{Schema}Node` トレイト経由 (`Employee::get(&g, &id)` 等)、
//!   辺アクセスは各 `Kind` への固有 impl (`Kind::of`/`get`/`between`/`iter`/
//!   `ids`/`len`)。`g.メソッド` は一切生成されない。
//!
//! v4.1 (`docs/edge_endpoints_v4_1.md`) の実証: `Boss` を役割名つき
//! (`subordinate`/`superior`) に書き換え、終点側 (`superior`) の each 制約
//! (入次数制約) を検証する。役割名を書いたので `.from()`/`.to()` は生成
//! されず、`.subordinate()`/`.superior()` を使う。`graph!` リテラルの構文
//! (`Boss(bob -[..]-> alice)`) は不変 (役割名は宣言側だけの語彙)。

/// ノードキー。v4.2 からは `graph_schema!` はこれも生成せず、
/// `{ノード型名}Id` という命名規約で参照するだけ (`docs/node_id_v4_2.md`)。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmployeeId(pub String);

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct Employee {
    pub name: String,
    pub id: u32,
}

/// ノードキー。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DepartmentId(pub String);

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct Department {
    pub name: String,
}

/// `Boss` 辺の積み荷。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct BossEdge {
    pub since: i32,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema OrgChart {
        node Employee;
        node Department;

        edge BelongsTo = Employee -> Department                        where each Employee: 1;
        edge Boss      = (subordinate: Employee) -[BossEdge]-> (superior: Employee)
                                                                        where each subordinate: 0..1;
        edge Reports   = Employee -> Employee                          where unique pair;
        // v4.1 (`docs/edge_endpoints_v4_1.md` §1) の実証: 終点側 (役割名
        // `department`) の each、つまり入次数制約 (「各部署の代表は高々1人」)。
        edge Leads     = (leader: Employee) -> (department: Department) where each department: 0..1;
    }
}

/// 導出エッジの例: `graph_schema!` が生成した `OrgChart` へ、保存されない
/// 計算結果を返す普通のメソッドを追記できることを示す
/// (`docs/graph_design_sketches.md` 決定「保存エッジ=フィールド、
/// 導出エッジ=getter」)。私有フィールド (`belongs_to`/`belongs_to_from_index`)
/// へ同一モジュールツリー内からアクセスできる、という通常の Rust 可視性規則
/// をそのまま使っている。
impl OrgChart {
    pub fn colleagues(&self, id: &EmployeeId) -> Vec<&Employee> {
        let Some(ids) = self.belongs_to_from_index.get(id) else {
            return Vec::new();
        };
        let dept_id = self
            .belongs_to
            .get(&ids[0])
            .expect("from_indexに載っている辺はstorageに必ず存在する")
            .to();

        self.employees
            .ids()
            .filter(|other| *other != id)
            .filter(|other| {
                self.belongs_to_from_index
                    .get(*other)
                    .and_then(|v| v.first())
                    .and_then(|bid| self.belongs_to.get(bid))
                    .map(|b| b.to() == dept_id)
                    .unwrap_or(false)
            })
            .map(|other| self.employees.get(other).unwrap())
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

            b.belongs_to(
                BelongsToId("bt-tanaka".to_string()),
                BelongsTo(emp("田中"), dept("営業部")),
            );
            b.belongs_to(
                BelongsToId("bt-sato".to_string()),
                BelongsTo(emp("佐藤"), dept("営業部")),
            );
            // Boss(部下, 上司): 佐藤の上司は田中。
            b.boss(
                BossId("boss-sato".to_string()),
                Boss(emp("佐藤"), emp("田中"), BossEdge { since: 2020 }),
            );
            b.reports(
                ReportsId("r1".to_string()),
                Reports(emp("田中"), emp("佐藤")),
            );
        })
        .expect("正常な組織図は構築に成功するはず")
    }

    #[test]
    fn 正常構築できる() {
        let g = build_healthy_chart();
        assert_eq!(Employee::get(&g, &emp("田中")).unwrap().name, "田中");
        assert_eq!(Department::get(&g, &dept("営業部")).unwrap().name, "営業");
    }

    #[test]
    fn belongs_toのofはeach1なので参照そのものを返す() {
        let g = build_healthy_chart();
        let d: &Department = BelongsTo::of(&g, &emp("田中"));
        assert_eq!(d.name, "営業");
    }

    #[test]
    #[should_panic(expected = "BelongsTo::of")]
    fn belongs_toのofは未知キーでパニックする() {
        let g = build_healthy_chart();
        let _ = BelongsTo::of(&g, &emp("存在しない社員"));
    }

    #[test]
    fn belongs_toのget_ofは未知キーでnoneを返す() {
        let g = build_healthy_chart();
        assert!(BelongsTo::get_of(&g, &emp("存在しない社員")).is_none());
        let d = BelongsTo::get_of(&g, &emp("田中")).expect("田中は営業部に所属しているはず");
        assert_eq!(d.name, "営業");
    }

    #[test]
    fn bossのofはeach0か1なのでoptionを返す() {
        let g = build_healthy_chart();

        let b: Option<(&Employee, &BossEdge)> = Boss::of(&g, &emp("佐藤"));
        let (boss_emp, attrs) = b.expect("佐藤の上司は田中のはず");
        assert_eq!(boss_emp.name, "田中");
        assert_eq!(attrs.since, 2020);

        let no_boss: Option<(&Employee, &BossEdge)> = Boss::of(&g, &emp("田中"));
        assert!(no_boss.is_none());
    }

    #[test]
    fn reportsのofは制約なしなのでvecを返す() {
        let g = build_healthy_chart();

        let subordinates: Vec<&Employee> = Reports::of(&g, &emp("田中"));
        assert_eq!(subordinates.len(), 1);
        assert_eq!(subordinates[0].name, "佐藤");

        let none: Vec<&Employee> = Reports::of(&g, &emp("佐藤"));
        assert!(none.is_empty());
    }

    #[test]
    fn belongs_toが0本の社員はeach違反になる() {
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
                OrgChartViolation::BelongsToEachViolation {
                    source: emp("鈴木"),
                    count: 0,
                }
            ),
            Ok(_) => panic!("each違反が検出されるはず"),
        }
    }

    #[test]
    fn bossが2本ある社員はeach違反になる() {
        let result = OrgChart::create(|b| {
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.employee(emp("佐藤"), Employee { name: "佐藤".to_string(), id: 2 });
            b.employee(emp("鈴木"), Employee { name: "鈴木".to_string(), id: 3 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            b.belongs_to(BelongsToId("bt1".to_string()), BelongsTo(emp("田中"), dept("営業部")));
            b.belongs_to(BelongsToId("bt2".to_string()), BelongsTo(emp("佐藤"), dept("営業部")));
            b.belongs_to(BelongsToId("bt3".to_string()), BelongsTo(emp("鈴木"), dept("営業部")));
            // 田中に上司を2人つける (each 0..1 違反)
            b.boss(BossId("b1".to_string()), Boss(emp("田中"), emp("佐藤"), BossEdge { since: 2018 }));
            b.boss(BossId("b2".to_string()), Boss(emp("田中"), emp("鈴木"), BossEdge { since: 2019 }));
        });

        assert!(matches!(
            result,
            Err(OrgChartViolation::BossEachViolation { .. })
        ));
    }

    #[test]
    fn 未知の部署への所属はエラーになる() {
        let result = OrgChart::create(|b| {
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.belongs_to(
                BelongsToId("bt1".to_string()),
                BelongsTo(emp("田中"), dept("存在しない部署")),
            );
        });

        match result {
            Err(violation) => assert_eq!(
                violation,
                OrgChartViolation::BelongsToUnknownTarget {
                    edge: BelongsToId("bt1".to_string()),
                    target: dept("存在しない部署"),
                }
            ),
            Ok(_) => panic!("未知の部署参照はエラーになるはず"),
        }
    }

    #[test]
    fn 辺キーが重複していると違反になる() {
        let result = OrgChart::create(|b| {
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.employee(emp("佐藤"), Employee { name: "佐藤".to_string(), id: 2 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            b.belongs_to(BelongsToId("dup".to_string()), BelongsTo(emp("田中"), dept("営業部")));
            b.belongs_to(BelongsToId("dup".to_string()), BelongsTo(emp("佐藤"), dept("営業部")));
        });

        assert!(matches!(
            result,
            Err(OrgChartViolation::BelongsToDuplicateKey(id)) if id == BelongsToId("dup".to_string())
        ));
    }

    #[test]
    fn reportsのbetweenはunique_pairなのでoptionを返す() {
        let g = build_healthy_chart();
        let r: Option<&Reports> = Reports::between(&g, &emp("田中"), &emp("佐藤"));
        assert!(r.is_some());
        assert!(Reports::between(&g, &emp("佐藤"), &emp("田中")).is_none());
    }

    #[test]
    fn 同じ対に2本目のreportsを張るとunique_pair違反になる() {
        let result = OrgChart::create(|b| {
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.employee(emp("佐藤"), Employee { name: "佐藤".to_string(), id: 2 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            b.belongs_to(BelongsToId("bt1".to_string()), BelongsTo(emp("田中"), dept("営業部")));
            b.belongs_to(BelongsToId("bt2".to_string()), BelongsTo(emp("佐藤"), dept("営業部")));
            b.reports(ReportsId("r1".to_string()), Reports(emp("田中"), emp("佐藤")));
            b.reports(ReportsId("r2".to_string()), Reports(emp("田中"), emp("佐藤")));
        });

        assert!(matches!(
            result,
            Err(OrgChartViolation::ReportsUniquePairViolation { .. })
        ));
    }

    #[test]
    fn getは辺キーで1本を検索する() {
        let g = build_healthy_chart();
        let e = BelongsTo::get(&g, &BelongsToId("bt-tanaka".to_string())).expect("存在するはず");
        assert_eq!(e.from(), &emp("田中"));
        assert_eq!(e.to(), &dept("営業部"));
        assert!(BelongsTo::get(&g, &BelongsToId("存在しない".to_string())).is_none());
    }

    #[test]
    fn create_collectingは複数の違反を全件収集する() {
        let result = OrgChart::create_collecting(|b| {
            b.employee(emp("鈴木"), Employee { name: "鈴木".to_string(), id: 3 });
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.employee(emp("佐藤"), Employee { name: "佐藤".to_string(), id: 2 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            // 鈴木をどの部署にも所属させない (belongs_to each違反その1)。
            b.belongs_to(BelongsToId("bt1".to_string()), BelongsTo(emp("田中"), dept("営業部")));
            b.belongs_to(BelongsToId("bt2".to_string()), BelongsTo(emp("佐藤"), dept("営業部")));
            // 田中に上司を2人つける (boss each違反その2、belongs_toとは独立)。
            b.boss(BossId("b1".to_string()), Boss(emp("田中"), emp("佐藤"), BossEdge { since: 2018 }));
            b.boss(BossId("b2".to_string()), Boss(emp("田中"), emp("鈴木"), BossEdge { since: 2019 }));
        });

        let violations = match result {
            Err(violations) => violations,
            Ok(_) => panic!("2件の違反が収集されるはず"),
        };
        assert_eq!(violations.len(), 2);
        assert!(violations
            .iter()
            .any(|v| matches!(v, OrgChartViolation::BelongsToEachViolation { .. })));
        assert!(violations
            .iter()
            .any(|v| matches!(v, OrgChartViolation::BossEachViolation { .. })));
    }

    #[test]
    fn create_collectingは違反がなければokを返す() {
        let result = OrgChart::create_collecting(|b| {
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            b.belongs_to(BelongsToId("bt1".to_string()), BelongsTo(emp("田中"), dept("営業部")));
        });
        assert!(result.is_ok());
    }

    #[test]
    fn employee_idsで全キーを列挙できる() {
        let g = build_healthy_chart();
        let mut ids: Vec<String> = Employee::ids(&g).map(|id| id.0.clone()).collect();
        ids.sort();
        assert_eq!(ids, vec!["佐藤".to_string(), "田中".to_string()]);
    }

    #[test]
    fn iterで表全体を走査できる() {
        let g = build_healthy_chart();
        let all: Vec<(&BelongsToId, &BelongsTo)> = BelongsTo::iter(&g).collect();
        assert_eq!(all.len(), 2);
        assert_eq!(BelongsTo::len(&g), 2);
    }

    #[test]
    fn colleagues_は同じ部署の他の社員を返す() {
        let g = OrgChart::create(|b| {
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.employee(emp("佐藤"), Employee { name: "佐藤".to_string(), id: 2 });
            b.employee(emp("鈴木"), Employee { name: "鈴木".to_string(), id: 3 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            b.department(dept("開発部"), Department { name: "開発".to_string() });
            b.belongs_to(BelongsToId("bt1".to_string()), BelongsTo(emp("田中"), dept("営業部")));
            b.belongs_to(BelongsToId("bt2".to_string()), BelongsTo(emp("佐藤"), dept("営業部")));
            b.belongs_to(BelongsToId("bt3".to_string()), BelongsTo(emp("鈴木"), dept("開発部")));
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

    #[test]
    fn タプルstructはマクロ外でも普通に構築できる() {
        // `docs/schema_v4.md` §3.1 原則6: 生成されたタプル struct は
        // マクロの外でも普通に構築できる。
        let e = BelongsTo(emp("田中"), dept("営業部"));
        assert_eq!(e.from(), &emp("田中"));
        assert_eq!(e.to(), &dept("営業部"));

        let b = Boss(emp("佐藤"), emp("田中"), BossEdge { since: 2020 });
        assert_eq!(b.payload().since, 2020);
    }

    #[test]
    fn 役割名つき辺はfromtoの代わりに役割名アクセサを持つ() {
        // `docs/edge_endpoints_v4_1.md` §1: 役割名を書いた辺は `.from()`/
        // `.to()` の代わりに `.subordinate()`/`.superior()` を生成する
        // (from/to は生成しない)。
        let b = Boss(emp("佐藤"), emp("田中"), BossEdge { since: 2020 });
        assert_eq!(b.subordinate(), &emp("佐藤"));
        assert_eq!(b.superior(), &emp("田中"));
    }

    #[test]
    fn leadsは部署ごとに代表を高々1人までしか持てない() {
        // 終点側 (役割名 `department`) の each 制約 = 入次数制約
        // (`docs/edge_endpoints_v4_1.md` §1 の新規解禁項目)。健全な構築では
        // 各部署に代表が0人または1人。
        let g = OrgChart::create(|b| {
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.employee(emp("佐藤"), Employee { name: "佐藤".to_string(), id: 2 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            b.belongs_to(BelongsToId("bt1".to_string()), BelongsTo(emp("田中"), dept("営業部")));
            b.belongs_to(BelongsToId("bt2".to_string()), BelongsTo(emp("佐藤"), dept("営業部")));
            b.leads(LeadsId("l1".to_string()), Leads(emp("田中"), dept("営業部")));
        })
        .expect("代表が1人の部署は健全なはず");

        // `of` は常に始点側 (`leader`) キーで検索する (入次数制約は `of` の
        // 戻り型には影響しない、`docs/edge_endpoints_v4_1.md` §1)。始点側は
        // 無制約なので `Vec` を返す。
        let departments_led_by_tanaka: Vec<&Department> = Leads::of(&g, &emp("田中"));
        assert_eq!(departments_led_by_tanaka.len(), 1);
        assert_eq!(departments_led_by_tanaka[0].name, "営業");

        let leads_edge = Leads::get(&g, &LeadsId("l1".to_string())).unwrap();
        assert_eq!(leads_edge.leader(), &emp("田中"));
        assert_eq!(leads_edge.department(), &dept("営業部"));
    }

    #[test]
    fn extendは要素単位apiの反復と同一の内容になる() {
        // `docs/bulk_construction.md`/`docs/graph_splice.md` §2: 統一
        // extend は insert/add の反復と完全に同一の意味論であるはず。
        // build_healthy_chart (要素単位で構築) と同じデータを extend で
        // 構築し、内容が一致することを確認する。ノード用・エッジ用の
        // 呼び分けが要らない (値の型から rustc が振り分ける) ことも実証する。
        let g1 = build_healthy_chart();
        let g2 = OrgChart::create(|b| {
            b.extend(vec![
                ("田中".to_string(), Employee { name: "田中".to_string(), id: 1 }),
                ("佐藤".to_string(), Employee { name: "佐藤".to_string(), id: 2 }),
            ]);
            b.extend(vec![(
                "営業部".to_string(),
                Department { name: "営業".to_string() },
            )]);
            b.extend(vec![
                ("bt-tanaka".to_string(), BelongsTo(emp("田中"), dept("営業部"))),
                ("bt-sato".to_string(), BelongsTo(emp("佐藤"), dept("営業部"))),
            ]);
            b.extend(vec![(
                "boss-sato".to_string(),
                Boss(emp("佐藤"), emp("田中"), BossEdge { since: 2020 }),
            )]);
            b.extend(vec![("r1".to_string(), Reports(emp("田中"), emp("佐藤")))]);
        })
        .expect("extendで構築した組織図も要素単位と同様に成功するはず");

        let employees_of = |g: &OrgChart| -> Vec<(String, Employee)> {
            let mut v: Vec<(String, Employee)> = Employee::iter(g)
                .map(|(id, e)| (id.0.clone(), e.clone()))
                .collect();
            v.sort_by(|a, b| a.0.cmp(&b.0));
            v
        };
        assert_eq!(employees_of(&g1), employees_of(&g2));

        let departments_of = |g: &OrgChart| -> Vec<(String, Department)> {
            let mut v: Vec<(String, Department)> = Department::iter(g)
                .map(|(id, d)| (id.0.clone(), d.clone()))
                .collect();
            v.sort_by(|a, b| a.0.cmp(&b.0));
            v
        };
        assert_eq!(departments_of(&g1), departments_of(&g2));

        // 辺は KeyedTable が挿入順を保持する仕様 (`docs/schema_v4.md` §3.2) な
        // ので、順序も含めてそのまま比較できる。
        let belongs_to_of =
            |g: &OrgChart| -> Vec<(BelongsToId, BelongsTo)> { BelongsTo::iter(g).map(|(id, e)| (id.clone(), e.clone())).collect() };
        assert_eq!(belongs_to_of(&g1), belongs_to_of(&g2));

        let boss_of =
            |g: &OrgChart| -> Vec<(BossId, Boss)> { Boss::iter(g).map(|(id, e)| (id.clone(), e.clone())).collect() };
        assert_eq!(boss_of(&g1), boss_of(&g2));

        let reports_of =
            |g: &OrgChart| -> Vec<(ReportsId, Reports)> { Reports::iter(g).map(|(id, e)| (id.clone(), e.clone())).collect() };
        assert_eq!(reports_of(&g1), reports_of(&g2));
    }

    #[test]
    fn extendの戻り値は入力順のidになる() {
        let g = OrgChart::create(|b| {
            let node_ids = b.extend(vec![
                ("田中".to_string(), Employee { name: "田中".to_string(), id: 1 }),
                ("佐藤".to_string(), Employee { name: "佐藤".to_string(), id: 2 }),
            ]);
            assert_eq!(node_ids, vec![emp("田中"), emp("佐藤")]);

            b.extend(vec![(
                "営業部".to_string(),
                Department { name: "営業".to_string() },
            )]);

            let edge_ids = b.extend(vec![
                ("bt-tanaka".to_string(), BelongsTo(emp("田中"), dept("営業部"))),
                ("bt-sato".to_string(), BelongsTo(emp("佐藤"), dept("営業部"))),
            ]);
            assert_eq!(
                edge_ids,
                vec![
                    BelongsToId("bt-tanaka".to_string()),
                    BelongsToId("bt-sato".to_string()),
                ]
            );
        })
        .expect("正常な組織図は構築に成功するはず");

        assert_eq!(Employee::get(&g, &emp("田中")).unwrap().name, "田中");
    }

    #[test]
    fn extendは空イテレータでも問題なく動く() {
        let result = OrgChart::create(|b| {
            let node_ids: Vec<EmployeeId> = b.extend(Vec::<(String, Employee)>::new());
            assert!(node_ids.is_empty());
            let edge_ids: Vec<BelongsToId> = b.extend(Vec::<(String, BelongsTo)>::new());
            assert!(edge_ids.is_empty());

            // 空のextendだけではノードも辺も無いので違反も無く成功するはず。
        });
        assert!(result.is_ok());
    }

    #[test]
    fn 同じ部署に2人のleaderをつけると入次数違反になる() {
        let result = OrgChart::create(|b| {
            b.employee(emp("田中"), Employee { name: "田中".to_string(), id: 1 });
            b.employee(emp("佐藤"), Employee { name: "佐藤".to_string(), id: 2 });
            b.department(dept("営業部"), Department { name: "営業".to_string() });
            b.belongs_to(BelongsToId("bt1".to_string()), BelongsTo(emp("田中"), dept("営業部")));
            b.belongs_to(BelongsToId("bt2".to_string()), BelongsTo(emp("佐藤"), dept("営業部")));
            // 営業部に代表を2人つける (each department: 0..1 違反、入次数)。
            b.leads(LeadsId("l1".to_string()), Leads(emp("田中"), dept("営業部")));
            b.leads(LeadsId("l2".to_string()), Leads(emp("佐藤"), dept("営業部")));
        });

        match result {
            Err(violation) => assert_eq!(
                violation,
                OrgChartViolation::LeadsEachViolation {
                    target: dept("営業部"),
                    count: 2,
                }
            ),
            Ok(_) => panic!("入次数違反が検出されるはず"),
        }
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

            tanaka_dept = BelongsTo(tanaka -> sales),
            sato_dept = BelongsTo(sato -> sales),
            tanaka_boss = Boss(tanaka -[BossEdge { since: 2020 }]-> sato),
        })
        .expect("正常な graph! リテラルは構築に成功するはず");

        assert_eq!(
            Employee::get(&g, &EmployeeId("tanaka".to_string())).unwrap().name,
            "田中"
        );
        assert_eq!(
            Department::get(&g, &DepartmentId("sales".to_string())).unwrap().name,
            "営業"
        );

        let d: &Department = BelongsTo::of(&g, &EmployeeId("tanaka".to_string()));
        assert_eq!(d.name, "営業");

        // Boss(部下 -> 上司) の方向規約により、tanaka の上司は sato。
        let (boss_emp, attrs) = Boss::of(&g, &EmployeeId("tanaka".to_string()))
            .expect("田中の上司は佐藤のはず");
        assert_eq!(boss_emp.name, "佐藤");
        assert_eq!(attrs.since, 2020);
    }

    #[test]
    #[rustfmt::skip]
    fn graphリテラルのeach違反はresultのerrになる() {
        let result = graphite::graph!(OrgChart {
            suzuki = Employee { name: "鈴木".into(), id: 3 },
            sales = Department { name: "営業".into() },
            // 鈴木を意図的にどの部署にも所属させない (belongs_to each:1 違反)
        });

        assert!(matches!(
            result,
            Err(OrgChartViolation::BelongsToEachViolation { .. })
        ));
    }

    // 項目G1 (`docs/ide_support_spec.md`): `graph!` の展開をキーごとの
    // `let` 束縛方式に変更した際の回帰テスト2件。

    #[test]
    #[rustfmt::skip]
    fn ノードキーにbを使っても_builder変数と衝突しない() {
        // 展開後のクロージャ引数は `__graphite_b` (`b` ではない) なので、
        // ノードキー `b` から生成される `let b = ..;` が builder 変数を
        // 隠してしまうことはない。
        let g = graphite::graph!(OrgChart {
            b = Employee { name: "B社員".into(), id: 10 },
            sales = Department { name: "営業".into() },

            b_dept = BelongsTo(b -> sales),
        })
        .expect("ノードキー b を使った graph! も構築に成功するはず");

        assert_eq!(
            Employee::get(&g, &EmployeeId("b".to_string())).unwrap().name,
            "B社員"
        );
        let d: &Department = BelongsTo::of(&g, &EmployeeId("b".to_string()));
        assert_eq!(d.name, "営業");
    }

    #[test]
    #[rustfmt::skip]
    fn エッジをノード宣言より先に書いても構築できる() {
        // let束縛は使用より前に必要なため、展開時に「全ノード宣言(記述順)
        // → 全エッジ(記述順)」の2段に並べ替えている。この並べ替えが正しく
        // 機能し、記述順に依存せず構築できることを確認する。
        let g = graphite::graph!(OrgChart {
            tanaka_dept = BelongsTo(tanaka -> sales),
            sato_dept = BelongsTo(sato -> sales),
            tanaka_boss = Boss(tanaka -[BossEdge { since: 2020 }]-> sato),

            tanaka = Employee { name: "田中".into(), id: 1 },
            sato = Employee { name: "佐藤".into(), id: 2 },
            sales = Department { name: "営業".into() },
        })
        .expect("エッジをノード宣言より先に書いても構築できるはず");

        let d: &Department = BelongsTo::of(&g, &EmployeeId("tanaka".to_string()));
        assert_eq!(d.name, "営業");

        let (boss_emp, attrs) = Boss::of(&g, &EmployeeId("tanaka".to_string()))
            .expect("田中の上司は佐藤のはず");
        assert_eq!(boss_emp.name, "佐藤");
        assert_eq!(attrs.since, 2020);
    }

    // v4: ノード項・エッジの積み荷はいずれも任意の式なので、graph! の外で
    // 構築済みの値をそのまま move で渡せる。
    #[test]
    #[rustfmt::skip]
    fn 外で構築した値をそのままgraphリテラルに渡せる() {
        let tanaka_value = Employee { name: "田中".to_string(), id: 1 };
        let promotion = BossEdge { since: 2021 };

        let g = graphite::graph!(OrgChart {
            tanaka = tanaka_value,
            sato = Employee { name: "佐藤".into(), id: 2 },
            sales = Department { name: "営業".into() },

            tanaka_dept = BelongsTo(tanaka -> sales),
            sato_dept = BelongsTo(sato -> sales),
            sato_boss = Boss(sato -[promotion]-> tanaka),
        })
        .expect("外で構築した値を渡した graph! も構築に成功するはず");

        assert_eq!(
            Employee::get(&g, &EmployeeId("tanaka".to_string())).unwrap().name,
            "田中"
        );
        let (boss_emp, attrs) = Boss::of(&g, &EmployeeId("sato".to_string()))
            .expect("佐藤の上司は田中のはず");
        assert_eq!(boss_emp.name, "田中");
        assert_eq!(attrs.since, 2021);
    }

    #[test]
    #[rustfmt::skip]
    fn タプルstructを直接構築してaddできる() {
        // `docs/schema_v4.md` §3.1: タプル struct はマクロ外でも普通に
        // 構築できる。graph! の脱糖結果と同じ形を手で書けることを示す。
        let g = OrgChart::create(|b| {
            let tanaka = b.insert("tanaka", Employee { name: "田中".into(), id: 1 });
            let sales = b.insert("sales", Department { name: "営業".into() });
            b.add("bt1", BelongsTo(tanaka.clone(), sales.clone()));
        })
        .expect("手動構築でも成功するはず");

        let d: &Department = BelongsTo::of(&g, &EmployeeId("tanaka".to_string()));
        assert_eq!(d.name, "営業");
    }
}
