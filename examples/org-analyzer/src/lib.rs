//! org-analyzer のロジック本体をライブラリとして公開する。
//!
//! `main.rs` (CLIエントリポイント) と `tests/` (統合テスト) の両方から
//! 同じロジックを参照できるよう、lib + bin の2ターゲット構成にしている
//! (bin単体では `tests/` から内部モジュールへアクセスできないため)。

pub mod analysis;
pub mod dataset;
pub mod reorg;
pub mod report;
pub mod schema;
