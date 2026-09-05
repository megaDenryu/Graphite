//! 役割名が生成される辺APIの固定名と衝突しないことを検査する。

use syn::Ident;

use crate::schema::syntax::{EdgeDecl, EdgeShape};

// 役割名と既存の辺APIが同じ名前空間で衝突しないことを検査する。
pub fn validate_edge_roles(edges: &[EdgeDecl]) -> syn::Result<()> {
    const RESERVED: &[&str] = &[
        "from",
        "to",
        "from_id",
        "to_id",
        "endpoints",
        "id",
        "record",
        "payload",
    ];
    for edge in edges {
        let roles: Vec<&Ident> = match &edge.shape {
            EdgeShape::Directed { from, to, payload } => [&from.role, &to.role]
                .into_iter()
                .chain(payload.iter().map(|value| &value.role))
                .collect(),
            EdgeShape::Undirected { payload, .. } => {
                payload.iter().map(|value| &value.role).collect()
            }
        };
        for role in roles {
            if RESERVED.contains(&role.to_string().as_str()) {
                return Err(syn::Error::new(
                    role.span(),
                    format!("役割名 `{role}` は生成される辺APIと衝突します。別の役割名を指定してください"),
                ));
            }
        }
    }
    Ok(())
}
