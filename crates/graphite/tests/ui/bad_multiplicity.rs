// 不正な多重度 (1) / (0..1) / (0..*) 以外はコンパイルエラーになるはず。

pub struct Employee {
    pub name: String,
}

pub struct Department {
    pub name: String,
}

fn main() {
    graphite::graph_schema! {
        schema Broken {
            node Employee;
            node Department;

            edge belongs_to: Employee -> Department (2..5);
        }
    }
}
