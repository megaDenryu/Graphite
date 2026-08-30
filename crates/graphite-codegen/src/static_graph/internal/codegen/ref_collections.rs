// 生成物6: 参照の層の集まり (`NodeRefs`/`EdgeRefs`、旧 `ノード参照達`/
// `辺参照達`)。Nodes・Edges (実体の層) から作る。内部の個体名・辺名の
// フィールド名は利用者がinstance宣言に書いた名前をそのまま使う (README
// 「生成される名前の公開契約」参照)。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::static_graph::literal::input::静的グラフ入力;

pub(super) fn ノード参照達を生成する(instance: &静的グラフ入力) -> TokenStream {
    let フィールド達 = instance.ノード宣言達.iter().map(|n| {
        let 名前 = &n.名前;
        let 参照型 = format_ident!("{}Ref", 名前, span = 名前.span());
        quote! { #名前: #参照型<'a> }
    });
    let 初期化達 = instance.ノード宣言達.iter().map(|n| {
        let 名前 = &n.名前;
        let 参照型 = format_ident!("{}Ref", 名前, span = 名前.span());
        quote! { #名前: #参照型 { entity: &nodes.#名前, nodes, edges } }
    });
    quote! {
        struct NodeRefs<'a> {
            #(#フィールド達,)*
        }
        impl<'a> NodeRefs<'a> {
            fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
                Self { #(#初期化達,)* }
            }
        }
    }
}

pub(super) fn 辺参照達を生成する(instance: &静的グラフ入力) -> TokenStream {
    let フィールド達 = instance.辺宣言達.iter().map(|e| {
        let 名前 = &e.名前;
        let 参照型 = format_ident!("{}Ref", 名前, span = 名前.span());
        quote! { #名前: #参照型<'a> }
    });
    let 初期化達 = instance.辺宣言達.iter().map(|e| {
        let 名前 = &e.名前;
        let 参照型 = format_ident!("{}Ref", 名前, span = 名前.span());
        quote! { #名前: #参照型 { entity: &edges.#名前, nodes, edges } }
    });
    quote! {
        struct EdgeRefs<'a> {
            #(#フィールド達,)*
        }
        impl<'a> EdgeRefs<'a> {
            fn new(nodes: &'a Nodes, edges: &'a Edges<'a>) -> Self {
                Self { #(#初期化達,)* }
            }
        }
    }
}
