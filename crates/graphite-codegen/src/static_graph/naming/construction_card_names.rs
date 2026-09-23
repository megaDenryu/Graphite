// `Nodes::new`/`Edges::new` (issue #41 のB分類、固定語彙) の意味カード。
// どちらも「値の計算を持たない素の構築子」という同じ性質を説明する点で
// `card_names` (辺アクセサ・役割アクセサ・積み荷アクセサ) とは別の関心事
// なので、このファイルへ分ける。値の供給は `naming::assembly_names` が
// 生成する組み立て関数の役目。

use proc_macro2::{Ident, Span};

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::{固定語彙, 固定語彙の所有者, 名前の由来, 意味項目, 追跡情報構築器};

use super::tracked_name::追跡付きの名前;

// `Nodes::new`。全個体を宣言順の位置引数に取る。instance自身の宣言
// (`graph`) だけを参照するため `.instance()` だけを読む。
pub(crate) fn 個体実体所有者構築メソッド名(
    意味モデル: &意味モデル,
    宣言元: &宣言元の対,
) -> 追跡付きの名前 {
    let ident = Ident::new("new", Span::call_site());
    let 引数列 = 意味モデル
        .個体列()
        .iter()
        .map(|個体| format!("{}: {}", 個体.名前(), 個体.実体型()))
        .collect::<Vec<_>>()
        .join(", ");
    let 追跡 = 追跡情報構築器::new(
        名前の由来::GraphiteLanguage(固定語彙::構築する(固定語彙の所有者::Nodes)),
        format!(
            "Graphite 静的グラフの個体実体の所有者 `Nodes` を構築する (Graphite の固定語彙)。全個体を\
             宣言順の位置引数にそのまま取り、値の計算は行わない。値ありの個体をinstance宣言の式から\
             計算して渡すのは `{}の個体を組み立てる` の役目。",
            意味モデル.グラフ名()
        ),
    )
    .意味項目を足す(意味項目::new("graph", 意味モデル.グラフ名()))
    .意味項目を足す(意味項目::new("引数 (宣言順)", 引数列))
    .固定語彙の宣言を添える("`Nodes::new` (`docs/static_graph.md` 「生成される名前の公開契約」)")
    .関係instance宣言を添える(宣言元.instance(), 意味モデル.グラフ宣言の形())
    .完成する();
    追跡付きの名前::new(ident, 追跡)
}

// `Edges::new`。`&'a Nodes` に加え、積み荷ありの具体辺すべてを宣言順の
// 位置引数に取る。
pub(crate) fn 辺実体所有者構築メソッド名(
    意味モデル: &意味モデル,
    宣言元: &宣言元の対,
) -> 追跡付きの名前 {
    let ident = Ident::new("new", Span::call_site());
    let 積み荷引数列 = 意味モデル
        .具体辺列()
        .iter()
        .filter(|辺| 辺.積み荷式().is_some())
        .map(|辺| format!("{}: {}", 辺.名前(), 辺.種別().積み荷().expect("積み荷ありのはず").型))
        .collect::<Vec<_>>()
        .join(", ");
    let mut 構築器 = 追跡情報構築器::new(
        名前の由来::GraphiteLanguage(固定語彙::構築する(固定語彙の所有者::Edges)),
        format!(
            "Graphite 静的グラフの辺実体の所有者 `Edges` を構築する (Graphite の固定語彙)。値の計算は\
             行わない。積み荷ありの具体辺の値をinstance宣言の式から計算して渡すのは\
             `{}の辺を組み立てる` の役目。",
            意味モデル.グラフ名()
        ),
    )
    .意味項目を足す(意味項目::new("graph", 意味モデル.グラフ名()))
    .意味項目を足す(意味項目::new("第1引数", "nodes: &Nodes"));
    if !積み荷引数列.is_empty() {
        構築器 = 構築器.意味項目を足す(意味項目::new("積み荷引数 (宣言順)", 積み荷引数列));
    }
    let 追跡 = 構築器
        .固定語彙の宣言を添える("`Edges::new` (`docs/static_graph.md` 「生成される名前の公開契約」)")
        .関係instance宣言を添える(宣言元.instance(), 意味モデル.グラフ宣言の形())
        .完成する();
    追跡付きの名前::new(ident, 追跡)
}
