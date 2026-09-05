//! schema module内の既定IDと既存ID型の明示指定を検証する。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 再設計待ち)。この
//! ファイルは検証対象1つ (既定IDと明示ID型) に対するテスト用スキーマとテス
//! ト関数の列を持つ。`each_declaration_order.rs` が `#[path]` で宣言を親に
//! 残したままテストを部分モジュールへ出す技法を実証したため、このファイルの
//! 分割が同じ宣言を各ファイルへ複製するという統合の根拠は成り立たない。検証
//! 観点ごとに部分モジュールへ分ける判定を issue #28 のやること4 で行う。超
//! 過を許す根拠の台帳は `docs/development/line_count_ledger.md` にある。

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

    #[allow(non_snake_case, dead_code, private_interfaces)]
    #[allow(
        clippy::needless_lifetimes,
        clippy::wrong_self_convention,
        clippy::clone_on_copy,
        clippy::write_literal
    )]
    pub mod QualifiedIds {
        include!("generated/schema_ids_qualified_ids.rs");
    }

    #[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/schema_ids_qualified_ids.rs";
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

        assert!(graph.person_by_id(&KnowsId(1)).is_some());
    }
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(
    clippy::needless_lifetimes,
    clippy::wrong_self_convention,
    clippy::clone_on_copy,
    clippy::write_literal
)]
pub mod MixedIds {
    include!("generated/schema_ids_mixed_ids.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/schema_ids_mixed_ids.rs";
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
        external_friend @ ExternalEdgeId(50) = ExternalFriend(left -- right),

        auto_a = AutomaticNode { name: "a" },
        auto_b @ MixedIds::AutomaticNodeId("custom-b".into()) = AutomaticNode { name: "b" },
        auto_edge = AutomaticLink(auto_a -> auto_b),
        boolean @ 1 == 1 = BooleanNode,
    })
    .expect("既定IDと明示IDを混在させたグラフを構築できるはず");

    assert_eq!(
        graph.external_node_by_id(&ExternalNodeId(10)).unwrap().name,
        "left"
    );
    assert!(graph.external_link_ids().next() == Some(&ExternalEdgeId(30)));
    assert_eq!(
        graph.automatic_node_by_id(&MixedIds::AutomaticNodeId("custom-b".into()))
        .unwrap()
        .name,
        "b"
    );
    assert_eq!(
        graph.automatic_link_ids().next(),
        Some(&MixedIds::AutomaticLinkId("auto_edge".into()))
    );
    assert!(graph.boolean_node_by_id(&true).is_some());
}

#[test]
fn debugは安全に表示できるidだけを含める() {
    let generated = MixedIds::AutomaticLink::new(
        MixedIds::AutomaticNodeId("a".into()),
        MixedIds::AutomaticNodeId("b".into()),
    );
    assert_eq!(
        format!("{generated:?}"),
        "AutomaticLink(AutomaticNodeId(\"a\"), AutomaticNodeId(\"b\"))"
    );

    let explicit = MixedIds::ExternalLink::new(ExternalNodeId(1), ExternalNodeId(2));
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
