//! 識別子の命名規則変換ヘルパー。
//!
//! `graph_schema!` と `graph!` の両方が同じ変換規則に従う必要がある
//! (`graph!` はスキーマの中身を知らずにビルダーメソッド名・属性型名を
//! 機械的に導出するため)。この対応がずれると `graph!` が生成する呼び出しが
//! `graph_schema!` の生成物と噛み合わずコンパイルエラーになる。

use proc_macro2::Ident;
use quote::format_ident;

/// スキーマ module 内のグラフ本体型名を生成する。
pub fn graph_type_ident(source: &Ident) -> Ident {
    Ident::new("Graph", source.span())
}

/// ノード型名・エッジ種別名から既定生成IDの型名を導出する。
pub fn generated_id_ident(source: &Ident) -> Ident {
    format_ident!("{}Id", source, span = source.span())
}

/// ノード型名・辺種別名から完成済みグラフ上の参照型名を導出する。
pub fn reference_ident(source: &Ident) -> Ident {
    format_ident!("{}Ref", source, span = source.span())
}

/// ノード型名・辺種別名から非公開の内部位置型名を導出する。
pub fn internal_position_ident(source: &Ident) -> Ident {
    format_ident!("__{}InternalPosition", source, span = source.span())
}

/// ノード型名・辺種別名から `graph!` の名前付き要素が保持する位置handle型名を導出する。
pub fn named_position_ident(source: &Ident) -> Ident {
    format_ident!("__{}NamedPosition", source, span = source.span())
}

/// `graph!` の左辺名からローカルwrapperの位置フィールド名を導出する。
pub fn named_binding_position_ident(source: &Ident) -> Ident {
    format_ident!("__graphite_named_{}", source, span = source.span())
}

/// 呼び出しsiteに生成する名前付きグラフwrapperのローカル型名。
pub fn named_graph_wrapper_ident(source: &Ident) -> Ident {
    format_ident!("__Graphite{}NamedGraph", source, span = source.span())
}

/// 名前付きグラフwrapperの型付き位置handle用の型引数名。
pub fn named_wrapper_parameter_ident(index: usize, source: &Ident) -> Ident {
    format_ident!("__GraphiteNamedPosition{}", index, span = source.span())
}

/// 辺種別名から凍結後の非公開レコード型名を導出する。
pub fn edge_record_ident(source: &Ident) -> Ident {
    format_ident!("__{}Record", source, span = source.span())
}

/// 辺種別名と端点の役割名から多重度違反variant名を導出する。
pub fn each_violation_ident(kind: &Ident, role: &Ident) -> Ident {
    let role_pascal = role
        .to_string()
        .split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<String>();
    format_ident!("{}{}EachViolation", kind, role_pascal, span = role.span())
}

/// freeze 中に使うエッジ表の一時変数名を生成する。
pub fn edge_storage_ident(accessor: &Ident) -> Ident {
    format_ident!("__graphite_{}", accessor, span = accessor.span())
}

/// `Builder`/`Graph`/名前付き位置型が共通で持つ、構築印 (`u64`) の
/// 非公開フィールド名を返す。構築印は `graph!` の1回の構築を識別する印で、
/// 名前付き位置が生成元と異なる `Graph` へ bind されるのを実行時に検出する
/// ために使う (`crates/graphite/src/lib.rs` の構築印発行関数を参照)。
pub fn construction_stamp_field_ident(span: proc_macro2::Span) -> Ident {
    Ident::new("__graphite_construction_stamp", span)
}

/// ノード型名から非公開ストレージ名を機械的に導出する。
pub fn node_storage_ident(source: &Ident) -> Ident {
    format_ident!(
        "__graphite_node_{}",
        to_snake_case(&source.to_string()),
        span = source.span()
    )
}

/// 辺種別名と役割名から `edge_as_role` 形式のNodeRefメソッド名を導出する。
pub fn traversal_method_ident(kind: &Ident, role: &Ident) -> Ident {
    Ident::new(
        &format!("{}_as_{}", to_snake_case(&kind.to_string()), role),
        role.span(),
    )
}

/// 役割名から辺種別側の `of_role` メソッド名を導出する。
pub fn role_query_method_ident(role: &Ident) -> Ident {
    Ident::new(&format!("of_{role}"), role.span())
}

