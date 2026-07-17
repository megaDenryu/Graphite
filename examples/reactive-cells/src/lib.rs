//! reactive-cells — Graphite (`graph_schema!`/`graph!`) で
//! 「リアクティブプログラミングのスパゲッティ」を倒す実証example。
//!
//! 詳しい経緯・設計判断は `README.md` を参照。`main.rs` (CLIエントリ
//! ポイント) と `tests/` (統合テスト) の両方から同じロジックを参照できる
//! よう、lib + bin の2ターゲット構成にしている
//! (`examples/org-analyzer` と同じ構成)。
//!
//! - [`antipattern`] — 敵: observer パターン (コールバック購読) のナイーブな
//!   実装。グリッチ・無限ループ・登録順依存の非決定性を実際に再現する。
//! - [`schema`] — セル (`Cell`) ノード1種 + 依存エッジ3種
//!   (`Feeds`/`Lhs`/`Rhs`) の `graph_schema!` 宣言。
//! - [`fixtures`] — `graph!` リテラルで組み立てる具体的な依存グラフ
//!   (ミニスプレッドシート・循環デモ用)。
//! - [`engine`] — 依存グラフを `topological_sort`/`reachable_from` に
//!   射影して使う再計算エンジン。
//! - [`report`] — `main.rs` 向けの読み物風出力ヘルパー。

pub mod antipattern;
pub mod engine;
pub mod fixtures;
pub mod report;
pub mod schema;
