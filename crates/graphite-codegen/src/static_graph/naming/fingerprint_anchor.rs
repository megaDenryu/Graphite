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

pub(crate) fn 指紋照合パスの起点(名前: &Ident, generated文字列のspan: Span) -> Ident {
    Ident::new(&名前.to_string(), generated文字列のspan)
}
