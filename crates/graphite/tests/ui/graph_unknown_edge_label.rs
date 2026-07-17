// v4: ハンドシェイクマクロは使わないため、存在しない辺種別 (Kind) の参照は
// 素の rustc 名前解決だけに委ねられる。`graph!` の辺項
// `key = Kind(from -> to)` はタプル struct 構築式 `Kind(from.clone(), to.clone())`
// へ脱糖するので、`Kind` が実在しなければ「そんな関数/型は無い」という
// E0425 が単独で出る (`docs/schema_v4.md` は「利用可能な辺種別一覧」付きの
// compile_error! を要求していない。v3 からの trade-off を踏襲)。

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmployeeId(pub String);

pub struct Employee {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DepartmentId(pub String);

pub struct Department {
    pub name: String,
}

graphite::graph_schema! {
    schema OrgChart {
        node Employee;
        node Department;

        edge BelongsTo = Employee -> Department where each Employee: 1;
    }
}

fn main() {
    #[rustfmt::skip]
    let _ = graphite::graph!(OrgChart {
        tanaka = Employee { name: "田中".into(), id: 1 },
        sales = Department { name: "営業".into() },

        oops = NotARealEdge(tanaka -> sales),
    });
}
