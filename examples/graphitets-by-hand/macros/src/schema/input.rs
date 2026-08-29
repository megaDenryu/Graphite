// 静的グラフ型! の入力DSLの構文木。
//
//   schema <名前> {
//     (node <名前>;)*
//     (edge <名前> = (<役割>: <型>) -> (<役割>: <型>) [where <制約>(, <制約>)*];)*
//     (edge <名前> = (<役割>: <型>) -[<役割>: <型>]-> (<役割>: <型>) [where ...];)*
//     (edge <名前> = <型> -- <型> [where <制約>(, <制約>)*];)*
//   }
//
// 辺宣言そのものの構文 (端点・積み荷) は input_edge.rs、where節の構文
// (多重度・対一意) は input_constraint.rs が担う。ここは骨格の構造体定義と
// トップレベル (schema { .. }) のパースだけを持つ。

#[path = "input_constraint.rs"]
mod input_constraint;
#[path = "input_edge.rs"]
mod input_edge;

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::Token;

syn::custom_keyword!(schema);
syn::custom_keyword!(node);
syn::custom_keyword!(edge);

pub struct 静的グラフ型入力 {
    // レイヤー1とレイヤー2の結線はRustの型解決 (種別ラベル・種別契約) に
    // 委ねるため、schema名自体はコード生成に使わない (`graph <名前>: schema名;`
    // の schema名 と同じ扱い)。ヘッダとして読み捨てる目的だけで保持する。
    #[allow(dead_code)]
    pub schema名: Ident,
    pub ノード宣言達: Vec<ノード宣言>,
    pub 辺宣言達: Vec<辺宣言>,
}

pub struct ノード宣言 {
    pub 名前: Ident,
}

// 積み荷は (役割名, 型) の組。有向辺だけが持てる。
pub enum 辺形状 {
    有向 { 始点役割: Ident, 始点型: Ident, 積み荷: Option<(Ident, Ident)>, 終点役割: Ident, 終点型: Ident },
    無向 { 型: Ident },
}

pub struct 辺宣言 {
    pub 名前: Ident,
    pub 形状: 辺形状,
    pub 制約達: Vec<制約>,
}

pub enum 制約 {
    多重度 { 役割: Ident, 下限: usize, 上限: usize },
    対一意,
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
