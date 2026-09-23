use super::*;

fn parse(source: &str) -> syn::File {
    syn::parse_str(source).unwrap()
}

#[test]
fn マクロ呼び出しを名前とトークンと行番号で集める() {
    let file = parse("graphite::dynamic_graph_schema! { schema X { node Person; } }");
    let calls = collect_macro_calls(&file);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "dynamic_graph_schema");
    assert_eq!(calls[0].line, 1);
}

#[test]
fn 種類の異なる複数のマクロ呼び出しを出現順に集める() {
    let file = parse(
        "graphite::static_graph_schema! { schema X { node Person; } }\n\
         println!(\"hello\");",
    );
    let calls = collect_macro_calls(&file);
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[0].name, "static_graph_schema");
    assert_eq!(calls[1].name, "println");
}

#[test]
fn 呼び出しが無ければ空を返す() {
    let file = parse("fn main() {}");
    assert!(collect_macro_calls(&file).is_empty());
}
