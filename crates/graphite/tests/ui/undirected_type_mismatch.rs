// v4.1: 無向辺の両端は同じノード型でなければならない
// (`docs/edge_endpoints_v4_1.md` §2)。

pub struct Person {
    pub name: String,
}

pub struct Company {
    pub name: String,
}

fn main() {
    graphite::graph_schema! {
        schema Broken {
            node Person;
            node Company;

            edge WorksWith = Person -- Company;
        }
    }
}
