use super::super::analyze::検査用にdslからスキーマ定義を組み立てる;

#[test]
fn 宣言の形をノード種別と辺種別ごとに組み立てる() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Commerce {
                node Person;
                node Product(id: self::ExternalProductId);
                edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where each buyer: 1..2, each product: 0..1, unique pair;
                edge Subscription = (member: Person) -> (product: Product) where each member: 1..*;
                edge Friend(id: ExternalEdgeId) = Person -- Person;
                edge Cable = Person -[cable: Cable]- Person;
            }",
    );
    assert_eq!(定義.宣言の形(), "schema Commerce");
    assert_eq!(定義.ノード定義の列()[0].宣言の形(), "node Person");
    assert_eq!(
        定義.ノード定義の列()[1].宣言の形(),
        "node Product(id: super::ExternalProductId)",
        "明示ID型は生成 module 内から解決できる形へ正規化した綴りで書く"
    );
    let 辺の列 = 定義.辺定義の列();
    assert_eq!(
        定義.辺の宣言の形(&辺の列[0]),
        "edge Purchase = (buyer: Person) -[info: TransactionInfo]-> (product: Product) where each buyer: 1..2, each product: 0..1, unique pair"
    );
    assert_eq!(
        定義.辺の宣言の形(&辺の列[1]),
        "edge Subscription = (member: Person) -> (product: Product) where each member: 1..*"
    );
    assert_eq!(
        定義.辺の宣言の形(&辺の列[2]),
        "edge Friend(id: ExternalEdgeId) = Person -- Person"
    );
    assert_eq!(
        定義.辺の宣言の形(&辺の列[3]),
        "edge Cable = Person -[cable: Cable]- Person"
    );
}

#[test]
fn ちょうど1本の多重度は範囲を畳んだ綴りになる() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Org {
                node Person;
                node Team;
                edge Belongs = (member: Person) -> (team: Team) where each member: 1;
            }",
    );
    assert_eq!(
        定義.辺の宣言の形(&定義.辺定義の列()[0]),
        "edge Belongs = (member: Person) -> (team: Team) where each member: 1"
    );
}
