//! `graph!` の入力 DSL のパース。
//!
//! 対応する文法 (v3、`docs/graph_literal_v3.md` 参照):
//!
//! ```text
//! graph!(OrgChart {
//!     tanaka = Employee { name: "田中".into(), id: 1 },
//!     sato   = sato_value,                              // 外で作った値を move
//!     sales  = Department { name: "営業".into() },
//!
//!     tanaka -[belongs_to]-> sales,
//!     tanaka -[boss = BossEdge { since: 2020 }]-> sato,
//! })
//! ```
//!
//! `graph!` はスキーマの中身 (`graph_schema!` が何を生成したか) を一切知らない。
//! ノード項・エッジ属性の右辺はいずれも任意の `syn::Expr` として受け取り、値の
//! 型そのものはパースしない (型はマクロではなく rustc の型推論に委ねる。
//! `docs/graph_literal_v3.md` §3 参照)。
//! (`-`, `[`, ident, `=`, `]`, `-`, `>` のトークン列の扱いは
//! `.claude/skills/proc-macro-dev/SKILL.md` の注意点を参照)。
//!
//! ## `syn::Expr` を回復パーサに混ぜる際のリスク (要実測・実装済み)
//!
//! v2 まではノード/エッジのペイロードを自前の
//! `Punctuated::<FieldValue, Token![,]>::parse_terminated` で読んでいたため、
//! パース失敗時に自分で `drain_rest` を呼んで安全に回復できていた。v3 は
//! ペイロードを `syn::Expr` に丸投げするため、式の中に構文ミス (例:
//! `Employee { name: "x".into() id: 1 }` のようなフィールド間カンマ抜け) が
//! あると、**syn 自身が内部で開く struct リテラル用の `{ .. }` サブバッファ**
//! でエラーが起き、そのサブバッファは呼び出し元 (このファイル) からは
//! 見えないため `drain_rest` を挟めない。
//!
//! 実際に syn 2.0.118 のソースを確認したところ (`src/group.rs`
//! `parse_delimited` の `crate::parse::get_unexpected(input)` 呼び出し、
//! `src/expr.rs` の `expr_struct_helper` の
//! `let punct: Token![,] = content.parse()?;`)、この懸念は正しいと確認できた:
//!
//! - `braced!`/`bracketed!`/`parenthesized!` で開く**すべての**ネストした
//!   `ParseBuffer` は、開いた側の `unexpected: Rc<Cell<Unexpected>>` を
//!   そのまま共有する (`get_unexpected` が clone するだけ)。この共有は
//!   syn 内部の struct リテラルパーサ (`expr_struct_helper`) が独自に開く
//!   `{ .. }` バッファにも及ぶ (このバッファも同じ `input` から
//!   `braced!(content in input)` されるため)。
//! - `expr_struct_helper` はフィールド間のカンマを
//!   `let punct: Token![,] = content.parse()?;` で読み、失敗時は `?` で
//!   即座に `Err` を返す (`content` の drain は一切しない)。
//! - この状態で `content` (syn 内部のバッファ) が drop すると、
//!   共有している `Unexpected` セルに「未消費トークンあり」が記録される。
//! - この記録は、**呼び出し階層のどこにあるどの `ParseBuffer` が実際に
//!   drop されようとしているか、ではなく、`Parser::parse2` (ここでは
//!   `GraphInput::parse_recovering.parse(input)` というトップレベル呼び出し
//!   1回全体) が最終的に `Ok` を返すかどうかだけを見て、"unexpected token"
//!   という無関係なエラーとして再浮上する** (`syn::parse::Parser::parse2`
//!   の `state.check_unexpected()`)。G4b の回復パーサはまさに「内側の `Err`
//!   を握り潰して続行し、全体としては `Ok` を返す」設計なので、この経路に
//!   直撃する。
//!
//! つまり、壊れた1項目だけを回復してもトップレベル呼び出し自体が
//! `Err(err_unexpected_token(..))` に化けてしまい、`lib.rs` 側の
//! `GraphInput::parse_recovering.parse(input)` が `Err` 分岐 (ヘッダ壊れ扱い)
//! に落ちて **全ての回復結果を握り潰してしまう** (v2 では発生しなかった
//! 深刻な退行)。
//!
//! ### 対処 (実装済み)
//!
//! ノード項の値・エッジ属性の値を「トークン木の生の列」として
//! (`.parse::<TokenTree>()` の繰り返しで、syn の構造化パースを一切経由せず)
//! 境界まで読み取り、**独立した新規トップレベル呼び出し**
//! `syn::parse2::<Expr>(captured)` で改めて式としてパースする
//! ([`parse_expr_isolated`])。`syn::parse2` は呼び出しごとに新しい
//! `Rc<Cell<Unexpected>>>` ルートを作るため、この独立呼び出し内で起きた
//! 汚染はそのローカルな `Result` に閉じ込められ、外側の回復パーサが共有する
//! セルには一切伝播しない。トークン木を生のまま読むだけの捕獲フェーズは
//! syn の構造化パースを経由しないので、それ自体が新たな汚染源になることも
//! ない。
//!
//! 実測は `crates/graphite/tests/ui/graph_partial_recovery.rs` (フィールド間
//! カンマ抜けで壊れた項目1件 + 正常な項目群) で行った。対処前は本コメントで
//! 説明した「トップレベル呼び出し自体が `Err` に化け、正常項目由来の型も
//! 全て消える」という回帰が実際に再現し (壊れていない `sales`/`belongs_to`
//! 由来のコードまで生成されなくなった)、対処後は壊れた項目1件分の
//! `compile_error!` のみが出て他の項目は正常に生成されることを確認した。

