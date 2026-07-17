//! `{Kind}::sources_of` (`docs/reverse_query.md`) の統合テスト。
//!
//! `orgchart_macro.rs` の既存スキーマは目的が違う (v4/v4.1 の実証) ので、
//! `sources_of` 専用の最小スキーマをこのファイルに用意する。カバーする
//! 組み合わせ (積み荷の有無 × 終点側 each 制約):
//!
//! - `Unconstrained`  : 積み荷あり、終点側制約なし → `Vec<(&NodeA, &Weight)>`
//! - `UnconstrainedNoPayload` : 積み荷なし、終点側制約なし → `Vec<&NodeA>`
//! - `AtMostOne`      : 積み荷なし、`each dst: 0..1` → `Option<&NodeA>`
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

        edge Unconstrained          = NodeA -[Weight]-> NodeB;
        edge UnconstrainedNoPayload = NodeA -> NodeB;
        edge AtMostOne              = (src: NodeA) -> (dst: NodeB)          where each dst: 0..1;
        edge ExactlyOne             = (src: NodeA) -[Weight]-> (dst: NodeB) where each dst: 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn na(id: &str) -> NodeAId {
        NodeAId(id.to_string())
    }

    fn nb(id: &str) -> NodeBId {
        NodeBId(id.to_string())
    }

    fn build() -> RevQuery {
        RevQuery::create(|g| {
            g.node_a(na("a1"), NodeA { name: "a1".to_string() });
            g.node_a(na("a2"), NodeA { name: "a2".to_string() });
            g.node_a(na("a3"), NodeA { name: "a3".to_string() });
            g.node_b(nb("b1"), NodeB { name: "b1".to_string() });
            g.node_b(nb("b2"), NodeB { name: "b2".to_string() });

            // Unconstrained: b1 に a2, a1 の順で入る (挿入順テスト用に敢えて
            // ノード宣言順とは逆順にする)。
            g.unconstrained(
                UnconstrainedId("u1".to_string()),
                Unconstrained(na("a2"), nb("b1"), Weight { w: 20 }),
            );
            g.unconstrained(
                UnconstrainedId("u2".to_string()),
                Unconstrained(na("a1"), nb("b1"), Weight { w: 10 }),
            );

            // UnconstrainedNoPayload: b1 に a3 のみ。
            g.unconstrained_no_payload(
                UnconstrainedNoPayloadId("un1".to_string()),
                UnconstrainedNoPayload(na("a3"), nb("b1")),
            );

            // AtMostOne: b1 の代表は a1 のみ (b2 は代表なし)。
            g.at_most_one(AtMostOneId("m1".to_string()), AtMostOne(na("a1"), nb("b1")));

            // ExactlyOne: b1, b2 ともにちょうど1本。
            g.exactly_one(
                ExactlyOneId("e1".to_string()),
                ExactlyOne(na("a1"), nb("b1"), Weight { w: 100 }),
            );
            g.exactly_one(
                ExactlyOneId("e2".to_string()),
                ExactlyOne(na("a2"), nb("b2"), Weight { w: 200 }),
            );
        })
        .expect("正常なグラフは構築に成功するはず")
    }

    #[test]
    fn 制約なしかつ積み荷ありはvecで積み荷付きで返り挿入順を保持する() {
        let g = build();
        let sources = Unconstrained::sources_of(&g, &nb("b1"));
        assert_eq!(sources.len(), 2);
        // 挿入順 (u1: a2, u2: a1) を保持する — ノード宣言順 (a1, a2, ...) では
        // ない。
        assert_eq!(sources[0].0.name, "a2");
        assert_eq!(sources[0].1.w, 20);
        assert_eq!(sources[1].0.name, "a1");
        assert_eq!(sources[1].1.w, 10);
    }

    #[test]
    fn 制約なしかつ積み荷ありは未知キーで空vecを返す() {
        let g = build();
        assert!(Unconstrained::sources_of(&g, &nb("存在しないb")).is_empty());
    }

    #[test]
    fn 制約なしかつ積み荷なしはvecでノード値のみ返す() {
        let g = build();
        let sources: Vec<&NodeA> = UnconstrainedNoPayload::sources_of(&g, &nb("b1"));
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].name, "a3");

        assert!(UnconstrainedNoPayload::sources_of(&g, &nb("b2")).is_empty());
    }

    #[test]
    fn 終点側0か1制約かつ積み荷なしはoptionを返す() {
        let g = build();

        let m: Option<&NodeA> = AtMostOne::sources_of(&g, &nb("b1"));
        assert_eq!(m.expect("b1の代表はa1のはず").name, "a1");

        let none: Option<&NodeA> = AtMostOne::sources_of(&g, &nb("b2"));
        assert!(none.is_none(), "b2には代表がいないはず");
    }

    #[test]
    fn 終点側0か1制約は未知キーでnoneを返す() {
        let g = build();
        assert!(AtMostOne::sources_of(&g, &nb("存在しないb")).is_none());
    }

    #[test]
    fn 終点側ちょうど1制約かつ積み荷ありは直接参照を返す() {
        let g = build();

        let (source, weight) = ExactlyOne::sources_of(&g, &nb("b1"));
        assert_eq!(source.name, "a1");
        assert_eq!(weight.w, 100);

        let (source2, weight2) = ExactlyOne::sources_of(&g, &nb("b2"));
        assert_eq!(source2.name, "a2");
        assert_eq!(weight2.w, 200);
    }

    #[test]
    #[should_panic(expected = "ExactlyOne::sources_of")]
    fn 終点側ちょうど1制約は未知キーでパニックする() {
        let g = build();
        let _ = ExactlyOne::sources_of(&g, &nb("存在しないb"));
    }

    #[test]
    fn 終点側ちょうど1制約のget_sources_ofは未知キーでnoneを返す() {
        let g = build();
        assert!(ExactlyOne::get_sources_of(&g, &nb("存在しないb")).is_none());
        let (source, weight) = ExactlyOne::get_sources_of(&g, &nb("b1")).expect("b1は存在するはず");
        assert_eq!(source.name, "a1");
        assert_eq!(weight.w, 100);
    }

    #[test]
    fn sources_ofは相手側から見た関係でありofとは非対称() {
        // Unconstrained::of(&g, &a1) は a1 を始点とする辺の終点側 (b1) を
        // 返す。sources_of(&g, &b1) はその逆で a1 を含む始点側の一覧を返す
        // (自分自身が相手にとってのsources_ofに現れることを確認する)。
        let g = build();
        let targets: Vec<(&NodeB, &Weight)> = Unconstrained::of(&g, &na("a1"));
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0].0.name, "b1");

        let sources = Unconstrained::sources_of(&g, &nb("b1"));
        assert!(sources.iter().any(|(src, _)| src.name == "a1"));
    }
}
