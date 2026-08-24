pub struct Person;
pub struct Details;

graphite::graph_schema! {
    schema Invalid {
        node Person;
        edge Knows = (knower: Person) -[payload_mut: Details]-> (known: Person);
    }
}

fn main() {}
