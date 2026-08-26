//! ノード表を組み立て、公開IDの重複を違反として記録する文を生成する。

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::schema::codegen::node_names::NodeInfo;

/// ノード種別1つ分のノード表を凍結時に組み立てる文を生成する。
/// 同じ公開IDが2回現れたら重複違反として記録し、後から来た値は捨てる。
pub(crate) fn gen_node_table_freeze_block(
    violation_ident: &Ident,
    n: &NodeInfo<'_>,
) -> TokenStream {
    let field = &n.field_ident;
    let dup_variant = n.dup_variant();
    quote! {
        let mut #field: graphite::KeyedTable<_, _> = graphite::KeyedTable::new();
        for (id, value) in self.#field {
            if !#field.insert(id.clone(), value) {
                __violations.push(#violation_ident::#dup_variant(id));
            }
        }
    }
}
