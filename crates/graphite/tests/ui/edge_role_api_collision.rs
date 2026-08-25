struct Person;

graphite::__graph_schema_inline_for_test! {
    schema ApiCollision {
        node Person;
        edge Knows = (from: Person) -> (known: Person);
    }
}

fn main() {}
