//! 生成名の導出規則を1箇所へ集約し、検査側と生成側が同じ規則を読むようにする。
//!
//! `graph_schema!` と `graph!` の両方が同じ変換規則に従う必要がある
//! (`graph!` はスキーマの中身を知らずにビルダーメソッド名・属性型名を
//! 機械的に導出するため)。この対応がずれると `graph!` が生成する呼び出しが
//! `graph_schema!` の生成物と噛み合わずコンパイルエラーになる。
//!
//! 生成名を導出する関数はすべてこの module 配下にあり、`Ident` を組み立てる
//! `format_ident!` を他の module へ直書きしない。

mod case_conversion;
mod element_names;
mod instance_names;
mod method_names;
mod reserved_generated_names;
mod schema_fixed_names;
mod storage_names;
mod violation_variant_names;

pub use case_conversion::to_snake_case;
pub use element_names::{
    edge_record_ident, generated_id_ident, internal_position_ident, named_position_ident,
    reference_ident,
};
pub use instance_names::{
    named_binding_position_ident, named_graph_wrapper_ident, named_wrapper_parameter_ident,
};
pub use method_names::{incident_method_ident, kind_api_method_ident, traversal_method_ident};
pub use reserved_generated_names::固定生成名の予約表;
pub use schema_fixed_names::{construction_stamp_field_ident, graph_type_ident};
pub use storage_names::{
    accessor_ident, edge_storage_ident, incident_index_field_ident, node_storage_ident,
    pair_index_field_ident, source_role_index_field_ident, target_role_index_field_ident,
};
pub use violation_variant_names::{
    duplicate_edge_key_variant_ident, duplicate_node_key_variant_ident, each_violation_ident,
    unique_pair_violation_variant_ident, unknown_endpoint_variant_ident,
    unknown_source_variant_ident, unknown_target_variant_ident,
};
