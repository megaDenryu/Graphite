// 生成物7: グラフ本体。ノード参照達と辺参照達の2つを持つだけ。

use proc_macro2::TokenStream;
use quote::quote;

use crate::literal::input::静的グラフ入力;

pub(super) fn 生成する(instance: &静的グラフ入力) -> TokenStream {
    let グラフ名 = &instance.グラフ名;
    quote! {
        struct #グラフ名<'a> {
            ノード参照達: ノード参照達<'a>,
            辺参照達: 辺参照達<'a>,
        }
        impl<'a> #グラフ名<'a> {
            fn new(ノード達: &'a ノード達, 辺達: &'a 辺達<'a>) -> Self {
                Self {
                    ノード参照達: ノード参照達::作る(ノード達, 辺達),
                    辺参照達: 辺参照達::作る(ノード達, 辺達),
                }
            }
        }
    }
}
