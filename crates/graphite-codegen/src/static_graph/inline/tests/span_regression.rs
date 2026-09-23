use proc_macro2::{LineColumn, TokenStream, TokenTree};

use crate::static_graph::inline::token_type_reference::dslトークンの型参照を組み立てる;
use crate::static_graph::inline::value_supply::個体供給関数を組み立てる;
use crate::static_graph::literal::input::静的グラフ入力;
use crate::static_graph::naming::{個体参照パス, 辺値参照パス, 辺参照パス};
use crate::static_graph::schema::input::静的グラフ型入力;
use crate::static_graph::semantic::意味モデル;

// `syn::parse_str` は実ソースの行・桁を追跡する (`quote!` のリテラルトークンは
// call-site既定span=行1桁0に固定され、行・桁を検証できない。span_probeでの
// 実測で確認済み)。instance_srcの2行目5桁目 (0始まり、"node " の直後) に
// 個体名 `太郎` が来るよう、意図的に行を分けて書く。
fn 複数行のフィクスチャから意味モデルを作る() -> 意味モデル {
    let schema_src = "schema 組織 {\n    node 社員;\n    node 部署;\n    edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;\n}\n";
    let instance_src = "graph 開発チーム;\nnode 太郎 = 社員 { 名前: \"太郎\".into() };\nnode 開発部 = 部署 { 名前: \"開発部\".into() };\nedge 太郎の所属 = 所属(太郎 -> 開発部);\n";
    let schema: 静的グラフ型入力 = syn::parse_str(schema_src).expect("schemaフィクスチャの構文解析");
    let instance: 静的グラフ入力 = syn::parse_str(instance_src).expect("instanceフィクスチャの構文解析");
    crate::static_graph::internal::検証してから意味モデルを組み立てる(schema, instance)
}

// トークン列から、テキストが一致する全Identを出現順に集める。
fn 一致するident列を集める(tokens: &TokenStream, text: &str, 収集先: &mut Vec<proc_macro2::Ident>) {
    for token in tokens.clone() {
        match token {
            TokenTree::Ident(ident) if ident == text => 収集先.push(ident),
            TokenTree::Group(group) => 一致するident列を集める(&group.stream(), text, 収集先),
            _ => {}
        }
    }
}

// トークン列から、テキストが一致する最初のIdentを探す。
fn 最初に一致するidentを探す(tokens: &TokenStream, text: &str) -> Option<proc_macro2::Ident> {
    let mut 見つかった列 = Vec::new();
    一致するident列を集める(tokens, text, &mut 見つかった列);
    見つかった列.into_iter().next()
}

fn 太郎を取り出す(意味モデル: &意味モデル) -> &crate::static_graph::semantic::個体 {
    意味モデル.個体列().iter().find(|個体| *個体.名前() == "太郎").expect("太郎が居るはず")
}

fn 太郎の所属を取り出す(意味モデル: &意味モデル) -> &crate::static_graph::semantic::具体辺 {
    意味モデル.具体辺列().iter().find(|辺| *辺.名前() == "太郎の所属").expect("太郎の所属が居るはず")
}

