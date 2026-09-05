//! ノードのキー重複を表す違反 variant とその表示を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::node_names::NodeInfo;

/// ノードのキー重複 (`Duplicate{Node}`、v3 から維持)。
pub(crate) fn gen_node_duplicate_key_case(
    violation_ident: &Ident,
    node: &NodeInfo<'_>,
) -> (TokenStream, TokenStream) {
    let v = node.dup_variant();
    let id = &node.id_ty;
    let type_name_str = node.type_ident.to_string();
    let variant = quote! {
        /// このノード種別のキーが重複している。
        #v(#id)
    };
    let display_arm = if node.id_ty.is_debug_printable() {
        quote! {
            #violation_ident::#v(id) => write!(f, "{}のキーが重複しています: {:?}", #type_name_str, id)
        }
    } else {
        quote! {
            #violation_ident::#v(_) => write!(f, "{}のキーが重複しています", #type_name_str)
        }
    };
    (variant, display_arm)
}
