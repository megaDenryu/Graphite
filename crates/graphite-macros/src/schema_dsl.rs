//! `graph_schema!` の入力 DSL のパース (構文木を組み立てるだけで、
//! ノード型の重複や未宣言参照といった意味検査は `schema_validate.rs` で行う)。
//!
//! 対応する文法 (`docs/edge_syntax_v3.md` 参照):
//!
//! ```text
//! pub struct Employee { pub name: String, pub id: u32 }
//! pub struct Department { pub name: String }
//! pub struct BossEdge { pub since: i32 }
//!
//! graphite::graph_schema! {
//!     schema OrgChart {
//!         node Employee;
//!         node Department;
//!
//!         edge belongs_to: Employee -> Department (1);
//!         edge boss:       Employee -[BossEdge]-> Employee (0..1);
//!         edge reports:    Employee -> Employee (0..*);
//!     }
//! }
//! ```
//!
//! ノード型・エッジ属性型はいずれも `graph_schema!` の外でユーザーが普通の
//! struct として宣言したものを参照するだけで、このマクロは生成しない。
//! ノード型名は端点照合に使うため単純 `Ident` のみ (モジュール修飾したい
//! 場合は `use` で名前をこのスコープに持ち込む)。エッジ属性型は照合には
//! 使わず参照するだけなので `syn::Path` (モジュール修飾可) を許す。
//!
//! エッジ宣言は `label: From -> To (mult);` (属性なし) または
//! `label: From -[Attrs]-> To (mult);` (属性あり) の形。`label:` の右側
//! 全体が関係型 (Rust の `f: impl Fn(A) -> B` と同じ読み方)、矢印内は
//! 積み荷 (属性型) のみという v3 の設計 (`docs/edge_syntax_v3.md` 参照)。

use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream};
use syn::{braced, bracketed, parenthesized, Ident, LitInt, Path, Token};

mod kw {
    syn::custom_keyword!(schema);
    syn::custom_keyword!(node);
    syn::custom_keyword!(edge);
}

/// 残りのトークンを丸ごと読み捨てて `ParseStream` を空にする。
///
/// syn の `ParseBuffer` は drop 時に「まだトークンが残っているか」を
/// チェックし、残っていれば共有の `Unexpected` セルにその位置を記録する
/// (`syn::parse::Parser::parse2` はこのセルを最終チェックで読み、"unexpected
/// token" エラーとして再浮上させる)。宣言単位のエラー回復 (G4) では、内側の
/// `Parse` 実装がデリミタの途中でエラーを返した後もそのデリミタの中身が
/// 未消費のまま残ることがあり、これを放置すると「回復して続行したはず」の
/// 箇所で無関係な "unexpected token" が幽霊のように出る。そのため、
/// デリミタ内 (`( .. )`/`[ .. ]`) でエラーを返す全ての箇所は、返す前に
/// この関数で中身を空にしておく。
fn drain_rest(content: ParseStream) {
    let _ = content.parse::<proc_macro2::TokenStream>();
}

/// `node Employee;`
/// `node Category(categories);` — `(識別子)` は内部ストレージの複数形
/// フィールド名を明示指定する省略可能な構文。省略時は素朴な `+ "s"`
/// (`crate::naming::plural_field_name`) にフォールバックする。
///
/// `Employee`/`Category` はユーザーが `graph_schema!` の外で宣言した普通の
/// struct への参照であり、このマクロは生成しない (`docs/edge_syntax_v2.md`
/// 参照)。型名は単純 `Ident` のみを受け付ける (エッジ端点の型名照合に文字列
/// 比較で使うため、`syn::Path` にすると `crate::Employee` と `Employee` を
/// 同一視できず照合が破綻する。モジュール修飾したい場合は `use` でこの
/// スコープに名前を持ち込むのが Rust の作法どおりの解決)。
pub struct NodeDecl {
    pub name: Ident,
    pub plural: Option<Ident>,
}

/// `node Type(plural)` の `(plural)` 部分の中身。単一の識別子のみを許す。
fn parse_plural_paren_body(content: ParseStream) -> syn::Result<Ident> {
    let plural_ident: Ident = content.parse()?;
    if !content.is_empty() {
        return Err(content.error("複数形指定は識別子ひとつのみ指定してください: `node Type(plural);`"));
    }
    Ok(plural_ident)
}

impl Parse for NodeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::node>()?;
        let name: Ident = input.parse()?;
        let plural = if input.peek(syn::token::Paren) {
            let content;
            parenthesized!(content in input);
            match parse_plural_paren_body(&content) {
                Ok(plural_ident) => Some(plural_ident),
                Err(e) => {
                    // G4a: drain_rest のコメント参照。エラー時に `content`
                    // (この `(..)` の中身) を読み捨ててから返す。
                    drain_rest(&content);
                    return Err(e);
                }
            }
        } else {
            None
        };
        input.parse::<Token![;]>()?;
        Ok(NodeDecl { name, plural })
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
        let result = parse_multiplicity_body(&content);
        if result.is_err() {
            // G4a: drain_rest のコメント参照。分岐が多いためこの関数を
            // 「本体を別関数に切り出し、エラー時は一括で drain する」形に
            // している (各 `return Err(..)` のたびに drain を書くと漏れが
            // 出やすいため)。
            drain_rest(&content);
        }
        result
    }
}

