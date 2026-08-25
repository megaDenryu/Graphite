pub struct Person;

graphite::__graph_schema_inline_for_test! {
    schema World {
        node Person;
    }
}

fn ref_outlives_graph<'graph>() -> World::PersonRef<'graph> {
    let graph = graphite::graph!(World {
        alice = Person,
    })
    .unwrap();
    graph.person_by_id(&World::PersonId("alice".into())).unwrap()
}

fn main() {}
