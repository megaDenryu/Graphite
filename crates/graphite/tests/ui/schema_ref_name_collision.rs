struct Person;
struct PersonRef;

graphite::__graph_schema_inline_for_test! {
    schema Collision {
        node Person;
        node PersonRef;
    }
}

fn main() {}
