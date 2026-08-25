struct Person;
struct __PersonNamedPosition;

graphite::__graph_schema_inline_for_test! {
    schema Collision {
        node Person;
        node __PersonNamedPosition;
    }
}

fn main() {}
