struct Person;

graphite::graph_schema! {
    schema ApiCollision {
        node Person;
        edge Knows = (from: Person) -> (known: Person);
    }
}

fn main() {}
