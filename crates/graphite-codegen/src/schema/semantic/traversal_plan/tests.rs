use super::*;
use crate::schema::semantic::analyze::検査用にdslからスキーマ定義を組み立てる;

// 探索操作を「種類と辺の添字」の並びへ落とし、順序だけを比べられるようにする。
fn 操作の並び(計画: &ノードの探索計画) -> Vec<(&'static str, usize)> {
    計画
        .操作の列()
        .iter()
        .map(|操作| match 操作 {
            探索操作::役割による探索 { 辺, 側, .. } => match 側 {
                EachSide::Source => ("始点役割", 辺.添字()),
                EachSide::Target => ("終点役割", 辺.添字()),
            },
            探索操作::接続による探索 { 辺 } => ("接続", 辺.添字()),
            探索操作::端点対による探索 { 辺 } => ("端点対", 辺.添字()),
        })
        .collect()
}

#[test]
fn 探索操作は辺の宣言順に始点役割_終点役割_端点対の順で並ぶ() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Org {
                node Person;
                edge Boss = (subordinate: Person) -> (superior: Person);
                edge Friends = Person -- Person;
            }",
    );
    assert_eq!(
        操作の並び(&定義.ノードごとの探索計画()[0]),
        vec![
            ("始点役割", 0),
            ("終点役割", 0),
            ("端点対", 0),
            ("接続", 1),
            ("端点対", 1),
        ],
        "外側は辺の宣言順、1つの辺の中では始点役割・終点役割・端点対の順"
    );
}

#[test]
fn 端点対の探索は位置0側のノードにだけ生える() {
    let 定義 = 検査用にdslからスキーマ定義を組み立てる(
        "schema Org {
                node Person;
                node Team;
                edge Belongs = (member: Person) -> (team: Team);
            }",
    );
    assert_eq!(
        操作の並び(&定義.ノードごとの探索計画()[0]),
        vec![("始点役割", 0), ("端点対", 0)]
    );
    assert_eq!(
        操作の並び(&定義.ノードごとの探索計画()[1]),
        vec![("終点役割", 0)],
        "終点側のノードには端点対の探索を生やさない"
    );
}
