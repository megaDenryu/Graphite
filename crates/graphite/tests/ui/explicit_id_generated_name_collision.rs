#[derive(Clone, PartialEq, Eq, Hash)]
struct KnowsId(u64);

struct Person;

graphite::graph_schema! {
    schema Collision {
        node Person(id: KnowsId);
        edge Knows = (knower: Person) -> (known: Person);
    }
}

fn main() {}
