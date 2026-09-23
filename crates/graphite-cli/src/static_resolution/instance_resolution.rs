//! 静的グラフのinstanceの解決 (issue #41 段階3 §2 第2段階)。
//!
//! 名簿 (`schema_registry::静的schema名簿`) に名前が無い残りの呼び出しの
//! うち、名簿の名前と一致するものをinstanceとみなして解決する。一致しない
//! のに追跡形式 (`generated = "...";` で始まる) の候補は「schemaが見つから
//! ない」エラーにする。

use std::error::Error;

use proc_macro2::TokenStream;

use graphite_codegen::DeclarationSite;

use super::schema_registry::静的schema名簿;
use super::FileMacros;
use crate::generation_plan::GenerationPlan;
use crate::generation_tree::GenerationTree;

// 静的グラフのinstanceを解決する。戻り値は解決したinstanceの件数。
pub(crate) fn instanceを解決する(
    tree: &GenerationTree,
    files: &[FileMacros],
    名簿: &静的schema名簿,
    plan: &mut GenerationPlan,
) -> Result<usize, Box<dyn Error>> {
    let mut 件数 = 0;
    for file in files {
        for call in &file.calls {
            if 候補として扱わない名前か(&call.name) {
                continue;
            }
            let Some((schema, schema_site)) = 名簿.探す(&call.name) else {
                if 追跡形式らしいか(&call.tokens) {
                    return Err(format!(
                        "{}:{}: schema `{}` がこのパッケージに見つかりません",
                        file.display_path, call.line, call.name
                    )
                    .into());
                }
                continue;
            };
            let instance = graphite_codegen::parse_tracked_static_instance(schema, call.tokens.clone())
                .map_err(|errors| file.source.format_errors(tree, errors))?;
            let target = file
                .source
                .generated_target(tree, &instance.generated_path().value())?;
            let site = DeclarationSite::new(file.display_path.clone(), call.line);
            let content = instance
                .render_module_source(&site, schema_site)
                .map_err(|error| file.source.format_errors(tree, vec![error]))?;
            plan.add(tree, target, content)?;
            件数 += 1;
        }
    }
    Ok(件数)
}

// instance候補として扱わない呼び出し名。(a) Graphiteのschema宣言マクロ自身
// (名簿とのマッチング対象にする意味が無く、`generated = "...";` で始まる
// 自分自身の入力を「typoしたinstance」と誤検出しても意味が無い)。
// (b) `quote!`。graphite-codegen/graphite-cli自身の単体試験は、DSL構文木を
// 組み立てる素材として `quote! { generated = "..."; graph X; .. }` の形の
// トークン列を大量に持つ (`parse_tracked_static_schema`/
// `parse_tracked_static_instance` の入力フィクスチャ)。これは実在の
// instance宣言ではなくデータであり、`quote!` の入力を「typoしたinstance」
// として解析しようとすると、Graphite自身のソースを `cargo xtask generate`
// で処理したときに大量の偽陽性を生む。`quote!` は文位置に直接書く
// instance宣言の記法と違い、常に他のマクロ (ここでは `quote!` 自身) の
// 引数としてしか存在しない値であり、検査の対象外にしてよい。
fn 候補として扱わない名前か(name: &str) -> bool {
    matches!(name, "dynamic_graph_schema" | "graph_schema" | "static_graph_schema" | "quote")
}

// `generated = "文字列";` から始まる呼び出しかを見る (完全な解析はしない)。
// instance候補のschema名が名簿に無いとき、この形で始まっていれば
// 「typoしたinstance」とみなしエラーにし、そうでなければ (`println!` 等)
// 対象外にする。
fn 追跡形式らしいか(tokens: &TokenStream) -> bool {
    let probe = |input: syn::parse::ParseStream| -> syn::Result<bool> {
        let 追跡形式の先頭か = (|| -> syn::Result<()> {
            let ident: syn::Ident = input.parse()?;
            if ident != "generated" {
                return Err(input.error("先頭が generated ではない"));
            }
            input.parse::<syn::Token![=]>()?;
            input.parse::<syn::LitStr>()?;
            input.parse::<syn::Token![;]>()?;
            Ok(())
        })()
        .is_ok();
        // 残りのトークンを読み捨てる (proc-macro-dev スキルの drain_rest と
        // 同じ理由。`Parser::parse2` は末尾に未消費トークンが残ると
        // 無関係な "unexpected token" エラーを返してしまう)。
        let _ = input.parse::<TokenStream>();
        Ok(追跡形式の先頭か)
    };
    syn::parse::Parser::parse2(probe, tokens.clone()).unwrap_or(false)
}
