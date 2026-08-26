//! `where each` の役割名が辺値型のフィールドを指すことを確かめる検査文を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;

/// `where each <参照名>: ..` の IDE 支援専用ゼロコスト検査文
/// (`docs/ide_support_spec.md` §1.9)。
///
/// `<参照名>` は名前付きフィールドの辺値型の役割名フィールドへ参照させる。
pub(crate) fn gen_each_type_check(edge: &EdgeInfo<'_>) -> TokenStream {
    let kind = edge.kind;
    let checks = edge
        .定義
        .記述順の役割の多重度制約()
        .iter()
        .map(|constraint| {
            let role = constraint.役割名();
            quote! {
                let _: fn(&#kind) = |edge| {
                    let _ = &edge.#role;
                };
            }
        });
    quote! { #(#checks)* }
}
