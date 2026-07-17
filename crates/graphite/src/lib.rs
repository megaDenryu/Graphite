//! Graphite — グラフ指向データ構造のランタイムライブラリ。
//!
//! このクレートは利用者が唯一 depend するクレートであり、
//! `graphite-macros` (proc-macro クレート) の内容を re-export する
//! (serde/serde_derive と同じ 2 クレート構成)。
//!
//! 水準1相当のジェネリックグラフ [`Graph`] (フェーズ2) に加え、フェーズ3で
//! 水準2相当の図式グラフスキーマを宣言する `graph_schema!` と、インスタンス
//! リテラル `graph!` を実装した (`graphite-macros` から re-export)。
//! `flow!` (`docs/flow_macro.md`) はこれらとは独立した別レイヤで、データの辺
//! (宣言) とは対照的な「関数の辺」(即時実行) を文位置マクロとして提供する。
//! 設計の一次資料:
//! - `../../../Bullet/docs/rust_graph_extension_sketch.md`
//! - `../../../Bullet/docs/graph_design_sketches.md`

mod graph;
mod keyed_table;

pub use graph::{CycleError, Graph, GraphBuilder, GraphError};
pub use keyed_table::KeyedTable;

pub use graphite_macros::{flow, graph, graph_schema};
