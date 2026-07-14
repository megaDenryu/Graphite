//! `graph!` のコード生成本体。
//!
//! `SchemaName::create(|b| { ... })` の呼び出し列へ脱糖する。`graph!` は
//! スキーマの中身を知らないので、ここで使う名前 (builder メソッド名・
//! newtype キー型名・属性型名) は `graph_schema!` (`schema_codegen.rs`) と
//! 全く同じ命名規則 (`crate::naming`) から機械的に導出する。両者がずれると
//! ここで生成した呼び出しがコンパイルエラーになる (メソッドが見つからない等)。

use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::instance_dsl::{FieldValue, GraphInput, GraphItem};
use crate::naming::{to_pascal_case, to_snake_case};

/// 項目h (フェーズ5): `graph!` 内のノード識別子はノード型を跨いで単一の
/// 平坦な名前空間 (README「名前空間に関する制約」節参照。型ごとに分ける
/// 再設計はフェーズ5では見送った)。この制約下では「同じ識別子を2回ノード
/// 宣言する」ミスが起きやすいため、`HashMap::insert` で黙って上書きする
/// のではなく、2回目の宣言をその場で `syn::Error` として報告する。
/// 最初の宣言の span も添えて「どこが最初か」を示す
/// (`schema_validate.rs::validate_unique_node_names` と同じパターン)。
fn build_key_types(items: &[GraphItem]) -> syn::Result<HashMap<String, Ident>> {
    let mut key_types: HashMap<String, Ident> = HashMap::new();
    let mut key_spans: HashMap<String, proc_macro2::Span> = HashMap::new();

    for item in items {
        if let GraphItem::Node(node) = item {
            let key_str = node.key.to_string();
            if let Some(&prev_span) = key_spans.get(&key_str) {
                let mut err = syn::Error::new(
                    node.key.span(),
                    format!("識別子 `{key_str}` は既に宣言されています"),
                );
                err.combine(syn::Error::new(prev_span, "最初の宣言はこちら"));
                return Err(err);
            }
            key_spans.insert(key_str.clone(), node.key.span());
            key_types.insert(key_str, node.type_name.clone());
        }
    }

    Ok(key_types)
}

pub fn generate(input: &GraphInput) -> syn::Result<TokenStream> {
    let schema_name = &input.schema_name;

    // key (識別子の文字列) -> 宣言時のノード型名。edge が端点の型を逆引きするための表。
    let key_types = build_key_types(&input.items)?;

    let mut calls: Vec<TokenStream> = Vec::new();
    for item in &input.items {
        match item {
            GraphItem::Node(node) => {
                let builder_method = format_ident!("{}", to_snake_case(&node.type_name.to_string()));
                let id_type = format_ident!("{}Id", node.type_name);
                let key_str = node.key.to_string();
                let type_name = &node.type_name;
                let field_tokens = fields_to_tokens(&node.fields);
                calls.push(quote! {
                    b.#builder_method(
                        #id_type(#key_str.to_string()),
                        #type_name { #(#field_tokens),* }
                    );
                });
            }
            GraphItem::Edge(edge) => {
                let from_type = key_types.get(&edge.from.to_string()).ok_or_else(|| {
                    syn::Error::new_spanned(
                        &edge.from,
                        format!(
                            "`{}` はこの graph! 呼び出し内でノードとして宣言されていません",
                            edge.from
                        ),
                    )
                })?;
                let to_type = key_types.get(&edge.to.to_string()).ok_or_else(|| {
                    syn::Error::new_spanned(
                        &edge.to,
                        format!(
                            "`{}` はこの graph! 呼び出し内でノードとして宣言されていません",
                            edge.to
                        ),
                    )
                })?;

                let from_id_type = format_ident!("{}Id", from_type);
                let to_id_type = format_ident!("{}Id", to_type);
                let from_key_str = edge.from.to_string();
                let to_key_str = edge.to.to_string();
                let label = &edge.label;

                let from_expr = quote! { #from_id_type(#from_key_str.to_string()) };
                let to_expr = quote! { #to_id_type(#to_key_str.to_string()) };

                // 項目5 (フェーズ4): `graph_schema!` が生成したハンドシェイク
                // 用マクロを呼び、未知のエッジラベルを親切なメッセージで検出
                // する。`graph!` はスキーマの中身を知らないので、スキーマ名
                // から名前を機械的に導出して呼ぶだけで済む
                // (`schema_codegen.rs::gen_edge_check_macro` 参照)。
                let check_macro = format_ident!("__graphite_check_edge_{}", schema_name);

                match &edge.attrs {
                    None => {
                        calls.push(quote! {
                            #check_macro!(#label);
                            b.#label(#from_expr, #to_expr);
                        });
                    }
                    Some(attr_fields) => {
                        let attrs_type =
                            format_ident!("{}Attrs", to_pascal_case(&label.to_string()));
                        let attr_tokens = fields_to_tokens(attr_fields);
                        calls.push(quote! {
                            #check_macro!(#label);
                            b.#label(
                                #from_expr,
                                #to_expr,
                                #attrs_type { #(#attr_tokens),* }
                            );
                        });
                    }
                }
            }
        }
    }

    Ok(quote! {
        #schema_name::create(|b| {
            #(#calls)*
        })
    })
}

fn fields_to_tokens(fields: &[FieldValue]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let name = &f.name;
            let value = &f.value;
            quote! { #name: #value }
        })
        .collect()
}
