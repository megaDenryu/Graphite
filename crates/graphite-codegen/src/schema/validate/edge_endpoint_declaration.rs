//! 辺の端点が宣言済みのノード型を指していることを検査する。

use std::collections::HashSet;

use quote::ToTokens;

use crate::schema::syntax::{EdgeDecl, EdgeShape, NodeDecl};

pub fn validate_edge_endpoints(nodes: &[NodeDecl], edges: &[EdgeDecl]) -> syn::Result<()> {
    let declared: Vec<String> = nodes.iter().map(|n| n.name.to_string()).collect();
    let declared_set: HashSet<&str> = declared.iter().map(|s| s.as_str()).collect();

    for edge in edges {
        let endpoints = match &edge.shape {
            EdgeShape::Directed { from, to, .. } => [&from.ty, &to.ty],
            EdgeShape::Undirected { first, second, .. } => [first, second],
        };
        for endpoint in endpoints {
            if !declared_set.contains(endpoint.to_string().as_str()) {
                return Err(syn::Error::new_spanned(
                    endpoint.to_token_stream(),
                    format!(
                        "エッジ `{}` の端点 `{}` は宣言されていないノード型です。宣言済みノード一覧: [{}]",
                        edge.kind,
                        endpoint,
                        declared.join(", ")
                    ),
                ));
            }
        }
    }
    Ok(())
}

// G4a (二次エラーの抑制): パース回復により1件以上の壊れた宣言があった
// ときに、`lib.rs` が `validate_edge_endpoints` の代わりに呼ぶ。
// 端点が未宣言のノード型を指す辺をエラーにはせず、生成対象から
// 除外する。壊れたノード宣言をたまたま参照しているだけの可能性が高く、
// そのまま `validate_edge_endpoints` を呼ぶと「壊れた宣言由来の
// compile_error!」1件のはずが「未知端点エラー」まで重ねて出てしまう
// (二次噴出) ため。
pub fn filter_edges_with_known_endpoints(
    nodes: &[NodeDecl],
    edges: Vec<EdgeDecl>,
) -> Vec<EdgeDecl> {
    let declared: HashSet<String> = nodes.iter().map(|n| n.name.to_string()).collect();
    edges
        .into_iter()
        .filter(|edge| {
            let endpoints = match &edge.shape {
                EdgeShape::Directed { from, to, .. } => [&from.ty, &to.ty],
                EdgeShape::Undirected { first, second, .. } => [first, second],
            };
            endpoints
                .iter()
                .all(|endpoint| declared.contains(&endpoint.to_string()))
        })
        .collect()
}
