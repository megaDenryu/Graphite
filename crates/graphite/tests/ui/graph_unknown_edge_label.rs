// `graph_schema!` が生成するハンドシェイク用マクロ (`__graphite_edge_OrgChart!`)
// により、存在しないエッジ種別の参照はまず親切な compile_error! で報告される。
// `graph!` はスキーマの中身を知らないままなので、加えてビルダーに対する通常の
// Rust メソッド解決も走り、`no method named ...` という rustc 標準のエラーも
// 重ねて出る (両方が stderr に現れる)。

pub struct Employee {
    pub name: String,
    pub id: u32,
}

pub struct Department {
    pub name: String,
}

graphite::graph_schema! {
    schema OrgChart {
        node Employee;
        node Department;

        edge Employee -[belongs_to]-> Department (1);
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
