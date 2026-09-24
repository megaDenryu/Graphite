//! instanceファイル本体 (issue #41 §2)。手書き到達点と同じ並び順 (個体参照
//! → 辺インスタンス参照 → 参照の層の集まり → グラフ本体 → 構築の入口) で、
//! `pub` 可視性と意味カードを添えて並べる。個体実体・積み荷の所有者
//! (旧`Nodes`/`Edges`) は独立structへ分けず`Graph`自身のフィールドへ統合
//! したため (`graph_struct.rs`冒頭コメント参照)、この並びに専用のファイルは
//! 無い。各生成物の中身は配下のmoduleが持ち、この module本体は並び順だけを
//! 知る。instanceの指紋定数は呼び出し側 (`static_graph::tracked::instance`)
//! が `crate::generated_source::生成ファイルの本文` 経由で別途足す。

mod construct;
mod edge_ref;
mod graph_struct;
mod node_ref;
mod ref_collections;

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::semantic::意味モデル;

// 内部構築子 (`__graphite_internal_new`) に添える`#[deprecated]`のnote。
// `pub(crate)`はクレート内のどこからでも呼べるため、可視性だけでは
// 「呼べるのは`construct!`だけ」という主張をstable Rustで強制できない。
// この`note`は`graph_struct`が構築子へ、`construct`が呼び出し側の
// `#[allow(deprecated)]`の対にする。
pub(super) const 内部構築子の非推奨NOTE: &str = "Graphite の内部構築子である。construct! を使うこと";

pub(crate) fn instance本体を組み立てる(
    意味モデル: &意味モデル,
    宣言元: &宣言元の対,
    generated_path: &str,
) -> TokenStream {
    let 個体参照列 = node_ref::個体参照列を組み立てる(意味モデル, 宣言元);
    let 辺インスタンス参照列 = edge_ref::辺インスタンス参照列を組み立てる(意味モデル, 宣言元);
    let node_refs = ref_collections::node_refs本体を組み立てる(意味モデル, 宣言元);
    let edge_refs = ref_collections::edge_refs本体を組み立てる(意味モデル, 宣言元);
    let graph = graph_struct::graph本体を組み立てる(意味モデル);
    let construct = construct::construct本体を組み立てる(意味モデル, 宣言元, generated_path);

    quote! {
        #個体参照列
        #辺インスタンス参照列
        #node_refs
        #edge_refs
        #graph
        #construct
    }
}
