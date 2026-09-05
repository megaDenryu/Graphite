//! ノード表・辺表とその索引を保持する非公開フィールドの名前を持つ。

use proc_macro2::Ident;
use quote::format_ident;

use super::case_conversion::to_snake_case;

// ノード型名・辺種別名から、内部ストレージのアクセサ名 (単数形 snake_case) を
// 導出する。builder の追加メソッド名と辺表のフィールド名を兼ねる。
pub fn accessor_ident(source: &Ident) -> Ident {
    Ident::new(&to_snake_case(&source.to_string()), source.span())
}

// ノード型名から非公開ストレージ名を機械的に導出する。
pub fn node_storage_ident(source: &Ident) -> Ident {
    format_ident!(
        "__graphite_node_{}",
        to_snake_case(&source.to_string()),
        span = source.span()
    )
}

// freeze 中に使うエッジ表の一時変数名を生成する。
pub fn edge_storage_ident(accessor: &Ident) -> Ident {
    format_ident!("__graphite_{}", accessor, span = accessor.span())
}

// 有向辺の始点役割索引のフィールド名を導出する。
pub fn source_role_index_field_ident(accessor: &Ident) -> Ident {
    format_ident!("{}_from_index", accessor)
}

// 有向辺の終点役割索引のフィールド名を導出する (`docs/reverse_query.md`)。
pub fn target_role_index_field_ident(accessor: &Ident) -> Ident {
    format_ident!("{}_to_index", accessor)
}

// 無向辺の接続索引のフィールド名を導出する。無向辺は方向の意味を持たないため
// 位置を名前に書かない。
pub fn incident_index_field_ident(accessor: &Ident) -> Ident {
    format_ident!("{}_index", accessor)
}

// 辺種別名から非公開の端点対索引フィールド名を導出する。
pub fn pair_index_field_ident(kind: &Ident) -> Ident {
    format_ident!(
        "__graphite_{}_by_pair",
        to_snake_case(&kind.to_string()),
        span = kind.span()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ノード表と辺表のフィールド名を導出できる() {
        let source = Ident::new("OrgChart", proc_macro2::Span::call_site());
        assert_eq!(accessor_ident(&source).to_string(), "org_chart");
        assert_eq!(
            node_storage_ident(&source).to_string(),
            "__graphite_node_org_chart"
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
    fn 役割索引と端点対索引のフィールド名を導出できる() {
        let accessor = Ident::new("boss", proc_macro2::Span::call_site());
        assert_eq!(
            source_role_index_field_ident(&accessor).to_string(),
            "boss_from_index"
        );
        assert_eq!(
            target_role_index_field_ident(&accessor).to_string(),
            "boss_to_index"
        );
        assert_eq!(
            incident_index_field_ident(&accessor).to_string(),
            "boss_index"
        );
        let kind = Ident::new("Friends", proc_macro2::Span::call_site());
        assert_eq!(
            pair_index_field_ident(&kind).to_string(),
            "__graphite_friends_by_pair"
        );
    }
}
