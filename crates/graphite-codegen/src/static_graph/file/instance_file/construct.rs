// このファイルは `construct` module (利用者が辿れる構築の唯一の入口) の
// 本体を組み立てる。`construct::nodes!`/`construct::edges!` は、instance
// 展開が呼び出し位置に置く値マクロ
// (`__graphite_values_{グラフ名}!`/`__graphite_payloads_{グラフ名}!`、
// `inline::value_supply`) を無修飾の名前で呼んで値ありの個体・積み荷を
// 計算し、内部構築子 (`{グラフ名}::Nodes::__graphite_internal_new`/
// `{グラフ名}::Edges::__graphite_internal_new`、
// `node_entities.rs`/`edge_entities.rs`) へ渡す。
//
// どちらの参照も、`super::`や`$crate::`のような固定深度・固定起点の修飾を
// 使わない。`macro_rules!`はマクロ名・項目パスのどちらも呼び出し位置
// (`construct::nodes!`/`construct::edges!`が実際に展開される場所) を
// 起点に解決するため (実測で確認済み)、`super::Nodes`は「呼び出し位置から
// 見てsuperの数が合わない」エラーになり、`$crate::#グラフ名::Nodes`は
// instanceの`mod`が関数の中にあると解決できない。`#グラフ名::Nodes`
// (グラフ名をそのまま冠した相対パス) だけが、instanceの`mod`宣言が
// クレートルート直下にあっても関数の中にあっても、呼び出し位置から見える
// 名前として解決される。値マクロも同じ理由で無修飾のまま呼ぶ。
//
// 値マクロは意図的に`pub(crate) use`を付けない (`inline::value_supply`の
// 冒頭コメント参照)。そのため`construct::nodes!`/`construct::edges!`を
// 呼んでよいのは、instance宣言と同じテキスト順スコープ (同じmodule、または
// instanceを置いた同じ関数の中) だけである。同じファイルの中でinstanceの
// 後ろに書いたインラインの子module (`mod x { .. }`) からは、macro_rules!の
// テキスト順スコープにより値マクロが見えてしまう残る穴がある
// (`docs/static_graph.md`「制約」節)。
//
// 内部構築子は`#[deprecated]`を持つ (`node_entities.rs`冒頭コメント参照)。
// この2つのマクロは自分自身の呼び出しを`#[allow(deprecated)]`で許すが、
// 利用者が`{グラフ名}::Nodes::__graphite_internal_new`等を直接呼ぶと
// 警告 (`#![deny(warnings)]`の下ではエラー) になる。

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{
    個体構築マクロ名, 個体値マクロ名, 内部構築子名, 構築モジュール名, 積み荷値マクロ名, 辺構築マクロ名,
};
use crate::static_graph::semantic::意味モデル;

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn construct本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let モジュール名 = 構築モジュール名(意味モデル);
    let モジュールdoc = doc属性を組み立てる(モジュール名.追跡());
    let nodes = nodesマクロを組み立てる(意味モデル, 宣言元);
    let edges = edgesマクロを組み立てる(意味モデル, 宣言元);

    quote! {
        #モジュールdoc
        pub mod #モジュール名 {
            #nodes
            #edges
        }
    }
}

fn nodesマクロを組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let マクロ名 = 個体構築マクロ名(意味モデル, 宣言元);
    let doc = doc属性を組み立てる(マクロ名.追跡());
    let グラフ名 = 意味モデル.グラフ名();
    let 値マクロ名 = 個体値マクロ名(グラフ名);
    let 内部構築子 = 内部構築子名();

    let 値あり個体名列: Vec<Ident> =
        意味モデル.個体列().iter().filter(|個体| !個体.値なし宣言か()).map(|個体| 個体.名前().clone()).collect();
    let 実行時個体名列: Vec<Ident> =
        意味モデル.個体列().iter().filter(|個体| 個体.値なし宣言か()).map(|個体| 個体.名前().clone()).collect();
    // マクロの仮引数パターン。metavariable名に個体名そのものを使い、呼び出す
    // 側から見て何番目の引数が何の個体かをマクロ定義自体が示す。
    let 仮引数パターン列: Vec<TokenStream> = 実行時個体名列.iter().map(|名前| quote! { $#名前:expr }).collect();
    // 内部構築子へ渡す実引数は宣言順。値ありの個体はタプル分解で束縛した
    // ローカル変数、値なしの個体はこのマクロの仮引数 (`$名前`) を使う。
    let 実引数順: Vec<TokenStream> = 意味モデル
        .個体列()
        .iter()
        .map(|個体| {
            let 名前 = 個体.名前();
            if 個体.値なし宣言か() {
                quote! { $#名前 }
            } else {
                quote! { #名前 }
            }
        })
        .collect();

    quote! {
        #doc
        macro_rules! #マクロ名 {
            (#(#仮引数パターン列),*) => {{
                let (#(#値あり個体名列,)*) = #値マクロ名!();
                #[allow(deprecated)]
                let __graphite_nodes = #グラフ名::Nodes::#内部構築子(#(#実引数順),*);
                __graphite_nodes
            }};
        }
        pub(crate) use #マクロ名;
    }
}

fn edgesマクロを組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let マクロ名 = 辺構築マクロ名(意味モデル, 宣言元);
    let doc = doc属性を組み立てる(マクロ名.追跡());
    let グラフ名 = 意味モデル.グラフ名();
    let 値マクロ名 = 積み荷値マクロ名(グラフ名);
    let 内部構築子 = 内部構築子名();

    let 積み荷あり辺名列: Vec<Ident> =
        意味モデル.具体辺列().iter().filter(|辺| 辺.積み荷式().is_some()).map(|辺| 辺.名前().clone()).collect();

    quote! {
        #doc
        macro_rules! #マクロ名 {
            ($nodes:expr) => {{
                let (#(#積み荷あり辺名列,)*) = #値マクロ名!();
                #[allow(deprecated)]
                let __graphite_edges = #グラフ名::Edges::#内部構築子($nodes, #(#積み荷あり辺名列),*);
                __graphite_edges
            }};
        }
        pub(crate) use #マクロ名;
    }
}
