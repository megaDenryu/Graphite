struct Person;
struct Details;

graphite::graph_schema! {
    schema MissingPayloadRole {
        node Person;
        edge Knows = (knower: Person) -[Details]-> (known: Person);
    }
}

fn main() {}
