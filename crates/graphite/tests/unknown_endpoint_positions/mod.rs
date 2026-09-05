//! このモジュールは、未知の端点キーと端点対の重複について、位置ごとの診断文の
//! 試験を集める。
//!
//! 4つの位置は同じ schema (`tests/unknown_endpoint_diagnostics.rs` が宣言) を
//! 共有し、位置ごとにID型の組み合わせを試験する。ファイル名を ASCII にするのは、
//! Rust が非ASCIIのmodule名からファイルを探せないためである。

mod directed_source;
mod directed_target;
mod undirected_endpoint;
mod unique_pair;
