#[derive(Clone, PartialEq, Eq, Hash)]
struct KnowsId(u64);

struct Person;

graphite::__graph_schema_inline_for_test! {
    schema Collision {
        node Person(id: KnowsId);
        edge Knows = (knower: Person) -> (known: Person);
    }
}

fn main() {}
