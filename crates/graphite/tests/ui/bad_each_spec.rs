// `each <role>: N..M` は下限が上限を超える範囲を拒否する。

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmployeeId(pub String);

pub struct Employee {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DepartmentId(pub String);

pub struct Department {
    pub name: String,
}

fn main() {
    graphite::__graph_schema_inline_for_test! {
        schema Broken {
            node Employee;
            node Department;

            edge BelongsTo = (employee: Employee) -> (department: Department) where each employee: 5..2;
        }
    }
}
