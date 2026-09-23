// 意味カード1: 具体個体参照 (`太郎Ref`、issue #41 §5.2)。

use crate::static_graph::trace::名前の由来;

use super::super::type_names::個体参照型名;
use super::{src_main, 開発チームの意味モデルを作る};

#[test]
fn 意味カード1_具体個体参照() {
    let 意味モデル = 開発チームの意味モデルを作る();
    let 太郎 = 意味モデル.個体列().iter().find(|個体| 個体.名前() == "太郎").unwrap();
    let 参照名 = 個体参照型名(&意味モデル, 太郎, &src_main());
    assert!(matches!(
        参照名.追跡().由来(),
        名前の由来::InstanceNode { 個体名 } if 個体名 == "太郎"
    ));
    assert_eq!(
        参照名.追跡().意味カード(),
        "Graphite 静的グラフの具体個体参照。\n\
         \n\
         - graph: `開発チーム`\n\
         - 個体: `太郎`\n\
         - 実体型: `社員`\n\
         \n\
         宣言: `src/main.rs` の `node 太郎: 社員 = ..`"
    );
}
