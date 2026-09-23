// 追跡付きの名前 (issue #41 §5.1)。公開生成物の名前を作れる唯一の型で、
// 追跡情報を必ず伴う。`quote::ToTokens` を実装するので、コード生成側は
// `quote! { #名前 }` のように直接埋め込める (`format_ident!` を直書きする
// 必要がなくなる)。
//
// 内部生成名 (issue #41 のC分類) は利用者トークンのspanを持たず、追跡情報
// も持たない。公開の追跡経路 (`追跡付きの名前`) へは混ざらない
// (`inline::value_supply`・`inline::token_anchor` の内部専用の名前として
// 使う)。

use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::static_graph::trace::追跡情報;

pub(crate) struct 追跡付きの名前 {
    ident: Ident,
    追跡: 追跡情報,
}

impl 追跡付きの名前 {
    pub(crate) fn new(ident: Ident, 追跡: 追跡情報) -> Self {
        Self { ident, 追跡 }
    }

    // 生成コードへ埋め込むのは `ToTokens` 経由 (`quote! { #名前 }`) であり、
    // `.ident()` は単体試験・意味カードを直接読む消費者だけが使う。
    #[allow(dead_code)]
    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }

    // `static_graph::file` が doc属性へ変換して生成ファイルへ出す
    // (`file::doc_render::doc属性を組み立てる`)。
    pub(crate) fn 追跡(&self) -> &追跡情報 {
        &self.追跡
    }
}

impl ToTokens for 追跡付きの名前 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
    }
}

// `inline::value_supply` の供給関数名・`inline::token_anchor` の錨関数名が
// 使う (`naming::internal_names` 参照)。
pub(crate) struct 内部生成名 {
    ident: Ident,
}

impl 内部生成名 {
    pub(crate) fn new(name: &str) -> Self {
        Self { ident: Ident::new(name, Span::call_site()) }
    }

    // `inline::value_supply` (供給関数の定義自体を組み立てる) が使う。この
    // 定義自体がその場展開の出力へ配線されていない間は、`inline::value_supply`
    // の単体試験だけが呼ぶ。生成ファイル側は `ToTokens` 経由で名前を埋め込む
    // だけで `.ident()` を呼ばない。
    #[allow(dead_code)]
    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }
}

impl ToTokens for 内部生成名 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.ident.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 内部生成名はcall_siteのspanを持つ() {
        let 名前 = 内部生成名::new("__graphite_x");
        assert_eq!(名前.ident().to_string(), "__graphite_x");
    }
}
