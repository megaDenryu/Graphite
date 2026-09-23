// 辺種別 (schemaの `edge` 宣言1件の意味モデル)。schema::input::辺宣言 を
// そのまま持つのではなく、この層独自の型として持つ。向き・役割の解決は
// `具体辺` (concrete_edge.rs) が構築時に1回だけ行い、`file::instance_file`・
// `file` の辺の配線・ロールアクセサはそこで解決済みの役割を
// 読むだけで、schema::input::辺形状 を突き合わせ直さない
// (static_graph/mod.rs の層図参照)。schemaの構造そのものを組み立てる
// `file::schema_file` は、この型が包む `形状()` を引き続き読む (schemaだけ
// から決まる生成物であり、instanceとの突き合わせを行わないため境界の対象
// 外)。

use proc_macro2::Ident;

use crate::static_graph::schema::input::{
    積み荷宣言, 制約, 多重度範囲, 辺形状, 辺宣言 as schema辺宣言,
};

#[derive(Clone)]
pub(crate) struct 辺種別 {
    名前: Ident,
    形状: 辺形状,
    制約列: Vec<制約>,
}

impl 辺種別 {
    pub(super) fn schemaから作る(宣言: &schema辺宣言) -> Self {
        Self { 名前: 宣言.名前.clone(), 形状: 宣言.形状.clone(), 制約列: 宣言.制約達.clone() }
    }

    pub(crate) fn 名前(&self) -> &Ident {
        &self.名前
    }

    pub(crate) fn 形状(&self) -> &辺形状 {
        &self.形状
    }

    pub(crate) fn 積み荷(&self) -> Option<&積み荷宣言> {
        self.形状.積み荷()
    }

    // schema宣言の形を、意味カードに埋め込める正規化した文字列で返す。
    // where節は `each <役割>: <範囲>` / `unique pair` を `, ` で連ねる。
    pub(crate) fn 宣言の形(&self) -> String {
        let 端点達 = match &self.形状 {
            辺形状::有向 { 始点役割, 始点型, 積み荷, 終点役割, 終点型 } => match 積み荷 {
                None => format!("({始点役割}: {始点型}) -> ({終点役割}: {終点型})"),
                Some(積み荷宣言 { 役割, 型 }) => {
                    format!("({始点役割}: {始点型}) -[{役割}: {型}]-> ({終点役割}: {終点型})")
                }
            },
            辺形状::無向 { 第1役割, 第1型, 積み荷, 第2役割, 第2型 } => match 積み荷 {
                None => format!("({第1役割}: {第1型}) -- ({第2役割}: {第2型})"),
                Some(積み荷宣言 { 役割, 型 }) => {
                    format!("({第1役割}: {第1型}) -[{役割}: {型}]- ({第2役割}: {第2型})")
                }
            },
        };
        match self.制約列を表示する() {
            Some(where節) => format!("edge {} = {端点達} where {where節}", self.名前),
            None => format!("edge {} = {端点達}", self.名前),
        }
    }

    // `each <役割>: <範囲>` / `unique pair` を `, ` で連ねたもの。意味カード
    // の「検証制約」項目 (`static_graph::naming::card_names`) が読む。
    pub(crate) fn 制約列を表示する(&self) -> Option<String> {
        if self.制約列.is_empty() {
            return None;
        }
        Some(self.制約列.iter().map(制約を表示する).collect::<Vec<_>>().join(", "))
    }
}

fn 制約を表示する(制約: &制約) -> String {
    match 制約 {
        制約::多重度 { 役割, 範囲 } => format!("each {役割}: {}", 範囲を表示する(*範囲)),
        制約::対一意 => "unique pair".to_string(),
    }
}

fn 範囲を表示する(範囲: 多重度範囲) -> String {
    match 範囲.上限() {
        Some(上限) if 上限 == 範囲.下限() => 範囲.下限().to_string(),
        Some(上限) => format!("{}..{上限}", 範囲.下限()),
        None => format!("{}..*", 範囲.下限()),
    }
}
