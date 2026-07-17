//! `flow!` の入力 DSL のパース (`docs/flow_macro.md` 参照)。
//!
//! 対応する文法:
//!
//! ```text
//! graphite::flow! {
//!     input -[parse]-> parsed,
//!     parsed -[validate]-> valid,          // fan-out
//!     parsed -[stats]-> report,
//!     (valid, report) -[merge]-> out,      // fan-in
//! };
//! ```
//!
//! 1項 (カンマ区切りの矢印文) の文法は `始点 (-[関数式]-> 束縛名)+`。
//! 2段目以降の `-[関数式]-> 束縛名` はチェーン形 (`x -[f]-> y -[g]-> z` ≡
//! `x -[f]-> y, y -[g]-> z`) を1項の中で連続させたもので、2段目以降の
//! 「始点」は直前の束縛名そのもの (構文的に再パースする必要が無いので
//! 専用のフィールドは持たない — [`FlowStmt`] のドキュメント参照)。
//!
//! 始点は任意の式、または `(式, 式, ..)` (fan-in。関数は多引数で呼ばれる)。
//! `-[関数式]->` の関数式も任意の式で、値の型はいずれもパースしない
//! (`graph!` のノード値・エッジ積み荷と同じ方針 — 型は rustc に委ねる)。
//!
//! ## 始点の境界検出とfan-inの判別
//!
//! 始点式は `graph!` のノード/エッジ判別 (`instance_dsl.rs` 冒頭コメント)
//! と同じ問題を抱える: 始点は任意の `syn::Expr` なので、どこまでが始点で
//! どこから矢印かは構造化パースだけでは分からない。ここでは `->`/`-[` の
//! 最初のトークンが必ず「`-`」であるという構造を使い、トップレベル (ネスト
//! したデリミタの中は除く) で最初に `-[` が現れる直前までを始点として捕獲する
//! ([`capture_until_arrow`])。捕獲した生トークン列は
//! [`parse_expr_isolated`]/[`syn::parse2::<ExprList>`] という**独立した
//! トップレベル呼び出し**で改めてパースする — `instance_dsl.rs` 冒頭コメント
//! 「`syn::Expr` を回復パーサに混ぜる際のリスク」と同じ理由 (syn が式の中で
//! 独自に開く `{ .. }` 等のサブバッファは呼び出し元からは見えず
//! `drain_rest` を挟めないため、生トークンの捕獲フェーズと構造化パースの
//! フェーズを分離して汚染を独立した呼び出しに閉じ込める)。
//!
//! `(式, 式, ..)` (fan-in) の判別は「捕獲した始点トークン列が、ちょうど1個の
//! 丸括弧グループそのものである」ことだけを見る ([`as_single_paren_group`])。
//! 中身が1要素だけ (`(x + 1)` のような単なる括弧グループ化) でも fan-in
//! 経路に乗るが、1要素の引数列で関数を呼ぶのは丸括弧の無い形と全く同じ
//! 結果になるため、単なるグループ化との衝突は実質的に起きない
//! (`docs/flow_macro.md` の仕様が要求する「`(式, 式, ..)` は常に多引数呼び出し」
//! を素直に実装した形)。
//!
//! ## `-[関数式]->` 内のエラー回復
//!
//! ブラケット `[ .. ]` の中身は `content.parse::<TokenStream2>()` で丸ごと
//! 捕獲する。これは「生トークンとして読むだけ」と「残りを空にする
//! (drain_rest)」を同時に行うため、後続の独立 `syn::parse2::<Expr>` 呼び出し
//! が失敗しても `content` (この `[ .. ]` のバッファ) に未消費トークンが残る
//! ことはない (`.claude/skills/proc-macro-dev/SKILL.md` の drain_rest 節参照)。

