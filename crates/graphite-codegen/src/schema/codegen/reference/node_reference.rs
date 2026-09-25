//! ノード種別ごとの `NodeRef` 型と、その読み取り・探索メソッドを生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::codegen::node_names::NodeInfo;
use crate::schema::codegen::reference::debug_implementation::gen_reference_debug_impl;
use crate::schema::codegen::traversal::gen_node_traversal_methods;
use crate::schema::semantic::ノードの探索計画;

// 生成物は利用側の crate で `clippy::expect_used` の検査を受けるため、到達したら
// バグである箇所を `.expect()` でなく、破られた不変条件を書いた `panic!` で表す (issue #48)。
const ノード参照の内部位置の不変条件: &str = "NodeRefの内部位置は凍結後に不変のノード表を指す(生成元と異なるGraphへの束縛はbindの構築印照合で防いでいるため、ここに到達する場合は内部位置の不変条件が別の原因で破れている)";

// ノード種別1つ分の `NodeRef` 型と、そこへ生やす読み取り・探索メソッドを生成する。
//
// `NodeRef` は親 `Graph` と内部位置だけを持つ複製可能な参照値である。値そのものは
// `Deref` でも読めるようにして、ノード値型のメソッドをそのまま呼べるようにする。
pub(crate) fn gen_node_reference_type(
    graph_ident: &Ident,
    n: &NodeInfo<'_>,
    探索計画: &ノードの探索計画,
    edges: &[EdgeInfo<'_>],
) -> TokenStream {
    let ty = &n.type_ident;
    let id_ty = &n.id_ty;
    let field = &n.field_ident;
    let reference = n.reference_ident();
    let internal_position = n.internal_position_ident();
    let span = ty.span();
    let node_ref_id_ident = Ident::new("id", span);
    let node_ref_value_ident = Ident::new("value", span);
    let node_debug_impl = gen_reference_debug_impl(&reference, n.id_ty.is_debug_printable());
    let traversal_methods = gen_node_traversal_methods(探索計画, edges);
    let reference_doc = format!(" 完成済みグラフ上の `{ty}` ノード個体。");
    let 宣言元への参照 = &n.宣言元への参照;
    let 内部位置の不変条件 = ノード参照の内部位置の不変条件;
    quote! {
        #[doc = #reference_doc]
        #宣言元への参照
        #[derive(Clone, Copy)]
        pub struct #reference<'graph> {
            graph: &'graph #graph_ident,
            internal_position: #internal_position,
        }

        impl<'graph> #reference<'graph> {
            /// このノード個体の公開IDを借用する。
            #宣言元への参照
            pub fn #node_ref_id_ident(self) -> &'graph #id_ty {
                let Some((id, _)) = self.graph.#field.get_at(self.internal_position.0) else {
                    panic!(#内部位置の不変条件)
                };
                id
            }

            /// このノード個体のノード値を借用する。
            #宣言元への参照
            pub fn #node_ref_value_ident(self) -> &'graph super::#ty {
                let Some((_, value)) = self.graph.#field.get_at(self.internal_position.0) else {
                    panic!(#内部位置の不変条件)
                };
                value
            }

            #(#traversal_methods)*
        }

        impl<'graph> std::ops::Deref for #reference<'graph> {
            type Target = super::#ty;

            fn deref(&self) -> &Self::Target {
                let Some((_, value)) = self.graph.#field.get_at(self.internal_position.0) else {
                    panic!(#内部位置の不変条件)
                };
                value
            }
        }

        #node_debug_impl
    }
}
