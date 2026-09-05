use super::*;
use crate::schema::semantic::analyze::検査用にdslからスキーマ定義を組み立てる;

// 違反定義を種類の名前の並びへ落とし、順序だけを比べられるようにする。
fn 違反の並び(定義: &crate::schema::semantic::スキーマ定義) -> Vec<String> {
    定義
        .違反定義の列()
        .iter()
        .map(|違反| match 違反 {
            違反定義::ノードのキーが重複した { .. } => {
                "ノードのキーが重複した".to_string()
            }
            違反定義::辺のキーが重複した { .. } => {
                "辺のキーが重複した".to_string()
            }
            違反定義::未知の始点を参照した { .. } => {
                "未知の始点を参照した".to_string()
            }
            違反定義::未知の終点を参照した { .. } => {
                "未知の終点を参照した".to_string()
            }
            違反定義::未知の端点を参照した { .. } => {
                "未知の端点を参照した".to_string()
            }
            違反定義::多重度に反した { 役割名, .. } => {
                format!("多重度に反した({役割名})")
            }
            違反定義::端点対が重複した { .. } => "端点対が重複した".to_string(),
        })
        .collect()
}

#[test]
fn 多重度違反はwhere節の記述順に並ぶ() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Org {
                node Person;
                edge Boss = (subordinate: Person) -> (superior: Person)
                    where each superior: 1, each subordinate: 0..1;
            }",
    );
    assert_eq!(
        違反の並び(&定義),
        vec![
            "ノードのキーが重複した",
            "辺のキーが重複した",
            "未知の始点を参照した",
            "未知の終点を参照した",
            "多重度に反した(superior)",
            "多重度に反した(subordinate)",
        ],
        "終点役割を先に書いたので、側の順ではなく記述順に並ぶ"
    );
}

#[test]
fn 無向辺は未知の端点1種類と端点対の重複を持つ() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Social {
                node Person;
                edge Friends = Person -- Person where unique pair;
            }",
    );
    assert_eq!(
        違反の並び(&定義),
        vec![
            "ノードのキーが重複した",
            "辺のキーが重複した",
            "未知の端点を参照した",
            "端点対が重複した",
        ]
    );
}
