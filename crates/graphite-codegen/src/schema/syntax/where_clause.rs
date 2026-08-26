//! `where` 節 (カンマ区切りの制約の列) を読む。

use syn::parse::ParseStream;
use syn::{Ident, Token};

use super::each_specification::{parse_each_spec, EachSpec};
use super::keywords as kw;

pub struct EachConstraint {
    pub role: Ident,
    pub spec: EachSpec,
}

/// `where` 節の制約1つ分。
pub enum Constraint {
    /// `each <役割名>: <spec>`。始点の役割名なら出次数、終点の役割名なら入次数を指す。
    /// どの意味になるかの判定は意味層
    /// (`schema::semantic::each制約が指す端点の側を判定する`) で行うため、
    /// ここではトークンをそのまま保持する。
    Each { ref_ident: Ident, spec: EachSpec },
    /// `unique pair`。
    UniquePair,
}

pub(super) fn parse_constraint(input: ParseStream) -> syn::Result<Constraint> {
    if input.peek(kw::each) {
        input.parse::<kw::each>()?;
        let ref_ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let spec = parse_each_spec(input)?;
        Ok(Constraint::Each { ref_ident, spec })
    } else if input.peek(kw::unique) {
        input.parse::<kw::unique>()?;
        input.parse::<kw::pair>()?;
        Ok(Constraint::UniquePair)
    } else {
        Err(input.error("`each <役割名>: <多重度>` または `unique pair` を期待しました"))
    }
}

/// `where` 節全体 (カンマ区切りの制約の列、`where` キーワード自体は省略可)。
#[derive(Default)]
pub struct WhereClause {
    pub each: Vec<EachConstraint>,
    pub unique_pair: bool,
}

/// `where` 節 (存在すれば) をパースする。`where` キーワードが無ければ
/// 制約なしの `WhereClause::default()` を返す。
pub(super) fn parse_optional_where_clause(input: ParseStream) -> syn::Result<WhereClause> {
    if !input.peek(Token![where]) {
        return Ok(WhereClause::default());
    }
    input.parse::<Token![where]>()?;

    let mut clause = WhereClause::default();
    loop {
        match parse_constraint(input)? {
            Constraint::Each { ref_ident, spec } => {
                if let Some(previous) = clause.each.iter().find(|item| item.role == ref_ident) {
                    let mut error = syn::Error::new(
                        ref_ident.span(),
                        format!("役割名 `{ref_ident}` の多重度が重複しています"),
                    );
                    error.combine(syn::Error::new(previous.role.span(), "最初の指定はこちら"));
                    return Err(error);
                }
                clause.each.push(EachConstraint {
                    role: ref_ident,
                    spec,
                });
            }
            Constraint::UniquePair => {
                clause.unique_pair = true;
            }
        }
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            // 末尾カンマの後 `;` が続く (次の制約が無い) ケースも許容する。
            if input.peek(Token![;]) {
                break;
            }
        } else {
            break;
        }
    }
    Ok(clause)
}
