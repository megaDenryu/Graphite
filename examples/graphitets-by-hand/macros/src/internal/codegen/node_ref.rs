// 生成物4: 個体ごとの具象参照struct。実体・ノード達・辺達を直接保持し
// (仕組みへは依存しない)、実体() と所属辺メソッド (自分が端点になっている
// 全辺インスタンス分、始点/終点/端点1/端点2のどの位置でも生やす) を持つ。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::literal::input::{ノード宣言, 辺宣言, 静的グラフ入力};

pub(super) fn 個体参照達を生成する(instance: &静的グラフ入力) -> TokenStream {
    let 生成達 = instance.ノード宣言達.iter().map(|個体| 一個体分を生成する(個体, &instance.辺宣言達));
    quote! { #(#生成達)* }
}

fn 一個体分を生成する(個体: &ノード宣言, 辺宣言達: &[辺宣言]) -> TokenStream {
    let 個体名 = &個体.名前;
    let 実体型 = &個体.実体型;
    let 参照名 = format_ident!("{}参照", 個体名, span = 個体名.span());

    let 所属辺メソッド達 = 辺宣言達.iter().filter(|辺| 辺.端点に含むか(個体名)).map(|辺| {
        let メソッド名 = &辺.名前;
        let 戻り値型 = format_ident!("{}参照", 辺.名前, span = 辺.名前.span());
        quote! {
            fn #メソッド名(&self) -> #戻り値型<'a> {
                #戻り値型 { 実体: &self.辺達.#メソッド名, ノード達: self.ノード達, 辺達: self.辺達 }
            }
        }
    });

    quote! {
        #[derive(Clone, Copy)]
        struct #参照名<'a> {
            実体: &'a #実体型,
            ノード達: &'a ノード達,
            辺達: &'a 辺達<'a>,
        }
        impl<'a> #参照名<'a> {
            fn 実体(&self) -> &'a #実体型 { self.実体 }
            #(#所属辺メソッド達)*
        }
    }
}
