//! async-dag のライブラリ部分。
//!
//! CLI 本体 (`main.rs`) と `tests/` 配下の統合テストの両方から使えるように、
//! スキーマ・グラフ射影・実行エンジンのロジックはすべてここに集約する
//! (バイナリ専用クレートだと `tests/` から `mod` へアクセスできないため)。

pub mod depgraph;
pub mod engine;
pub mod fixtures;
pub mod schema;
