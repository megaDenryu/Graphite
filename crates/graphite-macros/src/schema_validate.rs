//! `graph_schema!` の意味検査 (パース済み構文木に対する検証)。
//!
//! ここで弾く必要があるのは:
//! - ノード型名の重複宣言
//! - エッジ種別名 (Kind) の重複宣言
//! - エッジの端点 (`from`/`to`) が未宣言のノード型を指している場合
//! - `where each <role>: ..` が有向edgeの始点/終点roleと一致するか
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

use crate::naming::generated_id_ident;
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

/// role getterと既存のEdge APIが同じ値名前空間で衝突しないことを検査する。
pub fn validate_edge_roles(edges: &[EdgeDecl]) -> syn::Result<()> {
    const RESERVED: &[&str] = &[
        "from",
        "to",
        "from_id",
        "to_id",
        "endpoints",
        "payload",
        "of",
        "get_of",
        "sources_of",
        "get_sources_of",
        "get",
        "between",
        "iter",
        "ids",
        "len",
    ];
    for edge in edges {
        for role in edge
            .from_role
            .iter()
            .chain(edge.to_role.iter())
            .chain(edge.attrs_role.iter())
        {
            if RESERVED.contains(&role.to_string().as_str()) {
                return Err(syn::Error::new(
                    role.span(),
                    format!(
                        "役割名 `{role}` は生成されるEdge APIと衝突します。別の役割名を指定してください"
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// schema module の型名前空間へ生成する名前が互いに衝突しないことを検査する。
/// 既定IDと同名の既存IDを使う意図がある場合は `(id: K)` の明示指定を
/// 必須にし、Graphite が暗黙に既存型を拾う余地を残さない。
pub fn validate_generated_type_names(
    schema_name: &Ident,
    nodes: &[NodeDecl],
    edges: &[EdgeDecl],
) -> syn::Result<()> {
    let mut names: HashMap<String, (proc_macro2::Span, String)> = HashMap::new();

    let fixed = ["Graph", "Builder", "Violation"];
    for name in fixed {
        names.insert(
            name.to_string(),
            (schema_name.span(), format!("生成型 `{name}`")),
        );
    }
    for suffix in ["Node", "Edge", "Insertable", "DefaultId"] {
        let name = format!("{schema_name}{suffix}");
        names.insert(
            name.clone(),
            (schema_name.span(), format!("生成trait `{name}`")),
        );
    }

    let mut register = |name: String, span: proc_macro2::Span, description: String| {
        if let Some((previous_span, previous_description)) = names.get(&name) {
            let mut error = syn::Error::new(
                span,
                format!(
                    "schema module内の生成名 `{name}` が衝突します ({description} と {previous_description})。既存ID型を使う場合は `(id: 型)` を明示してください"
                ),
            );
            error.combine(syn::Error::new(*previous_span, "衝突する生成名はこちら"));
            return Err(error);
        }
        names.insert(name, (span, description));
        Ok(())
    };

    for node in nodes {
        register(
            node.name.to_string(),
            node.name.span(),
            format!("ノードマーカー `{}`", node.name),
        )?;
    }
    for edge in edges {
        register(
            edge.kind.to_string(),
            edge.kind.span(),
            format!("エッジ型 `{}`", edge.kind),
        )?;
    }
    for node in nodes.iter().filter(|node| node.id_ty.is_none()) {
        let name = generated_id_ident(&node.name).to_string();
        register(
            name.clone(),
            node.name.span(),
            format!("自動生成ID型 `{name}`"),
        )?;
    }
    for edge in edges.iter().filter(|edge| edge.id_ty.is_none()) {
        let name = generated_id_ident(&edge.kind).to_string();
        register(
            name.clone(),
            edge.kind.span(),
            format!("自動生成ID型 `{name}`"),
        )?;
    }
    for path in nodes
        .iter()
        .filter_map(|node| node.id_ty.as_ref())
        .chain(edges.iter().filter_map(|edge| edge.id_ty.as_ref()))
    {
        validate_explicit_id_name(path, &names)?;
    }

    Ok(())
}

fn validate_explicit_id_name(
    path: &syn::Path,
    generated_names: &HashMap<String, (proc_macro2::Span, String)>,
) -> syn::Result<()> {
    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return Ok(());
    }
    let name = path.segments[0].ident.to_string();
    let Some((generated_span, generated_description)) = generated_names.get(&name) else {
        return Ok(());
    };

    let mut error = syn::Error::new_spanned(
        path,
        format!(
            "明示ID型 `{name}` はschema moduleの{generated_description}と衝突します。親moduleの既存型を使う場合は `super::{name}` のように修飾してください"
        ),
    );
    error.combine(syn::Error::new(*generated_span, "衝突する生成名はこちら"));
    Err(error)
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
/// - `Source`: 始点roleの出次数制約
/// - `Target`: 終点roleの入次数制約
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EachSide {
    Source,
    Target,
}

/// `where each <参照名>: ..` の `<参照名>` がどちら側 (どの制約) を指すかを
/// 解決する。解決できない場合は診断つきの `syn::Error` を返す。
///
/// - 有向edge: `<参照名>` は始点/終点いずれかのrole名と一致する必要がある。
/// - 無向edge: endpoint roleが無いため `each` 自体を拒否する。
pub fn resolve_each_side(edge: &EdgeDecl, each_ident: &Ident) -> syn::Result<EachSide> {
    if !edge.directed {
        return Err(syn::Error::new_spanned(
            each_ident.to_token_stream(),
            format!("無向edge `{}` にはendpoint roleが無いため `each` は使えません。使える制約は `unique pair` のみです", edge.kind),
        ));
    }

    let from_role = edge
        .from_role
        .as_ref()
        .expect("有向edgeの始点roleはparse時に検査済み");
    let to_role = edge
        .to_role
        .as_ref()
        .expect("有向edgeの終点roleはparse時に検査済み");
    let s = each_ident.to_string();
    if s == from_role.to_string() {
        Ok(EachSide::Source)
    } else if s == to_role.to_string() {
        Ok(EachSide::Target)
    } else {
        Err(syn::Error::new_spanned(
            each_ident.to_token_stream(),
            format!(
                "エッジ `{}` の each はendpoint role (`{}`/`{}`) を参照してください。存在しないroleです: `{}`",
                edge.kind, from_role, to_role, s
            ),
        ))
    }
}

/// `where each <参照名>: ..` の意味解決が成功するかを検査する
/// (`resolve_each_side` 参照)。
pub fn validate_each_reference(edges: &[EdgeDecl]) -> syn::Result<()> {
    for edge in edges {
        for (each_ident, _spec) in &edge.constraints.each {
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
pub fn filter_edges_with_known_endpoints(
    nodes: &[NodeDecl],
    edges: Vec<EdgeDecl>,
) -> Vec<EdgeDecl> {
    let declared: HashSet<String> = nodes.iter().map(|n| n.name.to_string()).collect();
    edges
        .into_iter()
        .filter(|edge| {
            declared.contains(&edge.from.to_string()) && declared.contains(&edge.to.to_string())
        })
        .collect()
}
