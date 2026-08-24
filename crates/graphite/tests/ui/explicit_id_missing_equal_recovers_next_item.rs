#[derive(Clone, PartialEq, Eq, Hash)]
struct ExternalId(u64);

struct Person;

graphite::graph_schema! {
    schema Recovery {
        node Person(id: ExternalId);
    }
}

fn main() {
    let _ = graphite::graph!(Recovery {
        bob @ ExternalId(2),
        charlie @ ExternalId(3) = Person,
    });
}
