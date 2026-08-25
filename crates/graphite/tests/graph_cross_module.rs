//! v3 (`docs/graph_literal_v3.md` §4): ハンドシェイクマクロ全廃により、
//! `graph_schema!` と `graph!` の同一ファイル制約 (旧 G5、
//! `docs/ide_support_spec.md` 参照) が構造的に消滅したことを示すテスト。
//! v4 でも同様に、`graph!` が参照するのは (a) スキーマ struct の `create`
//! メソッド、(b) builder の総称 `insert`/`add`、(c) 各ノード型が impl する
//! `{Schema}Node` トレイト、という普通の Rust の型・メソッド・トレイトだけに
//! なったため、`use` でスコープに持ち込めば別モジュールから問題なく呼べる。

/// スキーマ宣言を専用モジュールに隔離する。
mod schema {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct EmployeeId(pub String);

    #[derive(Debug, Clone, PartialEq)]
    pub struct Employee {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct DepartmentId(pub String);

    #[derive(Debug, Clone, PartialEq)]
    pub struct Department {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BossEdge {
        pub since: i32,
    }

    #[rustfmt::skip]
    #[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod CrossModuleOrg {
    include!("generated/graph_cross_module_cross_module_org.rs");
}

    #[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/graph_cross_module_cross_module_org.rs";
    schema CrossModuleOrg {
            node Employee(id: EmployeeId);
            node Department(id: DepartmentId);

            edge BelongsTo = (employee: Employee) -> (department: Department) where each employee: 1;
            edge Boss = (subordinate: Employee) -[appointment: BossEdge]-> (superior: Employee) where each subordinate: 0..1;
        }
    }
}

/// `schema` モジュールを `use` してから `graph!` を呼ぶ、別モジュール側。
mod usage {
    use super::schema::*;

    #[test]
    #[rustfmt::skip]
    fn 別モジュールのschemaに対してgraphリテラルが構築できる() {
        let g = graphite::graph!(CrossModuleOrg {
            tanaka @ EmployeeId("tanaka".into()) = Employee { name: "田中".into() },
            sales @ DepartmentId("sales".into()) = Department { name: "営業".into() },

            tanaka_dept = BelongsTo(tanaka -> sales),
            tanaka_boss = Boss(tanaka -[BossEdge { since: 2020 }]-> tanaka),
        })
        .expect("別モジュールの schema に対する graph! も構築に成功するはず");

        assert_eq!(
            CrossModuleOrg::Employee::get(&g, &EmployeeId("tanaka".to_string())).unwrap().name,
            "田中"
        );
        let dept = g.tanaka().belongs_to_as_employee().department();
        assert_eq!(dept.name, "営業");
    }
}
