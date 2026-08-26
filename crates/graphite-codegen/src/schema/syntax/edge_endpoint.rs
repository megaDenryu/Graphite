//! 辺の端点 (`型名` または `(役割名: 型名)`) を読む。

use syn::parse::ParseStream;
use syn::{parenthesized, Ident, Token};

use super::token_drain::drain_rest;

pub struct DirectedEndpoint {
    pub role: Ident,
    pub ty: Ident,
}

/// 端点1つ分 (`Ident` または `(役割名: 型名)`)。
pub(super) struct Endpoint {
    pub(super) role: Option<Ident>,
    pub(super) ty: Ident,
}

/// 端点をパースする。`(` で始まれば役割名つき `(役割名: 型名)`、そうでなければ
/// 型名のみの `Ident`。
pub(super) fn parse_endpoint(input: ParseStream) -> syn::Result<Endpoint> {
    if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);
        match parse_endpoint_paren_body(&content) {
            Ok(v) => Ok(v),
            Err(e) => {
                // G4a: drain_rest のコメント参照。
                drain_rest(&content);
                Err(e)
            }
        }
    } else {
        let ty: Ident = input.parse()?;
        if input.peek(Token![:]) {
            return Err(syn::Error::new(
                ty.span(),
                "役割付き端点は括弧で囲み `(役割名: 型名)` と書いてください",
            ));
        }
        Ok(Endpoint { role: None, ty })
    }
}

/// `(役割名: 型名)` の `( .. )` の中身。
pub(super) fn parse_endpoint_paren_body(content: ParseStream) -> syn::Result<Endpoint> {
    let role: Ident = content.parse()?;
    content.parse::<Token![:]>()?;
    let ty: Ident = content.parse()?;
    if !content.is_empty() {
        return Err(content.error("端点は `(役割名: 型名)` の形式で指定してください"));
    }
    Ok(Endpoint {
        role: Some(role),
        ty,
    })
}
