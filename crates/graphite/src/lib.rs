//! Graphite — グラフ指向データ構造のランタイムライブラリ。
//!
//! このクレートは利用者が唯一 depend するクレートであり、
//! `graphite-macros` (proc-macro クレート) の内容を re-export する
//! (serde/serde_derive と同じ 2 クレート構成)。
//!
//! 現時点では足場のみで、グラフ型・アルゴリズムの実装はまだ無い。
//! 設計の一次資料:
//! - `../../../Bullet/docs/rust_graph_extension_sketch.md`
//! - `../../../Bullet/docs/graph_design_sketches.md`

// graphite-macros が proc-macro を持つようになったらここで re-export する。
// 例: pub use graphite_macros::{graph, graph_schema};
