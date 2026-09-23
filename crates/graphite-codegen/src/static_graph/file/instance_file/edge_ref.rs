// このファイルは辺インスタンスごとの具象参照struct (`{辺名}Ref`) を組み立
// てる。配線フィールドは `pub(super)` にし、役割アクセサ・積み荷アクセサは
// `pub` + 意味カードにする。役割・端点の対応は `具体辺形状` から直接読み、
// schemaの辺形状と突き合わせ直さない。辺値型の参照は
// `{schema名}::{種別}Edge` に修飾する。

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{
    edges変数名, entityフィールド名, nodes変数名, 個体参照型名, 役割アクセサの追跡情報を作る, 辺値参照パス, 辺参照型名,
    積み荷アクセサの追跡情報を作る,
};
use crate::static_graph::schema::input::積み荷宣言;
use crate::static_graph::semantic::{個体, 具体辺, 具体辺形状, 意味モデル};

use super::super::doc_render::doc属性を組み立てる;

pub(super) fn 辺インスタンス参照列を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let 生成列 = 意味モデル.具体辺列().iter().map(|辺| 一辺分を組み立てる(意味モデル, 辺, 宣言元));
    quote! { #(#生成列)* }
}

fn 一辺分を組み立てる(意味モデル: &意味モデル, 辺: &具体辺, 宣言元: &宣言元の対) -> TokenStream {
    let 参照名 = 辺参照型名(意味モデル, 辺, 宣言元);
    let 型doc = doc属性を組み立てる(参照名.追跡());
    let 型参照 = 辺値参照パス(意味モデル, 辺);

    let 役割アクセサ列 = 役割アクセサ列を組み立てる(意味モデル, 辺, 宣言元);
    let 積み荷アクセサ = 積み荷アクセサを組み立てる(辺, 宣言元);
    let entity = entityフィールド名();
    let nodes = nodes変数名();
    let edges = edges変数名();

    quote! {
        #型doc
        #[derive(Clone, Copy)]
        pub struct #参照名<'a> {
            pub(super) #entity: &'a #型参照<'a>,
            pub(super) #nodes: &'a Nodes,
            pub(super) #edges: &'a Edges<'a>,
        }
        impl<'a> #参照名<'a> {
            #役割アクセサ列
            #積み荷アクセサ
        }
    }
}

fn 役割アクセサ列を組み立てる(意味モデル: &意味モデル, 辺: &具体辺, 宣言元: &宣言元の対) -> TokenStream {
    let (第一役割, 第一個体, 第二役割, 第二個体) = 役割対応を取り出す(辺);
    let 第一アクセサ = 一アクセサを組み立てる(意味モデル, 辺, &第一役割, 第一個体, 宣言元);
    let 第二アクセサ = 一アクセサを組み立てる(意味モデル, 辺, &第二役割, 第二個体, 宣言元);
    quote! {
        #第一アクセサ
        #第二アクセサ
    }
}

fn 役割対応を取り出す(辺: &具体辺) -> (Ident, &個体, Ident, &個体) {
    match 辺.形状() {
        具体辺形状::有向 { 始点役割, 始点, 終点役割, 終点, .. } => {
            (始点役割.clone(), 始点, 終点役割.clone(), 終点)
        }
        具体辺形状::無向 { 第1役割, 端点1, 第2役割, 端点2, .. } => {
            (第1役割.clone(), 端点1, 第2役割.clone(), 端点2)
        }
    }
}

fn 一アクセサを組み立てる(
    意味モデル: &意味モデル,
    辺: &具体辺,
    役割名: &Ident,
    個体: &個体,
    宣言元: &宣言元の対,
) -> TokenStream {
    let 戻り値型 = 個体参照型名(意味モデル, 個体, 宣言元);
    let doc = doc属性を組み立てる(&役割アクセサの追跡情報を作る(辺, 役割名, 個体, 宣言元));
    let entity = entityフィールド名();
    let nodes = nodes変数名();
    let edges = edges変数名();
    quote! {
        #doc
        pub fn #役割名(&self) -> #戻り値型<'a> {
            #戻り値型 { #entity: self.#entity.#役割名, #nodes: self.#nodes, #edges: self.#edges }
        }
    }
}

fn 積み荷アクセサを組み立てる(辺: &具体辺, 宣言元: &宣言元の対) -> TokenStream {
    match 辺.種別().積み荷() {
        Some(積み荷宣言 { 役割, 型 }) => {
            let doc = doc属性を組み立てる(&積み荷アクセサの追跡情報を作る(辺, 宣言元));
            let entity = entityフィールド名();
            quote! {
                #doc
                pub fn #役割(&self) -> &'a #型 { &self.#entity.#役割 }
            }
        }
        None => TokenStream::new(),
    }
}
