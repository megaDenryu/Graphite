//! `graph_schema!` の入力 DSL のパース (構文木を組み立てるだけで、
//! ノード型の重複や未宣言参照といった意味検査は `schema_validate.rs` で行う)。
//!
//! 対応する文法 (v4、`docs/schema_v4.md` §1 参照):
//!
//! ```text
//! pub struct Person { pub name: String }
//! pub struct Team { pub name: String }
//! pub struct BossEdge { pub since: i32 }
//!
//! graphite::graph_schema! {
//!     schema Org {
//!         node Person;
//!         node Team;
//!
//!         edge BelongsTo = Person -> Team              where each Person: 1;
//!         edge Boss      = Person -[BossEdge]-> Person where each Person: 0..1;
//!         edge DependsOn = Service -> Service          where unique pair;
//!         edge Assigned  = Person -[Role]-> Project;   // 制約なし
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
//! エッジ宣言は `edge Kind = From -> To (where ...)?;` (属性なし) または
//! `edge Kind = From -[Attrs]-> To (where ...)?;` (属性あり) の形。
//! **`Kind` は新しい nominal 型として生成される** (透過的別名ではない)。
//! 旧多重度注釈 `(1)`/`(0..1)`/`(0..*)` は廃止 (字面ごと消滅、検出もしない)。
//!
//! `where` 節はカンマ区切りで複数の制約を書ける:
//! - `each <FromType>: 1` — 各始点ノードにつきちょうど1本
//! - `each <FromType>: 0..1` — 各始点につき高々1本
//! - `unique pair` — 同じ (始点, 終点) の対に2本目を張ることを禁止
//!
//! `each` の `<FromType>` が宣言の `From` と一致するかは意味検査
//! (`schema_validate.rs`) で行う。`each` と `unique pair` は独立した制約
//! として扱い、両方を同時に書くことも許す (`each 0..1` の下では同対2本は
//! 既に不可能なので `unique pair` の併記は冗長だが、実装を単純にするため
//! 特別扱い・警告はしない — `docs/schema_v4.md` §1 が明記する「実装時に
//! 単純な方を選ぶ」を適用した箇所)。
//!
//! ## v4.1 での拡張 (`docs/edge_endpoints_v4_1.md`)
//!
//! - 端点は `Ident` (型名のみ) または `(役割名: 型名)` の2形。役割名は
//!   両端同時か両方省略かの二択 (片方だけは構文エラー)。
//! - 柄は4形: `->` / `-[Attrs]->` (有向) / `--` / `-[Attrs]-` (無向)。
//!   無向辺には役割名を書けない (構文エラー)。
//! - `each <参照名>` の `<参照名>` は役割名つきの辺では役割名 (型名参照は
//!   意味検査でエラー)、無向辺ではノード型名 (次数制約) を指す。役割名により
//!   終点側の入次数制約 (`each <終点役割名>: ..`) も書けるようになる
//!   (意味解決は `schema_validate.rs::resolve_each_side`)。

use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream};
use syn::{braced, bracketed, parenthesized, Ident, LitInt, Path, Token};

mod kw {
    syn::custom_keyword!(schema);
    syn::custom_keyword!(node);
    syn::custom_keyword!(edge);
    syn::custom_keyword!(each);
    syn::custom_keyword!(unique);
    syn::custom_keyword!(pair);
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

/// `node Person;`
///
/// `Person` はユーザーが `graph_schema!` の外で宣言した普通の struct への
/// 参照であり、このマクロは生成しない。型名は単純 `Ident` のみを受け付ける
/// (エッジ端点の型名照合に文字列比較で使うため、`syn::Path` にすると
/// `crate::Person` と `Person` を同一視できず照合が破綻する。モジュール
/// 修飾したい場合は `use` でこのスコープに名前を持ち込むのが Rust の作法
/// どおりの解決)。
///
/// 内部ストレージの複数形フィールド名を明示指定する `node 型名(複数形);`
/// 構文はかつて存在したが、v4 でストレージ名が内部専用 (利用者から不可視)
/// になり明示する意義が消えたため廃止した (`docs/graph_splice.md` §3)。
/// 検出・移行診断は行わない。内部フィールド名は常に素朴な `+ "s"`
/// (`crate::naming::plural_field_name`) で生成する。
pub struct NodeDecl {
    pub name: Ident,
}

impl Parse for NodeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::node>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(NodeDecl { name })
    }
}

/// `each <FromType>: 1` / `each <FromType>: 0..1` の右辺。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EachSpec {
    One,
    ZeroOrOne,
}

const EACH_HELP: &str = "`each <型>: 1` または `each <型>: 0..1` の形式で指定してください";

