#[derive(Clone, PartialEq, Eq, Hash)]
struct ExternalId(u64);

struct Person;

graphite::graph_schema! {
    schema Explicit {
        node Person(id: ExternalId);
    }
}

fn main() {
    let _ = graphite::graph!(Explicit {
        person = Person,
    });
}

