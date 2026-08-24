#[derive(Clone)]
struct Person;

graphite::graph_schema! {
    schema World {
        node Person;
    }
}

fn main() {
    let people = vec![("spliced", Person)];
    let graph = graphite::graph!(World {
        declared = Person,
        ..people,
    })
    .unwrap();

    let _ = graph.spliced();
}
