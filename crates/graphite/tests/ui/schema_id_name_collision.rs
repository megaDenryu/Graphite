struct Person;
struct PersonId;

graphite::graph_schema! {
    schema Collision {
        node Person;
        node PersonId;
    }
}

fn main() {}

