// node/edge 各1件の宣言のパース。行の種類は先頭の `node`/`edge` キーワードで
// 確定する (右辺の形からの推測判別はしない)。node は3形態
// (`名前 = 型 { .. };` / `名前: 型 = 式;` / `名前: 型;`) を受理する。

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Expr, Token};

use super::{edge, node, ノード宣言, 辺宣言};

impl Parse for ノード宣言 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<node>()?;
        let 名前: Ident = input.parse()?;

        if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            let 実体型: Ident = input.parse()?;
            if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
                return Ok(ノード宣言 { 名前, 実体型, 値: None });
            }
            input.parse::<Token![=]>()?;
            let 式: Expr = input.parse()?;
            input.parse::<Token![;]>()?;
            return Ok(ノード宣言 { 名前, 実体型, 値: Some(式) });
        }

        input.parse::<Token![=]>()?;
        let 式: Expr = input.parse()?;
        input.parse::<Token![;]>()?;
        let 実体型 = 構造体リテラルから実体型を読む(&式)?;
        Ok(ノード宣言 { 名前, 実体型, 値: Some(式) })
    }
}

fn 構造体リテラルから実体型を読む(式: &Expr) -> syn::Result<Ident> {
    match 式 {
        Expr::Struct(構造体式) => {
            構造体式.path.segments.last().map(|segment| segment.ident.clone()).ok_or_else(|| 形式エラーを作る(式))
        }
        _ => Err(形式エラーを作る(式)),
    }
}

fn 形式エラーを作る(式: &Expr) -> syn::Error {
    syn::Error::new_spanned(式, "`node 名前: 型 = 式;` か `node 名前 = 型 { ... };` の形で書いてください")
}

impl Parse for 辺宣言 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<edge>()?;
        let 名前: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let 種別: Ident = input.parse()?;
        let 内容;
        parenthesized!(内容 in input);
        let 形状 = super::input_edge_body::辺形状を読む(&内容)?;
        input.parse::<Token![;]>()?;
        Ok(辺宣言 { 名前, 種別, 形状 })
    }
}
