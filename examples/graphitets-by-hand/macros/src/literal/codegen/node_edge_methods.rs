// 生成物4b: 個体ごとの所属辺メソッド。メソッド名は辺の宣言名をそのまま使い、
// 始点側の個体にも終点側の個体にも同じ名前で生やす。
//
// 台帳・ノード参照・辺参照は lib クレート (graphitets_by_hand::仕組み) の型で
// あり、{個体}参照 はそれへの型別名でしかないため、ここへ直接 inherent impl
// を書くと孤児規則 (E0116) に落ちる (issue #24 段階1で発見。詳細は
// static_graph.rs のコメント参照)。個体ごとにローカル trait を1つ立てて
// blanket ではなく 1 対 1 で impl することで回避する。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::literal::input::{ノード宣言, 辺宣言};

pub fn 所属辺メソッド達を生成する(ノード宣言達: &[ノード宣言], 辺宣言達: &[辺宣言]) -> TokenStream {
    let 生成達 = ノード宣言達.iter().map(|ノード| 一個体分を生成する(ノード, 辺宣言達));
    quote! { #(#生成達)* }
}

fn 一個体分を生成する(ノード: &ノード宣言, 辺宣言達: &[辺宣言]) -> TokenStream {
    let 所属辺達: Vec<&辺宣言> = 辺宣言達
        .iter()
        .filter(|辺| 辺.始点 == ノード.名前 || 辺.終点 == ノード.名前)
        .collect();
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
        quote! {
            fn #メソッド名(&self) -> #戻り値型<'a> {
                辺参照::new(&self.台帳.辺達.#メソッド名, self.台帳)
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
