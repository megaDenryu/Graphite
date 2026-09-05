//! 無向辺 (`docs/edge_endpoints_v4_1.md` §2) の統合テスト。
//!
//! `Friends` (積み荷なし) と `Wire` (積み荷あり) の2種別で:
//! - `of`/`between` の対称性 (どちらの位置に置かれても検索できる)
//! - `unique pair` の順序無視の同値判定
//! - 自己ループの許可と、次数 (`each`) では1本と数える仕様
//! - `.endpoints()` アクセサ (方向を示すアクセサは生成しない)
//! - 格納順 (挿入順) の保持
//!
//! を確認する。

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cable {
    pub ohm: i32,
}

#[rustfmt::skip]
#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(clippy::needless_lifetimes, clippy::wrong_self_convention, clippy::clone_on_copy, clippy::write_literal)]
pub mod Social {
    include!("generated/undirected_edges_social.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/undirected_edges_social.rs";
    schema Social {
        node Person;

        edge Friends = Person -- Person where unique pair;
        edge Wire = Person -[cable: Cable]- Person;
    }
}

use Social::{Friends, FriendsId, PersonId, Wire, WireId};

#[cfg(test)]
mod tests {
    use super::*;

    fn person(id: &str) -> PersonId {
        PersonId(id.to_string())
    }

    fn build_chart() -> Social::Graph {
        Social::Graph::create(|b| {
            b.person(
                person("alice"),
                Person {
                    name: "Alice".to_string(),
                },
            );
            b.person(
                person("bob"),
                Person {
                    name: "Bob".to_string(),
                },
            );
            b.person(
                person("carol"),
                Person {
                    name: "Carol".to_string(),
                },
            );

            b.friends(
                FriendsId("f1".to_string()),
                Friends::new(person("alice"), person("bob")),
            );
            b.friends(
                FriendsId("f2".to_string()),
                Friends::new(person("carol"), person("alice")),
            );
        })
        .expect("正常な友人関係は構築に成功するはず")
    }

    #[test]
    fn endpointsアクセサで両端を取得できる() {
        let g = build_chart();
        let f = g.friends_by_id(&FriendsId("f1".to_string())).unwrap();
        let (first, second) = f.endpoints();
        assert_eq!(
            (first.id(), second.id()),
            (&person("alice"), &person("bob"))
        );
        assert_eq!(
            Friends::new(person("alice"), person("bob")),
            Friends::new(person("bob"), person("alice"))
        );
    }

    #[test]
    fn 接続探索はどちらの位置に置かれても対称に検索できる() {
        let g = build_chart();

        // alice は f1 の位置0、f2 の位置1 に置かれているが、どちらからでも
        // 相手を辿れる。
        let alice = g.person_by_id(&person("alice")).unwrap();
        let mut friends_of_alice: Vec<String> = alice
            .friends_incident()
            .map(|edge| other(edge.endpoints(), alice).name.clone())
            .collect();
        friends_of_alice.sort();
        assert_eq!(
            friends_of_alice,
            vec!["Bob".to_string(), "Carol".to_string()]
        );

        let bob = g.person_by_id(&person("bob")).unwrap();
        let friends_of_bob: Vec<_> = bob
            .friends_incident()
            .map(|edge| other(edge.endpoints(), bob))
            .collect();
        assert_eq!(friends_of_bob.len(), 1);
        assert_eq!(friends_of_bob[0].name, "Alice");
    }

    #[test]
    fn betweenは順序を無視して対称に検索する() {
        let g = build_chart();
        // `Friends(alice -- bob)` で作った辺だが、between は逆順でも見つかる。
        let alice = g.person_by_id(&person("alice")).unwrap();
        let bob = g.person_by_id(&person("bob")).unwrap();
        let carol = g.person_by_id(&person("carol")).unwrap();
        assert!(alice.friends_between(bob).is_some());
        assert!(bob.friends_between(alice).is_some());
        assert!(alice.friends_between(carol).is_some());
        assert!(bob.friends_between(carol).is_none());
    }

    #[test]
    fn unique_pairは順序を無視した対で判定される() {
        // `alice -- bob` と `bob -- alice` は同じ対として扱われ、2本目は
        // unique pair 違反になる。
        let result = Social::Graph::create(|b| {
            b.person(
                person("alice"),
                Person {
                    name: "Alice".to_string(),
                },
            );
            b.person(
                person("bob"),
                Person {
                    name: "Bob".to_string(),
                },
            );
            b.friends(
                FriendsId("f1".to_string()),
                Friends::new(person("alice"), person("bob")),
            );
            b.friends(
                FriendsId("f2".to_string()),
                Friends::new(person("bob"), person("alice")),
            );
        });

        assert!(matches!(
            result,
            Err(Social::Violation::FriendsUniquePairViolation { .. })
        ));
    }

    #[test]
    fn 自己ループは許可され次数は1本と数える() {
        let g = Social::Graph::create(|b| {
            b.person(
                person("alice"),
                Person {
                    name: "Alice".to_string(),
                },
            );
            b.person(
                person("bob"),
                Person {
                    name: "Bob".to_string(),
                },
            );
            b.friends(
                FriendsId("self".to_string()),
                Friends::new(person("alice"), person("alice")),
            );
            b.friends(
                FriendsId("f1".to_string()),
                Friends::new(person("alice"), person("bob")),
            );
        })
        .expect("自己ループを含む友人関係も構築に成功するはず");

        // alice の次数は「自己ループ (1本) + bob との辺 (1本)」で2本。
        let alice = g.person_by_id(&person("alice")).unwrap();
        let friends_of_alice: Vec<_> = alice.friends_incident().collect();
        assert_eq!(friends_of_alice.len(), 2);

        assert!(alice.friends_between(alice).is_some());
    }

