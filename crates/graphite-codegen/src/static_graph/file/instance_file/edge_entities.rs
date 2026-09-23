// このファイルは `Edges` (辺の実体を唯一持つ生成物) の本体を組み立てる。
// 積み荷ありの具体辺は、積み荷式を供給関数呼び出し (`inline::value_supply`、
// issue #41 §3) に置き換える。フィールド型は `{schema名}::{種別}Edge<'a>`
// (schemaファイル、`naming::reference_paths::辺値参照パス`) を module越しに
// 修飾して書く (`所属Edge` はschema moduleの中にあり、instance module の
// `use super::*;` からは修飾なしで解決できない。issue #41 是正2)。向きの
// 判定は `具体辺形状` だけで完結し、schemaの辺形状 (`型形状`) と突き合わせ
// 直さない (役割はbuilder.rsが構築時に解決済み、issue #41 是正15)。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::naming::{
    edgesフィールドの追跡情報を作る, 構築メソッド名, 積み荷供給関数名, 辺値参照パス, 辺実体所有者型名,
};
use crate::static_graph::schema::input::積み荷宣言;
use crate::static_graph::semantic::{具体辺, 具体辺形状, 意味モデル};
use crate::static_graph::trace::固定語彙の所有者;

use super::super::doc_render::doc属性を組み立てる;

pub(super) fn edges本体を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元ファイルの綴り) -> TokenStream {
    let 型名 = 辺実体所有者型名(意味モデル);
    let 型doc = doc属性を組み立てる(型名.追跡());
    let 構築名 = 構築メソッド名(固定語彙の所有者::Edges, 意味モデル);
    let 構築doc = doc属性を組み立てる(構築名.追跡());

    let フィールド列 = 意味モデル.具体辺列().iter().map(|辺| フィールドを組み立てる(意味モデル, 辺, 宣言元));
    let 配線列 = 意味モデル.具体辺列().iter().map(|辺| 配線を組み立てる(意味モデル, 辺));

    quote! {
        #型doc
        pub struct #型名<'a> {
            #(#フィールド列,)*
        }
        impl<'a> #型名<'a> {
            #構築doc
            pub fn #構築名(nodes: &'a Nodes) -> Self {
                Self { #(#配線列,)* }
            }
        }
    }
}

fn フィールドを組み立てる(意味モデル: &意味モデル, 辺: &具体辺, 宣言元: &宣言元ファイルの綴り) -> TokenStream {
    let 名前 = 辺.名前();
    let 型参照 = 辺値参照パス(意味モデル, 辺);
    let doc = doc属性を組み立てる(&edgesフィールドの追跡情報を作る(意味モデル, 辺, 宣言元));
    quote! { #doc pub #名前: #型参照<'a> }
}

fn 配線を組み立てる(意味モデル: &意味モデル, 辺: &具体辺) -> TokenStream {
    let 名前 = 辺.名前();
    let 型参照 = 辺値参照パス(意味モデル, 辺);
    let 積み荷フィールド = 積み荷フィールドを組み立てる(辺);
    match 辺.形状() {
        具体辺形状::有向 { 始点役割, 始点, 終点役割, 終点, .. } => {
            let 始点名 = 始点.名前();
            let 終点名 = 終点.名前();
            quote! { #名前: #型参照 { #始点役割: &nodes.#始点名, #終点役割: &nodes.#終点名, #積み荷フィールド } }
        }
        具体辺形状::無向 { 第1役割, 端点1, 第2役割, 端点2, .. } => {
            let 端点1名 = 端点1.名前();
            let 端点2名 = 端点2.名前();
            quote! { #名前: #型参照 { #第1役割: &nodes.#端点1名, #第2役割: &nodes.#端点2名, #積み荷フィールド } }
        }
    }
}

fn 積み荷フィールドを組み立てる(辺: &具体辺) -> TokenStream {
    match (辺.種別().積み荷(), 辺.積み荷式()) {
        (Some(積み荷宣言 { 役割, .. }), Some(_)) => {
            let 供給関数 = 積み荷供給関数名(辺.名前());
            quote! { #役割: Self::#供給関数(), }
        }
        (None, None) => TokenStream::new(),
        _ => unreachable!("相互検証済みなので積み荷有無は一致している"),
    }
}
