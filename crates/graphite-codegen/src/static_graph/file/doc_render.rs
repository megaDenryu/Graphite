// 意味カードの本文を `#[doc = " .."]` 属性の列へ変換する (rustdoc/hoverの
// 表示規約。動的グラフの `crate::schema::codegen::declaration_doc` と同じ
// 書式で、内容行の先頭に半角スペースを1つ入れ、段落の区切りは空文字列の
// `#[doc = ""]` にする)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::trace::追跡情報;

pub(super) fn doc属性を組み立てる(追跡: &追跡情報) -> TokenStream {
    let 意味カード = 追跡.意味カード();
    let 行列 = 意味カード.split('\n').map(|行| {
        if 行.is_empty() {
            quote! { #[doc = ""] }
        } else {
            let 行 = format!(" {行}");
            quote! { #[doc = #行] }
        }
    });
    quote! { #(#行列)* }
}
