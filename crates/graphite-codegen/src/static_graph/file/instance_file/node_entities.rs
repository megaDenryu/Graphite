// このファイルは `Nodes` (個体の実体を唯一持つ生成物) の本体を組み立てる。
// フィールドは非公開であり (PR #45レビューA)、値ありの個体・積み荷を公開
// APIから差し替えられる穴を閉じる。内部構築子 (`__graphite_internal_new`、
// C分類・`pub(crate)`・doc無し) は値の計算を一切持たない素の構築子で、
// 全個体を宣言順の位置引数にそのまま取る。この内部構築子を呼べるのは同じ
// 生成moduleの中にある `construct::nodes!` (`construct.rs`) だけであり、
// 値ありの個体をinstance宣言の式から計算するのはそちらの役目。式そのものは
// 生成ファイルへ写さない。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{個体実体所有者型名, 内部構築子名};
use crate::static_graph::semantic::意味モデル;

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn nodes本体を組み立てる(意味モデル: &意味モデル, _宣言元: &宣言元の対) -> TokenStream {
    let 型名 = 個体実体所有者型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let 内部構築子 = 内部構築子名();

    let フィールド列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 実体型 = 個体.実体型();
        quote! { #名前: #実体型 }
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
            #[doc(hidden)]
            pub(crate) fn #内部構築子(#(#引数列),*) -> Self {
                Self { #(#初期化列,)* }
            }
        }
    }
}
