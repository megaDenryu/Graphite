// graph! はスキーマの中身を知らないため、存在しないエッジ種別を参照した場合の
// 検査はビルダーに対する通常の Rust メソッド解決に委ねられる (`no method
// named ...` という rustc 標準のエラーになる)。

graphite::graph_schema! {
    schema OrgChart {
        node Employee { name: String, id: u32 }
        node Department { name: String }

        edge belongs_to: Employee -> Department (1);
    }
}

fn main() {
    #[rustfmt::skip]
    let _ = graphite::graph!(OrgChart {
        tanaka: Employee { name: "田中".into(), id: 1 },
        sales: Department { name: "営業".into() },

        tanaka -[not_a_real_edge]-> sales,
    });
}
