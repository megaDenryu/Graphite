//! `graph!` の入力 DSL のパース。
//!
//! 対応する文法:
//!
//! ```text
//! graph!(OrgChart {
//!     tanaka: Employee { name: "田中".into(), id: 1 },
//!     sato:   Employee { name: "佐藤".into(), id: 2 },
//!     sales:  Department { name: "営業".into() },
//!
//!     tanaka -[belongs_to]-> sales,
//!     tanaka -[boss { since: 2020 }]-> sato,
//! })
//! ```
//!
//! `graph!` はスキーマの中身 (`graph_schema!` が何を生成したか) を一切知らない。
//! ノード宣言行からその場で「識別子 -> 型名」の対応表を組み立て、辺の端点の
//! 型名をそこから逆引きすることで、`graph_schema!` 側の命名規則
//! (`crate::naming`) とだけ整合させれば済むようにしている
//! (`-`, `[`, ident, `{`, `}`, `]`, `-`, `>` のトークン列の扱いは
//! `.claude/skills/proc-macro-dev/SKILL.md` の注意点を参照)。

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, bracketed, Expr, Ident, Token};

/// `name: expr` の 1 フィールド値 (ノードのフィールド初期化 / エッジ属性の両方)。
pub struct FieldValue {
    pub name: Ident,
    pub value: Expr,
}

impl Parse for FieldValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let value: Expr = input.parse()?;
        Ok(FieldValue { name, value })
    }
}

fn parse_field_values(input: ParseStream) -> syn::Result<Vec<FieldValue>> {
    let content;
    braced!(content in input);
    let fields = Punctuated::<FieldValue, Token![,]>::parse_terminated(&content)?;
    Ok(fields.into_iter().collect())
}

/// `tanaka: Employee { name: "田中".into(), id: 1 }`
pub struct NodeInstance {
    pub key: Ident,
    pub type_name: Ident,
    pub fields: Vec<FieldValue>,
}

/// `tanaka -[belongs_to]-> sales` / `tanaka -[boss { since: 2020 }]-> sato`
pub struct EdgeInstance {
    pub from: Ident,
    pub label: Ident,
    pub attrs: Option<Vec<FieldValue>>,
    pub to: Ident,
}

pub enum GraphItem {
    Node(NodeInstance),
    Edge(EdgeInstance),
}

impl Parse for GraphItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first: Ident = input.parse()?;

        if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            let type_name: Ident = input.parse()?;
            let fields = parse_field_values(input)?;
            Ok(GraphItem::Node(NodeInstance {
                key: first,
                type_name,
                fields,
            }))
        } else if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            let bracket_content;
            bracketed!(bracket_content in input);
            let label: Ident = bracket_content.parse()?;
            let attrs = if bracket_content.peek(syn::token::Brace) {
                Some(parse_field_values(&bracket_content)?)
            } else {
                None
            };
            if !bracket_content.is_empty() {
                return Err(bracket_content
                    .error("`-[label]->` または `-[label { attrs }]->` の形式で指定してください"));
            }
            input.parse::<Token![->]>()?;
            let to: Ident = input.parse()?;
            Ok(GraphItem::Edge(EdgeInstance {
                from: first,
                label,
                attrs,
                to,
            }))
        } else {
            Err(input.error(
                "`key: Type { .. }` (ノード) または `a -[label]-> b` (エッジ) の形式を期待しました",
            ))
        }
    }
}

/// `graph!` 全体: `SchemaName { item, item, ... }`。
pub struct GraphInput {
    pub schema_name: Ident,
    pub items: Vec<GraphItem>,
}

impl Parse for GraphInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let schema_name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut items = Vec::new();
        while !content.is_empty() {
            items.push(content.parse::<GraphItem>()?);
            if content.is_empty() {
                break;
            }
            content.parse::<Token![,]>()?;
        }

        Ok(GraphInput { schema_name, items })
    }
}
