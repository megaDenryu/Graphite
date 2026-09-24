//! schemaだけから決まる生成物のうち、その場展開に残す「node宣言の型
//! アンカー」を組み立てる。種別ごとの辺値 `pub struct {種別}Edge<'a>` は
//! 生成ファイル (`file::schema_file`) だけに存在し、instanceからは
//! schema module越しの修飾パス (`naming::reference_paths::辺値参照パス`)
//! で参照する (issue #41 段階3。以前はここでも非公開の同名structを並べて
//! いたが、公開生成物を生成ファイルへ移したので二重定義をやめた)。
//! `static_graph_schema!` の展開は型アンカーを macro_rules! と並ぶ実
//! アイテムとして出力し、schema トークンが macro_rules! 本体に焼き込まれた
//! 不活性なトークン列でなく rust-analyzer が解釈できる実際のRustアイテムに
//! なるようにする。

mod type_anchor;

use proc_macro2::TokenStream;

use crate::static_graph::schema::input::静的グラフ型入力;

pub(crate) fn 骨組みを生成する(schema: &静的グラフ型入力) -> TokenStream {
    type_anchor::型アンカーを生成する(schema)
}
