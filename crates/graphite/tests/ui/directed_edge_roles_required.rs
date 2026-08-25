struct Person;

graphite::__graph_schema_inline_for_test! {
    schema MissingRoles {
        node Person;
        edge Knows = Person -> Person;
    }
}

fn main() {}
