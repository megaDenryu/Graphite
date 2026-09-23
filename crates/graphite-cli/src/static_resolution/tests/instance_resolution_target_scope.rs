// `instanceを解決する` のうち、Cargo targetの境界 (`cargo_target` 参照) に
// 関わる振る舞いだけを検査する。「候補として扱うかどうかの判定」
// (`instance_resolution.rs`) とは責務が異なるため別ファイルへ分ける。

use quote::quote;

use super::*;

#[test]
fn 名簿の名前と一致するinstanceを解決して計画へ積む() {
    let tree = tree();
    let source = source();
    let file_target = target(&tree, &source);
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
    builder
        .追加する(&tree, &source, "src/main.rs", &file_target, &schema呼び出し, &mut plan)
        .unwrap();
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
        target: file_target,
    }];

    let 件数 = instanceを解決する(&tree, &files, &名簿, &mut plan).unwrap();
    assert_eq!(件数, 1);
    // schemaとinstanceの2つの生成先が計画に積まれている。
    assert_eq!(plan.declaration_count(), 2);
}

#[test]
fn 別のcargo_targetにある同名schemaはinstanceとして解決しない() {
    let tree = tree();
    let schema_source = source();
    let schema_target = target(&tree, &schema_source);
    let mut plan = GenerationPlan::new();
    let mut builder = 静的schema名簿ビルダー::default();

    let schema呼び出し = call(
        "static_graph_schema",
        quote! { generated = "generated/組織.rs"; schema 組織 { node 社員; } },
        3,
    );
    builder
        .追加する(&tree, &schema_source, "src/main.rs", &schema_target, &schema呼び出し, &mut plan)
        .unwrap();
    let 名簿 = builder.完成する();

    // `組織!` はsrcで定義したschemaの名前だが、この呼び出しはtests/foo.rs
    // という別のCargo targetにある。最後の識別子が一致するだけでは
    // instanceとして解決してはならない。
    let tests_source = SchemaSourceFile::new(manifest_dir().join("tests").join("foo.rs"));
    let tests_target = target(&tree, &tests_source);
    assert_ne!(schema_target, tests_target);

    let instance呼び出し = call(
        "組織",
        quote! { generated = "generated/開発チーム.rs"; graph 開発チーム; node 太郎: 社員; },
        10,
    );
    let files = vec![FileMacros {
        source: &tests_source,
        display_path: "tests/foo.rs".to_string(),
        calls: vec![instance呼び出し],
        target: tests_target,
    }];

    let error = instanceを解決する(&tree, &files, &名簿, &mut plan).err().unwrap();
    assert!(error.to_string().contains("見つかりません"));
    assert!(
        error.to_string().contains("target"),
        "別targetに同名のschemaがある旨の案内を含むはず: {error}"
    );
}
