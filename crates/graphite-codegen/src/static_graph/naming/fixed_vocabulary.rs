// Graphiteが定義する固定語彙の生成名 (issue #41 のB分類)。型そのもの
// (NodeRefs/EdgeRefs/Graph)・`entity()`・`Graph` の `node_refs`/
// `edge_refs` メソッドを対象にする。いずれも利用者のDSLに同名のトークンが
// 無いため、由来は `名前の由来::GraphiteLanguage` になる。構築の唯一の
// 入口 (`construct!`) は `naming::construct_fixed_vocabulary` が別に持つ
// (instance固有の実行時個体列を意味カードへ組み込む関心事がここと
// 異なるため)。個体実体・積み荷の所有者 (旧`Nodes`/`Edges`) は`Graph`
// 自身のフィールドへ統合し、公開契約からも型としても消えたため、この
// ファイルには対応する関数が無い (`graph_struct.rs`冒頭コメント参照)。

use proc_macro2::{Ident, Span};

use crate::static_graph::semantic::意味モデル;
use crate::static_graph::trace::{固定語彙, 名前の由来, 意味項目, 追跡情報構築器};

use super::tracked_name::追跡付きの名前;

// 固定語彙の意味カードに書く「公開契約」段落の表示テキスト。`Construct` は
// マクロであることが分かるよう `construct!` と表す。それ以外は識別子
// 文字列そのもの。全ての固定語彙のカードに `固定語彙:` 行を付ける。
fn 固定語彙の宣言表示(語彙: 固定語彙) -> String {
    match 語彙 {
        固定語彙::Construct => "construct!".to_string(),
        _ => 語彙.識別子文字列().to_string(),
    }
}

pub(super) fn 固定語彙の名前を作る(語彙: 固定語彙, 意味モデル: &意味モデル, 概要文: impl Into<String>) -> 追跡付きの名前 {
    let ident = Ident::new(語彙.識別子文字列(), Span::call_site());
    let 表示 = 固定語彙の宣言表示(語彙);
    let 追跡 = 追跡情報構築器::new(名前の由来::GraphiteLanguage(語彙), 概要文)
        .意味項目を足す(意味項目::new("graph", 意味モデル.グラフ名()))
        .固定語彙の宣言を添える(format!("`{表示}` (`docs/static_graph.md` 「生成される名前の公開契約」)"))
        .完成する();
    追跡付きの名前::new(ident, 追跡)
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

pub(crate) fn 実体アクセサメソッド名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::Entity,
        意味モデル,
        "Graphite 静的グラフの具体個体参照から実体を取り出す `entity` (Graphite の固定語彙)。",
    )
}

pub(crate) fn 個体参照メソッド名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::NodeRefsMethod,
        意味モデル,
        "Graphite 静的グラフの `Graph` が個体参照の集まりを返すメソッド `node_refs` (Graphite の固定語彙)。",
    )
}

pub(crate) fn 辺参照メソッド名(意味モデル: &意味モデル) -> 追跡付きの名前 {
    固定語彙の名前を作る(
        固定語彙::EdgeRefsMethod,
        意味モデル,
        "Graphite 静的グラフの `Graph` が辺参照の集まりを返すメソッド `edge_refs` (Graphite の固定語彙)。",
    )
}

