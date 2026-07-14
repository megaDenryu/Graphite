//! `graph_schema!` の入力 DSL のパース (構文木を組み立てるだけで、
//! ノード型の重複や未宣言参照といった意味検査は `schema_validate.rs` で行う)。
//!
//! 対応する文法 (`docs/rust_graph_extension_sketch.md` 水準2相当節):
//!
//! ```text
//! schema OrgChart {
//!     node Employee { name: String, id: u32 }
//!     node Department { name: String }
//!
//!     edge belongs_to: Employee -> Department (1);
//!     edge boss:       Employee -> Employee   (0..1) { since: i32 };
//!     edge reports:    Employee -> Employee   (0..*);
//! }
//! ```

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, parenthesized, Ident, LitInt, Token, Type};

mod kw {
    syn::custom_keyword!(schema);
    syn::custom_keyword!(node);
    syn::custom_keyword!(edge);
}

/// `name: Type` の 1 フィールド (ノードのフィールド / エッジ属性の両方で使う)。
pub struct FieldDecl {
    pub name: Ident,
    pub ty: Type,
}

impl Parse for FieldDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(FieldDecl { name, ty })
    }
}

fn parse_fields_block(input: ParseStream) -> syn::Result<Vec<FieldDecl>> {
    let content;
    braced!(content in input);
    let fields = Punctuated::<FieldDecl, Token![,]>::parse_terminated(&content)?;
    Ok(fields.into_iter().collect())
}

/// `node Employee { name: String, id: u32 }`
/// `node Category(categories) { name: String }` — `(識別子)` は内部ストレージの
/// 複数形フィールド名を明示指定する省略可能な構文 (項目4)。省略時は素朴な
/// `+ "s"` (`crate::naming::plural_field_name`) にフォールバックする。
pub struct NodeDecl {
    pub name: Ident,
    pub plural: Option<Ident>,
    pub fields: Vec<FieldDecl>,
}

impl Parse for NodeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::node>()?;
        let name: Ident = input.parse()?;
        let plural = if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            let plural_ident: Ident = content.parse()?;
            if !content.is_empty() {
                return Err(content.error("複数形指定は識別子ひとつのみ指定してください: `node Type(plural) { .. }`"));
            }
            Some(plural_ident)
        } else {
            None
        };
        let fields = parse_fields_block(input)?;
        Ok(NodeDecl {
            name,
            plural,
            fields,
        })
    }
}

/// 多重度。`(1)` / `(0..1)` / `(0..*)` の 3 種のみサポートする。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Multiplicity {
    One,
    ZeroOrOne,
    ZeroOrMany,
}

const MULTIPLICITY_HELP: &str =
    "多重度は (1) / (0..1) / (0..*) のいずれかのみサポートします";

impl Parse for Multiplicity {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);
        let lit: LitInt = content.parse()?;
        let value: u64 = lit.base10_parse()?;
        match value {
            1 => {
                if !content.is_empty() {
                    return Err(content.error(MULTIPLICITY_HELP));
                }
                Ok(Multiplicity::One)
            }
            0 => {
                content.parse::<Token![..]>()?;
                if content.peek(Token![*]) {
                    content.parse::<Token![*]>()?;
                    if !content.is_empty() {
                        return Err(content.error(MULTIPLICITY_HELP));
                    }
                    Ok(Multiplicity::ZeroOrMany)
                } else {
                    let upper: LitInt = content.parse()?;
                    let upper_value: u64 = upper.base10_parse()?;
                    if upper_value != 1 {
                        return Err(syn::Error::new(upper.span(), MULTIPLICITY_HELP));
                    }
                    if !content.is_empty() {
                        return Err(content.error(MULTIPLICITY_HELP));
                    }
                    Ok(Multiplicity::ZeroOrOne)
                }
            }
            _ => Err(syn::Error::new(lit.span(), MULTIPLICITY_HELP)),
        }
    }
}

/// `edge belongs_to: Employee -> Department (1);`
/// `edge boss: Employee -> Employee (0..1) { since: i32 };`
pub struct EdgeDecl {
    pub label: Ident,
    pub from: Ident,
    pub to: Ident,
    pub mult: Multiplicity,
    pub attrs: Option<Vec<FieldDecl>>,
}

impl Parse for EdgeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::edge>()?;
        let label: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let from: Ident = input.parse()?;
        input.parse::<Token![->]>()?;
        let to: Ident = input.parse()?;
        let mult: Multiplicity = input.parse()?;
        let attrs = if input.peek(syn::token::Brace) {
            Some(parse_fields_block(input)?)
        } else {
            None
        };
        input.parse::<Token![;]>()?;
        Ok(EdgeDecl {
            label,
            from,
            to,
            mult,
            attrs,
        })
    }
}

/// `schema OrgChart { ... }` 全体。
pub struct SchemaInput {
    pub schema_name: Ident,
    pub nodes: Vec<NodeDecl>,
    pub edges: Vec<EdgeDecl>,
}

impl Parse for SchemaInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::schema>()?;
        let schema_name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        while !content.is_empty() {
            if content.peek(kw::node) {
                nodes.push(content.parse::<NodeDecl>()?);
            } else if content.peek(kw::edge) {
                edges.push(content.parse::<EdgeDecl>()?);
            } else {
                return Err(content.error("`node` または `edge` 宣言を期待しました"));
            }
        }

        Ok(SchemaInput {
            schema_name,
            nodes,
            edges,
        })
    }
}
