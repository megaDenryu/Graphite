// 静的グラフ! の入力DSLの構文木とパーサ。
//
//   graph <グラフ名>;
//   (node <ノード名>: <実体型>;)*
//   (edge <辺名> = <種別>(<始点> -> <終点>);)*
//
// エラーは `?` でそのまま呼び出し元まで伝播する素直な構成なので、
// proc-macro-dev スキルが警告する ParseBuffer の drain 忘れは該当しない
// (内側の Err を握りつぶして Ok を返す回復パーサではないため)。

use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Token};

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
}

pub struct 辺宣言 {
    pub 名前: Ident,
    pub 種別: Ident,
    pub 始点: Ident,
    pub 終点: Ident,
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

impl Parse for ノード宣言 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<node>()?;
        let 名前 = input.parse()?;
        input.parse::<Token![:]>()?;
        let 実体型 = input.parse()?;
        input.parse::<Token![;]>()?;
        Ok(ノード宣言 { 名前, 実体型 })
    }
}

impl Parse for 辺宣言 {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<edge>()?;
        let 名前 = input.parse()?;
        input.parse::<Token![=]>()?;
        let 種別 = input.parse()?;
        let 端点達;
        parenthesized!(端点達 in input);
        let 始点 = 端点達.parse()?;
        端点達.parse::<Token![->]>()?;
        let 終点 = 端点達.parse()?;
        input.parse::<Token![;]>()?;
        Ok(辺宣言 { 名前, 種別, 始点, 終点 })
    }
}
