//! 辺の柄 (`->` / `-[役割名: 型]->` / `--` / `-[役割名: 型]-`) を読む。

use syn::parse::ParseStream;
use syn::{bracketed, Token};

use super::edge_payload::{parse_edge_bracket_body, EdgePayload};
use super::token_drain::drain_rest;

/// 柄 (4形: `->` / `-[役割名: Attrs]->` / `--` / `-[役割名: Attrs]-`) をパースし、
/// `(積み荷型, 有向か)` を返す。
///
/// 有向の柄 `-` + `>` から矢尻を落とすと無向の柄になる、という
/// `docs/edge_endpoints_v4_1.md` §2 の導出規則どおりに実装する: 最初の `-`
/// を読んだ後、`[Attrs]` (積み荷、あれば) を読み、最後に `->` (有向) か `-`
/// (無向) かで向きを判定する。
pub(super) fn parse_edge_arrow(input: ParseStream) -> syn::Result<(Option<EdgePayload>, bool)> {
    // 素の `->` (単一の複合トークン) を先読みして判定する。`-[`/`--` は
    // いずれも `-` と別トークンの2トークンなので `->` と誤って先読み
    // マッチすることはない。
    if input.peek(Token![->]) {
        input.parse::<Token![->]>()?;
        return Ok((None, true));
    }
    input.parse::<Token![-]>()?;
    if input.peek(syn::token::Bracket) {
        let bracket_content;
        bracketed!(bracket_content in input);
        let attrs = match parse_edge_bracket_body(&bracket_content) {
            Ok(v) => v,
            Err(e) => {
                // G4a: drain_rest のコメント参照。
                drain_rest(&bracket_content);
                return Err(e);
            }
        };
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            Ok((Some(attrs), true))
        } else {
            input.parse::<Token![-]>()?;
            Ok((Some(attrs), false))
        }
    } else {
        // 積み荷なし。ここまでで最初の `-` を消費済みなので、残りの `-`
        // (無向の柄 `--` の2文字目) を読む。
        input.parse::<Token![-]>()?;
        Ok((None, false))
    }
}
