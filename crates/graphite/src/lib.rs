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

mod compute;
mod graph;
mod keyed_table;
mod unordered_pair;

pub use compute::{ComputeGraph, ComputeGraphBuilder, ComputeGraphError};
pub use graph::{CycleError, Graph, GraphBuilder, GraphError};
pub use keyed_table::KeyedTable;
pub use unordered_pair::UnorderedPair;

/// `graph!` が有向の柄から辺値を構築するための内部契約。
#[doc(hidden)]
pub trait DirectedEdgeLiteral<From, To, Payload>: Sized {
    fn from_graph_literal(from: From, to: To, payload: Payload) -> Self;
}

/// `graph!` が無向の柄から辺値を構築するための内部契約。
#[doc(hidden)]
pub trait UndirectedEdgeLiteral<Endpoint, Payload>: Sized {
    fn from_graph_literal(first: Endpoint, second: Endpoint, payload: Payload) -> Self;
}

/// `graph!` が名前付き要素の内部位置から Graph-bound Ref を直接構築するための
/// 内部契約。公開 ID の索引は経由しない。
#[doc(hidden)]
pub trait NamedGraphElement<G> {
    type Reference<'graph>
    where
        G: 'graph;

    fn bind<'graph>(&self, graph: &'graph G) -> Self::Reference<'graph>;
}

pub use graphite_macros::{flow, graph, graph_schema};