/// 辺種別名から非公開の対索引フィールド名を導出する。
pub fn pair_index_field_ident(kind: &Ident) -> Ident {
    format_ident!(
        "__graphite_{}_by_pair",
        to_snake_case(&kind.to_string()),
        span = kind.span()
    )
}

/// `PascalCase` / `camelCase` の識別子を `snake_case` に変換する。
///
/// 例: `Employee` -> `employee`, `OrgChart` -> `org_chart`。
/// ノード型名からビルダーメソッド名・アクセサ名を導出するのに使う。
pub fn to_snake_case(ident: &str) -> String {
    let mut result = String::new();
    let mut prev_is_lower_or_digit = false;
    for (i, c) in ident.chars().enumerate() {
        if c.is_uppercase() {
            if i != 0 && prev_is_lower_or_digit {
                result.push('_');
            }
            result.extend(c.to_lowercase());
            prev_is_lower_or_digit = false;
        } else {
            result.push(c);
            prev_is_lower_or_digit = c.is_lowercase() || c.is_numeric();
        }
    }
    result
}

/// ノード型の内部ストレージ用フィールド名 (複数形) を導出する。
///
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_caseへ変換できる() {
        assert_eq!(to_snake_case("Employee"), "employee");
        assert_eq!(to_snake_case("OrgChart"), "org_chart");
        assert_eq!(to_snake_case("belongs_to"), "belongs_to");
    }

    #[test]
    fn 探索名を機械的に導出できる() {
        let kind = Ident::new("関係", proc_macro2::Span::call_site());
        let role = Ident::new("始点", proc_macro2::Span::call_site());
        assert_eq!(
            traversal_method_ident(&kind, &role).to_string(),
            "関係_as_始点"
        );
        assert_eq!(role_query_method_ident(&role).to_string(), "of_始点");
    }

    #[test]
    fn 固定グラフ型名を導出できる() {
        let source = Ident::new("Org", proc_macro2::Span::call_site());
        assert_eq!(graph_type_ident(&source).to_string(), "Graph");
    }

    #[test]
    fn 既定生成id型名を導出できる() {
        let source = Ident::new("Employee", proc_macro2::Span::call_site());
        assert_eq!(generated_id_ident(&source).to_string(), "EmployeeId");
    }

    #[test]
    fn 参照型と内部位置と名前付き位置と辺レコードの型名を導出できる() {
        let source = Ident::new("Purchase", proc_macro2::Span::call_site());
        assert_eq!(reference_ident(&source).to_string(), "PurchaseRef");
        assert_eq!(
            internal_position_ident(&source).to_string(),
            "__PurchaseInternalPosition"
        );
        assert_eq!(
            named_position_ident(&source).to_string(),
            "__PurchaseNamedPosition"
        );
        assert_eq!(edge_record_ident(&source).to_string(), "__PurchaseRecord");
    }

    #[test]
    fn 名前付きwrapperの内部名を導出できる() {
        let source = Ident::new("購入", proc_macro2::Span::call_site());
        assert_eq!(
            named_binding_position_ident(&source).to_string(),
            "__graphite_named_購入"
        );
        assert_eq!(
            named_graph_wrapper_ident(&source).to_string(),
            "__Graphite購入NamedGraph"
        );
        assert_eq!(
            named_wrapper_parameter_ident(2, &source).to_string(),
            "__GraphiteNamedPosition2"
        );
    }

    #[test]
    fn 役割名から多重度違反variant名を導出できる() {
        let kind = Ident::new("Purchase", proc_macro2::Span::call_site());
        let role = Ident::new("line_item", proc_macro2::Span::call_site());
        assert_eq!(
            each_violation_ident(&kind, &role).to_string(),
            "PurchaseLineItemEachViolation"
        );
    }

    #[test]
    fn エッジ表の一時変数名を導出できる() {
        let accessor = Ident::new("belongs_to", proc_macro2::Span::call_site());
        assert_eq!(
            edge_storage_ident(&accessor).to_string(),
            "__graphite_belongs_to"
        );
    }

    #[test]
    fn 構築印フィールド名を導出できる() {
        assert_eq!(
            construction_stamp_field_ident(proc_macro2::Span::call_site()).to_string(),
            "__graphite_construction_stamp"
        );
    }
}
