// 生成物1: 種別ラベル。1辺宣言につき「印となる空struct」を1つ吐く。
//
// pub は付けない。個体タグと同じ理由 (`literal/codegen/tags.rs` 参照) で、
// 非pubな実体型を公開interfaceへ漏らさないため (issue #24 段階1で発見)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::input::辺宣言;

pub(super) fn 種別ラベル達を生成する(辺宣言達: &[辺宣言]) -> TokenStream {
    let 生成達 = 辺宣言達.iter().map(|辺| {
        let 名前 = &辺.名前;
        quote! { struct #名前; }
    });
    quote! { #(#生成達)* }
}
