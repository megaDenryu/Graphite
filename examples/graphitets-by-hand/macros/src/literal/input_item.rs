// 1件の宣言 (ノード or 辺) のパースと判別。右辺の形で判別する:
// `型 { .. }` (構造体リテラル) ならノード、`種別(..)` (呼び出し風) なら辺。

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, token, ExprStruct, Token};

use super::{ノード宣言, 辺宣言};

pub(super) enum 宣言 {
    ノード(ノード宣言),
    辺(辺宣言),
}

impl Parse for 宣言 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let 名前: Ident = input.parse()?;
        input.parse::<Token![=]>()?;

        let 先読み = input.fork();
        let _型らしきもの: Ident = 先読み.parse().map_err(|_| 形式エラーを作る(名前.span()))?;

        if 先読み.peek(token::Brace) {
            let 式: ExprStruct = input.parse()?;
            let 実体型 = 式
                .path
                .segments
                .last()
                .ok_or_else(|| 形式エラーを作る(名前.span()))?
                .ident
                .clone();
            Ok(宣言::ノード(ノード宣言 { 名前, 実体型, 式 }))
        } else if 先読み.peek(token::Paren) {
            let 種別: Ident = input.parse()?;
            let 内容;
            parenthesized!(内容 in input);
            let 形状 = super::input_edge_body::辺形状を読む(&内容)?;
            Ok(宣言::辺(辺宣言 { 名前, 種別, 形状 }))
        } else {
            Err(形式エラーを作る(名前.span()))
        }
    }
}

fn 形式エラーを作る(span: proc_macro2::Span) -> syn::Error {
    syn::Error::new(
        span,
        "静的グラフ! のノードは `名前 = 型 { .. }`、辺は `名前 = 種別(..)` の形で書いてください",
    )
}
