//! 無向辺 (`docs/edge_endpoints_v4_1.md` §2) の統合テスト。
//!
//! `Friends` (積み荷なし) と `Wire` (積み荷あり) の2種別で:
//! - `of`/`between` の対称性 (どちらの位置に置かれても検索できる)
//! - `unique pair` の順序無視の同値判定
//! - 自己ループの許可と、次数 (`each`) では1本と数える仕様
//! - `.endpoints()` アクセサ (from/to という嘘の語彙を生成しない)
//! - 格納順 (挿入順) の保持
//! を確認する。

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersonId(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cable {
    pub ohm: i32,
}

#[rustfmt::skip]
graphite::graph_schema! {
    schema Social {
        node Person;

        edge Friends = Person -- Person where unique pair;
        edge Wire    = Person -[Cable]- Person;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn person(id: &str) -> PersonId {
        PersonId(id.to_string())
    }

    fn build_chart() -> Social {
        Social::create(|b| {
            b.person(person("alice"), Person { name: "Alice".to_string() });
            b.person(person("bob"), Person { name: "Bob".to_string() });
            b.person(person("carol"), Person { name: "Carol".to_string() });

            b.friends(FriendsId("f1".to_string()), Friends(person("alice"), person("bob")));
            b.friends(FriendsId("f2".to_string()), Friends(person("carol"), person("alice")));
        })
        .expect("正常な友人関係は構築に成功するはず")
    }

    #[test]
    fn endpointsアクセサで両端を取得できる() {
        let g = build_chart();
        let f = Friends::get(&g, &FriendsId("f1".to_string())).unwrap();
        let (p0, p1) = f.endpoints();
        assert_eq!((p0, p1), (&person("alice"), &person("bob")));
    }

    #[test]
    fn ofはどちらの位置に置かれても対称に検索できる() {
        let g = build_chart();

        // alice は f1 の位置0、f2 の位置1 に置かれているが、どちらからでも
        // 相手を辿れる。
        let mut friends_of_alice: Vec<String> =
            Friends::of(&g, &person("alice")).into_iter().map(|p| p.name.clone()).collect();
        friends_of_alice.sort();
        assert_eq!(friends_of_alice, vec!["Bob".to_string(), "Carol".to_string()]);

        let friends_of_bob: Vec<&Person> = Friends::of(&g, &person("bob"));
        assert_eq!(friends_of_bob.len(), 1);
        assert_eq!(friends_of_bob[0].name, "Alice");
    }

    #[test]
    fn betweenは順序を無視して対称に検索する() {
        let g = build_chart();
        // `Friends(alice -- bob)` で作った辺だが、between は逆順でも見つかる。
        assert!(Friends::between(&g, &person("alice"), &person("bob")).is_some());
        assert!(Friends::between(&g, &person("bob"), &person("alice")).is_some());
        assert!(Friends::between(&g, &person("alice"), &person("carol")).is_some());
        assert!(Friends::between(&g, &person("bob"), &person("carol")).is_none());
    }

    #[test]
    fn unique_pairは順序を無視した対で判定される() {
        // `alice -- bob` と `bob -- alice` は同じ対として扱われ、2本目は
        // unique pair 違反になる。
        let result = Social::create(|b| {
            b.person(person("alice"), Person { name: "Alice".to_string() });
            b.person(person("bob"), Person { name: "Bob".to_string() });
            b.friends(FriendsId("f1".to_string()), Friends(person("alice"), person("bob")));
            b.friends(FriendsId("f2".to_string()), Friends(person("bob"), person("alice")));
        });

        assert!(matches!(
            result,
            Err(SocialViolation::FriendsUniquePairViolation { .. })
        ));
    }

    #[test]
    fn 自己ループは許可され次数は1本と数える() {
        let g = Social::create(|b| {
            b.person(person("alice"), Person { name: "Alice".to_string() });
            b.person(person("bob"), Person { name: "Bob".to_string() });
            b.friends(FriendsId("self".to_string()), Friends(person("alice"), person("alice")));
            b.friends(FriendsId("f1".to_string()), Friends(person("alice"), person("bob")));
        })
        .expect("自己ループを含む友人関係も構築に成功するはず");

        // alice の次数は「自己ループ (1本) + bob との辺 (1本)」で2本。
        let friends_of_alice: Vec<&Person> = Friends::of(&g, &person("alice"));
        assert_eq!(friends_of_alice.len(), 2);

        assert!(Friends::between(&g, &person("alice"), &person("alice")).is_some());
    }

    #[test]
    fn 積み荷ありの無向辺はpayloadとendpointsを両方持つ() {
        let g = Social::create(|b| {
            b.person(person("alice"), Person { name: "Alice".to_string() });
            b.person(person("bob"), Person { name: "Bob".to_string() });
            b.wire(WireId("w1".to_string()), Wire(person("alice"), person("bob"), Cable { ohm: 5 }));
        })
        .expect("無向のwireも構築に成功するはず");

        let (other, cable) = Wire::of(&g, &person("bob"))
            .into_iter()
            .next()
            .expect("bob に接続する wire があるはず");
        assert_eq!(other.name, "Alice");
        assert_eq!(cable.ohm, 5);

        let w = Wire::get(&g, &WireId("w1".to_string())).unwrap();
        assert_eq!(w.endpoints(), (&person("alice"), &person("bob")));
        assert_eq!(w.payload().ohm, 5);
    }

    #[test]
    fn ofとiterは挿入順を保持する() {
        let g = Social::create(|b| {
            b.person(person("alice"), Person { name: "Alice".to_string() });
            b.person(person("bob"), Person { name: "Bob".to_string() });
            b.person(person("carol"), Person { name: "Carol".to_string() });
            b.person(person("dave"), Person { name: "Dave".to_string() });

            // alice を軸に、bob -> carol -> dave の順で辺を張る。
            b.friends(FriendsId("f1".to_string()), Friends(person("alice"), person("bob")));
            b.friends(FriendsId("f2".to_string()), Friends(person("carol"), person("alice")));
            b.friends(FriendsId("f3".to_string()), Friends(person("alice"), person("dave")));
        })
        .expect("構築に成功するはず");

        let names: Vec<String> =
            Friends::of(&g, &person("alice")).into_iter().map(|p| p.name.clone()).collect();
        assert_eq!(names, vec!["Bob".to_string(), "Carol".to_string(), "Dave".to_string()]);

        let ids: Vec<String> = Friends::ids(&g).map(|id| id.0.clone()).collect();
        assert_eq!(ids, vec!["f1".to_string(), "f2".to_string(), "f3".to_string()]);
    }

    #[test]
    fn 未知の端点を参照するとエラーになる() {
        let result = Social::create(|b| {
            b.person(person("alice"), Person { name: "Alice".to_string() });
            b.friends(FriendsId("f1".to_string()), Friends(person("alice"), person("存在しない")));
        });

        match result {
            Err(violation) => assert_eq!(
                violation,
                SocialViolation::FriendsUnknownEndpoint {
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

        let f: &Person = Friends::of(&g, &PersonId("bob".to_string())).into_iter().next().unwrap();
        assert_eq!(f.name, "Alice");

        let (w_other, cable) = Wire::of(&g, &PersonId("bob".to_string())).into_iter().next().unwrap();
        assert_eq!(w_other.name, "Alice");
        assert_eq!(cable.ohm, 8);
    }
}
