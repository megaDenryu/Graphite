#[derive(Clone, PartialEq, Eq, Hash)]
struct ExternalNodeId(u64);

#[derive(Clone, PartialEq, Eq, Hash)]
struct ExternalEdgeId(u64);

struct Person;

graphite::__graph_schema_inline_for_test! {
    schema Explicit {
        node Person(id: ExternalNodeId);
        edge Knows(id: ExternalEdgeId) = (knower: Person) -> (known: Person);
    }
}

fn main() {
    let _: Explicit::PersonId;
    let _: Explicit::KnowsId;
}
