// このファイルは個体ごとの具象参照struct (`{個体名}Ref`) を組み立てる。
// 実体・Nodes・Edgesへの配線フィールド (entity/nodes/edges) は
// `pub(super)` にし、docは付けない。`entity()` と所属辺メソッドは `pub` +
// 意味カードにする。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{
    edges変数名, entityフィールド名, nodes変数名, 個体参照型名, 実体アクセサメソッド名, 辺アクセサメソッドの追跡情報を作る,
    辺参照型名,
};
use crate::static_graph::semantic::{個体, 意味モデル};

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn 個体参照列を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let 生成列 = 意味モデル.個体列().iter().map(|個体| 一個体分を組み立てる(意味モデル, 個体, 宣言元));
    quote! { #(#生成列)* }
}

fn 一個体分を組み立てる(意味モデル: &意味モデル, 個体: &個体, 宣言元: &宣言元の対) -> TokenStream {
    let 実体型 = 個体.実体型();
    let 参照名 = 個体参照型名(意味モデル, 個体, 宣言元);
    let 型doc = doc属性を組み立てる(参照名.追跡());
    let entity名 = 実体アクセサメソッド名(意味モデル);
    let entity_doc = doc属性を組み立てる(entity名.追跡());
    let entity = entityフィールド名();
    let nodes = nodes変数名();
    let edges = edges変数名();

    let 所属辺メソッド列 = 意味モデル.この個体が端点になっている具体辺列(個体.名前()).map(|辺| {
        let メソッド名 = 辺.名前();
        let 戻り値型 = 辺参照型名(意味モデル, 辺, 宣言元);
        let doc =
            doc属性を組み立てる(&辺アクセサメソッドの追跡情報を作る(意味モデル, 個体, 辺, 宣言元));
        quote! {
            #doc
            pub fn #メソッド名(&self) -> #戻り値型<'a> {
                #戻り値型 { #entity: &self.#edges.#メソッド名, #nodes: self.#nodes, #edges: self.#edges }
            }
        }
    });

    quote! {
        #型doc
        #[derive(Clone, Copy)]
        pub struct #参照名<'a> {
            pub(super) #entity: &'a #実体型,
            pub(super) #nodes: &'a Nodes,
            pub(super) #edges: &'a Edges<'a>,
        }
        impl<'a> #参照名<'a> {
            #entity_doc
            pub fn #entity名(&self) -> &'a #実体型 { self.#entity }
            #(#所属辺メソッド列)*
        }
    }
}
