//! 検証の実行順序と二次エラーの抑制を配線し、個々の検査を配下の module へ委ねる。
//!
//! 注意: 検査を呼ぶ順序は診断の出る順序そのものであり、コンパイル失敗テスト
//! (`crates/graphite/tests/ui/*.stderr`) が依存している。並べ替えると
//! それらが壊れる。`validate_generated_type_names` をノード名と辺種別名が
//! 両方一意のときだけ呼ぶ条件も同じ理由で維持する。
//!
//! ここで弾く必要があるのは:
//! - ノード型名の重複宣言
//! - エッジ種別名 (Kind) の重複宣言
//! - エッジの端点 (`from`/`to`) が未宣言のノード型を指している場合
//! - `where each <役割名>: ..` が有向辺の始点/終点の役割名と一致するか
//! - 無向辺の両端が同じノード型であること (`docs/edge_endpoints_v4_1.md` §2)
//!
//! いずれも `syn::Error::new_spanned`/`syn::Error::new` で元トークンの span を
//! 保ったまま返す (`.claude/skills/proc-macro-dev/SKILL.md` の方針通り、
//! panic は使わない)。
//!
//! ## G4a (宣言単位のエラー回復) との関係
//!
//! `SchemaInput` 全体ではなく `&[NodeDecl]`/`&[EdgeDecl]` というスライスを
//! 受け取るシグネチャにしているのは、この module の配線がパース回復で「壊れた
//! 宣言を除いた残り」だけを検証にかけられるようにするため。特に
//! `validate_edge_endpoints`/`validate_each_reference` は、パース済みの
//! 宣言が1件でも壊れていた場合に直接は呼ばず、代わりに
//! [`filter_edges_with_known_endpoints`] で未知端点の辺を生成対象から除外する
//! (二次エラー抑制)。重複ノード名・重複エッジ種別名の診断は回復の有無に
//! よらず常に実行する (現行維持)。

mod each_reference;
mod edge_endpoint_declaration;
mod edge_role_name;
mod generated_name_collision;
mod undirected_endpoint_type;
mod unique_declaration_names;

use crate::schema::syntax::{SchemaInput, SchemaParse};
use each_reference::validate_each_reference;
use edge_endpoint_declaration::{filter_edges_with_known_endpoints, validate_edge_endpoints};
use edge_role_name::validate_edge_roles;
use generated_name_collision::validate_generated_type_names;
use undirected_endpoint_type::validate_undirected_same_type;
use unique_declaration_names::{validate_unique_edge_kinds, validate_unique_node_names};

pub(crate) fn validate(parsed: SchemaParse) -> Result<SchemaInput, Vec<syn::Error>> {
    match validate_recovering(parsed) {
        ValidationResult::Generated { schema, errors } if errors.is_empty() => Ok(schema),
        ValidationResult::Generated { errors, .. } | ValidationResult::Rejected(errors) => {
            Err(errors)
        }
    }
}

pub(crate) enum ValidationResult {
    Generated {
        schema: SchemaInput,
        errors: Vec<syn::Error>,
    },
    Rejected(Vec<syn::Error>),
}

pub(crate) fn validate_recovering(parsed: SchemaParse) -> ValidationResult {
    let SchemaParse {
        schema,
        errors: parse_errors,
    } = parsed;
    let has_parse_errors = !parse_errors.is_empty();
    let edges = if has_parse_errors {
        filter_edges_with_known_endpoints(&schema.nodes, schema.edges)
    } else {
        schema.edges
    };

    let mut validate_errors = Vec::new();
    let node_names_are_unique = collect_validation(
        validate_unique_node_names(&schema.nodes),
        &mut validate_errors,
    );
    if !has_parse_errors {
        collect_validation(
            validate_edge_endpoints(&schema.nodes, &edges),
            &mut validate_errors,
        );
    }
    let edge_names_are_unique =
        collect_validation(validate_unique_edge_kinds(&edges), &mut validate_errors);
    if node_names_are_unique && edge_names_are_unique {
        collect_validation(
            validate_generated_type_names(&schema.schema_name, &schema.nodes, &edges),
            &mut validate_errors,
        );
    }
    collect_validation(validate_undirected_same_type(&edges), &mut validate_errors);
    collect_validation(validate_edge_roles(&edges), &mut validate_errors);
    collect_validation(validate_each_reference(&edges), &mut validate_errors);

    if !validate_errors.is_empty() {
        let mut errors = parse_errors;
        errors.extend(validate_errors);
        return ValidationResult::Rejected(errors);
    }
    ValidationResult::Generated {
        schema: SchemaInput {
            schema_name: schema.schema_name,
            nodes: schema.nodes,
            edges,
        },
        errors: parse_errors,
    }
}

fn collect_validation(result: syn::Result<()>, errors: &mut Vec<syn::Error>) -> bool {
    match result {
        Ok(()) => true,
        Err(error) => {
            errors.push(error);
            false
        }
    }
}
