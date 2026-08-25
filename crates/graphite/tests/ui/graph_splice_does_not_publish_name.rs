#[derive(Clone)]
struct Person;

graphite::__graph_schema_inline_for_test! {
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
