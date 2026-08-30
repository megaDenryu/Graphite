//! schemaだけから決まる生成物 (instanceを見ない) をまとめる。まず「辺値
//! struct群 (種別ごとの `{種別}の辺<'a>`)」を、`静的グラフ型!` の展開へ
//! macro_rules! と並ぶ実アイテムとして出力する。これにより schema トークン
//! が macro_rules! 本体に焼き込まれた不活性なトークン列でなく、
//! rust-analyzer が解釈できる実際のRustアイテムになる (issue #24 段階2)。
//! 並び順だけをこのmodule本体が知り、各生成物の中身は配下のmoduleが持つ
//! (internal/codegen/mod.rs と同じ構成)。

mod edge_value;

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::input::静的グラフ型入力;

pub(crate) fn 骨組みを生成する(schema: &静的グラフ型入力) -> TokenStream {
    let 辺値struct達 = edge_value::辺値struct達を生成する(schema);
    quote! {
        #辺値struct達
    }
}
