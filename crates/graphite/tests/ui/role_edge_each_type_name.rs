// v4.1: 役割名つきの辺では each は役割名で参照しなければならない。型名参照は
// 同型端点で曖昧なためエラーになる (`docs/edge_endpoints_v4_1.md` §1)。

pub struct Employee {
    pub name: String,
}

fn main() {
    graphite::graph_schema! {
        schema Broken {
            node Employee;

            edge Boss = (subordinate: Employee) -> (superior: Employee) where each Employee: 0..1;
        }
    }
}
