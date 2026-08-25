pub struct Person;

graphite::__graph_schema_inline_for_test! {
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
    let edge = graph
        .friend_by_id(&Social::FriendId("friendship".to_string()))
        .unwrap();
    let _ = edge.from();
    let _ = edge.to();
}
