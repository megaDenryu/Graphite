struct Person;

graphite::graph_schema! {
    schema MissingRoles {
        node Person;
        edge Knows = Person -> Person;
    }
}

fn main() {}
