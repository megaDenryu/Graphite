//! 検証済みの構文モデルを読み、スキーマ定義を組み立てる。
//!
//! 検証を通過した構文だけを受け取る前提なので失敗しない署名にする。端点の名前
//! 解決に失敗した場合は「検証が漏れている」というバグなので `expect` で落とす。

use proc_macro2::Ident;

use super::edge_definition::{有向端点, 辺の向き, 辺定義};
use super::node_definition::{ノード定義, ノード定義番号};
use super::schema_definition::スキーマ定義;
use crate::schema::syntax::{EdgeDecl, EdgeShape, NodeDecl, SchemaInput};

/// 検証済みの `schema` 宣言からスキーマ定義を組み立てる。
pub fn 検証済み構文からスキーマ定義を組み立てる(
    構文: &SchemaInput,
) -> スキーマ定義 {
    let ノード定義の列: Vec<ノード定義> = 構文.nodes.iter().map(ノード定義::宣言から作る).collect();
    let 辺定義の列: Vec<辺定義> = 構文
        .edges
        .iter()
        .map(|宣言| {
            辺定義::宣言と向きから作る(宣言, 辺の向きを組み立てる(宣言, &構文.nodes))
        })
        .collect();
    スキーマ定義::定義の列から作る(
        構文.schema_name.clone(),
        ノード定義の列,
        辺定義の列,
    )
}

/// 辺宣言の端点に書かれたノード型名を、ノード定義番号へ解決した向きにする。
fn 辺の向きを組み立てる(
    宣言: &EdgeDecl, ノード宣言の列: &[NodeDecl]
) -> 辺の向き {
    match &宣言.shape {
        EdgeShape::Directed { from, to, .. } => 辺の向き::有向 {
            始点: 有向端点::役割名とノードから作る(
                from.role.clone(),
                型名からノード定義番号を求める(ノード宣言の列, &from.ty),
            ),
            終点: 有向端点::役割名とノードから作る(
                to.role.clone(),
                型名からノード定義番号を求める(ノード宣言の列, &to.ty),
            ),
        },
        // 無向辺の両端は同じノード型であることを `validate_undirected_same_type`
        // が保証済みなので、片方だけを見れば足りる。
        EdgeShape::Undirected { first, .. } => 辺の向き::無向 {
            端点のノード: 型名からノード定義番号を求める(
                ノード宣言の列,
                first,
            ),
        },
    }
}

fn 型名からノード定義番号を求める(
    ノード宣言の列: &[NodeDecl],
    型名: &Ident,
) -> ノード定義番号 {
    let 添字 = ノード宣言の列
        .iter()
        .position(|宣言| 宣言.name == *型名)
        .expect("validate() を通過していれば必ず見つかるはず");
    ノード定義番号::添字から作る(添字)
}

/// DSL の本文から、構文解析と検証を通してスキーマ定義まで組み立てる。
///
/// 意味層のテストが構文層と検証層に頼らずに済むよう、この module に置いて
/// 意味層の他のファイルのテストからも使えるようにする。
#[cfg(test)]
pub(super) fn 検査用にdslからスキーマ定義を組み立てる(
    dsl: &str,
) -> スキーマ定義 {
    use syn::parse::Parser;

    let 構文解析結果 = SchemaInput::parse_recovering
        .parse_str(dsl)
        .expect("テスト用 DSL は構文解析を通る");
    let 検証済み構文 =
        crate::schema::validate::validate(構文解析結果).expect("テスト用 DSL は検証を通る");
    検証済み構文からスキーマ定義を組み立てる(&検証済み構文)
}

#[cfg(test)]
mod tests {
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
}
