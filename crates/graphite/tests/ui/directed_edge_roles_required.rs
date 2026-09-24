struct Person;

graphite::__dynamic_graph_schema_inline_for_test! {
    schema MissingRoles {
        node Person;
        edge Knows = Person -> Person;
    }
}

fn main() {}
