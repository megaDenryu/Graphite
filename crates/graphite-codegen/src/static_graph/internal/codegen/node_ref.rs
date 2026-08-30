// 生成物4: 個体ごとの具象参照struct (`{個体名}Ref`、旧 `{個体名}参照`)。
// 実体・Nodes・Edgesを直接保持し (仕組みへは依存しない)、entity() と所属辺
// メソッド (自分が端点になっている全辺インスタンス分、始点/終点/第1役割/
// 第2役割のどの位置でも生やす) を持つ。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::static_graph::literal::input::{ノード宣言, 辺宣言, 静的グラフ入力};

pub(super) fn 個体参照達を生成する(instance: &静的グラフ入力) -> TokenStream {
    let 生成達 = instance.ノード宣言達.iter().map(|個体| 一個体分を生成する(個体, &instance.辺宣言達));
    quote! { #(#生成達)* }
}

fn 一個体分を生成する(個体: &ノード宣言, 辺宣言達: &[辺宣言]) -> TokenStream {
    let 個体名 = &個体.名前;
    let 実体型 = &個体.実体型;
    let 参照名 = format_ident!("{}Ref", 個体名, span = 個体名.span());

    let 所属辺メソッド達 = 辺宣言達.iter().filter(|辺| 辺.端点に含むか(個体名)).map(|辺| {
        let メソッド名 = &辺.名前;
        let 戻り値型 = format_ident!("{}Ref", 辺.名前, span = 辺.名前.span());
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
