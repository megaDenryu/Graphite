// instance宣言の入力DSLの構文木。
//
//   graph <グラフ名>;
//   (<名前> = <型> { <フィールド式, ...> },)*
//   (<名前> = <種別>(<始点> -> <終点>),)*
//   (<名前> = <種別>(<始点> -[<積み荷式>]-> <終点>),)*
//   (<名前> = <種別>(<始点> -- <終点>),)*
//   (<名前> = <種別>(<始点> -[<積み荷式>]- <終点>),)*
//
// schema名を書かないのは、schema名がそのままマクロ名になり (利用側は
// `<schema名>! { graph <名前>; .. }` と書く)、この構文木を組み立てる時点で
// どのschemaかは既に確定しているため (issue #24 段階2)。
//
// ノードか辺かの判別は input_item.rs、辺の右辺 `(...)` の中身 (無payload/
// payload/無向) の判別は input_edge_body.rs が担う。ここは骨格の構造体定義と
// トップレベル (graph宣言 + カンマ区切りの宣言列) のパースだけを持つ。

#[path = "input_edge_body.rs"]
mod input_edge_body;
#[path = "input_item.rs"]
mod input_item;

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, ExprStruct, Token};

syn::custom_keyword!(graph);

pub struct 静的グラフ入力 {
    pub グラフ名: Ident,
    pub ノード宣言達: Vec<ノード宣言>,
    pub 辺宣言達: Vec<辺宣言>,
}

pub struct ノード宣言 {
    pub 名前: Ident,
    pub 実体型: Ident,
    pub 式: ExprStruct,
}

pub enum 辺中身 {
    無積み荷,
    積み荷あり(Expr),
}

pub enum 辺形状 {
    有向 { 始点: Ident, 終点: Ident, 中身: 辺中身 },
    無向 { 端点1: Ident, 端点2: Ident, 中身: 辺中身 },
}

impl 辺形状 {
    pub(crate) fn 積み荷式(&self) -> Option<&Expr> {
        let 中身 = match self {
            辺形状::有向 { 中身, .. } => 中身,
            辺形状::無向 { 中身, .. } => 中身,
        };
        match 中身 {
            辺中身::無積み荷 => None,
            辺中身::積み荷あり(式) => Some(式),
        }
    }
}

pub struct 辺宣言 {
    pub 名前: Ident,
    pub 種別: Ident,
    pub 形状: 辺形状,
}

impl 静的グラフ入力 {
    pub(crate) fn 個体の実体型(&self, 個体: &Ident) -> &Ident {
        &self.ノード宣言達.iter().find(|n| &n.名前 == 個体).expect("検証済みなので端点は必ず宣言されている").実体型
    }
}

impl Parse for 静的グラフ入力 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<graph>()?;
        let グラフ名 = input.parse()?;
        input.parse::<Token![;]>()?;

        let 宣言達 = Punctuated::<input_item::宣言, Token![,]>::parse_terminated(input)?;
        let mut ノード宣言達 = Vec::new();
        let mut 辺宣言達 = Vec::new();
        for 宣言 in 宣言達 {
            match 宣言 {
                input_item::宣言::ノード(n) => ノード宣言達.push(n),
                input_item::宣言::辺(e) => 辺宣言達.push(e),
            }
        }
        Ok(静的グラフ入力 { グラフ名, ノード宣言達, 辺宣言達 })
    }
}
