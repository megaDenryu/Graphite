// 意味カード4: 固定語彙の構築メソッド (`Nodes::new`、issue #41 §5.2)。

use super::super::card_names::個体実体所有者構築メソッド名;
use super::{src_main, 開発チームの意味モデルを作る};

#[test]
fn 意味カード4_固定語彙の構築メソッド() {
    let 意味モデル = 開発チームの意味モデルを作る();
    let 名前 = 個体実体所有者構築メソッド名(&意味モデル, &src_main());
    assert_eq!(名前.ident().to_string(), "new");
    assert_eq!(
        名前.追跡().意味カード(),
        "Graphite 静的グラフの個体実体の所有者 `Nodes` を構築する (Graphite の固定語彙)。\n\
         \n\
         - graph: `開発チーム`\n\
         - 実行時供給が必要な個体 (引数の順): `開発部: 部署`\n\
         - instance 宣言の右辺式から作る個体: `太郎`・`次郎`・`一郎`\n\
         \n\
         固定語彙: `Nodes::new` (`docs/static_graph.md` 「生成される名前の公開契約」)\n\
         \n\
         関係する instance 宣言: `src/main.rs` の `graph 開発チーム`"
    );
}
