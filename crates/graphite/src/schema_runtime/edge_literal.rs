//! `graph!` が書かれた辺の柄から辺値を構築するための契約を、有向・無向の2つぶん所有する。

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
