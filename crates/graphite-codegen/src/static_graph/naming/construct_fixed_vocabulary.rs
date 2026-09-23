// Graphiteが定義する固定語彙のうち、構築の入口
// (`{instance名}::construct::nodes!`/`{instance名}::construct::edges!`)
// を対象にする。`naming::fixed_vocabulary` から分けた
// のは、値ありの個体・積み荷を差し替えられない構築の入口という関心事が
// 型・固定フィールド名の関心事と異なり、かつ意味カードにinstance固有の
// 実行時個体列を組み込む (`宣言元の対` を要る) ためである。

use proc_macro2::{Ident, Span};

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::{固定語彙, 名前の由来, 意味項目, 追跡情報構築器};

use super::fixed_vocabulary::固定語彙の名前を作る;
use super::tracked_name::追跡付きの名前;

// `{instance名}::construct`。値ありの個体・積み荷を
// instance宣言の式からのみ供給し、利用者が辿れる構築の入口をGraphiteが
// 所有するこのmoduleへ閉じる。
pub(crate) fn 構築モジュール名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::ConstructModule,
        意味モデル,
        "Graphite 静的グラフの構築の入口をまとめるmodule `construct` (Graphite の固定語彙)。",
    )
}

// `construct::nodes!` (固定語彙、B分類)。値ありの個体はinstance宣言の式から
// このマクロが計算し、値なしの個体だけを引数で受け取る。`Nodes::new`
// (C分類、内部専用) を直接呼ぶ迂回を防ぐ唯一の公開経路。
pub(crate) fn 個体構築マクロ名(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> 追跡付きの名前 {
    let 実行時個体列 = 意味モデル
        .個体列()
        .iter()
        .filter(|個体| 個体.値なし宣言か())
        .map(|個体| format!("{}: {}", 個体.名前(), 個体.実体型()))
        .collect::<Vec<_>>()
        .join(", ");
    let mut 構築器 = 追跡情報構築器::new(
        名前の由来::GraphiteLanguage(固定語彙::ConstructNodes),
        "Graphite 静的グラフの個体実体の所有者 `Nodes` を構築するマクロ `nodes` (Graphite の固定語彙)。値ありの個体はinstance宣言の式からこのマクロが計算し、値なしの個体だけを引数で受け取る。",
    )
    .意味項目を足す(意味項目::new("graph", 意味モデル.グラフ名()));
    if !実行時個体列.is_empty() {
        構築器 = 構築器.意味項目を足す(意味項目::new("実行時に渡す個体 (宣言順)", 実行時個体列));
    }
    let 追跡 = 構築器
        .意味項目を足す(意味項目::new("戻り値", "Nodes"))
        .固定語彙の宣言を添える("`construct::nodes!` (`docs/static_graph.md` 「生成される名前の公開契約」)")
        .関係instance宣言を添える(宣言元.instance(), 意味モデル.グラフ宣言の形())
        .完成する();
    追跡付きの名前::new(Ident::new("nodes", Span::call_site()), 追跡)
}

// `construct::edges!` (固定語彙、B分類)。積み荷ありの具体辺はすべて
// instance宣言の式からこのマクロが計算する。引数は端点を解決するための
// `&Nodes` 1つだけであり、積み荷を実行時に差し替える経路は無い。
pub(crate) fn 辺構築マクロ名(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> 追跡付きの名前 {
    let 追跡 = 追跡情報構築器::new(
        名前の由来::GraphiteLanguage(固定語彙::ConstructEdges),
        "Graphite 静的グラフの辺実体の所有者 `Edges` を構築するマクロ `edges` (Graphite の固定語彙)。積み荷ありの具体辺はすべてinstance宣言の式からこのマクロが計算する。",
    )
    .意味項目を足す(意味項目::new("graph", 意味モデル.グラフ名()))
    .意味項目を足す(意味項目::new("引数", "nodes: &Nodes"))
    .意味項目を足す(意味項目::new("戻り値", "Edges"))
    .固定語彙の宣言を添える("`construct::edges!` (`docs/static_graph.md` 「生成される名前の公開契約」)")
    .関係instance宣言を添える(宣言元.instance(), 意味モデル.グラフ宣言の形())
    .完成する();
    追跡付きの名前::new(Ident::new("edges", Span::call_site()), 追跡)
}
