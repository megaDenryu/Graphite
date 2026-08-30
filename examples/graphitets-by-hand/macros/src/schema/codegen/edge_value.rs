// 生成物1: 種別ごとの辺値struct。schemaだけから決まる (instanceを見ない)。
// 積み荷を持つ種別だけ積み荷フィールドを持つ。`静的グラフ型!` の展開へ
// macro_rules!と並ぶ実アイテムとして出力するため、同一schemaから`組織!`を
// 何度呼んでも (issue #24 段階2 コミット1のテスト参照)、この生成物自体は
// 1回しか展開されず重複定義にならない。

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::schema::input::{辺形状, 辺宣言, 静的グラフ型入力};

pub(super) fn 辺値struct達を生成する(schema: &静的グラフ型入力) -> TokenStream {
    let 生成達 = schema.辺宣言達.iter().map(一種別分を生成する);
    quote! { #(#生成達)* }
}

fn 一種別分を生成する(辺: &辺宣言) -> TokenStream {
    let 型名 = format_ident!("{}の辺", 辺.名前, span = 辺.名前.span());
    let 積み荷フィールド = 積み荷フィールドを生成する(辺.形状.積み荷());
    match &辺.形状 {
        辺形状::有向 { 始点役割, 始点型, 終点役割, 終点型, .. } => quote! {
            struct #型名<'a> {
                #始点役割: &'a #始点型,
                #終点役割: &'a #終点型,
                #積み荷フィールド
            }
        },
        辺形状::無向 { 第1役割, 第1型, 第2役割, 第2型, .. } => quote! {
            struct #型名<'a> {
                #第1役割: &'a #第1型,
                #第2役割: &'a #第2型,
                #積み荷フィールド
            }
        },
    }
}

fn 積み荷フィールドを生成する(積み荷: Option<&(Ident, Ident)>) -> TokenStream {
    match 積み荷 {
        Some((役割, 型)) => quote! { #役割: #型, },
        None => TokenStream::new(),
    }
}
