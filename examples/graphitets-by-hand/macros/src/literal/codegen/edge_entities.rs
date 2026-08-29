// 生成物3: 辺達。辺の実体の唯一の所有者。フィールド型は 辺<'a, 種別, 始点の
// 実体型, 終点の実体型> を直に書く (恋人関係<'a> のような種類ごとの type alias
// は手書き版の可読性のための工夫であり、生成には要らないと分かった)。

use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::literal::input::辺宣言;

pub fn 辺達を生成する(辺宣言達: &[辺宣言], 実体型索引: &HashMap<String, Ident>) -> TokenStream {
    let フィールド達 = 辺宣言達.iter().map(|辺| {
        let 名前 = &辺.名前;
        let 種別 = &辺.種別;
        let 始点実体型 = &実体型索引[&辺.始点.to_string()];
        let 終点実体型 = &実体型索引[&辺.終点.to_string()];
        quote! { #名前: 辺<'a, #種別, #始点実体型, #終点実体型> }
    });
    let 配線達 = 辺宣言達.iter().map(|辺| {
        let 名前 = &辺.名前;
        let 種別 = &辺.種別;
        let 始点 = &辺.始点;
        let 終点 = &辺.終点;
        quote! { #名前: 結ぶ(#種別, &ノード達.#始点, &ノード達.#終点) }
    });
    quote! {
        struct 辺達<'a> {
            #(#フィールド達,)*
        }
        impl<'a> 辺達<'a> {
            fn 張る(ノード達: &'a ノード達) -> Self {
                Self { #(#配線達,)* }
            }
        }
    }
}
