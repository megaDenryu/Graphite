//! 各サブコマンドの表示整形。`analysis` / `reorg` が返す構造化データを
//! 受け取り、人間が読みやすいテキストレポートを標準出力へ書く。
//!
//! サブコマンドごとにサブモジュールを分け、このファイルは公開面をまとめる。

mod anomaly;
mod chain;
mod reorg;
mod summary;

pub use anomaly::print_anomalies;
pub use chain::{print_chain, print_unknown_employee};
pub use reorg::print_reorg;
pub use summary::print_summary;
