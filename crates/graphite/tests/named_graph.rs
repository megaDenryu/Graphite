#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PurchaseInfo {
    pub amount: u32,
}

graphite::graph_schema! {
    schema NamedWorld {
        node Person;
        node Item;

        edge Purchase = (buyer: Person) -[info: PurchaseInfo]-> (item: Item);
        edge Knows = (knower: Person) -> (known: Person);
    }
}

use NamedWorld::{ItemId, Knows, KnowsId, PersonId, PurchaseId};

#[test]
fn 名前付きnodeとedgeは内部位置からrefを返す() {
    let graph = graphite::graph!(NamedWorld {
        太郎 @ PersonId("public-person-42".into()) = Person { name: "太郎".into() },
        本 @ ItemId("public-item-7".into()) = Item { name: "本".into() },
        購入 @ PurchaseId("public-purchase-9001".into()) =
            Purchase(太郎 -[PurchaseInfo { amount: 100 }]-> 本),
    })
    .expect("名前付きグラフを構築できるはず");

    assert_eq!(graph.太郎().id(), &PersonId("public-person-42".into()));
    assert_eq!(graph.太郎().name, "太郎");
    assert_eq!(graph.本().id(), &ItemId("public-item-7".into()));
    assert_eq!(
        graph.購入().id(),
        &PurchaseId("public-purchase-9001".into())
    );
    assert_eq!(graph.購入().buyer().id(), graph.太郎().id());
    assert_eq!(graph.購入().item().id(), graph.本().id());
    assert_eq!(graph.購入().payload().amount, 100);

    // 公開IDの動的検索は、静的アクセサとは独立した従来経路として残る。
    assert_eq!(
        NamedWorld::Person::get(&graph, &PersonId("public-person-42".into()))
            .unwrap()
            .name,
        "太郎"
    );
}

#[test]
fn spliceを名前付き辺の前後に置いても位置は正しい() {
    let middle = vec![
        (
            "middle-1".to_string(),
            Knows::new(PersonId("alice".into()), PersonId("bob".into())),
        ),
        (
            "middle-2".to_string(),
            Knows::new(PersonId("bob".into()), PersonId("alice".into())),
        ),
    ];

    let graph = graphite::graph!(NamedWorld {
        alice = Person { name: "Alice".into() },
        bob = Person { name: "Bob".into() },
        first = Knows(alice -> bob),
        ..middle,
        last = Knows(bob -> bob),
    })
    .expect("spliceを含む名前付きグラフを構築できるはず");

    assert_eq!(graph.first().id(), &KnowsId("first".into()));
    assert_eq!(graph.last().id(), &KnowsId("last".into()));
    assert_eq!(graph.last().knower().id(), &PersonId("bob".into()));
    assert_eq!(Knows::iter(&graph).count(), 4);
}

#[test]
fn 同じ関数内でgraphを2回展開してもwrapper名は衝突しない() {
    let first = graphite::graph!(NamedWorld {
        alice = Person { name: "Alice".into() },
    })
    .unwrap();
    let second = graphite::graph!(NamedWorld {
        bob = Person { name: "Bob".into() },
    })
    .unwrap();

    assert_eq!(first.alice().name, "Alice");
    assert_eq!(second.bob().name, "Bob");
}

#[test]
fn into_graphで名前付きapiを捨てて公開境界へ素のgraphを渡せる() {
    let named = graphite::graph!(NamedWorld {
        alice = Person { name: "Alice".into() },
    })
    .unwrap();

    let graph: NamedWorld::Graph = named.into_graph();
    assert!(NamedWorld::Person::get(&graph, &PersonId("alice".into())).is_some());
}