use proc_macro2::{Delimiter, TokenStream as TokenStream2, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::{bracketed, Expr, Ident, Token};

/// 次のトークンが矢印の開始 (`-[`) かどうかを判定する。`->` (素の矢印。
/// `flow!` の始点・関数式の境界には出現しない) と区別するため、`-` の直後が
/// `[` (ブラケットグループ) であることまで確認する。
fn at_arrow_start(input: ParseStream) -> bool {
    input.peek(Token![-]) && input.peek2(syn::token::Bracket)
}

/// 次のトップレベルの矢印開始 (`-[`) まで、トークン木を1つずつ捕獲する
/// (構造化パースを経由しないため、これ自体が [`syn::parse::ParseBuffer`] の
/// Drop 汚染を起こすことはない)。
fn capture_until_arrow(input: ParseStream) -> syn::Result<TokenStream2> {
    let mut collected = TokenStream2::new();
    loop {
        if input.is_empty() {
            return Err(input.error("矢印 `-[関数式]->` を期待しました"));
        }
        if at_arrow_start(input) {
            break;
        }
        let tt: TokenTree = input.parse()?;
        collected.extend(std::iter::once(tt));
    }
    Ok(collected)
}

/// 捕獲済みのトークン列を、**新規の独立したトップレベル呼び出し**として
/// `Expr` にパースする (ファイル冒頭のドキュメントコメント参照)。
fn parse_expr_isolated(tokens: TokenStream2, empty_span: proc_macro2::Span) -> syn::Result<Expr> {
    if tokens.is_empty() {
        return Err(syn::Error::new(empty_span, "式を期待しました"));
    }
    syn::parse2::<Expr>(tokens)
}

/// `(式, 式, ..)` のカンマ区切り式列 (独立呼び出し専用の小さな `Parse` 実装)。
struct ExprList(Vec<Expr>);

impl Parse for ExprList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let punctuated = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated(input)?;
        Ok(ExprList(punctuated.into_iter().collect()))
    }
}

/// 捕獲した始点トークン列が「1個の丸括弧グループそのもの」であれば、その
/// 中身のトークン列を返す (fan-in 判別。ファイル冒頭のドキュメントコメント
/// 参照)。丸括弧グループの前後に余分なトークンがある場合 (例: `(x).foo()`)
/// は該当しない (=通常の式として扱う)。
fn as_single_paren_group(tokens: &TokenStream2) -> Option<TokenStream2> {
    let mut iter = tokens.clone().into_iter();
    let TokenTree::Group(group) = iter.next()? else {
        return None;
    };
    if iter.next().is_some() {
        return None;
    }
    if group.delimiter() != Delimiter::Parenthesis {
        return None;
    }
    Some(group.stream())
}

/// 始点 (`任意の式` または `(式, 式, ..)`) をパースする。戻り値は関数呼び出し
/// の引数列そのもの (fan-in でなければ長さ1)。
fn parse_source(input: ParseStream) -> syn::Result<Vec<Expr>> {
    let span = input.span();
    let captured = capture_until_arrow(input)?;
    if let Some(inner) = as_single_paren_group(&captured) {
        let ExprList(exprs) = syn::parse2::<ExprList>(inner)?;
        return Ok(exprs);
    }
    let expr = parse_expr_isolated(captured, span)?;
    Ok(vec![expr])
}

/// `-[関数式]-> 束縛名` 一段分をパースする。呼び出し前提: `input` の次の
/// トークンが `-[` であること ([`at_arrow_start`] で確認済みの箇所からのみ
/// 呼ぶ)。
fn parse_arrow_step(input: ParseStream) -> syn::Result<(Expr, Ident)> {
    input.parse::<Token![-]>()?;
    let content;
    bracketed!(content in input);
    let span = content.span();
    // ブラケット内を丸ごと捕獲する (ファイル冒頭のドキュメントコメント
    // 「`-[関数式]->` 内のエラー回復」参照)。
    let captured = content.parse::<TokenStream2>()?;
    let func = parse_expr_isolated(captured, span)?;
    input.parse::<Token![->]>()?;
    let binding: Ident = input.parse()?;
    Ok((func, binding))
}

