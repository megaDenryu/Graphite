// 生成物4b: 個体ごとの所属辺メソッド。メソッド名は辺の宣言名をそのまま使い、
// 両端の個体に同じ名前で生やす。無向辺は無向辺参照::new を使う。
//
// 台帳・ノード参照・辺参照・無向辺参照は lib クレート (graphitets_by_hand::
// 仕組み) の型であり、{個体}参照 はそれへの型別名でしかないため、ここへ直接
// inherent implを書くと孤児規則 (E0116) に落ちる (issue #24 段階1で発見)。
// 個体ごとにローカルtraitを1つ立てて blanket ではなく 1対1 で impl すること
// で回避する。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::literal::input::{ノード宣言, 辺形状, 辺宣言};

pub(super) fn 所属辺メソッド達を生成する(ノード宣言達: &[ノード宣言], 辺宣言達: &[辺宣言]) -> TokenStream {
    let 生成達 = ノード宣言達.iter().map(|ノード| 一個体分を生成する(ノード, 辺宣言達));
    quote! { #(#生成達)* }
}

fn 一個体分を生成する(ノード: &ノード宣言, 辺宣言達: &[辺宣言]) -> TokenStream {
    let 所属辺達: Vec<&辺宣言> = 辺宣言達.iter().filter(|辺| 辺.端点に含むか(&ノード.名前)).collect();
    if 所属辺達.is_empty() {
        return TokenStream::new();
    }

    let 個体名 = &ノード.名前;
    let 参照名 = format_ident!("{}参照", 個体名);
    let トレイト名 = format_ident!("{}の辺", 個体名);

    let 宣言達 = 所属辺達.iter().map(|辺| {
        let メソッド名 = &辺.名前;
        let 戻り値型 = format_ident!("{}参照", 辺.名前);
        quote! { fn #メソッド名(&self) -> #戻り値型<'a>; }
    });
    let 実装達 = 所属辺達.iter().map(|辺| {
        let メソッド名 = &辺.名前;
        let 戻り値型 = format_ident!("{}参照", 辺.名前);
        let 構築関数 = match &辺.形状 {
            辺形状::有向 { .. } => quote! { 辺参照::new },
            辺形状::無向 { .. } => quote! { 無向辺参照::new },
        };
        quote! {
            fn #メソッド名(&self) -> #戻り値型<'a> {
                #構築関数(&self.台帳.辺達.#メソッド名, self.台帳)
            }
        }
    });

    quote! {
        trait #トレイト名<'a> {
            #(#宣言達)*
        }
        impl<'a> #トレイト名<'a> for #参照名<'a> {
            #(#実装達)*
        }
    }
}

impl 辺宣言 {
    fn 端点に含むか(&self, 個体名: &proc_macro2::Ident) -> bool {
        match &self.形状 {
            辺形状::有向 { 始点, 終点, .. } => 始点 == 個体名 || 終点 == 個体名,
            辺形状::無向 { 端点1, 端点2 } => 端点1 == 個体名 || 端点2 == 個体名,
        }
    }
}