#[test]
fn dslトークンの型参照は個体名トークンの実際の行と桁を保つ() {
    let 意味モデル = 複数行のフィクスチャから意味モデルを作る();
    let 太郎 = 太郎を取り出す(&意味モデル);
    let 元span始点 = 太郎.名前().span().start();

    // フォールバックのcall-site既定値 (行1桁0) ではなく、実ソースの位置を
    // 追跡できていることの前提確認 (`instance_src` の2行目にある)。
    assert_ne!(元span始点, LineColumn { line: 1, column: 0 });
    assert_eq!(元span始点.line, 2);

    let 生成パス = 個体参照パス(&意味モデル, 太郎.名前());
    let 生成ident = 最初に一致するidentを探す(&生成パス, "太郎Ref").expect("太郎Refが居るはず");
    assert_eq!(生成ident.span().start(), 元span始点, "個体参照パスは元の個体名トークンのspanを継承する");

    // `dslトークンの型参照を組み立てる` は個体の宣言出現 (2行目) と、辺の
    // 端点としての出現 (4行目) の2箇所から `太郎Ref` を作る (`端点型参照列`・
    // `個体型参照列` の2系統)。どちらも元のトークンのspanを保つはずなので、
    // 個体の宣言出現のspanが型参照本体の中に実在することを確かめる
    // (先頭一致だけを見ると辺の端点側を拾ってしまい、宣言出現側の検証が
    // 抜け落ちるため、全出現を集めて含まれるかを見る)。
    let 型参照本体 = dslトークンの型参照を組み立てる(&意味モデル);
    let mut 型参照内ident列 = Vec::new();
    一致するident列を集める(&型参照本体, "太郎Ref", &mut 型参照内ident列);
    assert!(
        型参照内ident列.iter().any(|ident| ident.span().start() == 元span始点),
        "DSLトークンの型参照は個体の宣言出現のspanを保つ個体型参照列を含む (実際の出現: {:?})",
        型参照内ident列.iter().map(|ident| ident.span().start()).collect::<Vec<_>>()
    );
}

#[test]
fn 個体供給関数の戻り値型は実体型トークンの実際の行と桁を保つ() {
    let 意味モデル = 複数行のフィクスチャから意味モデルを作る();
    let 太郎 = 太郎を取り出す(&意味モデル);
    let 元span始点 = 太郎.実体型().span().start();

    assert_ne!(元span始点, LineColumn { line: 1, column: 0 });
    assert_eq!(元span始点.line, 2);

    let nodes型 = quote::quote! { 開発チーム::Nodes };
    let 供給関数 = 個体供給関数を組み立てる(&nodes型, 太郎);
    let 戻り値型ident =
        最初に一致するidentを探す(&供給関数, "社員").expect("戻り値型の社員が居るはず");
    assert_eq!(
        戻り値型ident.span().start(),
        元span始点,
        "個体供給関数の戻り値型は個体宣言の実体型トークンのspanをそのまま使う"
    );
}

#[test]
fn 辺値参照パスは種別トークンの実際の行と桁を保つ() {
    let 意味モデル = 複数行のフィクスチャから意味モデルを作る();
    let 太郎の所属 = 太郎の所属を取り出す(&意味モデル);
    let 元span始点 = 太郎の所属.種別トークン().span().start();

    // `所属` はschema_srcの4行目 (`edge 所属 = ..`) に居る。instance_src側の
    // `所属(太郎 -> 開発部)` の`所属`ではなく、schema宣言側の種別トークンの
    // spanを使うことを確かめる (フォールバック既定値ではないことの前提確認)。
    assert_ne!(元span始点, LineColumn { line: 1, column: 0 });
    assert_eq!(元span始点.line, 4);

    let 生成パス = 辺値参照パス(&意味モデル, 太郎の所属);
    let 生成ident = 最初に一致するidentを探す(&生成パス, "所属Edge").expect("所属Edgeが居るはず");
    assert_eq!(生成ident.span().start(), 元span始点, "辺値参照パスは種別トークンのspanを継承する");
}

#[test]
fn 辺参照パスは辺名トークンの実際の行と桁を保つ() {
    let 意味モデル = 複数行のフィクスチャから意味モデルを作る();
    let 太郎の所属 = 太郎の所属を取り出す(&意味モデル);
    let 元span始点 = 太郎の所属.名前().span().start();

    // `太郎の所属` はinstance_srcの4行目 (`edge 太郎の所属 = ..`) に居る。
    assert_ne!(元span始点, LineColumn { line: 1, column: 0 });
    assert_eq!(元span始点.line, 4);

    let 生成パス = 辺参照パス(&意味モデル, 太郎の所属.名前());
    let 生成ident =
        最初に一致するidentを探す(&生成パス, "太郎の所属Ref").expect("太郎の所属Refが居るはず");
    assert_eq!(生成ident.span().start(), 元span始点, "辺参照パスは辺名トークンのspanを継承する");
}
