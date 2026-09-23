// instanceの個体・辺を組み立てる関数 (issue #41)。`Nodes::new`/`Edges::new`
// (生成ファイル側、値の計算を持たない素の構築子、`file::instance_file`) へ、
// 値ありの個体・積み荷は`value_supply`の素の供給関数呼び出しで、値なしの
// 個体はこの組み立て関数自身の引数でそのまま埋めて渡す。供給関数は組み立て
// 関数の本体の中へ入れ子で定義する (同じスコープに複数のinstanceを置くと、
// 供給関数の名前は個体名・辺名だけから決まりグラフ名を含まないため、
// 呼び出し位置に並ぶ自由関数のままでは同名の個体・辺を持つinstance同士で
// 名前が衝突する。入れ子にすると組み立て関数ごとに別スコープを持つため
// 衝突しない。ブロックの中の項目も外側のスコープの項目を見られるので、
// 値の式の解決は変わらない)。implを一切使わないため、instance展開の
// 呼び出し位置がユーザーの関数の中にあっても`non_local_definitions`の
// 対象にならない (`value_supply`と同じ理由)。呼び出し元のスコープを
// 汚さないよう、既定の可視性は`pub(crate)`にする。

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::doc_render::doc属性を組み立てる;
use crate::static_graph::naming::{個体供給関数名, 個体組み立て関数名, 積み荷供給関数名, 辺組み立て関数名, nodes変数名};
use crate::static_graph::semantic::意味モデル;

use super::value_supply::{個体供給関数を組み立てる, 積み荷供給関数を組み立てる};

// `{グラフ名}の個体を組み立てる() -> {instance名}::Nodes`。
pub(crate) fn 個体組み立て関数を組み立てる(
    instance名: &Ident,
    意味モデル: &意味モデル,
    宣言元: &宣言元の対,
) -> TokenStream {
    let 供給関数列: Vec<TokenStream> =
        意味モデル.個体列().iter().filter(|個体| !個体.値なし宣言か()).map(個体供給関数を組み立てる).collect();
    let 実引数列: Vec<TokenStream> = 意味モデル
        .個体列()
        .iter()
        .map(|個体| {
            let 名前 = 個体.名前();
            if 個体.値なし宣言か() {
                quote! { #名前 }
            } else {
                let 供給関数 = 個体供給関数名(名前);
                quote! { #供給関数() }
            }
        })
        .collect();
    let 仮引数列: Vec<TokenStream> = 意味モデル
        .個体列()
        .iter()
        .filter(|個体| 個体.値なし宣言か())
        .map(|個体| {
            let 名前 = 個体.名前();
            let 実体型 = 個体.実体型();
            quote! { #名前: #実体型 }
        })
        .collect();

    let 関数名 = 個体組み立て関数名(意味モデル, 宣言元);
    let doc = doc属性を組み立てる(関数名.追跡());

    quote! {
        #doc
        // グラフ名が大文字始まりの英語 (`graph Circle;` 等) でもよいため、
        // グラフ名を埋め込む関数名はsnake_caseの規約検査対象から外す。
        #[allow(non_snake_case)]
        pub(crate) fn #関数名(#(#仮引数列),*) -> #instance名::Nodes {
            #(#供給関数列)*
            #instance名::Nodes::new(#(#実引数列),*)
        }
    }
}

// `{グラフ名}の辺を組み立てる(nodes: &{instance名}::Nodes) -> {instance名}::Edges<'_>`。
pub(crate) fn 辺組み立て関数を組み立てる(
    instance名: &Ident,
    意味モデル: &意味モデル,
    宣言元: &宣言元の対,
) -> TokenStream {
    let 供給関数列: Vec<TokenStream> = 意味モデル
        .具体辺列()
        .iter()
        .filter(|辺| 辺.積み荷式().is_some())
        .map(積み荷供給関数を組み立てる)
        .collect();
    let 積み荷実引数列: Vec<TokenStream> = 意味モデル
        .具体辺列()
        .iter()
        .filter(|辺| 辺.積み荷式().is_some())
        .map(|辺| {
            let 供給関数 = 積み荷供給関数名(辺.名前());
            quote! { #供給関数() }
        })
        .collect();

    let 関数名 = 辺組み立て関数名(意味モデル, 宣言元);
    let doc = doc属性を組み立てる(関数名.追跡());
    let nodes = nodes変数名();

    quote! {
        #doc
        // グラフ名が大文字始まりの英語 (`graph Circle;` 等) でもよいため、
        // グラフ名を埋め込む関数名はsnake_caseの規約検査対象から外す。
        #[allow(non_snake_case)]
        pub(crate) fn #関数名(#nodes: &#instance名::Nodes) -> #instance名::Edges<'_> {
            #(#供給関数列)*
            #instance名::Edges::new(#nodes, #(#積み荷実引数列),*)
        }
    }
}
