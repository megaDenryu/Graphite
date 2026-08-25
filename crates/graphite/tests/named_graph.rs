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

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(
    clippy::needless_lifetimes,
    clippy::wrong_self_convention,
    clippy::clone_on_copy,
    clippy::write_literal
)]
pub mod NamedWorld {
    include!("generated/named_graph_named_world.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/named_graph_named_world.rs";
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
        graph.person_by_id(&PersonId("public-person-42".into()))
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
    assert_eq!(graph.knows_iter().count(), 4);
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
    assert!(graph.person_by_id(&PersonId("alice".into())).is_some());
}

#[test]
#[deny(non_snake_case)]
fn 大文字始まりの左辺名でも内部生成コードは警告を出さない() {
    let graph = graphite::graph!(NamedWorld {
        Alice = Person { name: "Alice".into() },
    })
    .expect("大文字始まりの左辺名でも構築できるはず");

    assert_eq!(graph.Alice().name, "Alice");
}

#[test]
#[should_panic(expected = "生成元と異なる Graph")]
fn 名前付き位置を生成元と異なるgraphへbindするとpanicする() {
    use graphite::NamedGraphElement;

    // `graph!` を経由せず `create_named` を手書きで直接呼び、名前付き位置を
    // クロージャの外へ持ち出す取り違え経路の再現。
    let (_graph_a, alice_position) = NamedWorld::Graph::create_named(|b, permit| {
        let (_, position) = b.insert_named(
            "alice",
            Person {
                name: "Alice".into(),
            },
            permit,
        );
        position
    })
    .expect("グラフAを構築できるはず");

    let (graph_b, _) = NamedWorld::Graph::create_named(|b, permit| {
        b.insert_named("bob", Person { name: "Bob".into() }, permit);
    })
    .expect("グラフBを構築できるはず");

    // 同じschemaの別グラフへ bind すると、構築印の不一致でpanicするはず。
    alice_position.bind(&graph_b);
}
