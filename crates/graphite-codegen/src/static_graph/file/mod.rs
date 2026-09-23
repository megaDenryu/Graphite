//! 静的グラフの生成ファイル本文 (issue #41 §2)。schemaファイル
//! (`schema_file`) とinstanceファイル (`instance_file`) の2つを持つ。
//! `static_graph::naming` から受け取った追跡付きの名前を `pub` + 意味カード
//! で並べる。指紋定数の埋め込み・ファイル先頭の案内コメントは
//! `crate::generated_source` (動的グラフと共有) が担うため、ここでは
//! 本体のTokenStreamだけを組み立てる。

mod doc_render;
mod instance_file;
mod schema_file;

pub(crate) use instance_file::instance本体を組み立てる;
pub(crate) use schema_file::schema本体を組み立てる;
