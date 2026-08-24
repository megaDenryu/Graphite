struct Person;

graphite::graph_schema! {
    schema Org {
        node Person;
        edge Relation = (source: Person) -> (target: Person);
    }
}

graphite::graph_schema! {
    schema Social {
        node Person;
        edge Relation = (source: Person) -> (target: Person);
    }
}

fn main() {
    let _: Org::PersonId = Social::PersonId("person".to_string());
    let _: Org::RelationId = Social::RelationId("relation".to_string());
}
