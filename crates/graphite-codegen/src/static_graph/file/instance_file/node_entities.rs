// このファイルは `Nodes` (個体の実体を唯一持つ生成物) の本体を組み立てる。
// フィールドは非公開であり、値ありの個体・積み荷を公開APIから差し替えられる
// 穴を閉じる。内部構築子 (`__graphite_internal_new`、C分類・`pub(crate)`・
// doc無し) は値の計算を一切持たない素の構築子で、全個体を宣言順の位置引数に
// そのまま取る。値ありの個体をinstance宣言の式から計算するのは
// `construct::nodes!` (`construct.rs`) の役目であり、式そのものは生成
// ファイルへ写さない。
//
// `pub(crate)`はクレート内のどこからでも呼べてしまい、「呼べるのは
// `construct::nodes!`だけ」という主張をstable Rustの可視性だけでは強制
// できない。`#[deprecated]`を添えて直接呼び出しを警告にし
// (`#![deny(warnings)]`の下ではエラーになる)、`construct::nodes!`の展開側
// では`#[allow(deprecated)]`で自分自身の呼び出しだけ許す
// (`file::instance_file::construct`)。直接の呼び出しを禁止でなく警告に
// するのは、stableのマクロ衛生では呼び出し元をマクロ展開だけに限定できない
// ため (回帰試験: `crates/graphite/tests/ui/static_internal_constructor_direct_call.rs`)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{個体実体所有者型名, 内部構築子名};
use crate::static_graph::semantic::意味モデル;

use crate::static_graph::doc_render::doc属性を組み立てる;
use super::内部構築子の非推奨NOTE;

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
            #[deprecated(note = #内部構築子の非推奨NOTE)]
            pub(crate) fn #内部構築子(#(#引数列),*) -> Self {
                Self { #(#初期化列,)* }
            }
        }
    }
}
