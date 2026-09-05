use super::*;
use crate::schema::semantic::{EachSide, RoleCardinality};

#[test]
fn 有向辺の端点の型名はノード定義番号へ解決される() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Org {
                node Person;
                node Team;
                edge Belongs = (member: Person) -> (team: Team);
            }",
    );
    let 辺 = &定義.辺定義の列()[0];
    assert_eq!(辺.始点のノード定義番号().添字(), 0, "Person は宣言順の0番");
    assert_eq!(辺.終点のノード定義番号().添字(), 1, "Team は宣言順の1番");
    assert!(辺.有向か());
}

#[test]
fn 始点役割へのeach制約は始点側の多重度だけを確定する() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Org {
                node Person;
                node Team;
                edge Belongs = (member: Person) -> (team: Team) where each member: 1;
            }",
    );
    let 辺 = &定義.辺定義の列()[0];
    assert_eq!(辺.側の多重度(EachSide::Source), RoleCardinality::Exact);
    assert_eq!(
        辺.側の多重度(EachSide::Target),
        RoleCardinality::Multiple,
        "制約の無い側は Multiple になる"
    );
}

#[test]
fn 終点役割へのeach制約は終点側の多重度だけを確定する() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Org {
                node Person;
                node Team;
                edge Belongs = (member: Person) -> (team: Team) where each team: 0..1;
            }",
    );
    let 辺 = &定義.辺定義の列()[0];
    assert_eq!(辺.側の多重度(EachSide::Source), RoleCardinality::Multiple);
    assert_eq!(辺.側の多重度(EachSide::Target), RoleCardinality::Optional);
}

#[test]
fn 無向辺の始点と終点は同じノード定義を指す() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Social {
                node Person;
                node Team;
                edge Friends = Person -- Person;
            }",
    );
    let 辺 = &定義.辺定義の列()[0];
    assert!(!辺.有向か());
    assert_eq!(辺.始点のノード定義番号(), 辺.終点のノード定義番号());
    assert_eq!(辺.始点のノード定義番号().添字(), 0);
}

#[test]
fn 積み荷の役割名と型パスを保持する() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Org {
                node Person;
                edge Boss = (subordinate: Person) -[appointment: BossEdge]-> (superior: Person);
            }",
    );
    let 積み荷 = 定義.辺定義の列()[0].積み荷().expect("積み荷がある");
    assert_eq!(積み荷.役割名().to_string(), "appointment");
    assert_eq!(積み荷.型パス().segments[0].ident.to_string(), "BossEdge");
}
