//! schema module 内に生成する型名が互いに衝突しないことを検査する。
//!
//! このファイルは1ファイル100行の原則の例外である。衝突検査は「1つの表へ
//! 全ての生成名を登録し、重複したらその場で診断を出す」という1本の手続きで
//! あり、登録する種類ごとに切ると表の共有が壊れるため、まとめて置いている。

use std::collections::HashMap;

use syn::Ident;

use crate::naming::{
    generated_id_ident, named_position_ident, reference_ident, 固定生成名の予約表,
};
use crate::schema::syntax::{EdgeDecl, NodeDecl};

/// 生成名衝突を報告するときに添える解決の助言。衝突した2つの生成名の
/// どちらが「今まさに登録しようとしている側」かによって、有効な解決策が
/// 変わるため種別として持つ。
///
/// - `既存id型を明示`: 自動生成ID型 (`{ノード名}Id` 等) との衝突。ユーザーが
///   既存のID型を使い回したい意図であることが多いため `(id: 型)` の明示を促す。
/// - `ノードまたは辺の名前を変更`: ノード名・エッジ型・参照型
///   (`{ノード名}Ref` 等) との衝突。これらの生成名はノード名/辺名から機械的に
///   導出されるため、`(id: 型)` を明示しても衝突は解消できない。宣言名自体の
///   変更を促す。
#[derive(Clone, Copy)]
pub(super) enum 生成名衝突助言 {
    既存id型を明示,
    ノードまたは辺の名前を変更,
}

impl 生成名衝突助言 {
    fn 文言(self) -> &'static str {
        match self {
            Self::既存id型を明示 => "既存ID型を使う場合は `(id: 型)` を明示してください",
            Self::ノードまたは辺の名前を変更 => "ノード名・辺名を変更してください",
        }
    }
}

/// schema module 内に生成する型名が互いに衝突しないことを検査する。
/// 既定IDと同名の既存IDを使う意図がある場合は `(id: K)` の明示指定を
/// 必須にし、Graphite が暗黙に既存型を拾う余地を残さない。
pub fn validate_generated_type_names(
    schema_name: &Ident,
    nodes: &[NodeDecl],
    edges: &[EdgeDecl],
) -> syn::Result<()> {
    let mut names: HashMap<String, (proc_macro2::Span, String)> = HashMap::new();

    // 予約表は生成側 (`schema::codegen::generate_module_body`) と共有する。
    // 検査だけが知る文字列表を持つと、固定生成名を増やしたときに登録漏れが出る。
    let 予約表 = 固定生成名の予約表::schema名から導出する(schema_name);
    for (生成名, 説明) in 予約表.衝突検査へ登録する項目() {
        names.insert(生成名, (schema_name.span(), 説明));
    }

    let mut register = |name: String,
                        span: proc_macro2::Span,
                        description: String,
                        advice: 生成名衝突助言| {
        if let Some((previous_span, previous_description)) = names.get(&name) {
            let mut error = syn::Error::new(
                span,
                format!(
                    "schema module内の生成名 `{name}` が衝突します ({description} と {previous_description})。{}",
                    advice.文言()
                ),
            );
            error.combine(syn::Error::new(*previous_span, "衝突する生成名はこちら"));
            return Err(error);
        }
        names.insert(name, (span, description));
        Ok(())
    };

    // ノード名は schema module 内で型としては生成されないが、
    // `{name}Id`/`{name}Ref` と `Graph` の種別メソッド名の由来であるため、
    // 名前の重複はここで検査する (登録を外すと `node Graph;` のような
    // 宣言が検査を通ってしまう)。
    for node in nodes {
        register(
            node.name.to_string(),
            node.name.span(),
            format!(
                "ノード名 `{}` (`{}Id`/`{}Ref` と Graph の種別メソッドの由来)",
                node.name, node.name, node.name
            ),
            生成名衝突助言::ノードまたは辺の名前を変更,
        )?;
    }
    for edge in edges {
        register(
            edge.kind.to_string(),
            edge.kind.span(),
            format!("エッジ型 `{}`", edge.kind),
            生成名衝突助言::ノードまたは辺の名前を変更,
        )?;
    }
    for node in nodes {
        let name = reference_ident(&node.name).to_string();
        register(
            name.clone(),
            node.name.span(),
            format!("参照型 `{name}`"),
            生成名衝突助言::ノードまたは辺の名前を変更,
        )?;
    }
    for edge in edges {
        let name = reference_ident(&edge.kind).to_string();
        register(
            name.clone(),
            edge.kind.span(),
            format!("参照型 `{name}`"),
            生成名衝突助言::ノードまたは辺の名前を変更,
        )?;
    }
    for node in nodes {
        let name = named_position_ident(&node.name).to_string();
        register(
            name.clone(),
            node.name.span(),
            format!("名前付き位置型 `{name}`"),
            生成名衝突助言::ノードまたは辺の名前を変更,
        )?;
    }
    for edge in edges {
        let name = named_position_ident(&edge.kind).to_string();
        register(
            name.clone(),
            edge.kind.span(),
            format!("名前付き位置型 `{name}`"),
            生成名衝突助言::ノードまたは辺の名前を変更,
        )?;
    }
    for node in nodes.iter().filter(|node| node.id_ty.is_none()) {
        let name = generated_id_ident(&node.name).to_string();
        register(
            name.clone(),
            node.name.span(),
            format!("自動生成ID型 `{name}`"),
            生成名衝突助言::既存id型を明示,
        )?;
    }
    for edge in edges.iter().filter(|edge| edge.id_ty.is_none()) {
        let name = generated_id_ident(&edge.kind).to_string();
        register(
            name.clone(),
            edge.kind.span(),
            format!("自動生成ID型 `{name}`"),
            生成名衝突助言::既存id型を明示,
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

pub(super) fn validate_explicit_id_name(
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
