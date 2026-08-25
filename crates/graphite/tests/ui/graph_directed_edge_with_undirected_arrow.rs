#[derive(Clone)]
struct Person;

graphite::__graph_schema_inline_for_test! {
    schema Social {
        node Person;
        edge Knows = (knower: Person) -> (known: Person);
    }
}

fn main() {
    let _ = graphite::graph!(Social {
        alice = Person,
        bob = Person,
        relation = Knows(alice -- bob),
    });
}
