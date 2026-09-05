use super::*;
use crate::schema::semantic::analyze::検査用にdslからスキーマ定義を組み立てる;

#[test]
fn each制約の範囲を多重度3値へ分類できる() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Org {
                node Person;
                node Team;
                edge ExactOne = (member: Person) -> (team: Team) where each member: 1;
                edge ZeroOrOne = (member: Person) -> (team: Team) where each member: 0..1;
                edge RangeMulti = (member: Person) -> (team: Team) where each member: 1..3;
                edge LowerOnly = (member: Person) -> (team: Team) where each member: 2..*;
                edge NoConstraint = (member: Person) -> (team: Team);
            }",
    );
    let 辺定義の列 = 定義.辺定義の列();
    assert_eq!(
        辺定義の列[0].側の多重度(EachSide::Source),
        RoleCardinality::Exact,
        "1 はちょうど1本"
    );
    assert_eq!(
        辺定義の列[1].側の多重度(EachSide::Source),
        RoleCardinality::Optional,
        "0..1 は高々1本"
    );
    assert_eq!(
        辺定義の列[2].側の多重度(EachSide::Source),
        RoleCardinality::Multiple,
        "1..3 は範囲指定なので Multiple"
    );
    assert_eq!(
        辺定義の列[3].側の多重度(EachSide::Source),
        RoleCardinality::Multiple,
        "2..* は下限だけなので Multiple"
    );
    assert_eq!(
        辺定義の列[4].側の多重度(EachSide::Source),
        RoleCardinality::Multiple,
        "制約が無ければ Multiple"
    );
}
