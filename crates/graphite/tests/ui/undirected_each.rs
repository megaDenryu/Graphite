struct Person;

graphite::graph_schema! {
    schema UndirectedEach {
        node Person;
        edge Friends = Person -- Person where each Person: 1;
    }
}

fn main() {}
