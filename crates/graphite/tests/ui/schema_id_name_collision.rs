struct Person;
struct PersonId;

graphite::__dynamic_graph_schema_inline_for_test! {
    schema Collision {
        node Person;
        node PersonId;
    }
}

fn main() {}

