// 意味カード4: 固定語彙の構築の入口 (`construct::nodes!`、issue #41 §5.2、
// PR #45レビューA・D)。`Nodes::new` はC分類の内部専用構築子へ降格したため、
// 利用者が辿る構築の入口は `construct::nodes!` になった。

use super::super::construct_fixed_vocabulary::個体構築マクロ名;
use super::{src_main, 開発チームの意味モデルを作る};

#[test]
fn 意味カード4_固定語彙の構築の入口() {
    let 意味モデル = 開発チームの意味モデルを作る();
    let 名前 = 個体構築マクロ名(&意味モデル, &src_main());
    assert_eq!(名前.ident().to_string(), "nodes");
    assert_eq!(
        名前.追跡().意味カード(),
        "Graphite 静的グラフの個体実体の所有者 `Nodes` を構築するマクロ `nodes` (Graphite の固定語彙)。値ありの個体はinstance宣言の式からこのマクロが計算し、値なしの個体だけを引数で受け取る。\n\
         \n\
         - graph: `開発チーム`\n\
         - 実行時に渡す個体 (宣言順): `開発部: 部署`\n\
         - 戻り値: `Nodes`\n\
         \n\
         固定語彙: `construct::nodes!` (`docs/static_graph.md` 「生成される名前の公開契約」)\n\
         \n\
         関係する instance 宣言: `src/main.rs` の `graph 開発チーム`"
    );
}
