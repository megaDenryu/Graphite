#[derive(Clone, PartialEq, Eq, Hash)]
struct ExternalId(u64);

struct Person;

graphite::__graph_schema_inline_for_test! {
    schema Explicit {
        node Person(id: ExternalId);
    }
}

fn main() {
    let _ = graphite::graph!(Explicit {
        person = Person,
    });
}

