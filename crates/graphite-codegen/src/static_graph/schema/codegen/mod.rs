//! schemaだけから決まる生成物 (instanceを見ない) をまとめる。「辺値struct群
//! (種別ごとの `{種別}Edge<'a>`)」と「node宣言の型アンカー」の2つを、
//! `static_schema!` の展開へ macro_rules! と並ぶ実アイテムとして出力する。
//! これにより schema トークンが macro_rules! 本体に焼き込まれた不活性な
//! トークン列でなく、rust-analyzer が解釈できる実際のRustアイテムになる。
//! 並び順だけをこのmodule本体が知り、各生成物の中身は配下のmoduleが持つ
//! (internal/codegen/mod.rs と同じ構成)。

mod edge_value;
mod type_anchor;

use proc_macro2::TokenStream;
use quote::quote;

use crate::static_graph::schema::input::静的グラフ型入力;

pub(crate) fn 骨組みを生成する(schema: &静的グラフ型入力) -> TokenStream {
    let 辺値struct達 = edge_value::辺値struct達を生成する(schema);
    let 型アンカー = type_anchor::型アンカーを生成する(schema);
    quote! {
        #辺値struct達
        #型アンカー
    }
}
