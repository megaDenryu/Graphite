//! 無向辺の両端が同じノード型であることを検査する。

use quote::ToTokens;

use crate::schema::syntax::{EdgeDecl, EdgeShape};

/// 無向辺の両端が同じノード型であることを検査する
/// (`docs/edge_endpoints_v4_1.md` §2「両端は同じノード型でなければならない」)。
pub fn validate_undirected_same_type(edges: &[EdgeDecl]) -> syn::Result<()> {
    for edge in edges {
        let EdgeShape::Undirected { first, second, .. } = &edge.shape else {
            continue;
        };
        if first != second {
            let mut err = syn::Error::new_spanned(
                second.to_token_stream(),
                format!(
                    "無向辺 `{}` の両端は同じノード型でなければなりません (`{}` != `{}`)。異なる型を対称に繋ぎたい場合は有向辺として書くか、ノードを昇格してください",
                    edge.kind, first, second
                ),
            );
            err.combine(syn::Error::new_spanned(
                first.to_token_stream(),
                "始点側の型はこちら",
            ));
            return Err(err);
        }
    }
    Ok(())
}
