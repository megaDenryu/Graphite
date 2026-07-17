// v4.1: 無向辺には役割名を書けない (役割の区別がある時点で対称ではない、
// `docs/edge_endpoints_v4_1.md` §2)。

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);

pub struct Person {
    pub name: String,
}

fn main() {
    graphite::graph_schema! {
        schema Broken {
            node Person;

            edge Friends = (a: Person) -- (b: Person);
        }
    }
}
