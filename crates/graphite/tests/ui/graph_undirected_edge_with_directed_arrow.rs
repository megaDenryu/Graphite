#[derive(Clone)]
struct Person;

graphite::graph_schema! {
    schema Social {
        node Person;
        edge Friends = Person -- Person;
    }
}

fn main() {
    let _ = graphite::graph!(Social {
        alice = Person,
        bob = Person,
        relation = Friends(alice -> bob),
    });
}
