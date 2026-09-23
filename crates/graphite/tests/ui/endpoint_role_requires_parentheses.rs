struct Person;

graphite::__dynamic_graph_schema_inline_for_test! {
    schema MissingParentheses {
        node Person;
        edge Knows = knower: Person -> (known: Person);
    }
}

fn main() {}
