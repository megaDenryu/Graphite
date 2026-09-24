// 値1件を、instanceの宣言位置に固定して計算するコードを組み立てる
// (`docs/static_graph.md`「値の式の名前解決」節)。この仕組みが名前解決を
// 宣言位置へ固定する核心であり、`value_supply.rs`はこの結果を束ねるだけ。
//
// - 関数内 (`in fn`) の場合は `let` でクロージャを束縛する。クロージャは
//   `move` を付けない。付けると借りるだけの式 (`社員 { 名前:
//   ローカル.clone() }`) までローカルを消費してしまい、宣言の後で
//   ローカルを使うコードがコンパイルできなくなる。`move` を付けなくても、
//   通常のRustのクロージャ捕捉推論が式の要求どおりに捕捉方法を選ぶため
//   (借りるだけの式は参照捕捉、値を消費する式は値捕捉)、値を move する式
//   では束縛マクロを2回呼ぶとクロージャが既に消費されているため2回目は
//   コンパイルエラーになる (FnOnce、通常のRustの意味と一致する)。
// - 項目位置の場合は捕捉しない関数として固定する。項目 (fn) の本体は
//   常にその項目が書かれた位置の通常のスコープで名前解決されるため、
//   `let` を使わなくても宣言位置の意味を保てる (モジュール直下にはそもそも
//   ローカル変数が存在しないので、捕捉できないことは制約にならない)。
//
// どちらの形でも、束縛マクロ (`値束縛マクロ名`) の本体はローカル変数
// (クロージャの束縛名) または宣言位置の関数呼び出しの結果を返すだけであり、
// 式そのものは束縛マクロの外へは出ない。束縛マクロが呼ばれる位置
// (`construct!`の展開結果である値マクロ) がinstanceの宣言位置と離れて
// いても、クロージャの
// 束縛名はmacro_rules!のローカル変数の衛生規則により宣言位置の束縛を指す
// (呼び出し位置に同名のローカル・関数があってもすり替わらない。実測は
// このファイル末尾の `tests` モジュール、および
// `crates/graphite/tests/static_value_expr_resolves_at_declaration_site.rs`・
// `crates/graphite/tests/static_value_expr_captures_locals_in_fn.rs`
// を参照)。

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Expr;

use crate::static_graph::literal::input::宣言位置;
use crate::static_graph::naming::{値キャプチャ変数名, 値関数名, 値束縛マクロ名};

pub(super) fn 値束縛を組み立てる(
    グラフ名: &Ident,
    名前: &Ident,
    型: Option<&Ident>,
    式: &Expr,
    位置: 宣言位置,
) -> TokenStream {
    let 束縛マクロ名 = 値束縛マクロ名(グラフ名, 名前);
    match 位置 {
        宣言位置::関数内位置 => {
            let キャプチャ変数名 = 値キャプチャ変数名(グラフ名, 名前);
            quote! {
                let #キャプチャ変数名 = || #式;
                macro_rules! #束縛マクロ名 {
                    () => {
                        (#キャプチャ変数名)()
                    };
                }
            }
        }
        宣言位置::項目位置 => {
            let 関数名 = 値関数名(グラフ名, 名前);
            let 戻り型 = 型.expect("項目位置の値は個体・積み荷どちらも型注釈を持つ");
            quote! {
                fn #関数名() -> #戻り型 {
                    #式
                }
                macro_rules! #束縛マクロ名 {
                    () => {
                        #関数名()
                    };
                }
            }
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
    fn 項目位置は捕捉しない関数と束縛マクロを生成する() {
        let 式: Expr = syn::parse_quote! { 社員 { 名前: "太郎".into() } };
        let コード = 値束縛を組み立てる(
            &識別子("開発チーム"),
            &識別子("太郎"),
            Some(&識別子("社員")),
            &式,
            宣言位置::項目位置,
        )
        .to_string();
        assert!(コード.contains("fn __graphite_value_太郎_開発チーム () -> 社員"));
        assert!(コード.contains("macro_rules ! __graphite_bind_太郎_開発チーム"));
        assert!(!コード.contains("let __graphite_captured"));
    }

    #[test]
    fn 関数内位置はlet束縛したクロージャと束縛マクロを生成する() {
        let 式: Expr = syn::parse_quote! { 社員 { 名前: 名前.clone() } };
        let コード = 値束縛を組み立てる(
            &識別子("開発チーム"),
            &識別子("太郎"),
            Some(&識別子("社員")),
            &式,
            宣言位置::関数内位置,
        )
        .to_string();
        assert!(コード.contains("let __graphite_captured_太郎_開発チーム = ||"));
        assert!(!コード.contains("move ||"));
        assert!(コード.contains("macro_rules ! __graphite_bind_太郎_開発チーム"));
        assert!(!コード.contains("fn __graphite_value_"));
    }
}
