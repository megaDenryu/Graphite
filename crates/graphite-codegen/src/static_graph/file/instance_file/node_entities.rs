// このファイルは `Nodes` (個体の実体を唯一持つ生成物) の本体を組み立てる。
// 値ありの個体は、初期化式を供給関数呼び出し (`inline::value_supply`、
// issue #41 §3) に置き換え、式そのものは生成ファイルへ写さない。値なしの
// 個体は宣言順の位置引数として `new` に加える (`internal/codegen/node_entities.rs`
// と同じ実行時供給)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::naming::{
    個体供給関数名, 個体実体所有者型名, 個体実体所有者構築メソッド名, nodesフィールドの追跡情報を作る,
};
use crate::static_graph::semantic::意味モデル;

use super::super::doc_render::doc属性を組み立てる;

pub(super) fn nodes本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元ファイルの綴り) -> TokenStream {
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
    let 引数列 = 意味モデル.個体列().iter().filter(|個体| 個体.値なし宣言か()).map(|個体| {
        let 名前 = 個体.名前();
        let 実体型 = 個体.実体型();
        quote! { #名前: #実体型 }
    });
    let 初期化列 = 意味モデル.個体列().iter().map(|個体| {
        let 名前 = 個体.名前();
        let 初期化式 = match 個体.値() {
            Some(_) => {
                let 供給関数 = 個体供給関数名(名前);
                quote! { Self::#供給関数() }
            }
            None => quote! { #名前 },
        };
        quote! { #名前: #初期化式 }
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
