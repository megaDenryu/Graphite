//! `{Kind}::sources_of` (`docs/reverse_query.md`) の統合テスト。
//!
//! `orgchart_macro.rs` の既存スキーマは目的が違う (v4/v4.1 の実証) ので、
//! `sources_of` 専用の最小スキーマをこのファイルに用意する。カバーする
//! 組み合わせ (積み荷の有無 × 終点側 each 制約):
//!
//! - `Unconstrained`  : 積み荷あり、終点側制約なし → `Vec<(NodeARef, &Weight)>`
//! - `UnconstrainedNoPayload` : 積み荷なし、終点側制約なし → `Vec<NodeARef>`
//! - `AtMostOne`      : 積み荷なし、`each dst: 0..1` → `Option<NodeARef>`
//! - `ExactlyOne`     : 積み荷あり、`each dst: 1` → 直接参照 (パニック +
//!   非パニック版 `get_sources_of`)
//!
//! いずれも役割名つき有向辺 (`docs/edge_endpoints_v4_1.md` §1) でなければ
//! 終点側の each は書けないため、終点側制約のある2種は役割名 (`src`/`dst`)
//! を使う。

/// ノード型。`graph_schema!` はこの型を生成せず参照するだけ。
#[derive(Debug, Clone, PartialEq)]
pub struct NodeA {
    pub name: String,
}

/// ノード型。
#[derive(Debug, Clone, PartialEq)]
pub struct NodeB {
    pub name: String,
}

/// 積み荷型。
#[derive(Debug, Clone, PartialEq)]
pub struct Weight {
    pub w: i32,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema RevQuery {
        node NodeA;
        node NodeB;

        edge Unconstrained = (source: NodeA) -[weight: Weight]-> (target: NodeB);
        edge UnconstrainedNoPayload = (source: NodeA) -> (target: NodeB);
        edge AtMostOne              = (src: NodeA) -> (dst: NodeB)          where each dst: 0..1;
        edge ExactlyOne = (src: NodeA) -[weight: Weight]-> (dst: NodeB) where each dst: 1;
    }
}

use RevQuery::{
    AtMostOne, AtMostOneId, ExactlyOne, ExactlyOneId, NodeAId, NodeBId, Unconstrained,
    UnconstrainedId, UnconstrainedNoPayload, UnconstrainedNoPayloadId,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn na(id: &str) -> NodeAId {
        NodeAId(id.to_string())
    }

    fn nb(id: &str) -> NodeBId {
        NodeBId(id.to_string())
    }

    fn build() -> RevQuery::Graph {
        RevQuery::Graph::create(|g| {
            g.node_a(
                na("a1"),
                NodeA {
                    name: "a1".to_string(),
                },
            );
            g.node_a(
                na("a2"),
                NodeA {
                    name: "a2".to_string(),
                },
            );
            g.node_a(
                na("a3"),
                NodeA {
                    name: "a3".to_string(),
                },
            );
            g.node_b(
                nb("b1"),
                NodeB {
                    name: "b1".to_string(),
                },
            );
            g.node_b(
                nb("b2"),
                NodeB {
                    name: "b2".to_string(),
                },
            );

            // Unconstrained: b1 に a2, a1 の順で入る (挿入順テスト用に敢えて
            // ノード宣言順とは逆順にする)。
            g.unconstrained(
                UnconstrainedId("u1".to_string()),
                Unconstrained::new(na("a2"), nb("b1"), Weight { w: 20 }),
            );
            g.unconstrained(
                UnconstrainedId("u2".to_string()),
                Unconstrained::new(na("a1"), nb("b1"), Weight { w: 10 }),
            );

            // UnconstrainedNoPayload: b1 に a3 のみ。
            g.unconstrained_no_payload(
                UnconstrainedNoPayloadId("un1".to_string()),
                UnconstrainedNoPayload::new(na("a3"), nb("b1")),
            );

            // AtMostOne: b1 の代表は a1 のみ (b2 は代表なし)。
            g.at_most_one(
                AtMostOneId("m1".to_string()),
                AtMostOne::new(na("a1"), nb("b1")),
            );

            // ExactlyOne: b1, b2 ともにちょうど1本。
            g.exactly_one(
                ExactlyOneId("e1".to_string()),
                ExactlyOne::new(na("a1"), nb("b1"), Weight { w: 100 }),
            );
            g.exactly_one(
                ExactlyOneId("e2".to_string()),
                ExactlyOne::new(na("a2"), nb("b2"), Weight { w: 200 }),
            );
        })
        .expect("正常なグラフは構築に成功するはず")
    }

    #[test]
    fn 制約なしかつ積み荷ありはiteratorで辺を返り挿入順を保持する() {
        let g = build();
        let sources: Vec<_> = Unconstrained::of_target(RevQuery::NodeB::get(&g, &nb("b1")).unwrap()).collect();
        assert_eq!(sources.len(), 2);
        // 挿入順 (u1: a2, u2: a1) を保持する — ノード宣言順 (a1, a2, ...) では
        // ない。
        assert_eq!(sources[0].source().name, "a2");
        assert_eq!(sources[0].payload().w, 20);
        assert_eq!(sources[1].source().name, "a1");
        assert_eq!(sources[1].payload().w, 10);
    }

    #[test]
    fn 制約なしかつ積み荷なしはvecでノード値のみ返す() {
        let g = build();
        let sources: Vec<_> = UnconstrainedNoPayload::of_target(RevQuery::NodeB::get(&g, &nb("b1")).unwrap()).collect();
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].source().name, "a3");

        assert!(UnconstrainedNoPayload::of_target(RevQuery::NodeB::get(&g, &nb("b2")).unwrap()).next().is_none());
    }

    #[test]
    fn 終点側0か1制約かつ積み荷なしはoptionを返す() {
        let g = build();

        let m = AtMostOne::of_dst(RevQuery::NodeB::get(&g, &nb("b1")).unwrap());
        assert_eq!(m.expect("b1の代表はa1のはず").src().name, "a1");

        let none = AtMostOne::of_dst(RevQuery::NodeB::get(&g, &nb("b2")).unwrap());
        assert!(none.is_none(), "b2には代表がいないはず");
    }

    #[test]
    fn 終点側ちょうど1制約かつ積み荷ありは直接参照を返す() {
        let g = build();

        let edge = ExactlyOne::of_dst(RevQuery::NodeB::get(&g, &nb("b1")).unwrap());
        assert_eq!(edge.src().name, "a1");
        assert_eq!(edge.payload().w, 100);

        let edge2 = ExactlyOne::of_dst(RevQuery::NodeB::get(&g, &nb("b2")).unwrap());
        assert_eq!(edge2.src().name, "a2");
        assert_eq!(edge2.payload().w, 200);
    }

    #[test]
    fn sources_ofは相手側から見た関係でありofとは非対称() {
        // Unconstrained::of_source(a1) は a1 を始点とする辺 (終点側は b1) を
        // 返す。sources_of(&g, &b1) はその逆で a1 を含む始点側の一覧を返す
        // (自分自身が相手にとってのsources_ofに現れることを確認する)。
        let g = build();
        let targets: Vec<_> = Unconstrained::of_source(RevQuery::NodeA::get(&g, &na("a1")).unwrap()).collect();
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].target().name, "b1");

        let sources = Unconstrained::of_target(RevQuery::NodeB::get(&g, &nb("b1")).unwrap());
        assert!(sources.into_iter().any(|edge| edge.source().name == "a1"));
    }
}
