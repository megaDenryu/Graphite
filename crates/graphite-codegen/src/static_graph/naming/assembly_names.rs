// instanceの個体・辺を組み立てる関数の名前。`graph <名前>;` トークンから
// 機械的に派生する名前であり分類はA。`Nodes::new`/`Edges::new`
// (生成ファイル側、全個体・全積み荷を宣言順の位置引数に取る) を、値ありの
// 個体・積み荷については instance展開側で計算した値で、値なしの個体に
// ついてはこの組み立て関数自身の引数で埋めて呼ぶ (`inline::assembly` が
// 本体を組み立てる)。この組み立て関数がimplを使わず値を橋渡しする器で
// あり、`non_local_definitions` を避ける (`docs/static_graph.md`
// 「追跡の契約」参照)。

use quote::format_ident;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::{名前の由来, 意味項目, 追跡情報構築器};

use super::tracked_name::追跡付きの名前;

pub(crate) fn 個体組み立て関数名(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> 追跡付きの名前 {
    let グラフ名 = 意味モデル.グラフ名();
    let ident = format_ident!("{}の個体を組み立てる", グラフ名, span = グラフ名.span());
    let 個体名列 =
        意味モデル.個体列().iter().map(|個体| format!("`{}`", 個体.名前())).collect::<Vec<_>>().join("・");
    let 追跡 = 追跡情報構築器::new(
        名前の由来::InstanceGraph { グラフ名: グラフ名.clone() },
        "Graphite 静的グラフの個体実体の所有者 `Nodes` を、instance宣言から組み立てる。",
    )
    .意味項目を足す(意味項目::new("graph", グラフ名))
    .意味項目を足す(意味項目::組み立て済みの値で("組み立てる個体 (宣言順)", &個体名列))
    .意味項目を足す(意味項目::new("戻り値", "Nodes"))
    .宣言を添える(宣言元.instance(), 意味モデル.グラフ宣言の形())
    .完成する();
    追跡付きの名前::new(ident, 追跡)
}

pub(crate) fn 辺組み立て関数名(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> 追跡付きの名前 {
    let グラフ名 = 意味モデル.グラフ名();
    let ident = format_ident!("{}の辺を組み立てる", グラフ名, span = グラフ名.span());
    let 辺名列 =
        意味モデル.具体辺列().iter().map(|辺| format!("`{}`", 辺.名前())).collect::<Vec<_>>().join("・");
    let 追跡 = 追跡情報構築器::new(
        名前の由来::InstanceGraph { グラフ名: グラフ名.clone() },
        "Graphite 静的グラフの辺実体の所有者 `Edges` を、instance宣言から組み立てる。",
    )
    .意味項目を足す(意味項目::new("graph", グラフ名))
    .意味項目を足す(意味項目::組み立て済みの値で("組み立てる具体辺 (宣言順)", &辺名列))
    .意味項目を足す(意味項目::new("戻り値", "Edges"))
    .宣言を添える(宣言元.instance(), 意味モデル.グラフ宣言の形())
    .完成する();
    追跡付きの名前::new(ident, 追跡)
}
