//! `graph!` が呼び出し箇所へ生成するローカルな名前を持つ。

use proc_macro2::Ident;
use quote::format_ident;

/// `graph!` の左辺名からローカルラッパーの位置フィールド名を導出する。
pub fn named_binding_position_ident(source: &Ident) -> Ident {
    format_ident!("__graphite_named_{}", source, span = source.span())
}

/// 呼び出し箇所に生成する名前付きグラフラッパーのローカル型名。
pub fn named_graph_wrapper_ident(source: &Ident) -> Ident {
    format_ident!("__Graphite{}NamedGraph", source, span = source.span())
}

/// 名前付きグラフラッパーの型付き位置型用の型引数名。
pub fn named_wrapper_parameter_ident(index: usize, source: &Ident) -> Ident {
    format_ident!("__GraphiteNamedPosition{}", index, span = source.span())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
