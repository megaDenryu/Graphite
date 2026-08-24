#[derive(Clone, PartialEq)]
struct Person;

#[derive(Clone, PartialEq)]
struct Product;

#[derive(Clone, PartialEq)]
struct Item;

#[derive(Clone, PartialEq)]
struct TransactionInfo {
    amount: u64,
}

graphite::graph_schema! {
    schema Commerce {
        node Person;
        node Product;

        edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where each buyer: 1..2, each product: 0..1, unique pair;
        edge Subscription = (member: Person) -> (product: Product) where each member: 1..*;
    }
}

graphite::graph_schema! {
    schema JapaneseRoles {
        node Person;
        node Item;

        edge Ownership = (所有者: Person) -> (所有物: Item) where each 所有者: 1;
    }
}

#[test]
fn 辺値はgraph外で名前付きフィールドから構築できる() {
    let purchase = Commerce::Purchase {
        buyer: Commerce::PersonId("alice".into()),
        product: Commerce::ProductId("book".into()),
        info: TransactionInfo { amount: 100 },
    };

    assert_eq!(purchase.buyer, Commerce::PersonId("alice".into()));
    assert_eq!(purchase.product, Commerce::ProductId("book".into()));
    assert_eq!(purchase.info.amount, 100);
}

#[test]
fn 両端点の役割名の多重度を独立に検証する() {
    let result = Commerce::Graph::create_collecting(|builder| {
        let alice = Commerce::PersonId("alice".into());
        let bob = Commerce::PersonId("bob".into());
        let book = Commerce::ProductId("book".into());
        builder.person(alice.clone(), Person);
        builder.person(bob.clone(), Person);
        builder.product(book.clone(), Product);
        builder.purchase(
            Commerce::PurchaseId("alice-book".into()),
            Commerce::Purchase::new(alice.clone(), book.clone(), TransactionInfo { amount: 100 }),
        );
        builder.purchase(
            Commerce::PurchaseId("bob-book".into()),
            Commerce::Purchase::new(bob, book, TransactionInfo { amount: 200 }),
        );
    });

    let violations = match result {
        Err(violations) => violations,
        Ok(_) => panic!("同じproductへの2本目は入次数上限違反になるはず"),
    };
    assert!(violations.iter().any(|violation| matches!(
        violation,
        Commerce::Violation::PurchaseProductEachViolation { count: 2, .. }
    )));
}

#[test]
fn 上限なし多重度も下限を検証する() {
    let result = Commerce::Graph::create_collecting(|builder| {
        builder.person(Commerce::PersonId("alice".into()), Person);
        builder.product(Commerce::ProductId("book".into()), Product);
    });

    let violations = match result {
        Err(violations) => violations,
        Ok(_) => panic!("memberごとに1本以上必要なはず"),
    };
    assert!(violations.iter().any(|violation| matches!(
        violation,
        Commerce::Violation::SubscriptionMemberEachViolation { count: 0, .. }
    )));
}

#[test]
fn 日本語の役割名から多重度違反variantを生成する() {
    let result = JapaneseRoles::Graph::create_collecting(|builder| {
        builder.person(JapaneseRoles::PersonId("alice".into()), Person);
        builder.item(JapaneseRoles::ItemId("book".into()), Item);
    });

    let violations = match result {
        Err(violations) => violations,
        Ok(_) => panic!("所有者ごとに辺が1本必要なはず"),
    };
    assert!(violations.iter().any(|violation| matches!(
        violation,
        JapaneseRoles::Violation::Ownership所有者EachViolation { count: 0, .. }
    )));
}
