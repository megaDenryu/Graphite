//! `graph!` の入力 DSL のパース。
//!
//! 対応する文法 (v4、`docs/schema_v4.md` §2 参照): **全行が `名前 = 値`**。
//! v4.2 (`docs/graph_splice.md` §1) でスプライス項 `..式` を追加した (下記
//! 「スプライス項」参照)。
//!
//! ```text
//! graph!(Org {
//!     alice = Person { name: "Alice".into() },
//!     bob   = Person { name: "Bob".into() },
//!     eng   = Team { name: "Engineering".into() },
//!
//!     a_team = BelongsTo(alice -> eng),
//!     b_boss = Boss(bob -[promo]-> alice),
//!     lead   = Assigned(alice -[Role { name: "lead".into() }]-> proj),
//!     ..staff,  // スプライス: 実行時コレクションから一括で流し込む
//! })
//! ```
//!
//! `graph!` はスキーマの中身 (`graph_schema!` が何を生成したか) を一切知らない。
//! ノード項の値・エッジの積み荷はいずれも任意の `syn::Expr` として受け取り、
//! 値の型そのものはパースしない (型はマクロではなく rustc の型推論に委ねる)。
//!
//! ## スプライス項 (`..式`、`docs/graph_splice.md` §1)
//!
//! 項の先頭が `..` なら、それは静的な `名前 = 値` の項ではなく実行時
//! コレクションのスプライスである。式は `IntoIterator<Item = (K, T)>`
//! (`K: Into<String>`) を実装している必要があり、`T` がノード型か辺種別かは
//! 静的な項と同様 rustc の型推論が決める。名前は識別子であり `..` から
//! 始まり得ないため、静的な項との曖昧性は無い (先頭トークンだけで判別できる)。
//! 脱糖先は統一 `extend` (`instance_codegen.rs` 参照)。スプライスの要素は
//! 静的な項と異なり名前を持たないため `key` を持たない
//! ([`SpreadInstance`] 参照)。
//!
//! ## ノード項とエッジ項の判別 (v4 での新しい曖昧性)
//!
//! v4 は両方とも `key = ...` から始まるため (旧版はエッジが `a -[label]-> b`
//! という別形だった)、`=` の右辺を見るまでノード項かエッジ項か分からない。
//! エッジ項の右辺は `Kind(from -> to)` / `Kind(from -[式]-> to)` という
//! 「識別子 + 丸括弧 1 つ」の形をしており、これは Rust の通常の呼び出し式
//! (`Kind(args)`) と字面上区別が付かない場合がある (例: `Person(args)` という
//! タプル struct 構築式もノード値として正当)。
//!
//! この曖昧性は **`->` が Rust の式構文には存在しない演算子である**ことを
//! 使って解消できる: 丸括弧の中身が `Ident (-> | -[式]->) Ident` という形に
//! **構造的に**マッチするなら、それは正当な Rust 式としては絶対に解釈できない
//! (`->` は関数シグネチャ・クロージャの戻り値注釈以外の式位置には出現しない)
//! ため、エッジ項として確定して良い。逆にこの形にマッチしなければ、丸括弧の
//! 中身が何であれ通常の `syn::Expr` としてパースを試みる (ノード項)。
//!
//! 具体的には [`looks_like_edge_literal`] で「識別子 + 丸括弧」に続く最初の
//! トークンが `-` (`->` と `-[` の共通の最初のトークン) かどうかだけを軽く
//! 覗き見て判定する。この軽い判定で「エッジのつもりらしい」と分かった場合は
//! [`EdgeLiteralInner`] の構造化パースへコミットし、そこで実際に失敗すれば
//! (例: 積み荷の式が壊れている) そのエラーをそのまま利用者に返す (曖昧性が
//! 無い以上、ノード式へフォールバックし直すと診断がかえって分かりにくくなる
//! ため)。
//!
//! ## `syn::Expr` を回復パーサに混ぜる際のリスク (要実測・実装済み)
//!
//! ノード/エッジのペイロードを自前の
//! `Punctuated::<FieldValue, Token![,]>::parse_terminated` ではなく `syn::Expr`
//! に丸投げすると、式の中に構文ミス (例:
//! `Person { name: "x".into() id: 1 }` のようなフィールド間カンマ抜け) が
//! あるとき、**syn 自身が内部で開く struct リテラル用の `{ .. }` サブバッファ**
//! でエラーが起き、そのサブバッファは呼び出し元 (このファイル) からは
//! 見えないため `drain_rest` を挟めない、という問題がある。
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
//! ### 対処 (実装済み)
//!
//! ノード項の値・エッジの積み荷を「トークン木の生の列」として
//! (`.parse::<TokenTree>()` の繰り返しで、syn の構造化パースを一切経由せず)
//! 境界まで読み取り、**独立した新規トップレベル呼び出し**
//! `syn::parse2::<T>(captured)` で改めてパースする ([`parse_expr_isolated`]/
//! [`try_parse_edge_literal`])。`syn::parse2` は呼び出しごとに新しい
//! `Rc<Cell<Unexpected>>>` ルートを作るため、この独立呼び出し内で起きた
//! 汚染はそのローカルな `Result` に閉じ込められ、外側の回復パーサが共有する
//! セルには一切伝播しない。トークン木を生のまま読むだけの捕獲フェーズは
//! syn の構造化パースを経由しないので、それ自体が新たな汚染源になることも
//! ない。

