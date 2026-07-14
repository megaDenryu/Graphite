//! ビルドパイプライン・オーケストレータのライブラリ部分。
//!
//! CLI 本体 (`main.rs`) と `tests/` 配下の統合テストの両方から使えるように、
//! パース・グラフ構築・分析・出力整形のロジックはすべてここに集約する
//! (バイナリ専用クレートだと `tests/` から `mod` へアクセスできないため)。

pub mod analysis;
pub mod builder;
pub mod parser;
pub mod report;
pub mod schema;
