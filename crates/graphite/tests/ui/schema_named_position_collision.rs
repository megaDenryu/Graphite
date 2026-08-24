struct Person;
struct __PersonNamedPosition;

graphite::graph_schema! {
    schema Collision {
        node Person;
        node __PersonNamedPosition;
    }
}

fn main() {}
