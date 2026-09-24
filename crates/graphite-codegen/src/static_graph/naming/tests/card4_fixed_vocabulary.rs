// 意味カード4: 固定語彙の構築の入口 (`construct!`、issue #41 §5.2)。
// `Graph`を実体化する唯一の入口であり、値ありの個体・積み荷を差し替える
// 経路は存在しない。

use super::super::construct_fixed_vocabulary::構築マクロ名;
use super::{src_main, 開発チームの意味モデルを作る};

#[test]
fn 意味カード4_固定語彙の構築の入口() {
    let 意味モデル = 開発チームの意味モデルを作る();
    let 名前 = 構築マクロ名(&意味モデル, &src_main());
    assert_eq!(名前.ident().to_string(), "construct");
    assert_eq!(
        名前.追跡().意味カード(),
        "Graphite 静的グラフの `Graph` を実体化するマクロ `construct` (Graphite の固定語彙)。値ありの個体・積み荷はinstance宣言の式からこのマクロが計算し、値なしの個体だけを宣言順の引数で受け取る。\n\
         \n\
         - graph: `開発チーム`\n\
         - 実行時に渡す個体 (宣言順): `開発部: 部署`\n\
         - 戻り値: `Graph`\n\
         \n\
         固定語彙: `construct!` (`docs/static_graph.md` 「生成される名前の公開契約」)\n\
         \n\
         関係する instance 宣言: `src/main.rs` の `graph 開発チーム`"
    );
}
