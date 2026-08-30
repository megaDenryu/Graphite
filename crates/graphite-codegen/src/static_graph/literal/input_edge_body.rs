// 辺の右辺 `(...)` の中身のパース。
//
//   始点 -> 終点              (無積み荷・有向)
//   始点 -[積み荷式]-> 終点    (積み荷あり・有向)
//   始点 -- 終点              (無積み荷・無向)
//   始点 -[積み荷式]- 終点     (積み荷あり・無向。有向の積み荷付き記法から
//                              矢尻を落とした形)

use proc_macro2::Ident;
use syn::parse::ParseStream;
use syn::{bracketed, Expr, Token};

use super::{辺中身, 辺形状};

pub(super) fn 辺形状を読む(内容: ParseStream) -> syn::Result<辺形状> {
    let 始点: Ident = 内容.parse()?;

    if 内容.peek(Token![->]) {
        内容.parse::<Token![->]>()?;
        let 終点: Ident = 内容.parse()?;
        return Ok(辺形状::有向 { 始点, 終点, 中身: 辺中身::無積み荷 });
    }
    if 内容.peek(Token![-]) && 内容.peek2(syn::token::Bracket) {
        内容.parse::<Token![-]>()?;
        let 積み荷角括弧;
        bracketed!(積み荷角括弧 in 内容);
        let 積み荷式: Expr = 積み荷角括弧.parse()?;
        if 内容.peek(Token![->]) {
            内容.parse::<Token![->]>()?;
            let 終点: Ident = 内容.parse()?;
            return Ok(辺形状::有向 { 始点, 終点, 中身: 辺中身::積み荷あり(積み荷式) });
        }
        内容.parse::<Token![-]>()?;
        let 端点2: Ident = 内容.parse()?;
        return Ok(辺形状::無向 { 端点1: 始点, 端点2, 中身: 辺中身::積み荷あり(積み荷式) });
    }
    if 内容.peek(Token![-]) {
        内容.parse::<Token![-]>()?;
        内容.parse::<Token![-]>()?;
        let 端点2: Ident = 内容.parse()?;
        return Ok(辺形状::無向 { 端点1: 始点, 端点2, 中身: 辺中身::無積み荷 });
    }
    Err(内容.error(
        "辺は `始点 -> 終点` / `始点 -[積み荷式]-> 終点` / `始点 -- 終点` / `始点 -[積み荷式]- 終点` の形で書いてください",
    ))
}
