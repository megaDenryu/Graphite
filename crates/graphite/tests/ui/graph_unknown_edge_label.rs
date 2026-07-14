// v3 (`docs/graph_literal_v3.md` §4): ハンドシェイクマクロを全廃したため、
// 存在しないエッジ種別の参照は素の rustc メソッド解決だけに委ねられる。
// `graph!` はビルダーに対して `no_such_label` メソッドを直接呼ぶだけの
// コードへ脱糖するので、`no method named ...` (E0599) が単独で出る
// (旧版にあった「利用可能なエッジ一覧」付きの compile_error! は無くなった。
// これは意図した trade-off — `docs/graph_literal_v3.md` §4 のユーザー決定)。

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
        tanaka = Employee { name: "田中".into(), id: 1 },
        sales = Department { name: "営業".into() },

        tanaka -[not_a_real_edge]-> sales,
    });
}
