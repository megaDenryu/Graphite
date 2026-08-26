//! スキーマ1つ分の意味モデル全体を所有し、添字ハンドルからの取り出しを提供する。

use proc_macro2::Ident;

use super::edge_definition::辺定義;
use super::node_definition::ノード定義;

/// `schema Name { .. }` 1つ分の確定した意味。コード生成層はこの値だけを読む。
pub struct スキーマ定義 {
    スキーマ名: Ident,
    ノード定義の列: Vec<ノード定義>,
    辺定義の列: Vec<辺定義>,
}

impl スキーマ定義 {
    pub(super) fn 定義の列から作る(
        スキーマ名: Ident,
        ノード定義の列: Vec<ノード定義>,
        辺定義の列: Vec<辺定義>,
    ) -> Self {
        Self {
            スキーマ名,
            ノード定義の列,
            辺定義の列,
        }
    }

    pub fn スキーマ名(&self) -> &Ident {
        &self.スキーマ名
    }

    /// ノード定義を宣言順で返す。添字は [`ノード定義番号`] と一致する。
    pub fn ノード定義の列(&self) -> &[ノード定義] {
        &self.ノード定義の列
    }

    /// 辺定義を宣言順で返す。
    pub fn 辺定義の列(&self) -> &[辺定義] {
        &self.辺定義の列
    }
}
