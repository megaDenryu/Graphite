//! `graph_schema!`/`graph!` の生成コードが依存する実行時契約を、意味ごとのモジュールへ配線する。
//!
//! ここに集めた型・トレイト・関数は利用者が直接書くものではなく、生成コードだけが
//! 名指しする内部契約である。生成コードは `::graphite::MultipleRoleIndex` のような
//! クレート直下の綴りを出力するため、クレートルート (`crate::lib`) 側で再公開する
//! 経路を必ず維持する。

mod construction_stamp;
mod edge_literal;
mod graph_mismatch;
mod named_construction;
mod named_element;
mod role_index;

pub use construction_stamp::次の構築印を発行する;
pub use edge_literal::{DirectedEdgeLiteral, UndirectedEdgeLiteral};
pub use graph_mismatch::GraphMismatch;
pub use named_construction::{build_named_graph, FreezableBuilder, NamedInsertPermit};
pub use named_element::NamedGraphElement;
pub use role_index::{ExactlyOneRoleIndex, MultipleRoleIndex, OptionalRoleIndex};
