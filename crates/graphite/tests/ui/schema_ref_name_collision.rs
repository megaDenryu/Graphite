struct Person;
struct PersonRef;

graphite::graph_schema! {
    schema Collision {
        node Person;
        node PersonRef;
    }
}

fn main() {}
