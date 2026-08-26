//! DSL の宣言を Graphite の意味論として確定した意味モデルを収める。
//!
//! ## この層が持ってよいもの・持ってはならないもの
//!
//! 意味モデルは Graphite 自身の語彙 (ノード定義・辺定義・向き・役割・多重度・
//! 公開ID型) だけで表す。`quote::` を use せず、`TokenStream` をフィールドに
//! 持たず、意味モデル型へ `ToTokens` を実装しない。生成コードの形をこの層が
//! 決めると、コード生成層が意味を判断する場所へ戻ってしまうためである。
//!
//! `proc_macro2::Ident` と `syn::Path` の保持は許す。これらは「利用者が書いた
//! 名前」そのものであり、span を保つことが IDE 支援 (`docs/ide_support_spec.md`)
//! に必要だからである。
//!
//! 生成名 (内部ストレージのフィールド名・索引名・違反 variant 名・アクセサ名)
//! は意味ではなく生成物の都合なので、この層は持たない。コード生成層が
//! [`crate::naming`] から導出する。ただし要素ごとの既定ID型名は例外で、
//! 公開APIの一部であり意味の一部として、この層が [`crate::naming`] から
//! 導出する (schema固定名は生成物の都合なので、生成層が予約表経由で導出する)。
//!
//! ## 自己参照を持たない理由
//!
//! 辺定義は端点のノード定義を参照するが、参照を直接持たず
//! [`ノード定義番号`] という添字ハンドルで持つ。所有権の循環を作らず、
//! スキーマ定義全体を1つの値として持ち回せるようにするためである。

mod analyze;
mod cardinality;
mod edge_definition;
mod endpoint_pairing;
mod node_definition;
mod public_id_type;
mod schema_definition;
mod traversal_plan;
mod violation_catalog;

// 再公開するのは、この層の外 (`schema::codegen` / `schema::validate` / `lib.rs`)
// が名前で参照するものだけに限る。層の内側だけで使う型は各ファイルに置いたまま
// にする。
pub use analyze::検証済み構文からスキーマ定義を組み立てる;
pub use cardinality::{each制約が指す端点の側を判定する, EachSide, RoleCardinality};
pub use edge_definition::{積み荷, 辺の向き, 辺定義};
pub use endpoint_pairing::{端点対のキーの形, 端点対の重複可否};
pub use node_definition::ノード定義;
pub use public_id_type::公開ID型;
pub use schema_definition::スキーマ定義;
pub use traversal_plan::{ノードの探索計画, 探索操作};
pub use violation_catalog::違反定義;
