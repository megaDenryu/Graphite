//! dialogue-engine のライブラリ部分。
//!
//! `src/main.rs` (CLI) と `tests/` (統合テスト) の両方から同じロジックを
//! 使えるように、モジュールをここに集約して公開する
//! (Cargo は `src/lib.rs` + `src/main.rs` が両方あるパッケージで、バイナリが
//! 暗黙にライブラリへ依存する構成を標準サポートしている)。

pub mod engine;
pub mod report;
pub mod schema;
pub mod validate;
