pub struct Person;

graphite::graph_schema! {
    schema World {
        node Person;
    }
}

fn ref_outlives_graph<'graph>() -> World::PersonRef<'graph> {
    let graph = graphite::graph!(World {
        alice = Person,
    })
    .unwrap();
    World::Person::get(&graph, &World::PersonId("alice".into())).unwrap()
}

fn main() {}
