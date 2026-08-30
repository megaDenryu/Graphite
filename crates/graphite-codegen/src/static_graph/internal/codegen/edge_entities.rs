// 生成物3: Edges (旧 辺達)。辺の実体の唯一の所有者。フィールド型は
// `{種別}Edge<'a>` (生成物1) を直に書く。フィールド名 (役割名) はschema側の
// 宣言から取る (instance側は個体名・積み荷式しか持たない)。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::static_graph::literal::input::{辺形状 as 具体形状, 辺宣言 as 具体辺宣言};
use crate::static_graph::schema::input::{辺形状 as 型形状, 静的グラフ型入力};

pub(super) fn 辺達を生成する(schema: &静的グラフ型入力, 辺宣言達: &[具体辺宣言]) -> TokenStream {
    let フィールド達 = 辺宣言達.iter().map(フィールドを生成する);
    let 配線達 = 辺宣言達.iter().map(|辺| {
        let 型宣言 = schema.辺宣言を種別名で探す(&辺.種別).expect("相互検証済みなので種別は必ず実在する");
        配線を生成する(辺, 型宣言)
    });
    quote! {
        struct Edges<'a> {
            #(#フィールド達,)*
        }
        impl<'a> Edges<'a> {
            fn new(nodes: &'a Nodes) -> Self {
                Self { #(#配線達,)* }
            }
        }
    }
}

fn フィールドを生成する(辺: &具体辺宣言) -> TokenStream {
    let 名前 = &辺.名前;
    let 型名 = format_ident!("{}Edge", 辺.種別, span = 辺.種別.span());
    quote! { #名前: #型名<'a> }
}

fn 配線を生成する(辺: &具体辺宣言, 型宣言: &crate::static_graph::schema::input::辺宣言) -> TokenStream {
    let 名前 = &辺.名前;
    let 型名 = format_ident!("{}Edge", 辺.種別, span = 辺.種別.span());
    let 積み荷フィールド = 積み荷フィールドを生成する(型宣言, 辺);
    match (&辺.形状, &型宣言.形状) {
        (具体形状::有向 { 始点, 終点, .. }, 型形状::有向 { 始点役割, 終点役割, .. }) => quote! {
            #名前: #型名 { #始点役割: &nodes.#始点, #終点役割: &nodes.#終点, #積み荷フィールド }
        },
        (具体形状::無向 { 端点1, 端点2, .. }, 型形状::無向 { 第1役割, 第2役割, .. }) => quote! {
            #名前: #型名 { #第1役割: &nodes.#端点1, #第2役割: &nodes.#端点2, #積み荷フィールド }
        },
        _ => unreachable!("相互検証済みなので向きは一致している"),
    }
}

fn 積み荷フィールドを生成する(型宣言: &crate::static_graph::schema::input::辺宣言, 辺: &具体辺宣言) -> TokenStream {
    match (型宣言.形状.積み荷(), 辺.形状.積み荷式()) {
        (Some((役割, _)), Some(式)) => quote! { #役割: #式, },
        (None, None) => TokenStream::new(),
        _ => unreachable!("相互検証済みなので積み荷有無は一致している"),
    }
}
