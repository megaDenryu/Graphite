// 意味カード2: 辺アクセサメソッド (`太郎の所属()`、issue #41 §5.2)。

use super::super::card_names::辺アクセサメソッドの追跡情報を作る;
use super::{src_main, 開発チームの意味モデルを作る};

#[test]
fn 意味カード2_辺アクセサメソッド() {
    let 意味モデル = 開発チームの意味モデルを作る();
    let 太郎 = 意味モデル.個体列().iter().find(|個体| 個体.名前() == "太郎").unwrap();
    let 太郎の所属 = 意味モデル.具体辺列().iter().find(|辺| 辺.名前() == "太郎の所属").unwrap();
    let 追跡 = 辺アクセサメソッドの追跡情報を作る(&意味モデル, 太郎, 太郎の所属, &src_main());
    assert_eq!(
        追跡.意味カード(),
        "Graphite 静的グラフの具体辺参照を返す。\n\
         \n\
         - graph: `開発チーム`\n\
         - 個体: `太郎`\n\
         - 具体辺: `太郎の所属`\n\
         - 辺種別: `所属`\n\
         - この個体の役割: `member`\n\
         - 戻り値: `太郎の所属Ref`\n\
         \n\
         宣言: `src/main.rs` の `edge 太郎の所属 = 所属(太郎 -> 開発部)`\n\
         \n\
         関係する schema 宣言: `src/main.rs` の `edge 所属 = (member: 社員) -> (team: 部署) where each member: 1`"
    );
}
