//! 種別ごとの追加メソッド (値の型を手書きで書く入口) を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;

/// 種別ごとの追加メソッド (`b.person(id, value)` 群) を生成する。
/// 値の型を手書きで書ける場合に使う、型推論に頼らない直接の入口である。
pub(crate) fn gen_builder_kind_methods(
    nodes: &[NodeInfo<'_>],
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let node_methods = nodes.iter().map(|n| {
        let accessor = &n.accessor_ident;
        let field = &n.field_ident;
        let id_ty = &n.id_ty;
        let ty = &n.type_ident;
        let 宣言元への参照 = &n.宣言元への参照;
        quote! {
            /// この種別のノードを公開IDと値の組で追加する。検査は凍結時に行う。
            #宣言元への参照
            pub fn #accessor(&mut self, id: #id_ty, value: super::#ty) -> &mut Self {
                self.#field.push((id, value));
                self
            }
        }
    });

    let edge_methods = edges.iter().map(|e| {
        let accessor = &e.accessor_ident;
        let id_ty = &e.id_ty;
        let kind = e.kind;
        let 宣言元への参照 = &e.宣言元への参照;
        quote! {
            /// この種別の辺を公開IDと辺値の組で追加する。検査は凍結時に行う。
            #宣言元への参照
            pub fn #accessor(&mut self, id: #id_ty, value: #kind) -> &mut Self {
                self.#accessor.push((id, value));
                self
            }
        }
    });

    quote! {
        #(#node_methods)*
        #(#edge_methods)*
    }
}
