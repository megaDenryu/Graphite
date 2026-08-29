// 生成物2: 種別契約。種別ごとに積み荷の型を仕組みの種別契約traitへimplする。
// これがレイヤー2への型チャネルになる (レイヤー2は具体の積み荷型を知らない
// まま `<種別 as 種別契約>::積み荷` と書けば型検査で結線される)。積み荷なしの
// 種別は `type 積み荷 = ();`。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::input::{辺形状, 辺宣言};

pub(super) fn 種別契約達を生成する(辺宣言達: &[辺宣言]) -> TokenStream {
    let 生成達 = 辺宣言達.iter().map(|辺| {
        let 名前 = &辺.名前;
        let 積み荷型 = 積み荷型を取り出す(辺);
        quote! {
            impl 種別契約 for #名前 {
                type 積み荷 = #積み荷型;
            }
        }
    });
    quote! { #(#生成達)* }
}

fn 積み荷型を取り出す(辺: &辺宣言) -> TokenStream {
    match &辺.形状 {
        辺形状::有向 { 積み荷: Some((_, 型)), .. } => quote! { #型 },
        _ => quote! { () },
    }
}
