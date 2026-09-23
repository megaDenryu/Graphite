// 生成物3: Edges (旧 辺達)。辺の実体の唯一の所有者。フィールド型は
// `{種別}Edge<'a>` (生成物1、`static_graph::naming::辺値型名`) を直に書く。
// フィールド名 (役割名) は `具体辺形状` が既に持つ (builder.rsが構築時に
// schemaから解決済み、issue #41 是正15。schemaの辺形状と突き合わせ直さ
// ない)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::naming::辺値型名;
use crate::static_graph::schema::input::積み荷宣言;
use crate::static_graph::semantic::{具体辺, 具体辺形状};

pub(super) fn 辺達を生成する(具体辺列: &[具体辺]) -> TokenStream {
    let フィールド達 = 具体辺列.iter().map(フィールドを生成する);
    let 配線達 = 具体辺列.iter().map(配線を生成する);
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

fn フィールドを生成する(辺: &具体辺) -> TokenStream {
    let 名前 = 辺.名前();
    let 型名 = 辺値型名(辺.種別(), &宣言元ファイルの綴り::分かっていない);
    quote! { #名前: #型名<'a> }
}

fn 配線を生成する(辺: &具体辺) -> TokenStream {
    let 名前 = 辺.名前();
    let 型名 = 辺値型名(辺.種別(), &宣言元ファイルの綴り::分かっていない);
    let 積み荷フィールド = 積み荷フィールドを生成する(辺);
    match 辺.形状() {
        具体辺形状::有向 { 始点役割, 始点, 終点役割, 終点, .. } => {
            let 始点名 = 始点.名前();
            let 終点名 = 終点.名前();
            quote! { #名前: #型名 { #始点役割: &nodes.#始点名, #終点役割: &nodes.#終点名, #積み荷フィールド } }
        }
        具体辺形状::無向 { 第1役割, 端点1, 第2役割, 端点2, .. } => {
            let 端点1名 = 端点1.名前();
            let 端点2名 = 端点2.名前();
            quote! { #名前: #型名 { #第1役割: &nodes.#端点1名, #第2役割: &nodes.#端点2名, #積み荷フィールド } }
        }
    }
}

fn 積み荷フィールドを生成する(辺: &具体辺) -> TokenStream {
    match (辺.種別().積み荷(), 辺.積み荷式()) {
        (Some(積み荷宣言 { 役割, .. }), Some(式)) => quote! { #役割: #式, },
        (None, None) => TokenStream::new(),
        _ => unreachable!("相互検証済みなので積み荷有無は一致している"),
    }
}
