use super::*;

fn parse(source: &str) -> syn::File {
    syn::parse_str(source).unwrap()
}

#[test]
fn 現行名の呼び出しを1件集める() {
    let file = parse("graphite::dynamic_graph_schema! { schema X { node Person; } }");
    let collected = collect_schema_macros(&file);
    assert_eq!(collected.invocations.len(), 1);
    assert!(collected.legacy_invocations.is_empty());
}

#[test]
fn 旧名の呼び出しは宣言行だけを持ち帰る() {
    let file = parse("graphite::graph_schema! { schema X { node Person; } }");
    let collected = collect_schema_macros(&file);
    assert!(collected.invocations.is_empty());
    assert_eq!(collected.legacy_invocations, vec![1]);
}

#[test]
fn 無関係なマクロは無視する() {
    let file = parse("println!(\"hello\");");
    let collected = collect_schema_macros(&file);
    assert!(collected.invocations.is_empty());
    assert!(collected.legacy_invocations.is_empty());
}
