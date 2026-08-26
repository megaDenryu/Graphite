//! ノード・辺に共通する明示ID型指定 `(id: 型パス)` を読む。

use syn::parse::ParseStream;
use syn::{parenthesized, Path, Token};

use super::keywords as kw;
use super::token_drain::drain_rest;

/// Node/Edge 共通の明示 ID 型指定 `(id: 型パス)` を読む。
pub(super) fn parse_optional_id_type(input: ParseStream) -> syn::Result<Option<Path>> {
    if !input.peek(syn::token::Paren) {
        return Ok(None);
    }

    let content;
    parenthesized!(content in input);
    let parsed = (|| {
        content.parse::<kw::id>()?;
        content.parse::<Token![:]>()?;
        let id_ty: Path = content.parse()?;
        if !content.is_empty() {
            return Err(content.error("ID 型は `(id: 型パス)` の形式で指定してください"));
        }
        Ok(id_ty)
    })();
    match parsed {
        Ok(id_ty) => Ok(Some(id_ty)),
        Err(error) => {
            drain_rest(&content);
            Err(error)
        }
    }
}
