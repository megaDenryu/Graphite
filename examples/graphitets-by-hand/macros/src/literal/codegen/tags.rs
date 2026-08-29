// 生成物1: 個体タグ。1ノード宣言につき「印となる空struct + ノードタグ実装」の
// 組を1つ吐く。個体の実体型が何かという対応は、この impl 1行だけが知っている。
//
// pub は付けない。ノードの実体型 (男・女 等) はマクロの外、呼び出し側が
// module-private のまま書くのが普通なので、個体タグを pub にすると
// 「pub な型が非pubな型を公開interfaceへ漏らす」(E0446) になる
// (issue #24 段階1で発見)。展開先はマクロ呼び出しと同じモジュールなので、
// pub が無くても仕組み側からの参照に支障はない。

use proc_macro2::TokenStream;
use quote::quote;

use crate::literal::input::ノード宣言;

pub fn 個体タグ達を生成する(ノード宣言達: &[ノード宣言]) -> TokenStream {
    let 個体タグ達 = ノード宣言達.iter().map(|ノード| {
        let 名前 = &ノード.名前;
        let 実体型 = &ノード.実体型;
        quote! {
            struct #名前;
            impl ノードタグ for #名前 { type 実体 = #実体型; }
        }
    });
    quote! { #(#個体タグ達)* }
}
