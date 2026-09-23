use quote::quote;

use super::*;

#[test]
fn 同名のschemaを2回追加すると重複エラーになる() {
    let tree = tree();
    let source = source();
    let mut plan = GenerationPlan::new();
    let mut builder = 静的schema名簿ビルダー::default();

    let 甲 = call(
        "static_graph_schema",
        quote! { generated = "generated/組織.rs"; schema 組織 { node 社員; } },
        3,
    );
    builder.追加する(&tree, &source, "src/main.rs", &甲, &mut plan).unwrap();

    let 乙 = call(
        "static_graph_schema",
        quote! { generated = "generated/組織2.rs"; schema 組織 { node 社員; } },
        10,
    );
    let error = builder.追加する(&tree, &source, "src/main.rs", &乙, &mut plan).err().unwrap();
    assert!(error.to_string().contains("重複"));
    assert!(error.to_string().contains("組織"));
}
