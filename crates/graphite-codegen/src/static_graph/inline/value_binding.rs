// 生成器は、値1件を、instanceの宣言位置に固定して計算するコードを組み立てる。
// この仕組みが名前解決を宣言位置へ固定する核心であり、`value_supply.rs`は
// この結果を束ねるだけ。生成器は、値の式を宣言位置に置いた捕捉しない
// `fn`の本体として固定し、束縛マクロ (`値束縛マクロ名`) はその`fn`の
// 呼び出し結果を返すだけにする。macro_rules!の衛生規則はローカル変数・
// ラベルにしか効かないため、Rustの名前解決は、束縛マクロが呼ぶ関数名
// (C分類の内部生成名`__graphite_value_*`) を`construct!`の呼び出し位置で
// 解決する。利用者が、construct!の呼び出し位置から見える場所に
// `__graphite_value_*`と同じ名前のfnを定義しないことが前提であり、この
// 前提は利用者の責任であり、docs/static_graph.mdの制約節が定めている。
//
// 参照: `docs/static_graph.md`「値の式の名前解決」節・「制約」節

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Expr;

use crate::static_graph::naming::{値関数名, 値束縛マクロ名};

pub(super) fn 値束縛を組み立てる(グラフ名: &Ident, 名前: &Ident, 型: &Ident, 式: &Expr) -> TokenStream {
    let 束縛マクロ名 = 値束縛マクロ名(グラフ名, 名前);
    let 関数名 = 値関数名(グラフ名, 名前);
    quote! {
        fn #関数名() -> #型 {
            #式
        }
        macro_rules! #束縛マクロ名 {
            () => {
                #関数名()
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;

    fn 識別子(名前: &str) -> Ident {
        Ident::new(名前, Span::call_site())
    }

    #[test]
    fn 捕捉しない関数と束縛マクロを生成する() {
        let 式: Expr = syn::parse_quote! { 社員 { 名前: "太郎".into() } };
        let コード =
            値束縛を組み立てる(&識別子("開発チーム"), &識別子("太郎"), &識別子("社員"), &式).to_string();
        assert!(コード.contains("fn __graphite_value_太郎_開発チーム () -> 社員"));
        assert!(コード.contains("macro_rules ! __graphite_bind_太郎_開発チーム"));
    }
}
