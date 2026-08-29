//! 静的グラフ型入力から生成物を並べる配線。並び順は「種別ラベル → 種別契約
//! → 役割アクセサ → 多重度の契約」。各生成物の中身は配下の module が持ち、
//! この module 本体は並び順だけを知る。

mod contract;
mod labels;
mod multiplicity;
mod role_accessors;

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::input::静的グラフ型入力;

pub(crate) fn スキーマコードを生成する(入力: &静的グラフ型入力) -> TokenStream {
    let 種別ラベル達 = labels::種別ラベル達を生成する(&入力.辺宣言達);
    let 種別契約達 = contract::種別契約達を生成する(&入力.辺宣言達);
    let 役割アクセサ達 = role_accessors::役割アクセサ達を生成する(&入力.辺宣言達);
    let 多重度契約達 = multiplicity::多重度契約達を生成する(&入力.ノード宣言達, &入力.辺宣言達);

    quote! {
        #種別ラベル達
        #種別契約達
        #役割アクセサ達
        #多重度契約達
    }
}
