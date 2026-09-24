struct Person;

graphite::__dynamic_graph_schema_inline_for_test! {
    schema DuplicateRole {
        node Person;
        edge Knows = (person: Person) -> (person: Person);
    }
}

fn main() {}
