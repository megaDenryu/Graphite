use quote::quote;

use super::*;

#[test]
fn 名簿に無い名前のtracked形式候補はschemaが見つからないエラーになる() {
    let tree = tree();
    let source = source();
    let mut plan = GenerationPlan::new();
    let 名簿 = 静的schema名簿ビルダー::default().完成する();

    let 候補 = call(
        "存在しないschema",
        quote! { generated = "generated/x.rs"; graph 何か; },
        5,
    );
    let files = vec![FileMacros {
        source: &source,
        display_path: "src/main.rs".to_string(),
        calls: vec![候補],
        target: target(&tree, &source),
    }];

    let error = instanceを解決する(&tree, &files, &名簿, &mut plan).err().unwrap();
    assert!(error.to_string().contains("存在しないschema"));
    assert!(error.to_string().contains("見つかりません"));
}

#[test]
fn tracked形式でない無関係なマクロは対象外にする() {
    let tree = tree();
    let source = source();
    let mut plan = GenerationPlan::new();
    let 名簿 = 静的schema名簿ビルダー::default().完成する();

    let 候補 = call("println", quote! { "hello" }, 5);
    let files = vec![FileMacros {
        source: &source,
        display_path: "src/main.rs".to_string(),
        calls: vec![候補],
        target: target(&tree, &source),
    }];

    let 件数 = instanceを解決する(&tree, &files, &名簿, &mut plan).unwrap();
    assert_eq!(件数, 0);
}

#[test]
fn quote系マクロのtracked形式に似た入力は対象外にする() {
    let tree = tree();
    let source = source();
    let mut plan = GenerationPlan::new();
    let 名簿 = 静的schema名簿ビルダー::default().完成する();

    // `quote!`/`quote_spanned!`/`parse_quote!` はいずれもgraphite-codegen/
    // graphite-cli自身の単体試験がフィクスチャ組み立てに使う。この3つの
    // 入力を「typoしたinstance」と誤検出しないことを確かめる
    // (`parse_quote!` は実際に誤検出が起きたことのあるマクロ)。
    for 名前 in ["quote", "quote_spanned", "parse_quote"] {
        let 候補 = call(名前, quote! { generated = "generated/x.rs"; graph 何か; }, 5);
        let files = vec![FileMacros {
            source: &source,
            display_path: "src/main.rs".to_string(),
            calls: vec![候補],
            target: target(&tree, &source),
        }];
        let 件数 = instanceを解決する(&tree, &files, &名簿, &mut plan).unwrap();
        assert_eq!(件数, 0, "{名前}! は対象外のはず");
    }
}

