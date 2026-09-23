// このファイルは `Nodes` (個体の実体を唯一持つ生成物) の本体を組み立てる。
// `new` は値の計算を一切持たない素の構築子であり、全個体を宣言順の位置
// 引数にそのまま取る。値ありの個体をinstance宣言の式から計算して渡すのは、
// instance展開側が呼び出し位置に生成する `{グラフ名}の個体を組み立てる`
// 関数 (`inline::assembly`) の役目であり、式そのものは生成ファイルへ
// 写さない。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{個体実体所有者型名, 個体実体所有者構築メソッド名, nodesフィールドの追跡情報を作る};
use crate::static_graph::semantic::意味モデル;

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn nodes本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let 型名 = 個体実体所有者型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    // §5.2 例4で書式を固定した意味カード (`naming::tests::card4_fixed_vocabulary`)。
    let 構築名 = 個体実体所有者構築メソッド名(意味モデル, 宣言元);
    let 構築doc = doc属性を組み立てる(構築名.追跡());

    let フィールド列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 実体型 = 個体.実体型();
        let doc = doc属性を組み立てる(&nodesフィールドの追跡情報を作る(意味モデル, 個体, 宣言元));
        quote! { #doc pub #名前: #実体型 }
    });
    let 引数列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 実体型 = 個体.実体型();
        quote! { #名前: #実体型 }
    });
    let 初期化列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        quote! { #名前 }
    });

    quote! {
        #型doc
        pub struct #型名 {
            #(#フィールド列,)*
        }
        impl #型名 {
            #構築doc
            pub fn #構築名(#(#引数列),*) -> Self {
                Self { #(#初期化列,)* }
            }
        }
    }
}
