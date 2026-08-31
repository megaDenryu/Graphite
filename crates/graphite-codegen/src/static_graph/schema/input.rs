// static_schema! の入力DSLの構文木。
//
//   schema <名前> {
//     (node <名前>;)*
//     (edge <名前> = (<役割>: <型>) -> (<役割>: <型>) [where <制約>(, <制約>)*];)*
//     (edge <名前> = (<役割>: <型>) -[<役割>: <型>]-> (<役割>: <型>) [where ...];)*
//     (edge <名前> = (<役割>: <型>) -- (<役割>: <型>) [where <制約>(, <制約>)*];)*
//     (edge <名前> = (<役割>: <型>) -[<役割>: <型>]- (<役割>: <型>) [where <制約>(, <制約>)*];)*
//   }
//
// 無向辺の積み荷付き記法 `-[役割: 型]-` は、有向の積み荷付き記法
// `-[役割: 型]->` から矢尻 (`>`) を落とした形 (hello-graph の規約に倣う)。
//
// 辺宣言そのものの構文 (端点・積み荷) は input_edge.rs、where節の構文
// (多重度・対一意) は input_constraint.rs が担う。ここは骨格の構造体定義と
// トップレベル (schema { .. }) のパースだけを持つ。

#[path = "input_constraint.rs"]
mod input_constraint;
#[path = "input_edge.rs"]
mod input_edge;
#[path = "multiplicity_range.rs"]
mod multiplicity_range;

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::Token;

pub use multiplicity_range::多重度範囲;

syn::custom_keyword!(schema);
syn::custom_keyword!(node);
syn::custom_keyword!(edge);

pub struct 静的グラフ型入力 {
    // `static_schema!` がこの名前そのものを `macro_rules! {schema名}` の名前
    // として使う (利用側は `<schema名>! { graph <名前>; .. }` と書く)。
    pub schema名: Ident,
    pub ノード宣言達: Vec<ノード宣言>,
    pub 辺宣言達: Vec<辺宣言>,
}

pub struct ノード宣言 {
    pub 名前: Ident,
}

// 積み荷は (役割名, 型) の組。無向辺も有向辺と同じく両端の役割名を持つ
// (端点1/端点2への合成は廃止。issue #24 段階2、オーナー裁定)。
pub enum 辺形状 {
    有向 { 始点役割: Ident, 始点型: Ident, 積み荷: Option<(Ident, Ident)>, 終点役割: Ident, 終点型: Ident },
    無向 { 第1役割: Ident, 第1型: Ident, 積み荷: Option<(Ident, Ident)>, 第2役割: Ident, 第2型: Ident },
}

pub struct 辺宣言 {
    pub 名前: Ident,
    pub 形状: 辺形状,
    pub 制約達: Vec<制約>,
}

pub enum 制約 {
    多重度 { 役割: Ident, 範囲: 多重度範囲 },
    対一意,
}

impl 静的グラフ型入力 {
    pub(crate) fn 辺宣言を種別名で探す(&self, 種別: &Ident) -> Option<&辺宣言> {
        self.辺宣言達.iter().find(|e| e.名前 == *種別)
    }
}

impl 辺形状 {
    // 積み荷の (役割名, 型) を返す。積み荷を持たない種別は None。
    pub(crate) fn 積み荷(&self) -> Option<&(Ident, Ident)> {
        match self {
            辺形状::有向 { 積み荷, .. } => 積み荷.as_ref(),
            辺形状::無向 { 積み荷, .. } => 積み荷.as_ref(),
        }
    }
}

impl Parse for 静的グラフ型入力 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<schema>()?;
        let schema名 = input.parse()?;
        let 本体;
        syn::braced!(本体 in input);

        let mut ノード宣言達 = Vec::new();
        let mut 辺宣言達 = Vec::new();
        while !本体.is_empty() {
            if 本体.peek(node) {
                ノード宣言達.push(本体.parse()?);
            } else if 本体.peek(edge) {
                辺宣言達.push(本体.parse()?);
            } else {
                return Err(本体.error("`node` または `edge` の宣言が必要です"));
            }
        }
        Ok(静的グラフ型入力 { schema名, ノード宣言達, 辺宣言達 })
    }
}

impl Parse for ノード宣言 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<node>()?;
        let 名前 = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(ノード宣言 { 名前 })
    }
}
