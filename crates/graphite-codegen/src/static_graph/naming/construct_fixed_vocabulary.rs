// Graphiteが定義する固定語彙のうち、構築の唯一の入口
// (`{instance名}::construct!`) を対象にする。`naming::fixed_vocabulary`
// から分けたのは、値なしの個体の実行時引数列という意味カードにinstance
// 固有の情報を組み込む関心事が型・固定フィールド名の関心事と異なり、かつ
// 意味カードにinstance固有の実行時個体列を組み込む (`宣言元の対` を要る)
// ためである。

use proc_macro2::{Ident, Span};

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::{固定語彙, 名前の由来, 意味項目, 追跡情報構築器};

use super::tracked_name::追跡付きの名前;

// `{instance名}::construct!` (固定語彙、B分類)。instance宣言で定義した
// 静的グラフを1回の呼び出しで実体化し、完成した `Graph` を返す。値ありの
// 個体・積み荷はinstance宣言の式からこのマクロが計算し、値なしの個体だけを
// 宣言順の引数で受け取る。個体実体・積み荷の所有者 (旧`Nodes`/`Edges`) や
// `Graph::new`のような中間の構築手順は公開契約に無く、この1マクロだけが
// 利用者が辿れる構築の入口である。
pub(crate) fn 構築マクロ名(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> 追跡付きの名前 {
    let 実行時個体列 = 意味モデル
        .個体列()
        .iter()
        .filter(|個体| 個体.値なし宣言か())
        .map(|個体| format!("{}: {}", 個体.名前(), 個体.実体型()))
        .collect::<Vec<_>>()
        .join(", ");
    let mut 構築器 = 追跡情報構築器::new(
        名前の由来::GraphiteLanguage(固定語彙::Construct),
        "Graphite 静的グラフの `Graph` を実体化するマクロ `construct` (Graphite の固定語彙)。値ありの個体・積み荷はinstance宣言の式からこのマクロが計算し、値なしの個体だけを宣言順の引数で受け取る。",
    )
    .意味項目を足す(意味項目::new("graph", 意味モデル.グラフ名()));
    if !実行時個体列.is_empty() {
        構築器 = 構築器.意味項目を足す(意味項目::new("実行時に渡す個体 (宣言順)", 実行時個体列));
    }
    let 追跡 = 構築器
        .意味項目を足す(意味項目::new("戻り値", "Graph"))
        .固定語彙の宣言を添える("`construct!` (`docs/static_graph.md` 「生成される名前の公開契約」)")
        .関係instance宣言を添える(宣言元.instance(), 意味モデル.グラフ宣言の形())
        .完成する();
    追跡付きの名前::new(Ident::new("construct", Span::call_site()), 追跡)
}
