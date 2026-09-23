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
    let files = vec![FileMacros { source: &source, display_path: "src/main.rs".to_string(), calls: vec![候補] }];

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
    let files = vec![FileMacros { source: &source, display_path: "src/main.rs".to_string(), calls: vec![候補] }];

    let 件数 = instanceを解決する(&tree, &files, &名簿, &mut plan).unwrap();
    assert_eq!(件数, 0);
}

#[test]
fn 名簿の名前と一致するinstanceを解決して計画へ積む() {
    let tree = tree();
    let source = source();
    let mut plan = GenerationPlan::new();
    let mut builder = 静的schema名簿ビルダー::default();

    let schema呼び出し = call(
        "static_graph_schema",
        quote! {
            generated = "generated/組織.rs";
            schema 組織 {
                node 社員;
                node 部署;
                edge 所属 = (member: 社員) -> (team: 部署) where each member: 1;
            }
        },
        3,
    );
    builder.追加する(&tree, &source, "src/main.rs", &schema呼び出し, &mut plan).unwrap();
    let 名簿 = builder.完成する();

    let instance呼び出し = call(
        "組織",
        quote! {
            generated = "generated/開発チーム.rs";
            graph 開発チーム;
            node 太郎 = 社員 { 名前: "太郎".into() };
            node 開発部: 部署;
            edge 太郎の所属 = 所属(太郎 -> 開発部);
        },
        10,
    );
    let files = vec![FileMacros {
        source: &source,
        display_path: "src/main.rs".to_string(),
        calls: vec![instance呼び出し],
    }];

    let 件数 = instanceを解決する(&tree, &files, &名簿, &mut plan).unwrap();
    assert_eq!(件数, 1);
    // schemaとinstanceの2つの生成先が計画に積まれている。
    assert_eq!(plan.declaration_count(), 2);
}

#[test]
fn 他のマクロの入力の中のinstanceはエラーになる() {
    let source = source();
    let mut builder = 静的schema名簿ビルダー::default();
    let tree = tree();
    let mut plan = GenerationPlan::new();
    let schema呼び出し = call(
        "static_graph_schema",
        quote! { generated = "generated/組織.rs"; schema 組織 { node 社員; } },
        3,
    );
    builder.追加する(&tree, &source, "src/main.rs", &schema呼び出し, &mut plan).unwrap();
    let 名簿 = builder.完成する();

    // 無関係な `println!` の入力の中に `組織! { .. }` が埋め込まれている。
    let 埋め込み呼び出し = call(
        "println",
        quote! { "{}", 組織! { generated = "generated/x.rs"; graph 何か; } },
        20,
    );
    let files = vec![FileMacros {
        source: &source,
        display_path: "src/main.rs".to_string(),
        calls: vec![埋め込み呼び出し],
    }];

    let error = 埋め込まれたinstanceを検査する(&files, &名簿).err().unwrap();
    assert!(error.to_string().contains("他のマクロの入力の中"));
    assert!(error.to_string().contains("組織"));
}
