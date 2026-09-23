// Graphiteが定義する固定語彙の生成名 (issue #41 のB分類)。型そのもの
// (Nodes/Edges/NodeRefs/EdgeRefs/Graph)・構築メソッド (`new`)・
// `entity()`・`Graph` の `node_refs`/`edge_refs` フィールドを対象にする。
// いずれも利用者のDSLに同名のトークンが無いため、由来は
// `名前の由来::GraphiteLanguage` になる (`static_graph::naming::card_names`
// の `個体実体所有者構築メソッド名` と同じ方針)。

use proc_macro2::{Ident, Span};

use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::{固定語彙, 固定語彙の所有者, 名前の由来, 意味項目, 追跡情報構築器};

use super::tracked_name::追跡付きの名前;

// 固定語彙の意味カードに書く「公開契約」段落の表示テキスト。`構築する` は
// 所有者型を冠して `Edges::new` のように表す (`card_names::個体実体所有者構築メソッド名`
// が `Nodes::new` に書く形と揃える)。それ以外は識別子文字列そのもの。
// 全ての固定語彙のカードに `固定語彙:` 行を付ける。
fn 固定語彙の宣言表示(語彙: 固定語彙) -> String {
    match 語彙 {
        固定語彙::構築する(所有者) => format!("{}::new", 所有者.型名()),
        _ => 語彙.識別子文字列().to_string(),
    }
}

fn 固定語彙の名前を作る(語彙: 固定語彙, 意味モデル: &意味モデル, 概要文: impl Into<String>) -> 追跡付きの名前 {
    let ident = Ident::new(語彙.識別子文字列(), Span::call_site());
    let 表示 = 固定語彙の宣言表示(語彙);
    let 追跡 = 追跡情報構築器::new(名前の由来::GraphiteLanguage(語彙), 概要文)
        .意味項目を足す(意味項目::new("graph", 意味モデル.グラフ名()))
        .固定語彙の宣言を添える(format!("`{表示}` (`docs/static_graph.md` 「生成される名前の公開契約」)"))
        .完成する();
    追跡付きの名前::new(ident, 追跡)
}

pub(crate) fn 個体実体所有者型名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::Nodes,
        意味モデル,
        "Graphite 静的グラフの個体実体の所有者 `Nodes` (Graphite の固定語彙)。",
    )
}

pub(crate) fn 辺実体所有者型名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::Edges,
        意味モデル,
        "Graphite 静的グラフの辺実体の所有者 `Edges` (Graphite の固定語彙)。",
    )
}

pub(crate) fn 個体参照集合型名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::NodeRefs,
        意味モデル,
        "Graphite 静的グラフの個体参照の集まり `NodeRefs` (Graphite の固定語彙)。",
    )
}

pub(crate) fn 辺参照集合型名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::EdgeRefs,
        意味モデル,
        "Graphite 静的グラフの辺参照の集まり `EdgeRefs` (Graphite の固定語彙)。",
    )
}

pub(crate) fn グラフ型名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::Graph,
        意味モデル,
        "Graphite 静的グラフの具体グラフ本体 `Graph` (Graphite の固定語彙)。",
    )
}

pub(crate) fn 構築メソッド名(所有者: 固定語彙の所有者, 意味モデル: &意味モデル) -> 追跡付きの名前 {
    let 型名 = 所有者.型名();
    固定語彙の名前を作る(
        固定語彙::構築する(所有者),
        意味モデル,
        format!("Graphite 静的グラフの `{型名}` を構築する (Graphite の固定語彙)。"),
    )
}

pub(crate) fn 実体アクセサメソッド名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::Entity,
        意味モデル,
        "Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。",
    )
}

pub(crate) fn 個体参照フィールド名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::NodeRefsフィールド,
        意味モデル,
        "Graphite 静的グラフの `Graph` が持つ個体参照の集まりへのフィールド `node_refs` (Graphite の固定語彙)。",
    )
}

pub(crate) fn 辺参照フィールド名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::EdgeRefsフィールド,
        意味モデル,
        "Graphite 静的グラフの `Graph` が持つ辺参照の集まりへのフィールド `edge_refs` (Graphite の固定語彙)。",
    )
}
