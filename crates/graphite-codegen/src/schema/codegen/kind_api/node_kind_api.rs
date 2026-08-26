//! ノード種別1つ分の種別API (`Graph` のメソッド) を生成する。

use proc_macro2::TokenStream;
use quote::quote;

use crate::naming::kind_api_method_ident;
use crate::schema::codegen::node_names::NodeInfo;

/// ノード種別1つ分の種別API (`Graph` のメソッド) を生成する。
///
/// IDE 支援 (`docs/ide_support_spec.md` §1.9, G3 ポリシー) のため、生成する
/// メソッド名にはノード型そのもののトークンのスパンを持たせる
/// (`accessor_ident` がノード型トークンのスパンを引き継いでいる)。
pub(crate) fn gen_node_kind_api_methods(node: &NodeInfo) -> TokenStream {
    let ty = &node.type_ident;
    let id_ty = &node.id_ty;
    let field = &node.field_ident;
    let accessor = &node.accessor_ident;
    let reference = node.reference_ident();
    let internal_position = node.internal_position_ident();
    let by_id = kind_api_method_ident(accessor, "by_id");
    let value_mut = kind_api_method_ident(accessor, "value_mut");
    let ids = kind_api_method_ident(accessor, "ids");
    let iter = kind_api_method_ident(accessor, "iter");
    let len = kind_api_method_ident(accessor, "len");
    quote! {
        /// 公開IDから完成済みグラフ上のノード個体を平均 O(1) で引く。
        pub fn #by_id<'graph>(&'graph self, id: &#id_ty) -> Option<#reference<'graph>> {
            let internal_position = #internal_position(self.#field.position(id)?);
            Some(#reference { graph: self, internal_position })
        }

        /// グラフの構造を保ったままノード値だけを可変借用する。
        pub fn #value_mut(&mut self, id: &#id_ty) -> Option<&mut super::#ty> {
            self.#field.get_mut(id)
        }

        /// この種別のノードの公開IDを挿入順に走査する。
        pub fn #ids<'graph>(&'graph self) -> impl Iterator<Item = &'graph #id_ty> {
            self.#field.ids()
        }

        /// この種別のノード個体を挿入順に走査する。追加確保はしない。
        pub fn #iter<'graph>(&'graph self) -> impl Iterator<Item = #reference<'graph>> + 'graph {
            self.#field.positions().map(move |position| #reference {
                graph: self,
                internal_position: #internal_position(position),
            })
        }

        /// この種別のノードの件数を返す。
        pub fn #len(&self) -> usize {
            self.#field.len()
        }
    }
}