fn parse_each_spec(input: ParseStream) -> syn::Result<EachSpec> {
    let lit: LitInt = input.parse()?;
    let value: u64 = lit.base10_parse()?;
    match value {
        1 => Ok(EachSpec::One),
        0 => {
            input.parse::<Token![..]>()?;
            let upper: LitInt = input.parse()?;
            let upper_value: u64 = upper.base10_parse()?;
            if upper_value != 1 {
                return Err(syn::Error::new(upper.span(), EACH_HELP));
            }
            Ok(EachSpec::ZeroOrOne)
        }
        _ => Err(syn::Error::new(lit.span(), EACH_HELP)),
    }
}

/// `where` 節の制約1つ分。
pub enum Constraint {
    /// `each <参照名>: <spec>`。`<参照名>` は役割名なしの辺では始点ノード型名、
    /// 役割名つきの辺では役割名 (始点側 = 出次数 / 終点側 = 入次数)、無向辺
    /// では (両端同型の) ノード型名 (= 次数) のいずれかを指す
    /// (`docs/edge_endpoints_v4_1.md`)。どの意味になるかの解決は意味検査
    /// (`schema_validate.rs::resolve_each_side`) で行うため、ここではトークン
    /// をそのまま保持する。
    Each { ref_ident: Ident, spec: EachSpec },
    /// `unique pair`。
    UniquePair,
}

fn parse_constraint(input: ParseStream) -> syn::Result<Constraint> {
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
        Err(input.error("`each <型>: <多重度>` または `unique pair` を期待しました"))
    }
}

/// `where` 節全体 (カンマ区切りの制約の列、`where` キーワード自体は省略可)。
#[derive(Default)]
pub struct WhereClause {
    pub each: Option<(Ident, EachSpec)>,
    pub unique_pair: bool,
}

