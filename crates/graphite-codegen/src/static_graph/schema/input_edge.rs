// edge宣言そのものの構文 (端点の役割・型、積み荷) のパース。where節の構文は
// input_constraint.rs が担う。無向辺も有向辺と同じく両端を `(役割: 型)` で
// 書く (役割名なしの裸形は廃止。issue #24 段階2、オーナー裁定)。

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
        return Err(input.error("辺の両端は `(役割名: 型)` の形で書いてください (無向辺も役割名が必須です)"));
    }
    let (第1役割, 第1型) = 役割付き端点を読む(input)?;

    if input.peek(Token![-]) && input.peek2(syn::token::Bracket) {
        input.parse::<Token![-]>()?;
        let 積み荷角括弧;
        bracketed!(積み荷角括弧 in input);
        let 積み荷役割: Ident = 積み荷角括弧.parse()?;
        積み荷角括弧.parse::<Token![:]>()?;
        let 積み荷型: Ident = 積み荷角括弧.parse()?;
        let 積み荷 = Some((積み荷役割, 積み荷型));

        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            let (終点役割, 終点型) = 役割付き端点を読む(input)?;
            return Ok(辺形状::有向 { 始点役割: 第1役割, 始点型: 第1型, 積み荷, 終点役割, 終点型 });
        }
        input.parse::<Token![-]>()?;
        let (第2役割, 第2型) = 役割付き端点を読む(input)?;
        return 無向を組み立てる(第1役割, 第1型, 積み荷, 第2役割, 第2型);
    }

    if input.peek(Token![->]) {
        input.parse::<Token![->]>()?;
        let (終点役割, 終点型) = 役割付き端点を読む(input)?;
        return Ok(辺形状::有向 { 始点役割: 第1役割, 始点型: 第1型, 積み荷: None, 終点役割, 終点型 });
    }
    if input.peek(Token![-]) {
        input.parse::<Token![-]>()?;
        input.parse::<Token![-]>()?;
        let (第2役割, 第2型) = 役割付き端点を読む(input)?;
        return 無向を組み立てる(第1役割, 第1型, None, 第2役割, 第2型);
    }
    Err(input.error(
        "辺は `(役割: 型) -> (役割: 型)` / `(役割: 型) -[役割: 型]-> (役割: 型)` / `(役割: 型) -- (役割: 型)` / `(役割: 型) -[役割: 型]- (役割: 型)` の形で書いてください",
    ))
}

fn 役割付き端点を読む(input: ParseStream) -> syn::Result<(Ident, Ident)> {
    let 丸括弧;
    parenthesized!(丸括弧 in input);
    let 役割: Ident = 丸括弧.parse()?;
    丸括弧.parse::<Token![:]>()?;
    let 型: Ident = 丸括弧.parse()?;
    Ok((役割, 型))
}

fn 無向を組み立てる(
    第1役割: Ident,
    第1型: Ident,
    積み荷: Option<(Ident, Ident)>,
    第2役割: Ident,
    第2型: Ident,
) -> syn::Result<辺形状> {
    if 第1型 != 第2型 {
        return Err(syn::Error::new_spanned(&第2型, "無向辺の両端は同じ型でなければなりません"));
    }
    Ok(辺形状::無向 { 第1役割, 第1型, 積み荷, 第2役割, 第2型 })
}
