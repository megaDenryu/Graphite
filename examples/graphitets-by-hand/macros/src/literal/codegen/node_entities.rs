// 生成物2: ノード達。ノードの実体の唯一の所有者。各フィールドの初期化式には
// ノード宣言の右辺の構造体リテラル式をそのまま埋め込む (リテラル中の式を
// そのままコード生成に埋め込む、graph! リテラルらしい形。旧設計の
// new(個体名の値: 型) という引数渡しは廃止した — issue #24 段階2)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::literal::input::ノード宣言;

pub(super) fn ノード達を生成する(ノード宣言達: &[ノード宣言]) -> TokenStream {
    let フィールド達 = ノード宣言達.iter().map(|ノード| {
        let 名前 = &ノード.名前;
        let 実体型 = &ノード.実体型;
        quote! { #名前: #実体型 }
    });
    let 初期化達 = ノード宣言達.iter().map(|ノード| {
        let 名前 = &ノード.名前;
        let 式 = &ノード.式;
        quote! { #名前: #式 }
    });
    quote! {
        struct ノード達 {
            #(#フィールド達,)*
        }
        impl ノード達 {
            fn 初期値() -> Self {
                Self { #(#初期化達,)* }
            }
        }
    }
}
