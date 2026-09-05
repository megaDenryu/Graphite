//! 違反列挙型の variant 名を、ノード型名・辺種別名・役割名から導出する。

use proc_macro2::Ident;
use quote::format_ident;

use super::case_conversion::to_pascal_case;

// ノードのキー重複を表す variant 名を導出する。
pub fn duplicate_node_key_variant_ident(node_type: &Ident) -> Ident {
    format_ident!("Duplicate{}", node_type)
}

// 辺のキー重複を表す variant 名を導出する。
pub fn duplicate_edge_key_variant_ident(kind: &Ident) -> Ident {
    format_ident!("{}DuplicateKey", kind)
}

// 有向辺が未知の始点キーを参照したことを表す variant 名を導出する。
pub fn unknown_source_variant_ident(kind: &Ident) -> Ident {
    format_ident!("{}UnknownSource", kind, span = kind.span())
}

// 有向辺が未知の終点キーを参照したことを表す variant 名を導出する。
pub fn unknown_target_variant_ident(kind: &Ident) -> Ident {
    format_ident!("{}UnknownTarget", kind, span = kind.span())
}

// 無向辺が未知の端点キーを参照したことを表す variant 名を導出する。
// 位置の区別が無いため1種類で足りる。
pub fn unknown_endpoint_variant_ident(kind: &Ident) -> Ident {
    format_ident!("{}UnknownEndpoint", kind, span = kind.span())
}

// 端点対の重複を表す variant 名を導出する。
pub fn unique_pair_violation_variant_ident(kind: &Ident) -> Ident {
    format_ident!("{}UniquePairViolation", kind, span = kind.span())
}

// 辺種別名と端点の役割名から多重度違反variant名を導出する。
pub fn each_violation_ident(kind: &Ident, role: &Ident) -> Ident {
    let role_pascal = to_pascal_case(&role.to_string());
    format_ident!("{}{}EachViolation", kind, role_pascal, span = role.span())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn キー重複と未知端点のvariant名を導出できる() {
        let node = Ident::new("Person", proc_macro2::Span::call_site());
        let kind = Ident::new("Boss", proc_macro2::Span::call_site());
        assert_eq!(
            duplicate_node_key_variant_ident(&node).to_string(),
            "DuplicatePerson"
        );
        assert_eq!(
            duplicate_edge_key_variant_ident(&kind).to_string(),
            "BossDuplicateKey"
        );
        assert_eq!(
            unknown_source_variant_ident(&kind).to_string(),
            "BossUnknownSource"
        );
        assert_eq!(
            unknown_target_variant_ident(&kind).to_string(),
            "BossUnknownTarget"
        );
        assert_eq!(
            unknown_endpoint_variant_ident(&kind).to_string(),
            "BossUnknownEndpoint"
        );
        assert_eq!(
            unique_pair_violation_variant_ident(&kind).to_string(),
            "BossUniquePairViolation"
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
}
