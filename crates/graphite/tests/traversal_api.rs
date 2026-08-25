#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    name: String,
}

graphite::graph_schema! {
    schema Traversal {
        node Person;
        node Product;

        edge Purchase = (buyer: Person) -> (product: Product);
        edge Mentor = (subordinate: Person) -> (superior: Person) where each subordinate: 0..1;
        edge 関係 = (始点: Person) -> (終点: Person) where unique pair;
        edge Friends = Person -- Person where unique pair;
    }
}

use Traversal::{
    Friends, FriendsId, Mentor, MentorId, PersonId, ProductId, Purchase, PurchaseId, 関係, 関係Id,
};

fn person(id: &str) -> PersonId {
    PersonId(id.to_string())
}

fn product(id: &str) -> ProductId {
    ProductId(id.to_string())
}

fn build() -> Traversal::Graph {
    Traversal::Graph::create(|b| {
        for name in ["alice", "bob", "carol"] {
            b.person(
                person(name),
                Person {
                    name: name.to_string(),
                },
            );
        }
        for name in ["book", "pen"] {
            b.product(
                product(name),
                Product {
                    name: name.to_string(),
                },
            );
        }

        b.purchase(
            PurchaseId("second".to_string()),
            Purchase::new(person("alice"), product("pen")),
        );
        b.purchase(
            PurchaseId("first".to_string()),
            Purchase::new(person("alice"), product("book")),
        );
        b.mentor(
            MentorId("mentor".to_string()),
            Mentor::new(person("alice"), person("bob")),
        );
        b.関係(
            関係Id("日本語".to_string()),
            関係::new(person("bob"), person("carol")),
        );
        b.friends(
            FriendsId("friends".to_string()),
            Friends::new(person("alice"), person("bob")),
        );
    })
    .unwrap()
}

#[test]
fn node_refと役割apiはedge_refを挿入順で返す() {
    let graph = build();
    let alice = Traversal::Person::get(&graph, &person("alice")).unwrap();
    let via_node: Vec<_> = alice
        .purchase_as_buyer()
        .map(|edge| edge.id().0.as_str())
        .collect();
    let via_marker: Vec<_> = Purchase::of_buyer(alice)
        .map(|edge| edge.id().0.as_str())
        .collect();

    assert_eq!(via_node, ["second", "first"]);
    assert_eq!(via_marker, via_node);
    assert_eq!(
        alice
            .mentor_as_subordinate()
            .expect("aliceにはmentorがある")
            .superior()
            .name,
        "bob"
    );
    assert_eq!(alice.mentor_as_superior().count(), 0);
}

#[test]
fn 日本語役割名と自己型辺の両役割は曖昧にならない() {
    let graph = build();
    let bob = Traversal::Person::get(&graph, &person("bob")).unwrap();
    let carol = Traversal::Person::get(&graph, &person("carol")).unwrap();

    assert_eq!(bob.関係_as_始点().next().unwrap().終点().name, "carol");
    assert_eq!(関係::of_終点(carol).next().unwrap().始点().name, "bob");
    assert_eq!(
        bob.mentor_as_superior().next().unwrap().subordinate().name,
        "alice"
    );
}

#[test]
fn betweenは生成元graphを検査し非panic版も提供する() {
    let first = build();
    let second = build();
    let alice = Traversal::Person::get(&first, &person("alice")).unwrap();
    let bob_other = Traversal::Person::get(&second, &person("bob")).unwrap();

    let error = 関係::try_between(alice, bob_other).unwrap_err();
    assert!(error.to_string().contains("同じ Graph の値"));
    assert!(std::panic::catch_unwind(|| 関係::between(alice, bob_other)).is_err());
    assert!(Friends::try_between(alice, bob_other).is_err());
}

#[test]
fn 無向辺はincidentと順序なしbetweenを提供する() {
    let graph = build();
    let alice = Traversal::Person::get(&graph, &person("alice")).unwrap();
    let bob = Traversal::Person::get(&graph, &person("bob")).unwrap();

    assert_eq!(alice.friends_incident().count(), 1);
    assert!(Friends::between(alice, bob).is_some());
    assert!(Friends::between(bob, alice).is_some());
}
