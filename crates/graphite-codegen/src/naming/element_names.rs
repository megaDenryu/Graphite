//! ノード型名・辺種別名から導出する、schema module 内の生成型名を持つ。

use proc_macro2::Ident;
use quote::format_ident;

// ノード型名・エッジ種別名から既定生成IDの型名を導出する。
pub fn generated_id_ident(source: &Ident) -> Ident {
    format_ident!("{}Id", source, span = source.span())
}

// ノード型名・辺種別名から完成済みグラフ上の参照型名を導出する。
pub fn reference_ident(source: &Ident) -> Ident {
    format_ident!("{}Ref", source, span = source.span())
}

// ノード型名・辺種別名から非公開の内部位置型名を導出する。
pub fn internal_position_ident(source: &Ident) -> Ident {
    format_ident!("__{}InternalPosition", source, span = source.span())
}

// ノード型名・辺種別名から `graph!` の名前付き要素が保持する位置型名を導出する。
pub fn named_position_ident(source: &Ident) -> Ident {
    format_ident!("__{}NamedPosition", source, span = source.span())
}

// 辺種別名から凍結後の非公開レコード型名を導出する。
pub fn edge_record_ident(source: &Ident) -> Ident {
    format_ident!("__{}Record", source, span = source.span())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
