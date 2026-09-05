// instance宣言の入力DSLの構文木。
//
//   graph <グラフ名>;
//   (node <名前> = <型> { <フィールド式, ...> };)*
//   (node <名前>: <型> = <式>;)*
//   (node <名前>: <型>;)*
//   (edge <名前> = <種別>(<始点> -> <終点>);)*
//   (edge <名前> = <種別>(<始点> -[<積み荷式>]-> <終点>);)*
//   (edge <名前> = <種別>(<始点> -- <終点>);)*
//   (edge <名前> = <種別>(<始点> -[<積み荷式>]- <終点>);)*
//
// 行の種類は先頭の `node`/`edge` キーワードで確定する (右辺の形からの
// 推測判別はしない)。node は3形態を受理する:
//   1. `名前 = 型 { .. };`   実体型はリテラルのパスから読む
//   2. `名前: 型 = 式;`      型を明示すれば右辺は任意の式でよい
//   3. `名前: 型;`           宣言のみ。実体値は `Nodes::new` へ実行時に渡す
//
// schema名を書かないのは、schema名がそのままマクロ名になり (利用側は
// `<schema名>! { graph <名前>; .. }` と書く)、この構文木を組み立てる時点で
// どのschemaかは既に確定しているため。
//
// node/edgeの各1件の宣言のパース (3形態の判別を含む) は input_item.rs、
// 辺の右辺 `(...)` の中身 (無payload/payload/無向) の判別は input_edge_body.rs
// が担う。ここは骨格の構造体定義とトップレベル (graph宣言 + node/edge宣言の
// 列) のパースだけを持つ。

#[path = "input_edge_body.rs"]
mod input_edge_body;
#[path = "input_item.rs"]
mod input_item;

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Token};

syn::custom_keyword!(graph);
syn::custom_keyword!(node);
syn::custom_keyword!(edge);

pub struct 静的グラフ入力 {
    pub グラフ名: Ident,
    pub ノード宣言達: Vec<ノード宣言>,
    pub 辺宣言達: Vec<辺宣言>,
}

pub struct ノード宣言 {
    pub 名前: Ident,
    pub 実体型: Ident,
    pub 値: Option<Expr>, // 値なし宣言 (`node 名前: 型;`) は None。実行時引数として `Nodes::new` へ渡す (node_entities.rs)
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

impl 辺宣言 {
    pub(crate) fn 端点に含むか(&self, 個体名: &Ident) -> bool {
        match &self.形状 {
            辺形状::有向 { 始点, 終点, .. } => 始点 == 個体名 || 終点 == 個体名,
            辺形状::無向 { 端点1, 端点2, .. } => 端点1 == 個体名 || 端点2 == 個体名,
        }
    }
}

impl Parse for 静的グラフ入力 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<graph>()?;
        let グラフ名 = input.parse()?;
        input.parse::<Token![;]>()?;

        let mut ノード宣言達 = Vec::new();
        let mut 辺宣言達 = Vec::new();
        while !input.is_empty() {
            if input.peek(node) {
                ノード宣言達.push(input.parse()?);
            } else if input.peek(edge) {
                辺宣言達.push(input.parse()?);
            } else {
                return Err(input.error("`node` または `edge` の宣言が必要です"));
            }
        }
        Ok(静的グラフ入力 { グラフ名, ノード宣言達, 辺宣言達 })
    }
}
