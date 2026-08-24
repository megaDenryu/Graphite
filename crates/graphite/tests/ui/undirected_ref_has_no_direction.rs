pub struct Person;

graphite::graph_schema! {
    schema Social {
        node Person;
        edge Friend = Person -- Person;
    }
}

fn main() {
    let graph = graphite::graph!(Social {
        alice = Person,
        bob = Person,
        friendship = Friend(alice -- bob),
    })
    .unwrap();
    let edge = Social::Friend::get(
        &graph,
        &Social::FriendId("friendship".to_string()),
    )
    .unwrap();
    let _ = edge.from();
    let _ = edge.to();
}
