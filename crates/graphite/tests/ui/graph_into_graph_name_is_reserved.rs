#[derive(Clone)]
struct Person;

graphite::__graph_schema_inline_for_test! {
    schema World {
        node Person;
    }
}

fn main() {
    let _ = graphite::graph!(World {
        into_graph = Person,
    });
}
