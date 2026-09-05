//! 端点対索引の型を組み立て、グラフの保管庫と凍結処理へ同じ型を渡す。

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::codegen::edge_names::EdgeInfo;
use crate::schema::semantic::{端点対のキーの形, 端点対の重複可否};

// 端点対索引の `HashMap` 型を組み立てる。
//
// 凍結処理が作る一時変数と `Graph` のフィールドは同じ型でなければならないため、
// キーの形 (順序付き/順序なし) と値の形 (1本/複数本) の判断はここへ集約する。
pub(crate) fn gen_pair_index_map_type(edge: &EdgeInfo<'_>) -> TokenStream {
    let edge_position = edge.internal_position_ident();
    let from_position = edge.from_node.internal_position_ident();
    let to_position = edge.to_node.internal_position_ident();
    let key = match edge.定義.端点対のキーの形() {
        端点対のキーの形::順序付きの対 => quote! { (#from_position, #to_position) },
        端点対のキーの形::順序なしの対 => {
            quote! { graphite::UnorderedPair<#from_position> }
        }
    };
    let value = match edge.定義.端点対の重複可否() {
        端点対の重複可否::対ごとに1本だけ許す => quote! { #edge_position },
        端点対の重複可否::対ごとに何本でも許す => quote! { Vec<#edge_position> },
    };
    quote! { std::collections::HashMap<#key, #value> }
}
