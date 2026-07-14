//! v3 (`docs/graph_literal_v3.md` §4): ハンドシェイクマクロ全廃により、
//! `graph_schema!` と `graph!` の同一ファイル制約 (旧 G5、
//! `docs/ide_support_spec.md` 参照) が構造的に消滅したことを示すテスト。
//!
//! `graph!` が参照するのは (a) スキーマ struct の `create` メソッド、
//! (b) builder の総称 `insert`、(c) builder の型名付きエッジメソッド
//! (`b.label(..)`) という普通の Rust の型・メソッドだけになったため、
//! `use` でスコープに持ち込めば別モジュールから問題なく呼べる。
//! v2 まではこれが `__graphite_edge_{Schema}!` (macro_rules、テキスト
//! スコープ) 経由だったため、`graph_schema!` と同一ファイル (正確には
//! 定義箇所より後の同一モジュール) でなければ機能しなかった。

/// スキーマ宣言を専用モジュールに隔離する。
mod schema {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Employee {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Department {
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct BossEdge {
        pub since: i32,
    }

    #[rustfmt::skip]
    graphite::graph_schema! {
        schema CrossModuleOrg {
            node Employee;
            node Department;

            edge Employee -[belongs_to]-> Department (1);
            edge Employee -[boss: BossEdge]-> Employee (0..1);
        }
    }
}

/// `schema` モジュールを `use` してから `graph!` を呼ぶ、別モジュール側。
/// v2 ではこの分離だけでハンドシェイクマクロが見えなくなり
/// (`__graphite_edge_CrossModuleOrg!` はテキストスコープなので `use` では
/// 持ち込めない)、未知ラベル検査が機能しなくなっていた。v3 はハンドシェイク
/// を使わないため、この分離自体が問題にならない。
mod usage {
    use super::schema::*;

    #[test]
    #[rustfmt::skip]
    fn 別モジュールのschemaに対してgraphリテラルが構築できる() {
        let g = graphite::graph!(CrossModuleOrg {
            tanaka = Employee { name: "田中".into() },
            sales = Department { name: "営業".into() },

            tanaka -[belongs_to]-> sales,
            tanaka -[boss = BossEdge { since: 2020 }]-> tanaka,
        })
        .expect("別モジュールの schema に対する graph! も構築に成功するはず");

        assert_eq!(
            g.employee(&EmployeeId("tanaka".to_string())).unwrap().name,
            "田中"
        );
        let dept: &Department = g.belongs_to(&EmployeeId("tanaka".to_string()));
        assert_eq!(dept.name, "営業");
    }
}