use proc_macro2::{Delimiter, TokenStream as TokenStream2, TokenTree};
use syn::parse::{Parse, ParseStream};
use syn::{braced, parenthesized, Expr, Ident, Token};

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
/// 呼び出し元が [`parse_expr_isolated`]/[`try_parse_edge_literal`] で
/// 独立に再パースする。
fn capture_until_top_level_comma(content: ParseStream) -> syn::Result<TokenStream2> {
    let mut collected = TokenStream2::new();
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
fn parse_expr_isolated(tokens: TokenStream2, empty_input_span: proc_macro2::Span) -> syn::Result<Expr> {
    if tokens.is_empty() {
        return Err(syn::Error::new(empty_input_span, "式を期待しました"));
    }
    syn::parse2::<Expr>(tokens)
}

/// 捕獲したトークン列が「エッジリテラルのつもり」に見えるかどうかを軽く
/// 判定する。ファイル冒頭のドキュメントコメント「ノード項とエッジ項の判別」
/// 参照。
///
/// 判定基準: `識別子 + 丸括弧グループ` という形にまず一致し、丸括弧の中身の
/// 最初の2トークンが `識別子` + `-` (パンクト) であること。`->`/`-[` は
/// いずれも最初のトークンが `-` の punct であり、この判定だけで両方拾える。
/// 通常の関数呼び出し・タプル struct 構築式の引数列がこの形 (最初の識別子の
/// 直後がハイフン) になることは実質的に無い (`->` は式の中に出現しない
/// トークン列のため)。
fn looks_like_edge_literal(tokens: &TokenStream2) -> bool {
    let mut top_level = tokens.clone().into_iter();
    let Some(TokenTree::Ident(_kind)) = top_level.next() else {
        return false;
    };
    let Some(TokenTree::Group(group)) = top_level.next() else {
        return false;
    };
    if group.delimiter() != Delimiter::Parenthesis {
        return false;
    }
    // 丸括弧グループの後に余分なトークンがあるなら (例: `Kind(..).method()`)
    // エッジリテラルの形ではない。
    if top_level.next().is_some() {
        return false;
    }

    let mut inner = group.stream().into_iter();
    let Some(TokenTree::Ident(_from)) = inner.next() else {
        return false;
    };
    matches!(inner.next(), Some(TokenTree::Punct(p)) if p.as_char() == '-')
}

/// `Kind(from -> to)` / `Kind(from -[式]-> to)` の内側構造。
/// [`looks_like_edge_literal`] が「エッジのつもり」と判定した捕獲済み
/// トークン列を、**独立したトップレベル呼び出し** `syn::parse2` でこの型に
/// パースする (ファイル冒頭のドキュメントコメント参照)。
struct EdgeLiteralInner {
    kind: Ident,
    from: Ident,
    attrs: Option<Expr>,
    to: Ident,
}

impl Parse for EdgeLiteralInner {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let kind: Ident = input.parse()?;
        let content;
        parenthesized!(content in input);

        let from: Ident = content.parse()?;
        let attrs = parse_instance_arrow_payload(&content)?;
        let to: Ident = content.parse()?;
        if !content.is_empty() {
            return Err(content.error(
                "`Kind(from -> to)` / `Kind(from -[式]-> to)` / `Kind(from -- to)` / `Kind(from -[式]- to)` の形式で指定してください",
            ));
        }
        if !input.is_empty() {
            return Err(input.error("余分なトークンがあります"));
        }

        Ok(EdgeLiteralInner { kind, from, attrs, to })
    }
}

/// 柄 (4形: `->` / `-[式]->` / `--` / `-[式]-`) をパースし、積み荷式
/// (あれば) を返す。`graph!` リテラルは有向/無向の区別を一切使わない
/// (脱糖は従来どおり素通し、`docs/edge_endpoints_v4_1.md` §3「graph! 側:
/// 辺コンストラクタ内の柄も同4形。脱糖は従来の機構のまま」) ため、
/// 戻り値は積み荷の有無だけで、向きの情報は捨てる。
fn parse_instance_arrow_payload(content: ParseStream) -> syn::Result<Option<Expr>> {
    if content.peek(Token![->]) {
        content.parse::<Token![->]>()?;
        return Ok(None);
    }
    content.parse::<Token![-]>()?;
    if content.peek(syn::token::Bracket) {
        let bracket_content;
        syn::bracketed!(bracket_content in content);
        let attrs_expr: Expr = match bracket_content.parse() {
            Ok(e) => e,
            Err(e) => {
                drain_rest(&bracket_content);
                return Err(e);
            }
        };
        if !bracket_content.is_empty() {
            let e = bracket_content.error("`-[式]->` または `-[式]-` の形式で指定してください");
            drain_rest(&bracket_content);
            return Err(e);
        }
        if content.peek(Token![->]) {
            content.parse::<Token![->]>()?;
        } else {
            content.parse::<Token![-]>()?;
        }
        Ok(Some(attrs_expr))
    } else {
        // 積み荷なしの無向 (`--`): 最初の `-` は既に消費済みなので、2文字目の
        // `-` を読む。
        content.parse::<Token![-]>()?;
        Ok(None)
    }
}

