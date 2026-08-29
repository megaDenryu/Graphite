// edge宣言そのものの構文 (端点の役割・型、積み荷) のパース。where節の構文は
// input_constraint.rs が担う。

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{bracketed, parenthesized, Token};

use super::edge;
use super::{辺形状, 辺宣言};

impl Parse for 辺宣言 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<edge>()?;
        let 名前 = input.parse()?;
        input.parse::<Token![=]>()?;
        let 形状 = 辺形状を読む(input)?;
        let 制約達 = super::input_constraint::制約節を読む(input)?;
        input.parse::<Token![;]>()?;
        Ok(辺宣言 { 名前, 形状, 制約達 })
    }
}

fn 辺形状を読む(input: ParseStream) -> syn::Result<辺形状> {
    if !input.peek(syn::token::Paren) {
        // 無向辺: <型> -- <型>
        let 型1: Ident = input.parse()?;
        input.parse::<Token![-]>()?;
        input.parse::<Token![-]>()?;
        let 型2: Ident = input.parse()?;
        if 型1 != 型2 {
            return Err(syn::Error::new_spanned(&型2, "無向辺の両端は同じ型でなければなりません"));
        }
        return Ok(辺形状::無向 { 型: 型1 });
    }

    let 始点丸括弧;
    parenthesized!(始点丸括弧 in input);
    let 始点役割: Ident = 始点丸括弧.parse()?;
    始点丸括弧.parse::<Token![:]>()?;
    let 始点型: Ident = 始点丸括弧.parse()?;

    let 積み荷 = if input.peek(Token![-]) && input.peek2(syn::token::Bracket) {
        input.parse::<Token![-]>()?;
        let 積み荷角括弧;
        bracketed!(積み荷角括弧 in input);
        let 積み荷役割: Ident = 積み荷角括弧.parse()?;
        積み荷角括弧.parse::<Token![:]>()?;
        let 積み荷型: Ident = 積み荷角括弧.parse()?;
        input.parse::<Token![->]>()?;
        Some((積み荷役割, 積み荷型))
    } else {
        input.parse::<Token![->]>()?;
        None
    };

    let 終点丸括弧;
    parenthesized!(終点丸括弧 in input);
    let 終点役割: Ident = 終点丸括弧.parse()?;
    終点丸括弧.parse::<Token![:]>()?;
    let 終点型: Ident = 終点丸括弧.parse()?;

    Ok(辺形状::有向 { 始点役割, 始点型, 積み荷, 終点役割, 終点型 })
}
