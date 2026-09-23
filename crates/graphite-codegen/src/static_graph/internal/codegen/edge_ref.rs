// 生成物5: 辺インスタンスごとの具象参照struct (`{辺名}Ref`、旧
// `{辺名}参照`)。役割アクセサ (有向は始点役割/終点役割、無向は第1役割/
// 第2役割) は戻り値を「個体の具象参照型 (`{個体名}Ref`)」にする。積み荷を
// 持つ種別だけ積み荷アクセサ (役割名) を持つ。

use proc_macro2::{Ident, TokenStream};
use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::naming::{個体参照型名, 辺値型名, 辺参照型名};
use crate::static_graph::schema::input::積み荷宣言;
use crate::static_graph::semantic::{個体, 具体辺, 具体辺形状, 意味モデル};

pub(super) fn 辺インスタンス参照達を生成する(意味モデル: &意味モデル) -> TokenStream {
    let 生成達 = 意味モデル.具体辺列().iter().map(|辺| 一辺分を生成する(意味モデル, 辺));
    quote! { #(#生成達)* }
}

fn 一辺分を生成する(意味モデル: &意味モデル, 辺: &具体辺) -> TokenStream {
    let 参照名 = 辺参照型名(意味モデル, 辺, &宣言元ファイルの綴り::分かっていない);
    let 型名 = 辺値型名(辺.種別(), &宣言元ファイルの綴り::分かっていない);

    let ロールアクセサ達 = ロールアクセサ達を生成する(意味モデル, 辺);
    let 積み荷アクセサ = 積み荷アクセサを生成する(辺);

    quote! {
        #[derive(Clone, Copy)]
        struct #参照名<'a> {
            entity: &'a #型名<'a>,
            nodes: &'a Nodes,
            edges: &'a Edges<'a>,
        }
        impl<'a> #参照名<'a> {
            #ロールアクセサ達
            #積み荷アクセサ
        }
    }
}

fn ロールアクセサ達を生成する(意味モデル: &意味モデル, 辺: &具体辺) -> TokenStream {
    let (第一役割, 第一個体, 第二役割, 第二個体) = ロール対応を取り出す(辺);
    let 第一アクセサ = 一アクセサを生成する(意味モデル, &第一役割, 第一個体);
    let 第二アクセサ = 一アクセサを生成する(意味モデル, &第二役割, 第二個体);
    quote! {
        #第一アクセサ
        #第二アクセサ
    }
}

fn ロール対応を取り出す(辺: &具体辺) -> (Ident, &個体, Ident, &個体) {
    match 辺.形状() {
        具体辺形状::有向 { 始点役割, 始点, 終点役割, 終点, .. } => {
            (始点役割.clone(), 始点, 終点役割.clone(), 終点)
        }
        具体辺形状::無向 { 第1役割, 端点1, 第2役割, 端点2, .. } => {
            (第1役割.clone(), 端点1, 第2役割.clone(), 端点2)
        }
    }
}

// self.entity (辺値struct) が既に持つ端点参照 (#役割名 フィールド) を読んで
// 個体参照へ包む。self.nodes を個体名で引き直さない (辺が辿りの根拠、
// static_graph.rs 冒頭コメントの方針と同じ)。
fn 一アクセサを生成する(意味モデル: &意味モデル, 役割名: &Ident, 個体: &個体) -> TokenStream {
    let 戻り値型 = 個体参照型名(意味モデル, 個体, &宣言元ファイルの綴り::分かっていない);
    quote! {
        fn #役割名(&self) -> #戻り値型<'a> {
            #戻り値型 { entity: self.entity.#役割名, nodes: self.nodes, edges: self.edges }
        }
    }
}

fn 積み荷アクセサを生成する(辺: &具体辺) -> TokenStream {
    match 辺.種別().積み荷() {
        Some(積み荷宣言 { 役割, 型 }) => quote! {
            fn #役割(&self) -> &'a #型 { &self.entity.#役割 }
        },
        None => TokenStream::new(),
    }
}
