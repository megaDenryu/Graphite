#[derive(Clone)]
struct Person;

graphite::graph_schema! {
    schema World {
        node Person;
    }
}

fn main() {
    let _ = graphite::graph!(World {
        into_graph = Person,
    });
}
