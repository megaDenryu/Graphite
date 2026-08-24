struct Person;

graphite::graph_schema! {
    schema DuplicateRole {
        node Person;
        edge Knows = (person: Person) -> (person: Person);
    }
}

fn main() {}
