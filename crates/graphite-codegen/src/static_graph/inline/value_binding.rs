// 値1件を、instanceの宣言位置に固定して計算するコードを組み立てる
// (`docs/static_graph.md`「値の式の名前解決」節)。この仕組みが名前解決を
// 宣言位置へ固定する核心であり、`value_supply.rs`はこの結果を束ねるだけ。
//
// 値の式は、常に宣言位置に置いた捕捉しない関数 (`fn`) の本体として固定
// される。項目 (fn) の本体は常にその項目が書かれた位置の通常のスコープで
// 名前解決されるため、instanceがモジュール直下 (項目の位置) にあっても
// 関数の中 (文の位置) にあっても、`let`を使わずに宣言位置の意味を保てる
// (Rustは入れ子の項目をブロックの中にも置ける)。関数の中に置いた`fn`は
// 外側の関数のローカル変数・引数を捕捉できないため、値の式がそれらを
// 参照すると通常のRustのE0434になり、外側のジェネリックの型引数を
// 参照すると通常のRustのE0401になる。症状と対処は
// `docs/static_graph.md`「値の式の名前解決」節に、compile-fail実測は
// `crates/graphite/tests/ui/
// static_value_expr_referencing_local_is_rejected.rs`に書く。
//
// 束縛マクロ (`値束縛マクロ名`) の本体はローカル変数を持たず、宣言位置の
// 関数呼び出しの結果を返すだけであり、式そのものは束縛マクロの外へは
// 出ない。束縛マクロが呼ばれる位置 (`construct!`の展開結果である値マクロ)
// がinstanceの宣言位置と離れていても、値の式そのものは宣言位置の`fn`の
// 本体に閉じているため、呼び出し位置に同名のローカル・関数があっても
// すり替わらない。束縛マクロの本体が呼ぶ関数名 (C分類の内部生成名
// `__graphite_value_*`) は、macro_rules!の衛生規則がローカル変数・
// ラベルにしか効かないため、`construct!`の呼び出し位置で解決される。
// 利用者がこの内部生成名と同じ名前の`fn`を呼び出し位置の近くで定義すると
// 乗っ取られる余地があるが、`__graphite_`で始まる名前の直接利用を
// Graphiteは支援しないため (`docs/static_graph.md`「制約」節)、この前提が
// 保たれる (実測は
// `crates/graphite/tests/static_value_expr_resolves_at_declaration_site.rs`
// を参照)。

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
