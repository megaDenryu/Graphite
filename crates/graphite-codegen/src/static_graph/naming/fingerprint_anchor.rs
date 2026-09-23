// 指紋照合コードが参照するmoduleパスの先頭識別子 (schema名/instance名) を
// `generated = "..."` リテラルのspanで作り直す。schema/instance module
// (`mod {名前} { include!(..); }`) の宣言を書き忘れた場合、このパスの
// 名前解決が失敗する (E0433) が、名前トークンのspanのままだと
// `schema {名前} { .. }`/`graph {名前};` の行を指してしまう。
// `generated = "...";` の書き忘れ・記入ミスがこの失敗の実際の原因である
// ことが多いため、このパスだけ `generated` リテラルの行を指すようにする
// (`docs/static_graph.md` 「追跡の契約」参照)。
//
// `schema_entry.rs`・`instance_entry.rs` の両方が同じ処理を要るため、ここへ
// 集約する (`format_ident!`/`span = ...` をnaming/だけに置く規律、
// `naming/mod.rs` 冒頭のコメント参照)。

use proc_macro2::{Ident, Span};

// `Ident::new(&名前.to_string(), span)` は使わない。生識別子 (`r#型`等) の
// `to_string()`は`r#`接頭辞を含む文字列を返すが、`Ident::new`はその文字列を
// 生識別子の記法として受理せずパニックする (`Ident::new`は生の文字列から
// 組み立て直す口であり、`r#`を構文として解釈しない)。`clone`してから
// `set_span`で差し替えれば、文字列を経由せず生識別子かどうかも含めてその
// まま引き継げる。
pub(crate) fn 指紋照合パスの起点(名前: &Ident, generated文字列のspan: Span) -> Ident {
    let mut 名前 = 名前.clone();
    名前.set_span(generated文字列のspan);
    名前
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn 生識別子でもパニックしない() {
        let 生識別子: Ident = syn::parse2(quote! { r#type }).unwrap();
        let 起点 = 指紋照合パスの起点(&生識別子, Span::call_site());
        assert_eq!(起点.to_string(), "r#type");
    }
}
