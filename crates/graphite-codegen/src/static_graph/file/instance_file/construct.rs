// このファイルは `construct!` (instance宣言で定義した静的グラフを実体化
// する、利用者が辿れる構築の唯一の入口) の本体を組み立てる。`construct!`
// は、instance の宣言位置 (`__static_graph_impl!`がその場展開する位置) に
// 置かれた値マクロ (`__graphite_values_{グラフ名}!`/
// `__graphite_payloads_{グラフ名}!`、`inline::value_supply`) を無修飾の
// 名前で呼んで値ありの個体・積み荷を計算し、内部構築子
// (`{グラフ名}::Graph::__graphite_internal_new`、`graph_struct.rs`) へ
// 個体・積み荷すべてを1回で渡す。値マクロ自身の本体は、宣言位置に置いた
// 個々の値の束縛マクロ (`inline::value_binding`) を呼ぶだけであり、
// instance宣言の値の式が`construct!`の呼び出し位置の名前解決に晒される
// ことはない (`docs/static_graph.md`「値の式の名前解決」節)。
//
// 内部構築子への参照は、`super::`や`$crate::`のような固定深度・固定起点の
// 修飾を使わない。`macro_rules!`はマクロ名・項目パスのどちらも呼び出し
// 位置 (`construct!`が実際に展開される場所) を起点に解決するため (実測で
// 確認済み)、`super::Graph`は「呼び出し位置から見てsuperの数が合わない」
// エラーになり、`$crate::#グラフ名::Graph`はinstanceの`mod`が関数の中に
// あると解決できない。`#グラフ名::Graph` (グラフ名をそのまま冠した相対
// パス) だけが、instanceの`mod`宣言がクレートルート直下にあっても関数の
// 中にあっても、呼び出し位置から見える名前として解決される。値マクロも
// 同じ理由で無修飾のまま呼ぶ。
//
// 値マクロは意図的に`pub(crate) use`を付けない (`inline::value_supply`の
// 冒頭コメント参照)。そのため`construct!`を呼んでよいのは、instance宣言
// と同じテキスト順スコープ (同じmodule、またはinstanceを置いた同じ関数の
// 中) だけである (`docs/static_graph.md`「制約」節)。
//
// 内部構築子は`#[deprecated]`を持つ (`graph_struct.rs`冒頭コメント参照)。
// `construct!`は自分自身の呼び出しを`#[allow(deprecated)]`で許すが、
// 利用者が`{グラフ名}::Graph::__graphite_internal_new`等を直接呼ぶと
// 警告 (`#![deny(warnings)]`の下ではエラー) になる。

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{個体値マクロ名, 内部構築子名, 構築マクロ名, 積み荷値マクロ名};
use crate::static_graph::semantic::意味モデル;

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn construct本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let マクロ名 = 構築マクロ名(意味モデル, 宣言元);
    let doc = doc属性を組み立てる(マクロ名.追跡());
    let グラフ名 = 意味モデル.グラフ名();
    let 個体値マクロ名 = 個体値マクロ名(グラフ名);
    let 積み荷値マクロ名 = 積み荷値マクロ名(グラフ名);
    let 内部構築子 = 内部構築子名();

    let 値あり個体名列: Vec<Ident> =
        意味モデル.個体列().iter().filter(|個体| !個体.値なし宣言か()).map(|個体| 個体.名前().clone()).collect();
    let 実行時個体名列: Vec<Ident> =
        意味モデル.個体列().iter().filter(|個体| 個体.値なし宣言か()).map(|個体| 個体.名前().clone()).collect();
    let 積み荷あり辺名列: Vec<Ident> =
        意味モデル.具体辺列().iter().filter(|辺| 辺.積み荷式().is_some()).map(|辺| 辺.名前().clone()).collect();

    // マクロの仮引数パターン。metavariable名に個体名そのものを使い、呼び出す
    // 側から見て何番目の引数が何の個体かをマクロ定義自体が示す。
    let 仮引数パターン列: Vec<TokenStream> = 実行時個体名列.iter().map(|名前| quote! { $#名前:expr }).collect();

    // 内部構築子へ渡す実引数は個体・積み荷の宣言順。値ありの個体・積み荷は
    // タプル分解で束縛したローカル変数、値なしの個体はこのマクロの仮引数
    // (`$名前`) を使う。
    let 個体実引数順 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        if 個体.値なし宣言か() { quote! { $#名前 } } else { quote! { #名前 } }
    });
    let 積み荷実引数順 = 積み荷あり辺名列.iter().map(|名前| quote! { #名前 });
    let 全実引数順: Vec<TokenStream> = 個体実引数順.chain(積み荷実引数順).collect();

    quote! {
        #doc
        macro_rules! #マクロ名 {
            (#(#仮引数パターン列),*) => {{
                let (#(#値あり個体名列,)*) = #個体値マクロ名!();
                let (#(#積み荷あり辺名列,)*) = #積み荷値マクロ名!();
                #[allow(deprecated)]
                {
                    #グラフ名::Graph::#内部構築子(#(#全実引数順),*)
                }
            }};
        }
        pub(crate) use #マクロ名;
    }
}
