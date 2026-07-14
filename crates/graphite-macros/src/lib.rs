//! graphite-macros — Graphite の proc-macro クレート。
//!
//! proc-macro クレートはランタイム型を直接持てない (手続き型マクロは
//! コンパイラプラグインの一種で、生成する側と生成されたコードが依存する側の
//! 型を同じクレートに置けない) ため、ランタイムクレート `graphite` とは
//! 分離されている。利用者はこのクレートに直接依存せず、`graphite` 経由で
//! re-export されたマクロを使う。
//!
//! 現時点では足場のみで、`graph_schema!` / `graph!` はまだ実装していない。
//! 設計の一次資料:
//! - `../../../Bullet/docs/rust_graph_extension_sketch.md`
//! - `../../../Bullet/docs/graph_design_sketches.md`

// 今後ここに graph_schema! / graph! などの proc-macro を実装する。
