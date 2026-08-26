//! `Graph`・`NodeRef` へ生やす公開メソッドの名前を持つ。

use proc_macro2::Ident;

use super::case_conversion::to_snake_case;

/// ノード型名・辺種別名の snake_case 形 (`accessor`) から、`Graph` および
/// `NodeRef` に生やす種別APIのメソッド名を導出する。
///
/// 種別APIとは、ある種別に属する個体の全体を対象にする読み取り・可変操作
/// (`by_id` / `value_mut` / `payload_mut` / `ids` / `iter` / `len` /
/// `between` / `try_between`) のことである。接尾辞は固定の英語であり、
/// 自然言語の複数形・省略形は生成しない (`bosses()` のような暗黙の複数形を
/// 作らない、という issue #9 の決定)。日本語スキーマでも同じ機械的連結で
/// `人物_by_id` / `購入_try_between` になる。
pub fn kind_api_method_ident(accessor: &Ident, suffix: &str) -> Ident {
    Ident::new(&format!("{accessor}_{suffix}"), accessor.span())
}

/// 辺種別名と役割名から `edge_as_role` 形式のNodeRefメソッド名を導出する。
pub fn traversal_method_ident(kind: &Ident, role: &Ident) -> Ident {
    Ident::new(
        &format!("{}_as_{}", to_snake_case(&kind.to_string()), role),
        role.span(),
    )
}

/// 辺種別名から無向辺の NodeRef が持つ接続クエリメソッド名を導出する。
pub fn incident_method_ident(kind: &Ident) -> Ident {
    Ident::new(
        &format!("{}_incident", to_snake_case(&kind.to_string())),
        kind.span(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 探索名を機械的に導出できる() {
        let kind = Ident::new("関係", proc_macro2::Span::call_site());
        let role = Ident::new("始点", proc_macro2::Span::call_site());
        assert_eq!(
            traversal_method_ident(&kind, &role).to_string(),
            "関係_as_始点"
        );
    }

    #[test]
    fn 種別apiのメソッド名を機械的に導出できる() {
        let accessor = Ident::new("employee", proc_macro2::Span::call_site());
        assert_eq!(
            kind_api_method_ident(&accessor, "by_id").to_string(),
            "employee_by_id"
        );
        assert_eq!(
            kind_api_method_ident(&accessor, "value_mut").to_string(),
            "employee_value_mut"
        );
        assert_eq!(
            kind_api_method_ident(&accessor, "len").to_string(),
            "employee_len"
        );
    }

    #[test]
    fn 日本語スキーマでも種別apiのメソッド名を導出できる() {
        let 人物 = Ident::new("人物", proc_macro2::Span::call_site());
        assert_eq!(
            kind_api_method_ident(&人物, "by_id").to_string(),
            "人物_by_id"
        );
        let 購入 = Ident::new("購入", proc_macro2::Span::call_site());
        assert_eq!(
            kind_api_method_ident(&購入, "try_between").to_string(),
            "購入_try_between"
        );
    }

    #[test]
    fn 無向辺の接続クエリメソッド名を導出できる() {
        let kind = Ident::new("Friends", proc_macro2::Span::call_site());
        assert_eq!(incident_method_ident(&kind).to_string(), "friends_incident");
    }
}
