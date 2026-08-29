// 対一意 (`unique pair`) の契約生成。種別自身へimplする (実体型へではない)。
// 有向・無向を問わず、全種別に1本ずつ生成する。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::input::{制約, 辺宣言};

pub(super) fn 対一意traitを生成する(辺宣言: &辺宣言) -> TokenStream {
    let 種別 = &辺宣言.名前;
    let トレイト名 = format_ident!("{}の対一意", 種別);
    let 有効 = 辺宣言.制約達.iter().any(|c| matches!(c, 制約::対一意));
    quote! {
        trait #トレイト名 {
            const 有効: bool;
        }
        impl #トレイト名 for #種別 {
            const 有効: bool = #有効;
        }
    }
}
