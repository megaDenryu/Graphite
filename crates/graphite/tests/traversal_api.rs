//! 走査APIを検証する統合テスト。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 再設計待ち)。この
//! ファイルは検証対象1つ (走査API) に対するテスト用スキーマとテスト関数の列
//! を持つ。`each_declaration_order.rs` が `#[path]` で宣言を親に残したまま
//! テストを部分モジュールへ出す技法を実証したため、このファイルの分割が同じ
//! 宣言を各ファイルへ複製するという統合の根拠は成り立たない。検証観点ごとに
//! 部分モジュールへ分ける判定を issue #28 のやること4 で行う。超過を許す根
//! 拠の台帳は `docs/development/line_count_ledger.md` にある。

#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    name: String,
}

#[allow(non_snake_case, dead_code, private_interfaces)]
#[allow(
    clippy::needless_lifetimes,
    clippy::wrong_self_convention,
    clippy::clone_on_copy,
    clippy::write_literal
)]
pub mod Traversal {
    include!("generated/traversal_api_traversal.rs");
}

#[rustfmt::skip]
graphite::graph_schema! {
    generated = "generated/traversal_api_traversal.rs";
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
fn node_refの役割探索はedge_refを挿入順で返す() {
    let graph = build();
    let alice = graph.person_by_id(&person("alice")).unwrap();
    let 購入順: Vec<_> = alice
        .purchase_as_buyer()
        .map(|edge| edge.id().0.as_str())
        .collect();

    assert_eq!(購入順, ["second", "first"]);
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
    let bob = graph.person_by_id(&person("bob")).unwrap();
    let carol = graph.person_by_id(&person("carol")).unwrap();

    assert_eq!(bob.関係_as_始点().next().unwrap().終点().name, "carol");
    assert_eq!(carol.関係_as_終点().next().unwrap().始点().name, "bob");
    assert_eq!(
        bob.mentor_as_superior().next().unwrap().subordinate().name,
        "alice"
    );
}

#[test]
fn betweenは生成元graphを検査し非panic版も提供する() {
    let first = build();
    let second = build();
    let alice = first.person_by_id(&person("alice")).unwrap();
    let bob_other = second.person_by_id(&person("bob")).unwrap();

    let error = alice.関係_try_between(bob_other).unwrap_err();
    assert!(error.to_string().contains("同じ Graph の値"));
    assert!(std::panic::catch_unwind(|| alice.関係_between(bob_other)).is_err());
    assert!(alice.friends_try_between(bob_other).is_err());
}

#[test]
fn 無向辺はincidentと順序なしbetweenを提供する() {
    let graph = build();
    let alice = graph.person_by_id(&person("alice")).unwrap();
    let bob = graph.person_by_id(&person("bob")).unwrap();

    assert_eq!(alice.friends_incident().count(), 1);
    assert!(alice.friends_between(bob).is_some());
    assert!(bob.friends_between(alice).is_some());
}
