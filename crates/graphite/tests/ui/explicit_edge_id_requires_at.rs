#[derive(Clone, PartialEq, Eq, Hash)]
struct ExternalEdgeId(u64);

struct Person;

graphite::__graph_schema_inline_for_test! {
    schema ExplicitEdge {
        node Person;
        edge Knows(id: ExternalEdgeId) = (knower: Person) -> (known: Person);
    }
}

fn main() {
    let _ = graphite::graph!(ExplicitEdge {
        a = Person,
        b = Person,
        relation = Knows(a -> b),
    });
}
