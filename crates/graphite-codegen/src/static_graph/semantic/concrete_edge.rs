// 具体辺 (instanceの `edge` 宣言1件の意味モデル)。端点をIdentのままでなく
// 個体を解決して持ち、役割名もschemaから解決済みのまま持つ (builder.rs で
// 1回だけ解決する)。これにより `file` 側は「種別名で探す」「向きをschemaの
// 辺形状と突き合わせて役割名を取り出す」といった再引き当てを行わず、
// schema::input の型も import しない。
//
// 種別トークン・端点トークンは、instanceに書かれたそのままのIdent (解決済み
// の辺種別・個体とは別に持つ)。DSLトークンの型参照 (`inline/token_type_reference.rs`)
// が、宣言箇所ではなくinstanceでの出現箇所そのもののspanを使うために要る。

use proc_macro2::Ident;
use syn::Expr;

use super::{個体, 辺種別};

#[derive(Clone)]
pub(crate) enum 具体辺形状 {
    有向 {
        始点役割: Ident,
        始点: 個体,
        始点トークン: Ident,
        終点役割: Ident,
        終点: 個体,
        終点トークン: Ident,
        積み荷式: Option<Expr>,
    },
    無向 {
        第1役割: Ident,
        端点1: 個体,
        端点1トークン: Ident,
        第2役割: Ident,
        端点2: 個体,
        端点2トークン: Ident,
        積み荷式: Option<Expr>,
    },
}

#[derive(Clone)]
pub(crate) struct 具体辺 {
    名前: Ident,
    種別: 辺種別,
    種別トークン: Ident,
    形状: 具体辺形状,
}

impl 具体辺 {
    pub(super) fn new(名前: Ident, 種別: 辺種別, 種別トークン: Ident, 形状: 具体辺形状) -> Self {
        Self { 名前, 種別, 種別トークン, 形状 }
    }

    pub(crate) fn 名前(&self) -> &Ident {
        &self.名前
    }

    pub(crate) fn 種別(&self) -> &辺種別 {
        &self.種別
    }

    // instanceに書かれた種別トークンそのもの (schemaの `edge 所属` 宣言の
    // 名前ではなく、instanceの `所属(..)` の出現箇所)。DSLトークンの型参照の
    // span、instanceファイルの型参照 (`naming::reference_paths`) が使う。
    pub(crate) fn 種別トークン(&self) -> &Ident {
        &self.種別トークン
    }

    pub(crate) fn 形状(&self) -> &具体辺形状 {
        &self.形状
    }

    pub(crate) fn 積み荷式(&self) -> Option<&Expr> {
        match &self.形状 {
            具体辺形状::有向 { 積み荷式, .. } => 積み荷式.as_ref(),
            具体辺形状::無向 { 積み荷式, .. } => 積み荷式.as_ref(),
        }
    }

    pub(crate) fn 端点に含むか(&self, 個体名: &Ident) -> bool {
        match &self.形状 {
            具体辺形状::有向 { 始点, 終点, .. } => 始点.名前() == 個体名 || 終点.名前() == 個体名,
            具体辺形状::無向 { 端点1, 端点2, .. } => 端点1.名前() == 個体名 || 端点2.名前() == 個体名,
        }
    }

    // 個体が端点になっているとき、この辺種別上でのその個体の役割名を
    // すべて返す (宣言順)。自己ループ辺 (始点と終点が同じ個体) では
    // 個体が両方の役割を同時に持つため2件返る。`naming::card_names` の
    // 辺アクセサメソッドの意味カードが「この個体の役割」を漏れなく示すために
    // 使う。
    pub(crate) fn 個体の役割一覧(&self, 個体名: &Ident) -> Vec<&Ident> {
        match &self.形状 {
            具体辺形状::有向 { 始点役割, 始点, 終点役割, 終点, .. } => [(始点, 始点役割), (終点, 終点役割)]
                .into_iter()
                .filter(|(端点, _)| 端点.名前() == 個体名)
                .map(|(_, 役割)| 役割)
                .collect(),
            具体辺形状::無向 { 第1役割, 端点1, 第2役割, 端点2, .. } => [(端点1, 第1役割), (端点2, 第2役割)]
                .into_iter()
                .filter(|(端点, _)| 端点.名前() == 個体名)
                .map(|(_, 役割)| 役割)
                .collect(),
        }
    }

    // 両端点のトークン (instance宣言の出現順、宣言箇所 `node 開発部 ..` では
    // なくこの辺の `所属(太郎 -> 開発部)` の出現箇所そのもの)。DSLトークンの
    // 型参照が端点ごとに個体参照型の型参照を1つずつ足すために使う。
    pub(crate) fn 端点トークン列(&self) -> [&Ident; 2] {
        match &self.形状 {
            具体辺形状::有向 { 始点トークン, 終点トークン, .. } => [始点トークン, 終点トークン],
            具体辺形状::無向 { 端点1トークン, 端点2トークン, .. } => [端点1トークン, 端点2トークン],
        }
    }

    // instanceの宣言の形を、意味カードに埋め込める正規化した文字列で返す。
    // 積み荷式は `-[..]->` / `-[..]-` に畳む (design_principles.md 原則5.2、
    // 式の種類には依存させない)。
    pub(crate) fn 宣言の形(&self) -> String {
        let 種別名 = self.種別.名前();
        match &self.形状 {
            具体辺形状::有向 { 始点, 終点, 積み荷式, .. } => {
                let 矢印 = match 積み荷式 {
                    None => "->".to_string(),
                    Some(_) => "-[..]->".to_string(),
                };
                format!("edge {} = {種別名}({} {矢印} {})", self.名前, 始点.名前(), 終点.名前())
            }
            具体辺形状::無向 { 端点1, 端点2, 積み荷式, .. } => {
                let 記号 = match 積み荷式 {
                    None => "--".to_string(),
                    Some(_) => "-[..]-".to_string(),
                };
                format!("edge {} = {種別名}({} {記号} {})", self.名前, 端点1.名前(), 端点2.名前())
            }
        }
    }
}