fn parse_multiplicity_body(content: ParseStream) -> syn::Result<Multiplicity> {
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

/// `edge belongs_to: Employee -> Department (1);`
/// `edge boss: Employee -[BossEdge]-> Employee (0..1);`
///
/// 属性型 (`BossEdge` 等) はユーザーが `graph_schema!` の外で宣言した普通の
/// struct への参照であり、このマクロは生成しない (`docs/edge_syntax_v3.md`
/// 参照)。
pub struct EdgeDecl {
    pub label: Ident,
    pub from: Ident,
    pub to: Ident,
    pub mult: Multiplicity,
    pub attrs_ty: Option<Path>,
}

impl Parse for EdgeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::edge>()?;
        let label: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let from: Ident = input.parse()?;
        // `->` (属性なし) か `-[Attrs]->` (属性あり) かは、まず素の `->`
        // (単一の複合トークン) を先読みして判定する。`-[` は `-` と `[..]`
        // の2トークンなので `->` と誤って先読みマッチすることはない。
        let attrs_ty = if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            None
        } else {
            input.parse::<Token![-]>()?;
            let bracket_content;
            bracketed!(bracket_content in input);
            let attrs_ty = match parse_edge_bracket_body(&bracket_content) {
                Ok(v) => v,
                Err(e) => {
                    // G4a: drain_rest のコメント参照。
                    drain_rest(&bracket_content);
                    return Err(e);
                }
            };
            input.parse::<Token![->]>()?;
            Some(attrs_ty)
        };
        let to: Ident = input.parse()?;
        let mult: Multiplicity = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(EdgeDecl {
            label,
            from,
            to,
            mult,
            attrs_ty,
        })
    }
}

/// `-[型パス]->` の `[ .. ]` の中身。`syn::Path` として受けるため
/// `edges::BossEdge` のようなモジュール修飾も許す (ノード型名と違い端点照合
/// に使わないため、単純 `Ident` に制限する必要がない)。
fn parse_edge_bracket_body(content: ParseStream) -> syn::Result<Path> {
    let path: Path = content.parse()?;
    if !content.is_empty() {
        return Err(content.error("`-[型パス]->` の形式で指定してください"));
    }
    Ok(path)
}

/// `schema OrgChart { ... }` 全体。
pub struct SchemaInput {
    pub schema_name: Ident,
    pub nodes: Vec<NodeDecl>,
    pub edges: Vec<EdgeDecl>,
}

/// 宣言単位で回復パースした結果 (`docs/ide_support_spec.md` G4a)。
pub struct SchemaParse {
    pub schema: SchemaInput,
    /// 個々の宣言のパースに失敗した箇所を蓄積したもの。空なら全宣言が
    /// 正常にパースできている。
    pub errors: Vec<syn::Error>,
}

impl SchemaInput {
    /// 宣言単位の回復パーサ (G4a)。
    ///
    /// ## 回復戦略
    ///
    /// - ヘッダ (`schema Name {`) 自体が壊れている場合は回復せず `Err` を
    ///   返す (`schema` キーワード・スキーマ名・開きブレースが揃わないと
    ///   ボディの走査自体を始められないため)。
    /// - ボディ内は `node`/`edge` 宣言単位でパースする。1宣言のパースに
    ///   失敗したら、その `syn::Error` を `errors` に蓄積し、次の宣言境界
    ///   まで読み飛ばして続行する。
    /// - **境界の定義**: ボディの `ParseStream` からトークン木を1つずつ
    ///   読み飛ばし、次に `node`/`edge` キーワードが先頭に現れるか入力が
    ///   尽きるまで進める。`node`/`edge` いずれの宣言も `;` で終わるため
    ///   `;` 区切りの境界定義も選べるが、キーワード探索は proc_macro2 の
    ///   `( .. )`/`[ .. ]` がまるごと1つの `Group` トークン木として扱われる
    ///   性質にただ乗りできる (多重度・エッジラベルの中身にどんなトークンが
    ///   あっても、Group 単位で一括に読み飛ばされるので誤って途中で止まらない)
    ///   うえ、両宣言に共通して使え実装も単純で誤爆しにくいためこちらを
    ///   採用した。
    pub fn parse_recovering(input: ParseStream) -> syn::Result<SchemaParse> {
        input.parse::<kw::schema>()?;
        let schema_name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        let mut errors = Vec::new();

        while !content.is_empty() {
            if content.peek(kw::node) {
                match content.parse::<NodeDecl>() {
                    Ok(n) => nodes.push(n),
                    Err(e) => {
                        errors.push(e);
                        skip_to_decl_boundary(&content);
                    }
                }
            } else if content.peek(kw::edge) {
                match content.parse::<EdgeDecl>() {
                    Ok(ed) => edges.push(ed),
                    Err(e) => {
                        errors.push(e);
                        skip_to_decl_boundary(&content);
                    }
                }
            } else {
                errors.push(content.error("`node` または `edge` 宣言を期待しました"));
                skip_to_decl_boundary(&content);
            }
        }

        Ok(SchemaParse {
            schema: SchemaInput {
                schema_name,
                nodes,
                edges,
            },
            errors,
        })
    }
}

/// 次の `node`/`edge` キーワード (もしくは入力終端) まで、トークン木を
/// 1つずつ読み飛ばす。[`SchemaInput::parse_recovering`] のドキュメント
/// コメント (境界の定義) を参照。
fn skip_to_decl_boundary(content: ParseStream) {
    while !content.is_empty() && !content.peek(kw::node) && !content.peek(kw::edge) {
        // `content.parse::<TokenTree>()` は必ず1つトークン木を消費する
        // (`content` が空でないことは while 条件で保証済み) ので、
        // 無限ループにはならない。
        let _ = content.parse::<TokenTree>();
    }
}
