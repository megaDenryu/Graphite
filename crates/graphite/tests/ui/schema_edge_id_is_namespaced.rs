#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PersonId(String);

struct Person;

graphite::graph_schema! {
    schema Org {
        node Person;
        edge Relation = Person -> Person;
    }
}

graphite::graph_schema! {
    schema Social {
        node Person;
        edge Relation = Person -> Person;
    }
}

fn main() {
    let _: Org::RelationId = Social::RelationId("relation".to_string());
}
