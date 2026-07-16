//! state-machine のロジック本体をライブラリとして公開する。
//!
//! `main.rs` (CLI エントリポイント) と `tests/` (統合テスト) の両方から
//! 同じロジックを参照できるよう、lib + bin の2ターゲット構成にしている
//! (bin単体では `tests/` から内部モジュールへアクセスできないため。
//! `examples/org-analyzer` と同じ構成)。

pub mod fsm;
pub mod schema;
pub mod validate;
