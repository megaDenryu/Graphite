//! `edge 種別名 = 端点 柄 端点 (where ...)?;` の宣言を読む。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。
//! このファイルは有向・無向と役割名の整合の判定を持つ。この判定は柄の向きと
//! 両端点の役割名の有無の組み合わせを1つの表として持つため、このファイルの
//! 分割は、読み手から表を追う手がかりを奪う。超過を許す根拠の台帳は
//! `docs/development/line_count_ledger.md` にある。

use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::{Ident, Path, Token};

use super::edge_arrow::parse_edge_arrow;
use super::edge_endpoint::{parse_endpoint, DirectedEndpoint, Endpoint};
use super::edge_payload::EdgePayload;
use super::identifier_type::parse_optional_id_type;
use super::keywords as kw;
use super::where_clause::{parse_optional_where_clause, WhereClause};

pub enum EdgeShape {
    Directed {
        from: DirectedEndpoint,
        to: DirectedEndpoint,
        payload: Option<EdgePayload>,
    },
    Undirected {
        first: Ident,
        second: Ident,
        payload: Option<EdgePayload>,
    },
}

// `edge` 宣言1つ分。`kind` は新しい nominal 型として生成される
// (`docs/schema_v4.md` §1)。型名なので慣習上 PascalCase だが、パース段階では
// ケースを検査せず単なる `Ident` として読む。
pub struct EdgeDecl {
    pub kind: Ident,         // エッジ種別名
    pub id_ty: Option<Path>, // 既存の公開 ID 型。`None` なら `{kind}Id` newtype を生成する
    pub shape: EdgeShape,
    pub constraints: WhereClause,
}

impl Parse for EdgeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::edge>()?;
        let kind: Ident = input.parse()?;
        let id_ty = parse_optional_id_type(input)?;
        input.parse::<Token![=]>()?;
        let from_ep = parse_endpoint(input)?;
        let (attrs, directed) = parse_edge_arrow(input)?;
        let to_ep = parse_endpoint(input)?;

        let shape = match (directed, from_ep, to_ep) {
            (
                true,
                Endpoint {
                    role: Some(from_role),
                    ty: from_type,
                },
                Endpoint {
                    role: Some(to_role),
                    ty: to_type,
                },
            ) => {
                validate_unique_roles(
                    [&from_role, &to_role]
                        .into_iter()
                        .chain(attrs.iter().map(|payload| &payload.role)),
                )?;
                EdgeShape::Directed {
                    from: DirectedEndpoint {
                        role: from_role,
                        ty: from_type,
                    },
                    to: DirectedEndpoint {
                        role: to_role,
                        ty: to_type,
                    },
                    payload: attrs,
                }
            }
            (true, Endpoint { role: None, ty, .. }, _) | (true, _, Endpoint { role: None, ty }) => {
                return Err(syn::Error::new(
                    ty.span(),
                    "有向辺の端点は役割名を付けて `(役割名: 型名)` と書いてください",
                ));
            }
            (
                false,
                Endpoint {
                    role: None,
                    ty: first,
                },
                Endpoint {
                    role: None,
                    ty: second,
                },
            ) => EdgeShape::Undirected {
                first,
                second,
                payload: attrs,
            },
            (
                false,
                Endpoint {
                    role: Some(role), ..
                },
                _,
            )
            | (
                false,
                _,
                Endpoint {
                    role: Some(role), ..
                },
            ) => {
                return Err(syn::Error::new(
                    role.span(),
                    "無向辺 (`--`/`-[役割名: 型]-`) には役割名を書けません。役割の区別がある場合は有向辺を使ってください",
                ));
            }
        };

        let constraints = parse_optional_where_clause(input)?;
        input.parse::<Token![;]>()?;
        Ok(EdgeDecl {
            kind,
            id_ty,
            shape,
            constraints,
        })
    }
}

pub(super) fn validate_unique_roles<'a>(
    roles: impl IntoIterator<Item = &'a Ident>,
) -> syn::Result<()> {
    let mut seen: Vec<&Ident> = Vec::new();
    for role in roles {
        if let Some(previous) = seen.iter().find(|previous| ***previous == *role) {
            let mut error = syn::Error::new(
                role.span(),
                format!("同一辺内で役割名 `{role}` が重複しています"),
            );
            error.combine(syn::Error::new(previous.span(), "最初の役割はこちら"));
            return Err(error);
        }
        seen.push(role);
    }
    Ok(())
}
