//! `graph_schema!` の意味検査 (パース済み構文木に対する検証)。
//!
//! ここで弾く必要があるのは:
//! - ノード型名の重複宣言
//! - エッジ種別名 (Kind) の重複宣言
//! - エッジの端点 (`from`/`to`) が未宣言のノード型を指している場合
//! - `where each <FromType>: ..` の `<FromType>` がエッジの `from` と
//!   一致しない場合 (`docs/schema_v4.md` §1)
//!
//! いずれも `syn::Error::new_spanned`/`syn::Error::new` で元トークンの span を
//! 保ったまま返す (`.claude/skills/proc-macro-dev/SKILL.md` の方針通り、
//! panic は使わない)。
//!
//! ## G4a (宣言単位のエラー回復) との関係
//!
//! `SchemaInput` 全体ではなく `&[NodeDecl]`/`&[EdgeDecl]` というスライスを
//! 受け取るシグネチャにしているのは、`lib.rs` 側がパース回復で「壊れた宣言を
//! 除いた残り」だけを検証にかけられるようにするため。特に
//! `validate_edge_endpoints`/`validate_each_type_matches_from` は、パース済みの
//! 宣言が1件でも壊れていた場合に `lib.rs` が直接は呼ばず、代わりに
//! [`filter_edges_with_known_endpoints`] で未知端点のエッジを黙って除外する
//! (二次エラー抑制)。重複ノード名・重複エッジ種別名の診断は回復の有無に
//! よらず常に実行する (現行維持)。

use std::collections::{HashMap, HashSet};

use quote::ToTokens;

use crate::schema_dsl::{EdgeDecl, NodeDecl};

pub fn validate_unique_node_names(nodes: &[NodeDecl]) -> syn::Result<()> {
    let mut seen: HashMap<String, proc_macro2::Span> = HashMap::new();
    for node in nodes {
        let name = node.name.to_string();
        if let Some(&prev_span) = seen.get(&name) {
            let mut err = syn::Error::new(
                node.name.span(),
                format!("ノード型 `{name}` が重複して宣言されています"),
            );
            err.combine(syn::Error::new(prev_span, "最初の宣言はこちら"));
            return Err(err);
        }
        seen.insert(name, node.name.span());
    }
    Ok(())
}

pub fn validate_unique_edge_kinds(edges: &[EdgeDecl]) -> syn::Result<()> {
    let mut seen: HashMap<String, proc_macro2::Span> = HashMap::new();
    for edge in edges {
        let name = edge.kind.to_string();
        if let Some(&prev_span) = seen.get(&name) {
            let mut err = syn::Error::new(
                edge.kind.span(),
                format!("エッジ種別 `{name}` が重複して宣言されています"),
            );
            err.combine(syn::Error::new(prev_span, "最初の宣言はこちら"));
            return Err(err);
        }
        seen.insert(name, edge.kind.span());
    }
    Ok(())
}

pub fn validate_edge_endpoints(nodes: &[NodeDecl], edges: &[EdgeDecl]) -> syn::Result<()> {
    let declared: Vec<String> = nodes.iter().map(|n| n.name.to_string()).collect();
    let declared_set: HashSet<&str> = declared.iter().map(|s| s.as_str()).collect();

    for edge in edges {
        for endpoint in [&edge.from, &edge.to] {
            if !declared_set.contains(endpoint.to_string().as_str()) {
                return Err(syn::Error::new_spanned(
                    endpoint.to_token_stream(),
                    format!(
                        "エッジ `{}` の端点 `{}` は宣言されていないノード型です。宣言済みノード一覧: [{}]",
                        edge.kind,
                        endpoint,
                        declared.join(", ")
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// `where each <FromType>: ..` の `<FromType>` がエッジの `from` と一致するかを
/// 検査する (`docs/schema_v4.md` §1「`<FromType>` は始点の型名と一致しなければ
/// ならない」)。
pub fn validate_each_type_matches_from(edges: &[EdgeDecl]) -> syn::Result<()> {
    for edge in edges {
        if let Some((from_type, _spec)) = &edge.constraints.each {
            if from_type.to_string() != edge.from.to_string() {
                return Err(syn::Error::new_spanned(
                    from_type.to_token_stream(),
                    format!(
                        "`each {}` はエッジ `{}` の始点型 `{}` と一致しません (each は常に始点側の出次数を指定します)",
                        from_type, edge.kind, edge.from
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// G4a (二次エラーの抑制): パース回復により1件以上の壊れた宣言があった
/// ときに、`lib.rs` が [`validate_edge_endpoints`] の代わりに呼ぶ。
/// 端点が未宣言のノード型を指すエッジをエラーにはせず、黙って生成対象から
/// 除外する。壊れたノード宣言をたまたま参照しているだけの可能性が高く、
/// そのまま `validate_edge_endpoints` を呼ぶと「壊れた宣言由来の
/// compile_error!」1件のはずが「未知端点エラー」まで重ねて出てしまう
/// (二次噴出) ため。
pub fn filter_edges_with_known_endpoints(nodes: &[NodeDecl], edges: Vec<EdgeDecl>) -> Vec<EdgeDecl> {
    let declared: HashSet<String> = nodes.iter().map(|n| n.name.to_string()).collect();
    edges
        .into_iter()
        .filter(|edge| {
            declared.contains(&edge.from.to_string()) && declared.contains(&edge.to.to_string())
        })
        .collect()
}