/// 矢印文1段分。`func` は `-[..]->` の中身、`binding` はその段の束縛名。
pub struct FlowStep {
    pub func: Expr,
    pub binding: Ident,
}

/// 矢印文1項分 (チェーン形含む): `始点 -[f]-> y -[g]-> z` 。
///
/// 2段目以降の「始点」を構文的に持たないのは意図的: チェーン形の2段目の
/// 始点は必ず直前の段の束縛名そのもの ([`crate::flow_codegen::generate`] が
/// 生成する `let` 束縛の識別子を再利用する) であり、これは構文ではなく脱糖
/// 側の責務。`docs/flow_macro.md`: 「チェーン形 ... も許す (≡ ... の糖衣)」。
pub struct FlowStmt {
    pub source: Vec<Expr>,
    pub steps: Vec<FlowStep>,
}

impl Parse for FlowStmt {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let source = parse_source(input)?;
        let mut steps = Vec::new();
        loop {
            let (func, binding) = parse_arrow_step(input)?;
            steps.push(FlowStep { func, binding });
            if at_arrow_start(input) {
                continue;
            }
            break;
        }
        Ok(FlowStmt { source, steps })
    }
}

/// `flow!` 全体: カンマ区切りの矢印文の列。`graph!` のようなヘッダ
/// (スキーマ名) は無い。
pub struct FlowInput {
    pub stmts: Vec<FlowStmt>,
}

/// 項単位で回復パースした結果 (`docs/flow_macro.md`: 「項単位のエラー回復
/// (G4 方針、drain_rest 厳守)」)。
pub struct FlowParse {
    pub flow: FlowInput,
    /// 個々の項のパースに失敗した箇所を蓄積したもの。空なら全項が正常に
    /// パースできている。
    pub errors: Vec<syn::Error>,
}

impl FlowInput {
    /// 項単位の回復パーサ (G4)。
    ///
    /// `flow!` には `graph!`/`graph_schema!` のような「壊れていたら全体を
    /// 諦める」ヘッダが無いため (スキーマ名も波括弧宣言も不要)、常にこの
    /// 関数がトップレベルの `Parser::parse2` 呼び出しそのものになる。ループは
    /// `input` が空になるまで必ず進む (各分岐が最低1トークンを消費するため)
    /// ので `input` は関数終了時に必ず空になり、`syn::parse::ParseBuffer` の
    /// Drop 時未消費チェックを汚染することはない。
    ///
    /// ## 境界の定義
    ///
    /// 項はカンマ区切りなので、1項のパースに失敗したら次のトップレベルの
    /// `,` (もしくは入力終端) までトークン木を1つずつ読み飛ばす
    /// ([`skip_to_comma_boundary`])。`instance_dsl.rs`/`schema_dsl.rs` と同じ
    /// 境界定義。
    pub fn parse_recovering(input: ParseStream) -> syn::Result<FlowParse> {
        let mut stmts = Vec::new();
        let mut errors = Vec::new();

        while !input.is_empty() {
            match input.parse::<FlowStmt>() {
                Ok(stmt) => stmts.push(stmt),
                Err(e) => {
                    errors.push(e);
                    skip_to_comma_boundary(input);
                }
            }

            if input.is_empty() {
                break;
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                // skip_to_comma_boundary はカンマ直前か入力終端まで進める
                // 保証なので通常はここに来ないはずだが、保険として1トークン
                // 読み飛ばして無限ループを避ける。
                errors.push(input.error("`,` を期待しました"));
                let _ = input.parse::<TokenTree>();
            }
        }

        Ok(FlowParse {
            flow: FlowInput { stmts },
            errors,
        })
    }
}

/// 次のトップレベルの `,` (もしくは入力終端) まで、トークン木を1つずつ
/// 読み飛ばす。[`FlowInput::parse_recovering`] のドキュメントコメント
/// (境界の定義) を参照。
fn skip_to_comma_boundary(input: ParseStream) {
    while !input.is_empty() && !input.peek(Token![,]) {
        let _ = input.parse::<TokenTree>();
    }
}