use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream};
use syn::{braced, bracketed, Expr, Ident, Token};

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

/// 次のトップレベルの `,` (もしくは入力終端) まで、トークン木を1つずつ
/// 捕獲する (構造化パースを一切経由しないため、これ自体が
/// [`ParseBuffer`] の Drop 汚染を起こすことはない)。捕獲したトークン列は
/// 呼び出し元が [`parse_expr_isolated`] で独立に再パースする。
fn capture_until_top_level_comma(
    content: ParseStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut collected = proc_macro2::TokenStream::new();
    while !content.is_empty() && !content.peek(Token![,]) {
        let tt: TokenTree = content.parse()?;
        collected.extend(std::iter::once(tt));
    }
    Ok(collected)
}

/// 捕獲済みのトークン列を、**新規の独立したトップレベル呼び出し**として
/// `Expr` にパースする。このファイル冒頭のドキュメントコメント「syn::Expr
/// を回復パーサに混ぜる際のリスク」参照: `syn::parse2` は呼ぶたびに新しい
/// `Rc<Cell<Unexpected>>` を作るため、ここで起きるエラーは呼び出し元
/// (G4b の回復パーサ) が共有する `Unexpected` セルを汚染しない。
fn parse_expr_isolated(tokens: proc_macro2::TokenStream, empty_input_span: proc_macro2::Span) -> syn::Result<Expr> {
    if tokens.is_empty() {
        return Err(syn::Error::new(empty_input_span, "式を期待しました"));
    }
    syn::parse2::<Expr>(tokens)
}

/// `tanaka = Employee { name: "田中".into(), id: 1 }` / `tanaka = tanaka_value`
pub struct NodeInstance {
    pub key: Ident,
    pub value: Expr,
}

/// `tanaka -[belongs_to]-> sales` / `tanaka -[boss = BossEdge { since: 2020 }]-> sato`
pub struct EdgeInstance {
    pub from: Ident,
    pub label: Ident,
    pub attrs: Option<Expr>,
    pub to: Ident,
}

/// `-[label]->` / `-[label = 式]->` の `[ .. ]` の中身。
///
/// エッジの属性値は既に `bracketed!` で囲われた `bracket_content` の中に
/// あるため、境界は「`]` まで (=このバッファの残り全部)」で確定している。
/// ノード項の値と違って「次のトップレベル `,`」を自前で探す必要はなく、
/// 単純に残り全トークンを捕獲すればよい。
fn parse_edge_label_and_attrs(bracket_content: ParseStream) -> syn::Result<(Ident, Option<Expr>)> {
    let label: Ident = bracket_content.parse()?;
    let attrs = if bracket_content.peek(Token![=]) {
        bracket_content.parse::<Token![=]>()?;
        let span = bracket_content.span();
        // 構造化パースを経由せず生トークンとして残り全部を捕獲してから、
        // 独立した新規トップレベル呼び出しで式としてパースする (このファイル
        // 冒頭のドキュメントコメント参照)。
        let captured: proc_macro2::TokenStream = bracket_content.parse()?;
        Some(parse_expr_isolated(captured, span)?)
    } else {
        None
    };
    if !bracket_content.is_empty() {
        return Err(bracket_content.error("`-[label]->` または `-[label = 式]->` の形式で指定してください"));
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

        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let span = input.span();
            // ノード項の値は (エッジ属性と違って) 次のトップレベル `,` が
            // 境界になる。構造化パースを経由せず生トークンとして捕獲してから、
            // 独立した新規トップレベル呼び出しで式としてパースする (ファイル
            // 冒頭のドキュメントコメント参照)。
            let captured = capture_until_top_level_comma(input)?;
            let value = parse_expr_isolated(captured, span)?;
            Ok(GraphItem::Node(NodeInstance { key: first, value }))
        } else if input.peek(Token![-]) {
            input.parse::<Token![-]>()?;
            let bracket_content;
            bracketed!(bracket_content in input);
            let (label, attrs) = match parse_edge_label_and_attrs(&bracket_content) {
                Ok(v) => v,
                Err(e) => {
                    // G4b: drain_rest のコメント参照。ここは
                    // `parse_edge_label_and_attrs` 内で syn 構造化パースを
                    // 経由しない捕獲方式に変えたため、通常この分岐に来る
                    // 頃には `bracket_content` は既に空だが、想定外の失敗
                    // (例: ラベル自体の parse 失敗) に備えて保険で呼ぶ。
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
                "`key = 式` (ノード) または `a -[label]-> b` (エッジ) の形式を期待しました",
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
    ///   トークン (`=` か `-`) を見るまで分からない。そのため「次のキーワード
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
