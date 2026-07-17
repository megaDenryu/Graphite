//! `flow!` のコード生成本体 (`docs/flow_macro.md` 参照)。
//!
//! 項の記述順に `let 束縛名 = (関数式)(始点..);` を並べるだけの、**消去可能な
//! 純粋な脱糖** (即時実行)。`graph!`/`graph_schema!` と異なり、スキーマ・
//! builder・キー付き格納は一切関与しない — `flow!` は「関数適用の連鎖に
//! 名前を与える」という1点だけを担う糖衣であり、生成される `let` 文はユーザー
//! が手で書いたのと全く同じものになる (未使用なら通常どおり `unused
//! variable` 警告が出ることも含めて — 抑制しない)。
//!
//! チェーン形 (`x -[f]-> y -[g]-> z`) の2段目以降は、直前の段で生成した
//! 束縛識別子 (`y`) をそのまま次の呼び出しの引数に使う。fan-in
//! (`(valid, report) -[merge]-> out`) は始点の式列をそのままカンマ区切りの
//! 引数として展開する。所有権 (move 済み値の再利用等) は生成した `let` 文が
//! 素直な Rust コードである以上、rustc の通常の借用検査にそのまま委ねる
//! (`docs/flow_macro.md`: 「正直な脱糖なので Rust の所有権規則がそのまま
//! 見える」)。

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

use crate::flow_dsl::{FlowInput, FlowStmt};

/// 束縛名の重複を検査する (`graph!` の `collect_declared_keys` と同じ
/// 親切さ: 最初の宣言箇所の span を併記する)。`flow!` はスキーマを持たず
/// 「未宣言参照」の検証もしない (`docs/flow_macro.md`: 先行未定義の名前を
/// 使えば rustc の普通のエラーに任せる) ため、意味検査はこの重複チェック
/// だけで足りる。
fn check_unique_bindings(stmts: &[FlowStmt]) -> syn::Result<()> {
    let mut seen: HashMap<String, Span> = HashMap::new();
    for stmt in stmts {
        for step in &stmt.steps {
            let name = step.binding.to_string();
            if let Some(&prev_span) = seen.get(&name) {
                let mut err = syn::Error::new(
                    step.binding.span(),
                    format!("束縛名 `{name}` は既に宣言されています"),
                );
                err.combine(syn::Error::new(prev_span, "最初の宣言はこちら"));
                return Err(err);
            }
            seen.insert(name, step.binding.span());
        }
    }
    Ok(())
}

/// `input.stmts` は既に項単位の回復パース ([`crate::flow_dsl::FlowInput::
/// parse_recovering`]) を経ており、パースに失敗した項は除かれている。
/// ここでは残った項だけから `let` 文の列を生成する。
pub fn generate(input: &FlowInput) -> syn::Result<TokenStream> {
    check_unique_bindings(&input.stmts)?;

    let mut lets: Vec<TokenStream> = Vec::new();
    for stmt in &input.stmts {
        let mut prev_binding: Option<Ident> = None;
        for (i, step) in stmt.steps.iter().enumerate() {
            let func = &step.func;
            let binding = &step.binding;
            let call = if i == 0 {
                let args = &stmt.source;
                quote! { (#func)(#(#args),*) }
            } else {
                let prev = prev_binding
                    .as_ref()
                    .expect("チェーン形の2段目以降は前段の束縛が必ずあるはず");
                quote! { (#func)(#prev) }
            };
            lets.push(quote! {
                let #binding = #call;
            });
            prev_binding = Some(binding.clone());
        }
    }

    Ok(quote! { #(#lets)* })
}
