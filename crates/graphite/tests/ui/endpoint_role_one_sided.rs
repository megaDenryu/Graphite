// 有向辺では両端の役割名が必須であるため、片側だけの宣言は構文エラーになる。

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmployeeId(pub String);

pub struct Employee {
    pub name: String,
}

fn main() {
    graphite::graph_schema! {
        schema Broken {
            node Employee;

            edge Boss = (subordinate: Employee) -> Employee where each subordinate: 0..1;
        }
    }
}
