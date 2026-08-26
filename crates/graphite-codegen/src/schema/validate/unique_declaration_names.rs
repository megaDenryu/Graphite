//! ノード型名と辺種別名がそれぞれ一意に宣言されていることを検査する。

use std::collections::HashMap;

use crate::schema::syntax::{EdgeDecl, NodeDecl};

pub fn validate_unique_node_names(nodes: &[NodeDecl]) -> syn::Result<()> {
    let mut seen: HashMap<String, proc_macro2::Span> = HashMap::new();
    for node in nodes {
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

pub fn validate_unique_edge_kinds(edges: &[EdgeDecl]) -> syn::Result<()> {
    let mut seen: HashMap<String, proc_macro2::Span> = HashMap::new();
    for edge in edges {
        let name = edge.kind.to_string();
        if let Some(&prev_span) = seen.get(&name) {
            let mut err = syn::Error::new(
                edge.kind.span(),
                format!("エッジ種別 `{name}` が重複して宣言されています"),
            );
            err.combine(syn::Error::new(prev_span, "最初の宣言はこちら"));
            return Err(err);
        }
        seen.insert(name, edge.kind.span());
    }
    Ok(())
}