/// [`looks_like_edge_literal`] が真を返した捕獲済みトークン列を、実際に
/// [`EdgeLiteralInner`] としてパースする。ここで返る `Err` は「エッジの
/// つもりだが壊れている」という確定した診断であり (曖昧性はもう無い)、
/// ノード式へのフォールバックはしない (フォールバックすると `->` を含む
/// トークン列が `syn::Expr` としても失敗し、かえって分かりにくい
/// "expected expression" に化けてしまうため)。
fn parse_edge_literal_isolated(tokens: TokenStream2) -> syn::Result<EdgeLiteralInner> {
    syn::parse2::<EdgeLiteralInner>(tokens)
}

/// `alice = Person { name: "Alice".into() }` / `alice = alice_value`
pub struct NodeInstance {
    pub key: Ident,
    pub value: Expr,
}

/// `a_team = BelongsTo(alice -> eng)` / `b_boss = Boss(bob -[promo]-> alice)`
///
/// `docs/schema_v4.md` §2: 全行が `名前 = 値` であり、エッジ項の名前も
/// (ノードと同様) キーの束縛である。
pub struct EdgeInstance {
    pub key: Ident,
    pub kind: Ident,
    pub from: Ident,
    pub attrs: Option<Expr>,
    pub to: Ident,
}

/// `..式` — 実行時コレクションからノード/辺を一括で流し込む
/// (`docs/graph_splice.md` §1)。式の型は `IntoIterator<Item = (K, T)>` で、
/// `K: Into<String>`・`T` がノード型か辺種別かは静的な項と同様 rustc の
/// 型推論が決める。スプライスの要素は名前を持たない (名前は静的な項だけの
/// 概念) ため、`NodeInstance`/`EdgeInstance` と異なり `key` フィールドが
/// 無い。
pub struct SpreadInstance {
    pub expr: Expr,
}

pub enum GraphItem {
    Node(NodeInstance),
    Edge(EdgeInstance),
    Spread(SpreadInstance),
}

impl Parse for GraphItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // スプライス項 (`..式`) は Rust の struct update 構文 `..rest` の
        // 借用で、項の先頭が `..` かどうかだけで静的な項 (`key = 値`) と
        // 判別できる (`名前 = 値` の名前は識別子であり `..` から始まり得ない
        // ため曖昧性は無い)。
        if input.peek(Token![..]) {
            input.parse::<Token![..]>()?;
            let span = input.span();
            // 構造化パースを経由せず生トークンとして捕獲してから、独立した
            // 新規トップレベル呼び出しで再パースする (ファイル冒頭の
            // ドキュメントコメント参照)。
            let captured = capture_until_top_level_comma(input)?;
            let expr = parse_expr_isolated(captured, span)?;
            return Ok(GraphItem::Spread(SpreadInstance { expr }));
        }

        let key: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let span = input.span();
        // 構造化パースを経由せず生トークンとして捕獲してから、独立した
        // 新規トップレベル呼び出しで再パースする (ファイル冒頭のドキュメント
        // コメント参照)。
        let captured = capture_until_top_level_comma(input)?;

        if looks_like_edge_literal(&captured) {
            let EdgeLiteralInner { kind, from, attrs, to } = parse_edge_literal_isolated(captured)?;
            Ok(GraphItem::Edge(EdgeInstance {
                key,
                kind,
                from,
                attrs,
                to,
            }))
        } else {
            let value = parse_expr_isolated(captured, span)?;
            Ok(GraphItem::Node(NodeInstance { key, value }))
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
    /// - ボディはカンマ区切りの項目 (ノード / エッジ、どちらも `key = ..`
    ///   の形、またはスプライス `..式`) 単位でパースする。
    /// - **境界の定義**: 「項目はカンマ区切り」という構文上の性質を使い、
    ///   次のトップレベルの `,` (もしくは入力終端) まで、トークン木を1つ
    ///   ずつ読み飛ばす境界とする。proc_macro2 では `{ .. }`/`[ .. ]`/
    ///   `( .. )` の中身がまるごと1つの `Group` トークン木として扱われる
    ///   ため、その中にあるカンマを誤ってトップレベルの区切りだと誤認する
    ///   ことはない。
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
