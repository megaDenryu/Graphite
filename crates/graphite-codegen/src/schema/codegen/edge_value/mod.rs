//! 構築時に利用者が組み立てる辺値型を、部品ごとの生成へ振り分けて束ねる。

pub(crate) mod constructor;
pub(crate) mod debug_implementation;
pub(crate) mod literal_implementation;
pub(crate) mod struct_definition;

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use constructor::gen_edge_value_constructor;
use debug_implementation::gen_edge_value_debug_impl;
use literal_implementation::gen_edge_value_literal_impl;
use struct_definition::gen_edge_value_struct_definition;

// 辺種別ごとの公開名前付きフィールド値型を生成する。有向辺の端点と積み荷の
// フィールド名はスキーマの役割名をそのまま使う。無向辺は順序なし対を
// `endpoints` フィールドへ保持する。いずれもグラフを所有・借用しない普通のRust値。
pub(crate) fn gen_edge_value_structs(edges: &[EdgeInfo<'_>]) -> Vec<TokenStream> {
    edges
        .iter()
        .map(|e| {
            let kind = e.kind;
            let struct_def = gen_edge_value_struct_definition(e);
            let constructor = gen_edge_value_constructor(e);
            let literal_impl = gen_edge_value_literal_impl(e);
            let debug_impl = gen_edge_value_debug_impl(e);
            let struct_doc = format!(" 構築時に組み立てる `{kind}` 辺の値。");
            let 宣言元への参照 = &e.宣言元への参照;
            let derives = gen_edge_value_derives(e);
            quote! {
                #[doc = #struct_doc]
                #宣言元への参照
                #derives
                #struct_def

                impl #kind {
                    #constructor
                }

                #literal_impl
                #debug_impl
            }
        })
        .collect()
}

// 辺値型へ付ける導出属性を生成する。
//
// 利用者定義の積み荷へトレイトを要求しない契約を守るため、`PartialEq` を導出するのは
// 積み荷のない辺に限る。積み荷のある辺で導出すると、積み荷の型が `PartialEq` の実装を
// 強いられる (issue #27)。端点の公開ID型は表のキーとして `Eq + Hash` を既に要求して
// いるため、端点だけで構成される辺値の等値比較は利用者へ新しい要求を課さない。
// `Debug` を導出せず手書きしているのも同じ契約による (`debug_implementation.rs`)。
fn gen_edge_value_derives(e: &EdgeInfo<'_>) -> TokenStream {
    if e.payload().is_none() {
        quote! { #[derive(Clone, PartialEq)] }
    } else {
        quote! { #[derive(Clone)] }
    }
}
