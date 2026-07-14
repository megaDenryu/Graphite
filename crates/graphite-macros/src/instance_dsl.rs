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

use proc_macro2::TokenTree;
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
    match Punctuated::<FieldValue, Token![,]>::parse_terminated(&content) {
        Ok(fields) => Ok(fields.into_iter().collect()),
        Err(e) => {
            // G4b: drain_rest のコメント参照。
            drain_rest(&content);
            Err(e)
        }
    }
}

/// 残りのトークンを丸ごと読み飛ばして `ParseStream` を空にする。
///
/// syn の `ParseBuffer` は drop 時に「まだトークンが残っているか」を
/// チェックし、残っていれば共有の `Unexpected` セルにその位置を記録する
/// (`syn::parse::Parser::parse2` はこのセルを最終チェックで読み、"unexpected
/// token" エラーとして再浮上させる)。項目単位のエラー回復 (G4b) では、内側の
/// `Parse` 実装がデリミタの途中でエラーを返した後もそのデリミタの中身が
/// 未消費のまま残ることがあり、これを放置すると「回復して続行したはず」の
/// 箇所で無関係な "unexpected token" が幽霊のように出る。そのため、
/// デリミタ内 (`{ .. }`/`[ .. ]`) でエラーを返す全ての箇所は、返す前に
/// この関数で中身を空にしておく。
fn drain_rest(content: ParseStream) {
    let _ = content.parse::<proc_macro2::TokenStream>();
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

/// `-[label]->` / `-[label { attrs }]->` の `[ .. ]` の中身。
fn parse_edge_label_and_attrs(
    bracket_content: ParseStream,
) -> syn::Result<(Ident, Option<Vec<FieldValue>>)> {
    let label: Ident = bracket_content.parse()?;
    let attrs = if bracket_content.peek(syn::token::Brace) {
        Some(parse_field_values(bracket_content)?)
    } else {
        None
    };
    if !bracket_content.is_empty() {
        return Err(bracket_content
            .error("`-[label]->` または `-[label { attrs }]->` の形式で指定してください"));
    }
    Ok((label, attrs))
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
            let (label, attrs) = match parse_edge_label_and_attrs(&bracket_content) {
                Ok(v) => v,
                Err(e) => {
                    // G4b: drain_rest のコメント参照。
                    drain_rest(&bracket_content);
                    return Err(e);
                }
            };
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

/// 項目単位で回復パースした結果 (`docs/ide_support_spec.md` G4b)。
pub struct GraphParse {
    pub graph: GraphInput,
    /// 個々の項目のパースに失敗した箇所を蓄積したもの。空なら全項目が
    /// 正常にパースできている。
    pub errors: Vec<syn::Error>,
}

impl GraphInput {
    /// 項目単位の回復パーサ (G4b)。
    ///
    /// ## 回復戦略
    ///
    /// - ヘッダ (`SchemaName {`) 自体が壊れている場合は回復せず `Err` を
    ///   返す。
    /// - ボディはカンマ区切りの項目 (ノード宣言 / エッジ) 単位でパースする。
    ///   `graph_schema!` 側 (`node`/`edge` キーワードで境界を判定) とは違い、
    ///   `graph!` の項目は先頭が常に識別子で、ノード宣言かエッジかは2番目の
    ///   トークン (`:` か `-`) を見るまで分からない。そのため「次のキーワード
    ///   まで」という境界定義が使えない。
    /// - **境界の定義**: 代わりに「項目はカンマ区切り」という構文上の性質を
    ///   使い、次のトップレベルの `,` (もしくは入力終端) まで、トークン木を
    ///   1つずつ読み飛ばす境界とする。proc_macro2 では `{ .. }` (フィールド
    ///   初期化子) や `[ .. ]` (エッジラベル) の中身がまるごと1つの `Group`
    ///   トークン木として扱われるため、その中にあるカンマを誤ってトップ
    ///   レベルの区切りだと誤認することはない (`graph_schema!` 側と同じ
    ///   Group 単位読み飛ばしの原理)。
    pub fn parse_recovering(input: ParseStream) -> syn::Result<GraphParse> {
        let schema_name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut items = Vec::new();
        let mut errors = Vec::new();

        while !content.is_empty() {
            match content.parse::<GraphItem>() {
                Ok(item) => items.push(item),
                Err(e) => {
                    errors.push(e);
                    skip_to_comma_boundary(&content);
                }
            }

            if content.is_empty() {
                break;
            }
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else {
                // skip_to_comma_boundary はカンマ直前か入力終端まで進める
                // 保証なので通常はここに来ないはずだが、保険として1トークン
                // 読み飛ばして無限ループを避ける。
                errors.push(content.error("`,` を期待しました"));
                let _ = content.parse::<TokenTree>();
            }
        }

        Ok(GraphParse {
            graph: GraphInput { schema_name, items },
            errors,
        })
    }
}

/// 次のトップレベルの `,` (もしくは入力終端) まで、トークン木を1つずつ
/// 読み飛ばす。[`GraphInput::parse_recovering`] のドキュメントコメント
/// (境界の定義) を参照。
fn skip_to_comma_boundary(content: ParseStream) {
    while !content.is_empty() && !content.peek(Token![,]) {
        let _ = content.parse::<TokenTree>();
    }
}
