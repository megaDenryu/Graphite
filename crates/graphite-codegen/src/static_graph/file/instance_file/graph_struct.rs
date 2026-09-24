// このファイルは `Graph` (具体グラフ本体、Graph型名を固定名にする確定判断
// による) の本体を組み立てる。個体実体・積み荷の所有者 (旧`Nodes`/`Edges`、
// issue #41 段階1) は別structへ分けず、`Graph`自身の非公開フィールドへ
// 統合する。分けていた設計では、辺の実体 (`{種別}Edge<'a>`) が
// 端点個体への `&'a 実体型` を保持しており、`Graph`が`Nodes`と`Edges`の
// 両方を所有すると自己参照 (`Edges`がその`Graph`自身の中の`Nodes`を指す)
// になるため両立しなかった。この生成器は、具体辺の端点がinstance宣言の
// 時点で個体名として確定していることを使い、辺の実体には積み荷だけを
// 持たせ、端点はロールアクセサ (`node_ref.rs`・`edge_ref.rs`) が呼び出しの
// たびに`&self.#graph.{個体名}`を直接読むことで解決する (積み荷の値だけを
// フィールドとして格納し、参照は一切持たない)。これにより`Graph`が個体・
// 積み荷すべてを直接所有しても自己参照が生じない。
//
// フィールドは非公開にし、読み出しは`node_refs()`/`edge_refs()`という
// アクセサメソッドで行う (固定語彙、B分類)。非公開にすることで、利用者が
// 構造体リテラルで`Graph`を直接作る迂回を防ぐ (`node_ref.rs`の冒頭コメント
// 参照)。内部構築子 (`__graphite_internal_new`、C分類・`pub(crate)`・
// `#[deprecated]`) は個体・積み荷すべてを宣言順の位置引数に取る素の構築子
// であり、`construct!` (`construct.rs`) だけがこれを呼ぶ。`pub(crate)`は
// クレート内のどこからでも呼べてしまい、「呼べるのは`construct!`だけ」
// という主張をstable Rustの可視性だけでは強制できない。`#[deprecated]`を
// 添えて直接呼び出しを警告にし (`#![deny(warnings)]`の下ではエラーになる)、
// `construct!`の展開側では`#[allow(deprecated)]`で自分自身の呼び出しだけ
// 許す (回帰試験: `crates/graphite/tests/ui/static_internal_constructor_direct_call.rs`)。
//
// `Nodes`/`Edges`という型そのものが公開契約から消えたため、由来の異なる
// 実体を組み合わせて不整合な`Graph`を作る経路も構造的に無い (以前の
// `Graph::new(&nodes, &edges)`のような、由来の食い違う組を渡せる公開
// constructorが存在しない)。

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::static_graph::naming::{
    グラフ型名, 個体参照メソッド名, 個体参照集合型名, 内部構築子名, 辺参照メソッド名, 辺参照集合型名, graphフィールド名,
};
use crate::static_graph::schema::input::積み荷宣言;
use crate::static_graph::semantic::意味モデル;

use super::内部構築子の非推奨NOTE;
use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn graph本体を組み立てる(意味モデル: &意味モデル) -> TokenStream {
    let 型名 = グラフ型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let node_refsメソッド = 個体参照メソッド名(意味モデル);
    let node_refsdoc = doc属性を組み立てる(node_refsメソッド.追跡());
    let edge_refsメソッド = 辺参照メソッド名(意味モデル);
    let edge_refsdoc = doc属性を組み立てる(edge_refsメソッド.追跡());
    let node_refs型 = 個体参照集合型名(意味モデル);
    let edge_refs型 = 辺参照集合型名(意味モデル);
    let graph = graphフィールド名();
    let 内部構築子 = 内部構築子名();

    let 積み荷列: Vec<(&Ident, &Ident)> = 意味モデル
        .具体辺列()
        .iter()
        .filter_map(|辺| {
            let 積み荷宣言 { 型, .. } = 辺.種別().積み荷()?;
            Some((辺.名前(), 型))
        })
        .collect();

    let 個体フィールド列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 実体型 = 個体.実体型();
        quote! { #名前: #実体型 }
    });
    let 積み荷フィールド列 = 積み荷列.iter().map(|(名前, 型)| quote! { #名前: #型 });

    let 個体引数列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 実体型 = 個体.実体型();
        quote! { #名前: #実体型 }
    });
    let 積み荷引数列 = 積み荷列.iter().map(|(名前, 型)| quote! { #名前: #型 });

    let 個体初期化列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        quote! { #名前 }
    });
    let 積み荷初期化列 = 積み荷列.iter().map(|(名前, _)| quote! { #名前 });

    quote! {
        #型doc
        pub struct #型名 {
            #(#個体フィールド列,)*
            #(#積み荷フィールド列,)*
        }
        impl #型名 {
            #[doc(hidden)]
            #[deprecated(note = #内部構築子の非推奨NOTE)]
            pub(crate) fn #内部構築子(#(#個体引数列,)* #(#積み荷引数列,)*) -> Self {
                Self { #(#個体初期化列,)* #(#積み荷初期化列,)* }
            }
            #node_refsdoc
            pub fn #node_refsメソッド(&self) -> #node_refs型<'_> {
                #node_refs型 { #graph: self }
            }
            #edge_refsdoc
            pub fn #edge_refsメソッド(&self) -> #edge_refs型<'_> {
                #edge_refs型 { #graph: self }
            }
        }
    }
}
