//! `graph_schema!` の意味検査 (パース済み構文木に対する検証)。
//!
//! ここで弾く必要があるのは:
//! - ノード型名の重複宣言
//! - エッジ種別名 (Kind) の重複宣言
//! - エッジの端点 (`from`/`to`) が未宣言のノード型を指している場合
//! - `where each <参照名>: ..` の `<参照名>` の意味解決 (`docs/schema_v4.md`
//!   §1 / `docs/edge_endpoints_v4_1.md`): 役割名なしの辺では型名が `from` と
//!   一致するか、役割名つきの辺では役割名 (始点/終点いずれか) と一致するか、
//!   無向辺では (両端同型の) 型名と一致するか
//! - 無向辺の両端が同じノード型であること (`docs/edge_endpoints_v4_1.md` §2)
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
use syn::Ident;

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

/// `where each <参照名>` が意味する側 (出次数/入次数/次数)。
///
/// - `Source`: 出次数制約 (役割名なしの辺の従来どおりの意味 / 役割名つきの辺で
///   始点側の役割名を参照した場合)
/// - `Target`: 入次数制約 (役割名つきの辺で終点側の役割名を参照した場合、
///   `docs/edge_endpoints_v4_1.md` §1 の新規解禁項目)
/// - `Degree`: 次数制約 (無向辺、`docs/edge_endpoints_v4_1.md` §2)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EachSide {
    Source,
    Target,
    Degree,
}

/// `where each <参照名>: ..` の `<参照名>` がどちら側 (どの制約) を指すかを
/// 解決する。解決できない場合は診断つきの `syn::Error` を返す。
///
/// - 無向辺: `<参照名>` は (両端同型の) ノード型名と一致しなければならない
///   (次数制約、`docs/edge_endpoints_v4_1.md` §2)。役割名は無向辺には
///   存在しない (パース時点で既に拒否済み)。
/// - 役割名つきの有向辺: `<参照名>` は始点/終点いずれかの役割名と一致しなければ
///   ならない。型名参照はエラー (`docs/edge_endpoints_v4_1.md` §1「型名参照は
///   エラー (同型端点で曖昧なため)」)。
/// - 役割名なしの有向辺: `<参照名>` は始点の型名と一致しなければならない
///   (`docs/schema_v4.md` §1、旧来どおり)。
pub fn resolve_each_side(edge: &EdgeDecl, each_ident: &Ident) -> syn::Result<EachSide> {
    if !edge.directed {
        if each_ident.to_string() == edge.from.to_string() {
            return Ok(EachSide::Degree);
        }
        return Err(syn::Error::new_spanned(
            each_ident.to_token_stream(),
            format!(
                "無向辺 `{}` の each は接続先の型 `{}` を指定してください (次数制約であり、役割名は存在しません)",
                edge.kind, edge.from
            ),
        ));
    }

    match (&edge.from_role, &edge.to_role) {
        (Some(from_role), Some(to_role)) => {
            let s = each_ident.to_string();
            if s == from_role.to_string() {
                Ok(EachSide::Source)
            } else if s == to_role.to_string() {
                Ok(EachSide::Target)
            } else {
                Err(syn::Error::new_spanned(
                    each_ident.to_token_stream(),
                    format!(
                        "役割名つきの辺 `{}` の each は役割名 (`{}`/`{}`) で参照してください。型名参照はできません: `{}`",
                        edge.kind, from_role, to_role, s
                    ),
                ))
            }
        }
        (None, None) => {
            if each_ident.to_string() == edge.from.to_string() {
                Ok(EachSide::Source)
            } else {
                Err(syn::Error::new_spanned(
                    each_ident.to_token_stream(),
                    format!(
                        "`each {}` はエッジ `{}` の始点型 `{}` と一致しません (each は常に始点側の出次数を指定します)",
                        each_ident, edge.kind, edge.from
                    ),
                ))
            }
        }
        _ => unreachable!("役割名は両端同時か両方省略かのいずれかであることをparse時に検査済み"),
    }
}

/// `where each <参照名>: ..` の意味解決が成功するかを検査する
/// (`resolve_each_side` 参照)。
pub fn validate_each_reference(edges: &[EdgeDecl]) -> syn::Result<()> {
    for edge in edges {
        if let Some((each_ident, _spec)) = &edge.constraints.each {
            resolve_each_side(edge, each_ident)?;
        }
    }
    Ok(())
}

/// 無向辺の両端が同じノード型であることを検査する
/// (`docs/edge_endpoints_v4_1.md` §2「両端は同じノード型でなければならない」)。
pub fn validate_undirected_same_type(edges: &[EdgeDecl]) -> syn::Result<()> {
    for edge in edges {
        if !edge.directed && edge.from.to_string() != edge.to.to_string() {
            let mut err = syn::Error::new_spanned(
                edge.to.to_token_stream(),
                format!(
                    "無向辺 `{}` の両端は同じノード型でなければなりません (`{}` != `{}`)。異なる型を対称に繋ぎたい場合は有向辺として書くか、ノードを昇格してください",
                    edge.kind, edge.from, edge.to
                ),
            );
            err.combine(syn::Error::new_spanned(
                edge.from.to_token_stream(),
                "始点側の型はこちら",
            ));
            return Err(err);
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
