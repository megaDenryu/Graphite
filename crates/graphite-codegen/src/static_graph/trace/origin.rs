// 生成名の由来。issue #41 が要求する「由来追跡」の材料であり、公開生成物の
// 分類 (A: 利用者語彙由来 / B: Graphiteの固定語彙) はここから導く
// (フィールドとして別に持たない)。span はF12の着地点に使う元トークンの位置
// (`Span::call_site()` ではなく利用者が書いたトークンの位置) を返す。

use proc_macro2::{Ident, Span};

use super::fixed_vocabulary_kind::固定語彙;

#[derive(Clone)]
pub(crate) enum 名前の由来 {
    SchemaEdge { 種別名: Ident },
    SchemaRole { #[allow(dead_code)] 種別名: Ident, 役割: Ident },
    SchemaPayloadRole { #[allow(dead_code)] 種別名: Ident, 積み荷役割: Ident },
    InstanceNode { 個体名: Ident },
    InstanceEdge { 辺名: Ident },
    // 内側の `固定語彙` は現状spanの計算には使わず (`Span::call_site()`
    // 固定) 読み戻していないが、A/B分類のB側がどの固定語彙かを保持する
    // ための正当なデータであり、将来の意味カード生成の選択肢を保つ
    // (issue #41、`static_graph::naming::fixed_vocabulary` が構築する)。
    #[allow(dead_code)]
    GraphiteLanguage(固定語彙),
}

impl 名前の由来 {
    // F12の着地点・診断のspanに使う。GraphiteLanguage (固定語彙) は利用者が
    // 書いたトークンを持たないため呼び出し側のspan (`Span::call_site()`)
    // を返す。SchemaRole は端点の役割アクセサ自身のトークン (`役割`) を返す
    // (種別名ではない。役割アクセサが指すのは役割そのものであり、種別は
    // 「関係する schema 宣言」段落で別途示す)。DSLトークンの型参照・値供給
    // 関数の実際のspanは `naming::reference_paths`・`naming::internal_names`
    // が具体辺・個体のトークンから直接作るため、この関数は単体試験だけが呼ぶ。
    #[allow(dead_code)]
    pub(crate) fn span(&self) -> Span {
        match self {
            Self::SchemaEdge { 種別名 } => 種別名.span(),
            Self::SchemaRole { 役割, .. } => 役割.span(),
            Self::SchemaPayloadRole { 積み荷役割, .. } => 積み荷役割.span(),
            Self::InstanceNode { 個体名 } => 個体名.span(),
            Self::InstanceEdge { 辺名 } => 辺名.span(),
            Self::GraphiteLanguage(_) => Span::call_site(),
        }
    }

    // issue #41 のA/B分類。GraphiteLanguage だけがB (固定語彙)。単体試験
    // だけが呼ぶ (上記と同じ理由)。
    #[allow(dead_code)]
    pub(crate) fn 利用者語彙由来か(&self) -> bool {
        !matches!(self, Self::GraphiteLanguage(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::fixed_vocabulary_kind::固定語彙の所有者;

    #[test]
    fn 固定語彙由来はb分類である() {
        let 由来 = 名前の由来::GraphiteLanguage(固定語彙::構築する(固定語彙の所有者::Nodes));
        assert!(!由来.利用者語彙由来か());
        // proc_macro2::Span は PartialEq を持たないため、spanが取得できる
        // (panicしない) ことだけを確かめる。
        let _ = 由来.span();
    }

    #[test]
    fn instance由来はa分類である() {
        let 個体名 = Ident::new("太郎", Span::call_site());
        let 由来 = 名前の由来::InstanceNode { 個体名: 個体名.clone() };
        assert!(由来.利用者語彙由来か());
        let _ = 由来.span();
    }
}
