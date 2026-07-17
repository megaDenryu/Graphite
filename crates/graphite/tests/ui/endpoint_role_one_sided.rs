// v4.1: 役割名は両端同時に書くか、両方省略するかの二択 (片方だけは構文
// エラー、`docs/edge_endpoints_v4_1.md` §1)。

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
