// 生成物4: 個体ごとの具象参照struct (`{個体名}Ref`、旧 `{個体名}参照`)。
// 実体・Nodes・Edgesを直接保持し (仕組みへは依存しない)、entity() と所属辺
// メソッド (自分が端点になっている全辺インスタンス分、始点/終点/第1役割/
// 第2役割のどの位置でも生やす) を持つ。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::宣言元ファイルの綴り;
use crate::static_graph::naming::{個体参照型名, 辺参照型名};
use crate::static_graph::semantic::{個体, 意味モデル};

pub(super) fn 個体参照達を生成する(意味モデル: &意味モデル) -> TokenStream {
    let 生成達 = 意味モデル.個体列().iter().map(|個体| 一個体分を生成する(意味モデル, 個体));
    quote! { #(#生成達)* }
}

fn 一個体分を生成する(意味モデル: &意味モデル, 個体: &個体) -> TokenStream {
    let 実体型 = 個体.実体型();
    let 参照名 = 個体参照型名(意味モデル, 個体, &宣言元ファイルの綴り::分かっていない);

    let 所属辺メソッド達 = 意味モデル.この個体が端点になっている具体辺列(個体.名前()).map(|辺| {
        let メソッド名 = 辺.名前();
        let 戻り値型 = 辺参照型名(意味モデル, 辺, &宣言元ファイルの綴り::分かっていない);
        quote! {
            fn #メソッド名(&self) -> #戻り値型<'a> {
                #戻り値型 { entity: &self.edges.#メソッド名, nodes: self.nodes, edges: self.edges }
            }
        }
    });

    quote! {
        #[derive(Clone, Copy)]
        struct #参照名<'a> {
            entity: &'a #実体型,
            nodes: &'a Nodes,
            edges: &'a Edges<'a>,
        }
        impl<'a> #参照名<'a> {
            fn entity(&self) -> &'a #実体型 { self.entity }
            #(#所属辺メソッド達)*
        }
    }
}
