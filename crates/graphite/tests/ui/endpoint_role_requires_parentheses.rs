struct Person;

graphite::graph_schema! {
    schema MissingParentheses {
        node Person;
        edge Knows = knower: Person -> (known: Person);
    }
}

fn main() {}