    #[test]
    fn 積み荷ありの無向辺はpayloadとendpointsを両方持つ() {
        let g = Social::Graph::create(|b| {
            b.person(
                person("alice"),
                Person {
                    name: "Alice".to_string(),
                },
            );
            b.person(
                person("bob"),
                Person {
                    name: "Bob".to_string(),
                },
            );
            b.wire(
                WireId("w1".to_string()),
                Wire::new(person("alice"), person("bob"), Cable { ohm: 5 }),
            );
        })
        .expect("無向のwireも構築に成功するはず");

        let bob = g.person_by_id(&person("bob")).unwrap();
        let wire = bob
            .wire_incident()
            .next()
            .expect("bob に接続する wire があるはず");
        assert_eq!(other(wire.endpoints(), bob).name, "Alice");
        assert_eq!(wire.payload().ohm, 5);

        let w = g.wire_by_id(&WireId("w1".to_string())).unwrap();
        let (first, second) = w.endpoints();
        assert_eq!(
            (first.id(), second.id()),
            (&person("alice"), &person("bob"))
        );
        // 積み荷のある無向辺の辺値は `PartialEq` を導出しない (その導出が積み荷の型へ
        // トレイトを要求しないため。issue #27)。このテストは端点だけを
        // `UnorderedPair` へ包み直し、グラフへ収めた辺 (alice, bob の順で構築した) の
        // 端点の対が、逆順に構築した辺値の端点の対と等しいことを確かめる。
        let bob_alice順 = Wire::new(person("bob"), person("alice"), Cable { ohm: 5 });
        let (逆順の端点1, 逆順の端点2) = bob_alice順.endpoints();
        assert_eq!(
            graphite::UnorderedPair::new(first.id(), second.id()),
            graphite::UnorderedPair::new(逆順の端点1, 逆順の端点2)
        );
        assert_eq!(w.cable().ohm, 5);
    }

    #[test]
    fn 接続探索とiterは挿入順を保持する() {
        let g = Social::Graph::create(|b| {
            b.person(
                person("alice"),
                Person {
                    name: "Alice".to_string(),
                },
            );
            b.person(
                person("bob"),
                Person {
                    name: "Bob".to_string(),
                },
            );
            b.person(
                person("carol"),
                Person {
                    name: "Carol".to_string(),
                },
            );
            b.person(
                person("dave"),
                Person {
                    name: "Dave".to_string(),
                },
            );

            // alice を軸に、bob -> carol -> dave の順で辺を張る。
            b.friends(
                FriendsId("f1".to_string()),
                Friends::new(person("alice"), person("bob")),
            );
            b.friends(
                FriendsId("f2".to_string()),
                Friends::new(person("carol"), person("alice")),
            );
            b.friends(
                FriendsId("f3".to_string()),
                Friends::new(person("alice"), person("dave")),
            );
        })
        .expect("構築に成功するはず");

        let alice = g.person_by_id(&person("alice")).unwrap();
        let names: Vec<String> = alice
            .friends_incident()
            .map(|edge| other(edge.endpoints(), alice).name.clone())
            .collect();
        assert_eq!(
            names,
            vec!["Bob".to_string(), "Carol".to_string(), "Dave".to_string()]
        );

        let ids: Vec<String> = g.friends_ids().map(|id| id.0.clone()).collect();
        assert_eq!(
            ids,
            vec!["f1".to_string(), "f2".to_string(), "f3".to_string()]
        );
    }

    #[test]
    fn 未知の端点を参照するとエラーになる() {
        let result = Social::Graph::create(|b| {
            b.person(
                person("alice"),
                Person {
                    name: "Alice".to_string(),
                },
            );
            b.friends(
                FriendsId("f1".to_string()),
                Friends::new(person("alice"), person("存在しない")),
            );
        });

        match result {
            Err(violation) => assert_eq!(
                violation,
                Social::Violation::FriendsUnknownEndpoint {
                    edge: FriendsId("f1".to_string()),
                    endpoint: person("存在しない"),
                }
            ),
            Ok(_) => panic!("未知の端点参照はエラーになるはず"),
        }
    }
}

/// `graph!` リテラルでの無向辺構築 (`docs/edge_endpoints_v4_1.md` §2:
/// リテラルの記法は積み荷ありの `-[X]-`、積み荷なしの `--` いずれも
/// 有向と同じ脱糖機構に素通しされる)。
#[cfg(test)]
mod graph_literal_tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn graphリテラルで無向辺を構築できる() {
        let g = graphite::graph!(Social {
            alice = Person { name: "Alice".into() },
            bob   = Person { name: "Bob".into() },

            f1 = Friends(alice -- bob),
            w1 = Wire(alice -[Cable { ohm: 8 }]- bob),
        })
        .expect("graph! での無向辺構築は成功するはず");

        let f = g.bob().friends_incident().next().unwrap();
        assert_eq!(other(f.endpoints(), g.bob()).name, "Alice");

        let wire = g.bob().wire_incident().next().unwrap();
        assert_eq!(other(wire.endpoints(), g.bob()).name, "Alice");
        assert_eq!(wire.payload().ohm, 8);
    }
}
fn other<'g>(
    endpoints: (Social::PersonRef<'g>, Social::PersonRef<'g>),
    node: Social::PersonRef<'g>,
) -> Social::PersonRef<'g> {
    if endpoints.0.id() == node.id() {
        endpoints.1
    } else {
        endpoints.0
    }
}
