//! `graph_schema!` の意味検査 (パース済み構文木に対する検証)。
//!
//! ここで弾く必要があるのは:
//! - ノード型名の重複宣言
//! - エッジ種別名の重複宣言
//! - エッジの端点 (`from`/`to`) が未宣言のノード型を指している場合
//!
//! いずれも `syn::Error::new_spanned`/`syn::Error::new` で元トークンの span を
//! 保ったまま返す (`.claude/skills/proc-macro-dev/SKILL.md` の方針通り、
//! panic は使わない)。

use std::collections::HashMap;

use quote::ToTokens;

use crate::schema_dsl::SchemaInput;

pub fn validate(schema: &SchemaInput) -> syn::Result<()> {
    validate_unique_node_names(schema)?;
    validate_edge_endpoints(schema)?;
    validate_unique_edge_labels(schema)?;
    Ok(())
}

fn validate_unique_node_names(schema: &SchemaInput) -> syn::Result<()> {
    let mut seen: HashMap<String, proc_macro2::Span> = HashMap::new();
    for node in &schema.nodes {
        let name = node.name.to_string();
        if let Some(&prev_span) = seen.get(&name) {
            let mut err = syn::Error::new(
                node.name.span(),
                format!("ノード型 `{name}` が重複して宣言されています"),
            );
            err.combine(syn::Error::new(prev_span, "最初の宣言はこちら"));
            return Err(err);
        }
        seen.insert(name, node.name.span());
    }
    Ok(())
}

fn validate_unique_edge_labels(schema: &SchemaInput) -> syn::Result<()> {
    let mut seen: HashMap<String, proc_macro2::Span> = HashMap::new();
    for edge in &schema.edges {
        let name = edge.label.to_string();
        if let Some(&prev_span) = seen.get(&name) {
            let mut err = syn::Error::new(
                edge.label.span(),
                format!("エッジ種別 `{name}` が重複して宣言されています"),
            );
            err.combine(syn::Error::new(prev_span, "最初の宣言はこちら"));
            return Err(err);
        }
        seen.insert(name, edge.label.span());
    }
    Ok(())
}

fn validate_edge_endpoints(schema: &SchemaInput) -> syn::Result<()> {
    let declared: Vec<String> = schema.nodes.iter().map(|n| n.name.to_string()).collect();
    let declared_set: std::collections::HashSet<&str> =
        declared.iter().map(|s| s.as_str()).collect();

    for edge in &schema.edges {
        for endpoint in [&edge.from, &edge.to] {
            if !declared_set.contains(endpoint.to_string().as_str()) {
                return Err(syn::Error::new_spanned(
                    endpoint.to_token_stream(),
                    format!(
                        "エッジ `{}` の端点 `{}` は宣言されていないノード型です。宣言済みノード一覧: [{}]",
                        edge.label,
                        endpoint,
                        declared.join(", ")
                    ),
                ));
            }
        }
    }
    Ok(())
}
