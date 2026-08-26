//! `each <役割名>: N | N..M | N..*` の右辺 (多重度の範囲) を読む。

use proc_macro2::Span;
use syn::parse::ParseStream;
use syn::{LitInt, Token};

/// `each <役割名>: N | N..M | N..*` の右辺。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EachSpec {
    min: usize,
    max: Option<usize>,
}

impl EachSpec {
    fn new(min: usize, max: Option<usize>, error_span: Span) -> syn::Result<Self> {
        if let Some(upper) = max {
            if min <= upper {
                return Ok(Self { min, max });
            }
            return Err(syn::Error::new(
                error_span,
                format!("多重度の下限 {min} は上限 {upper} 以下でなければなりません"),
            ));
        }
        Ok(Self { min, max })
    }

    pub fn min(self) -> usize {
        self.min
    }

    pub fn max(self) -> Option<usize> {
        self.max
    }

    pub fn is_exactly_one(self) -> bool {
        self.min == 1 && self.max == Some(1)
    }

    pub fn is_zero_or_one(self) -> bool {
        self.min == 0 && self.max == Some(1)
    }
}

pub(super) const EACH_HELP: &str =
    "`each <役割名>: N`、`N..M`、`N..*` のいずれかで指定してください";

pub(super) fn parse_each_spec(input: ParseStream) -> syn::Result<EachSpec> {
    let lit: LitInt = input.parse()?;
    let min: usize = lit.base10_parse()?;
    if !input.peek(Token![..]) {
        return EachSpec::new(min, Some(min), lit.span());
    }
    input.parse::<Token![..]>()?;
    if input.peek(Token![*]) {
        input.parse::<Token![*]>()?;
        return EachSpec::new(min, None, lit.span());
    }
    let upper: LitInt = input
        .parse()
        .map_err(|_| syn::Error::new(lit.span(), EACH_HELP))?;
    let max: usize = upper.base10_parse()?;
    EachSpec::new(min, Some(max), upper.span())
}

#[cfg(test)]
impl EachSpec {
    /// 下限と上限を検査せずに直接組み立てる。多重度の分類だけを試す意味層の
    /// テストが、DSL の文字列を経由せずに範囲を作れるようにする。
    pub(crate) fn 検査用に作る(min: usize, max: Option<usize>) -> Self {
        Self { min, max }
    }
}
