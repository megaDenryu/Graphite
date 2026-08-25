struct Person;

graphite::__graph_schema_inline_for_test! {
    schema UndirectedEach {
        node Person;
        edge Friends = Person -- Person where each Person: 1;
    }
}

fn main() {}
