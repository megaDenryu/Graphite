#[derive(Clone, PartialEq, Eq, Hash)]
struct ExternalNodeId(u64);

#[derive(Clone, PartialEq, Eq, Hash)]
struct ExternalEdgeId(u64);

struct Person;

graphite::graph_schema! {
    schema Explicit {
        node Person(id: ExternalNodeId);
        edge Knows(id: ExternalEdgeId) = Person -> Person;
    }
}

fn main() {
    let _: Explicit::PersonId;
    let _: Explicit::KnowsId;
}
