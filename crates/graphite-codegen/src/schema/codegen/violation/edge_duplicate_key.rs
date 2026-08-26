//! 辺のキー重複を表す違反 variant とその表示を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::edge_names::EdgeInfo;

/// 辺のキー重複 (`{Kind}DuplicateKey`、v4 で追加。辺も第一級キーを持つため)。
pub(crate) fn gen_edge_duplicate_key_case(
    violation_ident: &Ident,
    edge: &EdgeInfo<'_>,
) -> (TokenStream, TokenStream) {
    let kind_str = edge.kind.to_string();
    let edge_id = &edge.id_ty;
    let dup_key = edge.duplicate_key_variant();
    let variant = quote! {
        /// このエッジ種別のキーが重複している。
        #dup_key(#edge_id)
    };
    let display_arm = if edge.id_ty.is_debug_printable() {
        quote! {
            #violation_ident::#dup_key(id) => write!(f, "{}のキーが重複しています: {:?}", #kind_str, id)
        }
    } else {
        quote! {
            #violation_ident::#dup_key(_) => write!(f, "{}のキーが重複しています", #kind_str)
        }
    };
    (variant, display_arm)
}
