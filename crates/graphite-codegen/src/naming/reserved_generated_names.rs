//! schema 名だけから決まる固定生成名の予約表を持ち、衝突検査と生成が同じ表を読む。
//!
//! ノード型名・辺種別名から派生する生成名 (`{名前}Id`・`{名前}Ref` 等) はこの表に
//! 含めない。それらは要素ごとに増える名前であり、
//! [`super::element_names`] の関数から衝突検査が個別に導出する。
//! 内部専用の生成名 (`__{名前}InternalPosition`・`__{辺種別}Record`・
//! `__graphite_{辺種別}_by_pair`) も含めない。現行の衝突検査が対象にしていない
//! 範囲であり、含めると検査が受理する schema の集合が変わる。

use proc_macro2::Ident;

use super::schema_fixed_names::{
    builder_type_ident, default_id_trait_ident, edge_trait_ident, graph_type_ident,
    insertable_trait_ident, node_trait_ident, violation_type_ident,
};

/// schema module へ必ず1つずつ生成される固定名の一覧。
///
/// 生成側はこの表からトークンを取り出し、衝突検査側は
/// [`Self::衝突検査へ登録する項目`] が返す文字列を予約語として登録する。
/// 両者が同じ表を読むため、固定名を増やしたときに検査側の登録漏れが起きない。
pub struct 固定生成名の予約表 {
    グラフ型名: Ident,
    構築器型名: Ident,
    違反列挙型名: Ident,
    ノード挿入トレイト名: Ident,
    辺挿入トレイト名: Ident,
    挿入可能トレイト名: Ident,
    既定id生成トレイト名: Ident,
}

impl 固定生成名の予約表 {
    pub fn schema名から導出する(schema_name: &Ident) -> Self {
        Self {
            グラフ型名: graph_type_ident(schema_name),
            構築器型名: builder_type_ident(schema_name),
            違反列挙型名: violation_type_ident(schema_name),
            ノード挿入トレイト名: node_trait_ident(schema_name),
            辺挿入トレイト名: edge_trait_ident(schema_name),
            挿入可能トレイト名: insertable_trait_ident(schema_name),
            既定id生成トレイト名: default_id_trait_ident(schema_name),
        }
    }

    pub fn グラフ型名(&self) -> &Ident {
        &self.グラフ型名
    }

    pub fn 構築器型名(&self) -> &Ident {
        &self.構築器型名
    }

    pub fn 違反列挙型名(&self) -> &Ident {
        &self.違反列挙型名
    }

    pub fn ノード挿入トレイト名(&self) -> &Ident {
        &self.ノード挿入トレイト名
    }

    pub fn 辺挿入トレイト名(&self) -> &Ident {
        &self.辺挿入トレイト名
    }

    pub fn 挿入可能トレイト名(&self) -> &Ident {
        &self.挿入可能トレイト名
    }

    pub fn 既定id生成トレイト名(&self) -> &Ident {
        &self.既定id生成トレイト名
    }

    /// 衝突検査へ予約語として登録する (生成名, 診断文へ書く説明) の列を返す。
    pub fn 衝突検査へ登録する項目(&self) -> Vec<(String, String)> {
        let 生成型 = [&self.グラフ型名, &self.構築器型名, &self.違反列挙型名];
        let 生成トレイト = [
            &self.ノード挿入トレイト名,
            &self.辺挿入トレイト名,
            &self.挿入可能トレイト名,
            &self.既定id生成トレイト名,
        ];
        生成型
            .into_iter()
            .map(|名前| (名前.to_string(), format!("生成型 `{名前}`")))
            .chain(
                生成トレイト
                    .into_iter()
                    .map(|名前| (名前.to_string(), format!("生成trait `{名前}`"))),
            )
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 固定生成名を衝突検査へ登録する形で列挙できる() {
        let schema名 = Ident::new("Org", proc_macro2::Span::call_site());
        let 予約表 = 固定生成名の予約表::schema名から導出する(&schema名);
        let 項目 = 予約表.衝突検査へ登録する項目();
        assert_eq!(項目.len(), 7);
        assert_eq!(項目[0], ("Graph".to_string(), "生成型 `Graph`".to_string()));
        assert_eq!(
            項目[3],
            ("OrgNode".to_string(), "生成trait `OrgNode`".to_string())
        );
    }
}
