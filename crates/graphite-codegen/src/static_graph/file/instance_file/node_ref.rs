// このファイルは個体ごとの具象参照struct (`{個体名}Ref`) を組み立てる。
// 配線フィールドは借用した `graph: &'a Graph` の1つだけであり、非公開に
// し、docは付けない。非公開にすることで、利用者が構造体リテラルで
// `{個体名}Ref`を直接作り、由来の異なる`Graph`を混ぜた不整合な値を
// 組み立てる迂回を防ぐ。`Graph`・`NodeRefs`・`EdgeRefs`のフィールドも
// 同じ理由で非公開にしてあり (`graph_struct.rs`・`ref_collections.rs`)、
// 読み出しは`node_refs()`/`edge_refs()`等のメソッドで行う
// (`docs/static_graph.md`「制約」節参照)。
// `entity()` と所属辺メソッドは `pub` + 意味カードにする。実体は
// `&self.#graph.{個体名}` (`Graph`が個体名をそのまま非公開フィールド名に
// する、`graph_struct.rs`参照) を直接読むだけであり、`Graph`を経由する
// ため由来の異なる実体を混ぜる余地が無い。

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::declaration_sites::宣言元の対;
use crate::static_graph::naming::{
    個体参照型名, 実体アクセサメソッド名, 辺アクセサメソッドの追跡情報を作る, 辺参照型名, graphフィールド名,
};
use crate::static_graph::semantic::{個体, 意味モデル};

use crate::static_graph::doc_render::doc属性を組み立てる;

pub(super) fn 個体参照列を組み立てる(意味モデル: &意味モデル, 宣言元: &宣言元の対) -> TokenStream {
    let 生成列 = 意味モデル.個体列().iter().map(|個体| 一個体分を組み立てる(意味モデル, 個体, 宣言元));
    quote! { #(#生成列)* }
}

fn 一個体分を組み立てる(意味モデル: &意味モデル, 個体: &個体, 宣言元: &宣言元の対) -> TokenStream {
    let 実体型 = 個体.実体型();
    let 個体名 = 個体.名前();
    let 参照名 = 個体参照型名(意味モデル, 個体, 宣言元);
    let 型doc = doc属性を組み立てる(参照名.追跡());
    let entity名 = 実体アクセサメソッド名(意味モデル);
    let entity_doc = doc属性を組み立てる(entity名.追跡());
    let graph = graphフィールド名();

    let 所属辺メソッド列 = 意味モデル.この個体が端点になっている具体辺列(個体.名前()).map(|辺| {
        let メソッド名 = 辺.名前();
        let 戻り値型 = 辺参照型名(意味モデル, 辺, 宣言元);
        let doc =
            doc属性を組み立てる(&辺アクセサメソッドの追跡情報を作る(意味モデル, 個体, 辺, 宣言元));
        quote! {
            #doc
            pub fn #メソッド名(&self) -> #戻り値型<'a> {
                #戻り値型 { #graph: self.#graph }
            }
        }
    });

    quote! {
        #型doc
        #[derive(Clone, Copy)]
        pub struct #参照名<'a> {
            #graph: &'a Graph,
        }
        impl<'a> #参照名<'a> {
            #entity_doc
            pub fn #entity名(&self) -> &'a #実体型 { &self.#graph.#個体名 }
            #(#所属辺メソッド列)*
        }
    }
}
