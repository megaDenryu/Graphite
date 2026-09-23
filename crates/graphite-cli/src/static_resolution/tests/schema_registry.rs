use quote::quote;

use super::*;

#[test]
fn 同名のschemaを2回追加すると重複エラーになる() {
    let tree = tree();
    let source = source();
    let target = target(&tree, &source);
    let mut plan = GenerationPlan::new();
    let mut builder = 静的schema名簿ビルダー::default();

    let 甲 = call(
        "static_graph_schema",
        quote! { generated = "generated/組織.rs"; schema 組織 { node 社員; } },
        3,
    );
    builder.追加する(&tree, &source, "src/main.rs", &target, &甲, &mut plan).unwrap();

    let 乙 = call(
        "static_graph_schema",
        quote! { generated = "generated/組織2.rs"; schema 組織 { node 社員; } },
        10,
    );
    let error = builder
        .追加する(&tree, &source, "src/main.rs", &target, &乙, &mut plan)
        .err()
        .unwrap();
    assert!(error.to_string().contains("重複"));
    assert!(error.to_string().contains("組織"));
}

#[test]
fn 別のcargo_targetなら同名のschemaを許す() {
    let tree = tree();
    let src_source = source();
    let src_target = target(&tree, &src_source);
    let tests_source = SchemaSourceFile::new(std::path::PathBuf::from("/repo/tests/foo.rs"));
    let tests_target = target(&tree, &tests_source);
    assert_ne!(src_target, tests_target);

    let mut plan = GenerationPlan::new();
    let mut builder = 静的schema名簿ビルダー::default();

    let 甲 = call(
        "static_graph_schema",
        quote! { generated = "generated/組織.rs"; schema 組織 { node 社員; } },
        3,
    );
    builder
        .追加する(&tree, &src_source, "src/main.rs", &src_target, &甲, &mut plan)
        .unwrap();

    let 乙 = call(
        "static_graph_schema",
        quote! { generated = "generated/組織.rs"; schema 組織 { node 社員; } },
        5,
    );
    builder
        .追加する(&tree, &tests_source, "tests/foo.rs", &tests_target, &乙, &mut plan)
        .unwrap();

    let 名簿 = builder.完成する();
    assert_eq!(名簿.len(), 2);
}
