//! schema module内の既定IDと既存ID型の明示指定を検証する。

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExternalNodeId(pub u64);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ExternalEdgeId(pub u64);

pub struct ExternalNode {
    pub name: &'static str,
}

pub struct AutomaticNode {
    pub name: &'static str,
}

pub struct BooleanNode;

mod 修飾済みid {
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub struct KnowsId(pub u64);

    pub struct Person;

    graphite::graph_schema! {
        schema QualifiedIds {
            node Person(id: super::KnowsId);
            edge Knows = (knower: Person) -> (known: Person);
        }
    }

    #[test]
    fn 生成名と同名の既存idはsuperで修飾できる() {
        let graph = graphite::graph!(QualifiedIds {
            a @ KnowsId(1) = Person,
            b @ KnowsId(2) = Person,
            relation = Knows(a -> b),
        })
        .expect("修飾した既存ID型を利用できるはず");

        assert!(QualifiedIds::Person::get(&graph, &KnowsId(1)).is_some());
    }
}

graphite::graph_schema! {
    schema MixedIds {
        node ExternalNode(id: ExternalNodeId);
        node AutomaticNode;
        node BooleanNode(id: bool);

        edge ExternalLink(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each source: 1;
        edge ExternalIncoming(id: ExternalEdgeId) = (source: ExternalNode) -> (target: ExternalNode) where each target: 1;
        edge ExternalFriend(id: ExternalEdgeId) = ExternalNode -- ExternalNode;
        edge AutomaticLink = (source: AutomaticNode) -> (target: AutomaticNode);
    }
}

#[test]
#[rustfmt::skip]
fn defaultと明示のid型を同じschemaで使える() {
    let graph = graphite::graph!(MixedIds {
        left @ ExternalNodeId(10) = ExternalNode { name: "left" },
        right @ ExternalNodeId(20) = ExternalNode { name: "right" },
        external_edge @ ExternalEdgeId(30) = ExternalLink(left -> right),
        external_edge_reverse @ ExternalEdgeId(31) = ExternalLink(right -> left),
        external_incoming @ ExternalEdgeId(40) = ExternalIncoming(left -> right),
        external_incoming_reverse @ ExternalEdgeId(41) = ExternalIncoming(right -> left),
        external_friend @ ExternalEdgeId(50) = ExternalFriend(left -> right),

        auto_a = AutomaticNode { name: "a" },
        auto_b @ MixedIds::AutomaticNodeId("custom-b".into()) = AutomaticNode { name: "b" },
        auto_edge = AutomaticLink(auto_a -> auto_b),
        boolean @ 1 == 1 = BooleanNode,
    })
    .expect("既定IDと明示IDを混在させたグラフを構築できるはず");

    assert_eq!(
        MixedIds::ExternalNode::get(&graph, &ExternalNodeId(10)).unwrap().name,
        "left"
    );
    assert!(MixedIds::ExternalLink::ids(&graph).next() == Some(&ExternalEdgeId(30)));
    assert_eq!(
        MixedIds::AutomaticNode::get(
            &graph,
            &MixedIds::AutomaticNodeId("custom-b".into()),
        )
        .unwrap()
        .name,
        "b"
    );
    assert_eq!(
        MixedIds::AutomaticLink::ids(&graph).next(),
        Some(&MixedIds::AutomaticLinkId("auto_edge".into()))
    );
    assert!(MixedIds::BooleanNode::get(&graph, &true).is_some());
}

#[test]
fn debugは安全に表示できるidだけを含める() {
    let generated = MixedIds::AutomaticLink(
        MixedIds::AutomaticNodeId("a".into()),
        MixedIds::AutomaticNodeId("b".into()),
    );
    assert_eq!(
        format!("{generated:?}"),
        "AutomaticLink(AutomaticNodeId(\"a\"), AutomaticNodeId(\"b\"))"
    );

    let explicit = MixedIds::ExternalLink(ExternalNodeId(1), ExternalNodeId(2));
    assert_eq!(format!("{explicit:?}"), "ExternalLink");
}

#[test]
fn violationは既定生成idだけを表示する() {
    let generated = match MixedIds::Graph::create(|builder| {
        builder.automatic_node(
            MixedIds::AutomaticNodeId("duplicate".into()),
            AutomaticNode { name: "first" },
        );
        builder.automatic_node(
            MixedIds::AutomaticNodeId("duplicate".into()),
            AutomaticNode { name: "second" },
        );
    }) {
        Err(violation) => violation,
        Ok(_) => panic!("重複した既定IDは拒否されるはず"),
    };
    assert!(generated
        .to_string()
        .contains("AutomaticNodeId(\"duplicate\")"));

    let explicit = match MixedIds::Graph::create(|builder| {
        builder.external_node(ExternalNodeId(900), ExternalNode { name: "first" });
        builder.external_node(ExternalNodeId(900), ExternalNode { name: "second" });
    }) {
        Err(violation) => violation,
        Ok(_) => panic!("重複した明示IDは拒否されるはず"),
    };
    assert_eq!(explicit.to_string(), "ExternalNodeのキーが重複しています");
    assert!(!explicit.to_string().contains("900"));
}
