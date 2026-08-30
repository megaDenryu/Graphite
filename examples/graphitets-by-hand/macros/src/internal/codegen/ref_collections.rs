// 生成物6: 参照の層の集まり。ノード達・辺達 (実体の層) から作る。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::literal::input::静的グラフ入力;

pub(super) fn ノード参照達を生成する(instance: &静的グラフ入力) -> TokenStream {
    let フィールド達 = instance.ノード宣言達.iter().map(|n| {
        let 名前 = &n.名前;
        let 参照型 = format_ident!("{}参照", 名前);
        quote! { #名前: #参照型<'a> }
    });
    let 初期化達 = instance.ノード宣言達.iter().map(|n| {
        let 名前 = &n.名前;
        let 参照型 = format_ident!("{}参照", 名前);
        quote! { #名前: #参照型 { 実体: &ノード達.#名前, ノード達, 辺達 } }
    });
    quote! {
        struct ノード参照達<'a> {
            #(#フィールド達,)*
        }
        impl<'a> ノード参照達<'a> {
            fn 作る(ノード達: &'a ノード達, 辺達: &'a 辺達<'a>) -> Self {
                Self { #(#初期化達,)* }
            }
        }
    }
}

pub(super) fn 辺参照達を生成する(instance: &静的グラフ入力) -> TokenStream {
    let フィールド達 = instance.辺宣言達.iter().map(|e| {
        let 名前 = &e.名前;
        let 参照型 = format_ident!("{}参照", 名前);
        quote! { #名前: #参照型<'a> }
    });
    let 初期化達 = instance.辺宣言達.iter().map(|e| {
        let 名前 = &e.名前;
        let 参照型 = format_ident!("{}参照", 名前);
        quote! { #名前: #参照型 { 実体: &辺達.#名前, ノード達, 辺達 } }
    });
    quote! {
        struct 辺参照達<'a> {
            #(#フィールド達,)*
        }
        impl<'a> 辺参照達<'a> {
            fn 作る(ノード達: &'a ノード達, 辺達: &'a 辺達<'a>) -> Self {
                Self { #(#初期化達,)* }
            }
        }
    }
}
