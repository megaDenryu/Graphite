//! schema 名から導出する、schema module 内に1つずつだけ生成される名前を持つ。

use proc_macro2::{Ident, Span};
use quote::format_ident;

// スキーマ module 内のグラフ本体型名を生成する。
pub fn graph_type_ident(source: &Ident) -> Ident {
    Ident::new("Graph", source.span())
}

// スキーマ module 内の構築器型名を生成する。
pub fn builder_type_ident(schema_name: &Ident) -> Ident {
    format_ident!("Builder", span = schema_name.span())
}

// スキーマ module 内の違反列挙型名を生成する。
pub fn violation_type_ident(schema_name: &Ident) -> Ident {
    format_ident!("Violation", span = schema_name.span())
}

// ノード挿入トレイト名を導出する。`graph!` が値の型名を一切知らずに済ませる
// ための境界であり、名前を schema ごとに一意にして生成 module 内の
// `node Node;` のような宣言と衝突する余地を減らす
// (`schema::codegen::insertable_trait::marker_traits::gen_node_trait_and_impls`
// のコメント参照)。
pub fn node_trait_ident(schema_name: &Ident) -> Ident {
    format_ident!("{}Node", schema_name)
}

// 辺挿入トレイト名を導出する。ノード挿入トレイトと同じ理由で生やす
// (書き込み側専用。読み取り側は各辺種別型への固有 impl なのでトレイトを
// 介さない)。
pub fn edge_trait_ident(schema_name: &Ident) -> Ident {
    format_ident!("{}Edge", schema_name)
}

// ノード用・辺用の挿入トレイトを単一の `extend` へ橋渡しする共通 supertrait 名を
// 導出する (`schema::codegen::insertable_trait::trait_definition::gen_insertable_traits`
// のコメントと `docs/graph_splice.md` §2 参照)。
pub fn insertable_trait_ident(schema_name: &Ident) -> Ident {
    format_ident!("{}Insertable", schema_name)
}

// 束縛名の文字列から既定IDを作れる要素だけが実装するトレイト名を導出する。
pub fn default_id_trait_ident(schema_name: &Ident) -> Ident {
    format_ident!("{}DefaultId", schema_name)
}

// `Builder`/`Graph`/名前付き位置型が共通で持つ、構築印 (`u64`) の
// 非公開フィールド名を返す。構築印は `graph!` の1回の構築を識別する印で、
// 名前付き位置が生成元と異なる `Graph` へ bind されるのを実行時に検出する
// ために使う (`crates/graphite/src/schema_runtime/construction_stamp.rs` の
// 構築印発行関数を参照)。
pub fn construction_stamp_field_ident(span: Span) -> Ident {
    Ident::new("__graphite_construction_stamp", span)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 固定グラフ型名を導出できる() {
        let source = Ident::new("Org", Span::call_site());
        assert_eq!(graph_type_ident(&source).to_string(), "Graph");
        assert_eq!(builder_type_ident(&source).to_string(), "Builder");
        assert_eq!(violation_type_ident(&source).to_string(), "Violation");
    }

    #[test]
    fn schema名つきのトレイト名を導出できる() {
        let source = Ident::new("Org", Span::call_site());
        assert_eq!(node_trait_ident(&source).to_string(), "OrgNode");
        assert_eq!(edge_trait_ident(&source).to_string(), "OrgEdge");
        assert_eq!(insertable_trait_ident(&source).to_string(), "OrgInsertable");
        assert_eq!(default_id_trait_ident(&source).to_string(), "OrgDefaultId");
    }

    #[test]
    fn 構築印フィールド名を導出できる() {
        assert_eq!(
            construction_stamp_field_ident(Span::call_site()).to_string(),
            "__graphite_construction_stamp"
        );
    }
}
