// 配線用の固定識別子。`entity`/`nodes`/`edges` は個体参照・辺インスタンス
// 参照が内部で持つ非公開のフィールド・構築引数の名前であり、
// Graphiteが定義する固定語彙ではあるが doc は持たない
// (`fixed_vocabulary` が返す `pub` 契約の名前とは別。§5.3「配線用の生成物
// にはdocを付けない」)。生成器の複数ファイル (`file::instance_file` 配下)
// が同じ綴りを繰り返し書いていたため、ここへ1箇所だけ置く。

use proc_macro2::{Ident, Span};

pub(crate) fn entityフィールド名() -> Ident {
    Ident::new("entity", Span::call_site())
}

pub(crate) fn nodes変数名() -> Ident {
    Ident::new("nodes", Span::call_site())
}

pub(crate) fn edges変数名() -> Ident {
    Ident::new("edges", Span::call_site())
}
