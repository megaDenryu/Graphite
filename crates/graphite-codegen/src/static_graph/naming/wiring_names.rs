// 配線用の固定識別子。`graph` は、個体参照・辺インスタンス参照・
// `NodeRefs`/`EdgeRefs` が内部で持つ非公開の唯一のフィールド (借用した
// `&'a Graph`) の名前であり、Graphiteが定義する固定語彙ではあるが doc は
// 持たない (`fixed_vocabulary` が返す `pub` 契約の名前とは別。§5.3「配線用
// の生成物にはdocを付けない」)。生成器の複数ファイル
// (`file::instance_file` 配下) が同じ綴りを繰り返し書いていたため、ここへ
// 1箇所だけ置く。

use proc_macro2::{Ident, Span};

pub(crate) fn graphフィールド名() -> Ident {
    Ident::new("graph", Span::call_site())
}
