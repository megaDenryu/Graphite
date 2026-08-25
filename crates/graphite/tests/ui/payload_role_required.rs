struct Person;
struct Details;

graphite::__graph_schema_inline_for_test! {
    schema MissingPayloadRole {
        node Person;
        edge Knows = (knower: Person) -[Details]-> (known: Person);
    }
}

fn main() {}
