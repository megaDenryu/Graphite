//! schemaごとのRustモジュールが、生成型と問い合わせ名前空間を分離することを確認する。

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema Org {
        node Person;
        edge Relation = (source: Person) -> (target: Person);
    }
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema Social {
        node Person;
        edge Relation = (source: Person) -> (target: Person);
    }
}

#[test]
#[rustfmt::skip]
fn 同じノード値型と辺名を複数schemaで使っても生成型が衝突しない() {
    let org: Org::Graph = graphite::graph!(Org {
        manager = Person { name: "Manager".into() },
        member = Person { name: "Member".into() },
        manages = Relation(manager -> member),
    })
    .expect("Orgの構築に成功するはず");

    let social: Social::Graph = graphite::graph!(Social {
        alice = Person { name: "Alice".into() },
        bob = Person { name: "Bob".into() },
        follows = Relation(alice -> bob),
    })
    .expect("Socialの構築に成功するはず");

    assert_eq!(Org::Person::get(&org, &Org::PersonId("manager".into())).unwrap().name, "Manager");
    assert_eq!(Social::Person::get(&social, &Social::PersonId("alice".into())).unwrap().name, "Alice");
    assert_eq!(Org::Relation::len(&org), 1);
    assert_eq!(Social::Relation::len(&social), 1);
}

#[derive(Debug, Clone, PartialEq)]
pub struct 人物 {
    pub 名前: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct 取引情報 {
    pub 金額: u64,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema 世界 {
        node 人物;
        edge 関係 = (始点: 人物) -[明細: 取引情報]-> (終点: 人物);
    }
}

#[test]
#[rustfmt::skip]
fn 日本語のschema名とノード名と辺名と束縛名を通常の識別子として使える() {
    let graph: 世界::Graph = graphite::graph!(世界 {
        太郎 = 人物 { 名前: "太郎".into() },
        次郎 = 人物 { 名前: "次郎".into() },
        知人 = 関係(太郎 -[取引情報 { 金額: 100 }]-> 次郎),
    })
    .expect("日本語識別子を使ったグラフの構築に成功するはず");

    assert_eq!(世界::人物::get(&graph, &世界::人物Id("太郎".into())).unwrap().名前, "太郎");
    assert_eq!(世界::関係::len(&graph), 1);
    let edge = 世界::関係::ids(&graph)
        .next()
        .and_then(|id| 世界::関係::get(&graph, id))
        .expect("日本語roleを持つedgeを取得できるはず");
    assert_eq!(edge.始点, 世界::人物Id("太郎".into()));
    assert_eq!(edge.終点, 世界::人物Id("次郎".into()));
    assert_eq!(edge.明細.金額, 100);
}
