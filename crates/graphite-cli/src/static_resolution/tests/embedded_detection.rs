// `embedded_detection::埋め込まれたinstanceを検査する` の単体試験。
// `instance_resolution.rs` (`instanceを解決する` の単体試験) とは検査対象の
// 生成器関数が異なるため、別ファイルへ分ける。行数合わせではなく、
// production側のmodule分割 (`embedded_detection.rs`/`instance_resolution.rs`)
// にそのまま対応する。

use quote::quote;

use super::*;

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

#[test]
fn 埋め込みのエラーは識別子自身の行を指し外側の呼び出しの開始行ではない() {
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

    // `quote!` はリテラルトークンにcall-site既定span (行1桁0) しか付けない
    // ため (proc-macro-devスキルの計測で確認済み)、`str::parse` で実際に
    // 複数行のソース文字列を構文解析し、外側の呼び出しの開始行 (20行目)
    // より後ろの行 (3行目、この文字列自身の3行目) に埋め込みを置く。
    let tokens: proc_macro2::TokenStream =
        "\"{}\",\n    1,\n    組織! { generated = \"generated/x.rs\"; graph 何か; }"
            .parse()
            .unwrap();
    let 埋め込み呼び出し = call("println", tokens, 20);
    let files = vec![FileMacros {
        source: &source,
        display_path: "src/main.rs".to_string(),
        calls: vec![埋め込み呼び出し],
    }];

    let error = 埋め込まれたinstanceを検査する(&files, &名簿).err().unwrap();
    assert!(
        error.to_string().contains("src/main.rs:3:"),
        "外側の呼び出しの開始行(20)ではなく識別子自身の行(3)を指すはず: {error}"
    );
}