/// `where` 節 (存在すれば) をパースする。`where` キーワードが無ければ
/// 制約なしの `WhereClause::default()` を返す。
fn parse_optional_where_clause(input: ParseStream) -> syn::Result<WhereClause> {
    if !input.peek(Token![where]) {
        return Ok(WhereClause::default());
    }
    input.parse::<Token![where]>()?;

    let mut clause = WhereClause::default();
    loop {
        match parse_constraint(input)? {
            Constraint::Each { ref_ident, spec } => {
                clause.each = Some((ref_ident, spec));
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

/// `edge Boss = Person -[BossEdge]-> Person where each Person: 0..1;`
/// `edge Boss = (subordinate: Employee) -> (superior: Employee) where each subordinate: 0..1;`
/// `edge Friends = Person -- Person where unique pair;`
///
/// 属性型 (`BossEdge` 等) はユーザーが `graph_schema!` の外で宣言した普通の
/// struct への参照であり、このマクロは生成しない。
///
/// v4.1 (`docs/edge_endpoints_v4_1.md`) で追加された2点:
/// - 端点に役割名を付けられる (`(役割名: 型名)`)。両端同時か両方省略かの
///   二択で、無向辺には付けられない (いずれも構文エラーとして案内する)。
/// - 柄が4形になる: `->` / `-[Attrs]->` (有向) / `--` / `-[Attrs]-` (無向)。
pub struct EdgeDecl {
    /// エッジ種別名。新しい nominal 型として生成される (`docs/schema_v4.md`
    /// §1)。型名なので慣習上 PascalCase だが、パース段階ではケースを検査
    /// しない (単なる `Ident`)。
    pub kind: Ident,
    pub from: Ident,
    pub to: Ident,
    /// 始点の役割名 (`(subordinate: Employee)` の `subordinate`)。役割名を
    /// 書いた辺のみ `Some`。両端同時か両方省略かのいずれかであることは
    /// パース時点で検査済み (`docs/edge_endpoints_v4_1.md` §1)。
    pub from_role: Option<Ident>,
    /// 終点の役割名。
    pub to_role: Option<Ident>,
    /// 有向 (`->`/`-[Attrs]->`) か無向 (`--`/`-[Attrs]-`) か。
    pub directed: bool,
    pub attrs_ty: Option<Path>,
    pub constraints: WhereClause,
}

/// 端点1つ分 (`Ident` または `(役割名: 型名)`)。
struct Endpoint {
    role: Option<Ident>,
    ty: Ident,
}

/// 端点をパースする。`(` で始まれば役割名つき `(役割名: 型名)`、そうでなければ
/// 型名のみの `Ident`。
fn parse_endpoint(input: ParseStream) -> syn::Result<Endpoint> {
    if input.peek(syn::token::Paren) {
        let content;
        parenthesized!(content in input);
        match parse_endpoint_paren_body(&content) {
            Ok(v) => Ok(v),
            Err(e) => {
                // G4a: drain_rest のコメント参照。
                drain_rest(&content);
                Err(e)
            }
        }
    } else {
        let ty: Ident = input.parse()?;
        Ok(Endpoint { role: None, ty })
    }
}

/// `(役割名: 型名)` の `( .. )` の中身。
fn parse_endpoint_paren_body(content: ParseStream) -> syn::Result<Endpoint> {
    let role: Ident = content.parse()?;
    content.parse::<Token![:]>()?;
    let ty: Ident = content.parse()?;
    if !content.is_empty() {
        return Err(content.error("端点は `(役割名: 型名)` の形式で指定してください"));
    }
    Ok(Endpoint { role: Some(role), ty })
}

/// 柄 (4形: `->` / `-[Attrs]->` / `--` / `-[Attrs]-`) をパースし、
/// `(積み荷型, 有向か)` を返す。
///
/// 有向の柄 `-` + `>` から矢尻を落とすと無向の柄になる、という
/// `docs/edge_endpoints_v4_1.md` §2 の導出規則どおりに実装する: 最初の `-`
/// を読んだ後、`[Attrs]` (積み荷、あれば) を読み、最後に `->` (有向) か `-`
/// (無向) かで向きを判定する。
fn parse_edge_arrow(input: ParseStream) -> syn::Result<(Option<Path>, bool)> {
    // 素の `->` (単一の複合トークン) を先読みして判定する。`-[`/`--` は
    // いずれも `-` と別トークンの2トークンなので `->` と誤って先読み
    // マッチすることはない。
    if input.peek(Token![->]) {
        input.parse::<Token![->]>()?;
        return Ok((None, true));
    }
    input.parse::<Token![-]>()?;
    if input.peek(syn::token::Bracket) {
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
        if input.peek(Token![->]) {
            input.parse::<Token![->]>()?;
            Ok((Some(attrs_ty), true))
        } else {
            input.parse::<Token![-]>()?;
            Ok((Some(attrs_ty), false))
        }
    } else {
        // 積み荷なし。ここまでで最初の `-` を消費済みなので、残りの `-`
        // (無向の柄 `--` の2文字目) を読む。
        input.parse::<Token![-]>()?;
        Ok((None, false))
    }
}

impl Parse for EdgeDecl {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::edge>()?;
        let kind: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let from_ep = parse_endpoint(input)?;
        let (attrs_ty, directed) = parse_edge_arrow(input)?;
        let to_ep = parse_endpoint(input)?;

        // 役割名の妥当性検査 (`docs/edge_endpoints_v4_1.md` §1/§2)。
        if !directed {
            // 無向辺には役割名を書けない (役割の区別がある時点で対称ではない)。
            if let Some(bad_role) = from_ep.role.as_ref().or(to_ep.role.as_ref()) {
                return Err(syn::Error::new(
                    bad_role.span(),
                    "無向辺 (`--`/`-[Attrs]-`) には役割名を書けません。役割の区別がある場合は役割名つき有向辺を使ってください",
                ));
            }
        } else {
            match (&from_ep.role, &to_ep.role) {
                (Some(_), Some(_)) | (None, None) => {}
                (Some(r), None) => {
                    return Err(syn::Error::new(
                        r.span(),
                        "役割名は両端同時に書くか、両方省略するかのいずれかにしてください (この辺は始点にだけ役割名があります)",
                    ));
                }
                (None, Some(r)) => {
                    return Err(syn::Error::new(
                        r.span(),
                        "役割名は両端同時に書くか、両方省略するかのいずれかにしてください (この辺は終点にだけ役割名があります)",
                    ));
                }
            }
        }

        let constraints = parse_optional_where_clause(input)?;
        input.parse::<Token![;]>()?;
        Ok(EdgeDecl {
            kind,
            from: from_ep.ty,
            to: to_ep.ty,
            from_role: from_ep.role,
            to_role: to_ep.role,
            directed,
            attrs_ty,
            constraints,
        })
    }
}

/// `-[型パス]->` / `-[型パス]-` の `[ .. ]` の中身。`syn::Path` として受けるため
/// `edges::BossEdge` のようなモジュール修飾も許す (ノード型名と違い端点照合
/// に使わないため、単純 `Ident` に制限する必要がない)。
fn parse_edge_bracket_body(content: ParseStream) -> syn::Result<Path> {
    let path: Path = content.parse()?;
    if !content.is_empty() {
        return Err(content.error("`-[型パス]->` または `-[型パス]-` の形式で指定してください"));
    }
    Ok(path)
}

/// `schema Org { ... }` 全体。
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
    ///   性質にただ乗りできる (where 節・エッジラベルの中身にどんなトークンが
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
