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
fn 同じグラフ名で同じgenerated文字列を持つ2つのinstanceは重複エラーになる() {
    let tree = tree();
    let source = source();
    let file_target = target(&tree, &source);
    let mut plan = GenerationPlan::new();
    let mut builder = 静的schema名簿ビルダー::default();

    let schema呼び出し = call(
        "static_graph_schema",
        quote! { generated = "generated/組織.rs"; schema 組織 { node 社員; } },
        3,
    );
    builder
        .追加する(&tree, &source, "src/main.rs", &file_target, &schema呼び出し, &mut plan)
        .unwrap();
    let 名簿 = builder.完成する();

    // `src/a.rs`・`src/b.rs`が別のmoduleでも、それぞれの`generated`は
    // 宣言元ファイルからの相対パスであり、同じ文字列 `generated/T.rs`を
    // 選べば実在するファイル (`src/a/generated/T.rs`・`src/b/generated/T.rs`)
    // は別でも、値マクロの名前は完全に一致する (`instance_duplication`参照)。
    // このフィクスチャは1ファイルに2つの`組織!`呼び出しを置くことで、
    // 別ファイル・別moduleにある実際の事故を1つのtargetの中で近似する。
    let a呼び出し = call("組織", quote! { generated = "generated/T.rs"; graph T; node 甲: 社員; }, 10);
    let b呼び出し = call("組織", quote! { generated = "generated/T.rs"; graph T; node 甲: 社員; }, 20);
    let files = vec![FileMacros {
        source: &source,
        display_path: "src/main.rs".to_string(),
        calls: vec![a呼び出し, b呼び出し],
        target: file_target,
    }];

    let error = instanceを解決する(&tree, &files, &名簿, &mut plan).err().unwrap();
    assert!(error.to_string().contains("重複"));
    assert!(error.to_string().contains("generated/T.rs"));
    assert!(error.to_string().contains("src/main.rs:10"));
    assert!(error.to_string().contains("src/main.rs:20"));
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

