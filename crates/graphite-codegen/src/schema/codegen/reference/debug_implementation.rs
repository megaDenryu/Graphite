//! ノード参照・辺参照の `Debug` 実装を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

// `NodeRef`/`EdgeRef` の `Debug` impl を生成する。`&Graph` は表示しない。
//
// ID型・値型 (辺の場合は積み荷) に `Debug` を無条件要求しない契約
// (`gen_edge_value_structs` の同種の契約と対) を守る必要がある。当初は
// `where #id_ty: std::fmt::Debug` のような条件付き `impl` を試みたが、
// `#reference<'graph>` はライフタイムのみが型引数でID型・値型はmacro展開時
// に確定した具体型であるため、その `where` 節はジェネリック型引数を介した
// 遅延検査にはならず、定義時に即座に充足性が検査されることを実測で
// 確認した (2026-08-25、`cargo build --workspace --all-targets` で
// 利用者定義の非Debug型を使う既存テストが軒並みコンパイルエラーになった)。
// そのため `gen_edge_value_structs` の debug_impl と同じ方針
// (macro展開時に安全と判定できる範囲だけを表示する無条件 `impl`) を採る。
// 安全と判定できるのは自動生成ID型 (`gen_default_id_types` が常に
// `#[derive(Debug, ..)]` を付ける) の場合のみで、値型・積み荷型は利用者
// 定義でありmacroからは判定できないため表示対象に含めない。
pub(crate) fn gen_reference_debug_impl(reference: &Ident, id_is_generated: bool) -> TokenStream {
    let body = if id_is_generated {
        quote! {
            f.debug_struct(stringify!(#reference))
                .field("id", &self.id())
                .finish_non_exhaustive()
        }
    } else {
        quote! { f.write_str(stringify!(#reference)) }
    };
    quote! {
        impl<'graph> std::fmt::Debug for #reference<'graph> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                #body
            }
        }
    }
}
