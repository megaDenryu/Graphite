//! Graphite は、グラフ指向データ構造を Rust の型システムに乗せるランタイムライブラリである。
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
//!
//! このファイルは公開facadeであり、モジュールの配線と再公開だけを持つ。実行時の
//! 4概念 (汎用Graph・計算グラフ・schema生成コード向けの実行時契約・キー付き要素表)
//! の内訳と依存の向きは `docs/development/runtime_structure.md` を参照。

mod compute;
mod graph;
mod keyed_table;
mod schema_runtime;
mod unordered_pair;

pub use compute::{ComputeGraph, ComputeGraphBuilder, ComputeGraphError};
pub use graph::{CycleError, Graph, GraphBuilder, GraphError};
pub use keyed_table::KeyedTable;
pub use unordered_pair::UnorderedPair;

pub use schema_runtime::GraphMismatch;

#[doc(hidden)]
pub use schema_runtime::{
    build_named_graph, DirectedEdgeLiteral, ExactlyOneRoleIndex, FreezableBuilder,
    MultipleRoleIndex, NamedGraphElement, NamedInsertPermit, OptionalRoleIndex,
    UndirectedEdgeLiteral, 次の構築印を発行する,
};

#[doc(hidden)]
pub use graphite_macros::__graph_schema_inline_for_test;
pub use graphite_macros::{flow, graph, graph_schema};
