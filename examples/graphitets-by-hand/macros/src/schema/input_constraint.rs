// where節 (多重度・対一意) の構文のパース。
//
//   where each <役割>: N | N..M | N..* (, ...)*
//   where unique pair

use proc_macro2::Ident;
use syn::parse::ParseStream;
use syn::{LitInt, Token};

use super::制約;

syn::custom_keyword!(each);
syn::custom_keyword!(unique);
syn::custom_keyword!(pair);

pub(super) fn 制約節を読む(input: ParseStream) -> syn::Result<Vec<制約>> {
    if !input.peek(Token![where]) {
        return Ok(Vec::new());
    }
    input.parse::<Token![where]>()?;
    let mut 制約達 = Vec::new();
    loop {
        制約達.push(単一の制約を読む(input)?);
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else {
            break;
        }
    }
    Ok(制約達)
}

fn 単一の制約を読む(input: ParseStream) -> syn::Result<制約> {
    if input.peek(unique) {
        input.parse::<unique>()?;
        input.parse::<pair>()?;
        return Ok(制約::対一意);
    }
    input.parse::<each>()?;
    let 役割: Ident = input.parse()?;
    input.parse::<Token![:]>()?;
    let 下限: LitInt = input.parse()?;
    let 下限値: usize = 下限.base10_parse()?;
    if !input.peek(Token![..]) {
        return Ok(制約::多重度 { 役割, 下限: 下限値, 上限: 下限値 });
    }
    input.parse::<Token![..]>()?;
    if input.peek(Token![*]) {
        input.parse::<Token![*]>()?;
        return Ok(制約::多重度 { 役割, 下限: 下限値, 上限: usize::MAX });
    }
    let 上限: LitInt = input.parse()?;
    let 上限値: usize = 上限.base10_parse()?;
    Ok(制約::多重度 { 役割, 下限: 下限値, 上限: 上限値 })
}
