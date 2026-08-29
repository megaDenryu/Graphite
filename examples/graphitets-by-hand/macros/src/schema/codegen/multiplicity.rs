// 生成物4: 多重度の契約。位置キー (始点位置/終点位置をジェネリック引数に持つ
// 1trait) と役割名キー (役割ごとに別trait) の両方を、種別ごと・schemaが知る
// 全node実体型ごとに生成する。対象の実体型・役割には宣言された制約の値を、
// それ以外には無制約 (0..usize::MAX) を割り当てる (レイヤー2は自分の入力
// トークンしか見えず、schemaのどのnode型がどの位置の制約対象かを知らないため、
// 全型分を用意して機械的に参照できるようにする)。無向辺は役割を持てない
// (`each` を書く構文自体が無い) ため、位置キー・役割名キーいずれのtraitも
// 生成しない。対一意だけは有向・無向を問わず生成する。
//
// 位置キー生成は multiplicity_position.rs、役割名キー生成は
// multiplicity_role.rs、対一意生成は multiplicity_unique.rs が担う。ここは
// 並び順の配線と、両方が使う共有ヘルパー (制約値の取り出し・実体型ごとの
// impl列挙) だけを持つ。

#[path = "multiplicity_position.rs"]
mod multiplicity_position;
#[path = "multiplicity_role.rs"]
mod multiplicity_role;
#[path = "multiplicity_unique.rs"]
mod multiplicity_unique;

use proc_macro2::TokenStream;
use quote::quote;

use crate::schema::input::{ノード宣言, 制約, 辺宣言};

pub(super) fn 多重度契約達を生成する(ノード宣言達: &[ノード宣言], 辺宣言達: &[辺宣言]) -> TokenStream {
    let 位置キー達 = 辺宣言達.iter().filter_map(|辺| multiplicity_position::位置キーtraitを生成する(辺, ノード宣言達));
    let 役割キー達 = 辺宣言達.iter().flat_map(|辺| multiplicity_role::役割キーtrait達を生成する(辺, ノード宣言達));
    let 対一意達 = 辺宣言達.iter().map(multiplicity_unique::対一意traitを生成する);
    quote! { #(#位置キー達)* #(#役割キー達)* #(#対一意達)* }
}

pub(super) fn 制約から多重度を取り出す(辺宣言: &辺宣言, 役割: &proc_macro2::Ident) -> (usize, usize) {
    辺宣言
        .制約達
        .iter()
        .find_map(|c| match c {
            制約::多重度 { 役割: r, 下限, 上限 } if r == 役割 => Some((*下限, *上限)),
            _ => None,
        })
        .unwrap_or((0, usize::MAX))
}

// 実体型ごとにimplする多重度trait本体 (トレイト名・トレイト引数は呼び出し元が
// 組み立てる。役割キーは非ジェネリックなので トレイト引数 は空のTokenStream)。
// 対象型だけ宣言された制約を使い、それ以外は無制約とする。
pub(super) fn 実体型ごとのimpl達を生成する(
    トレイト名: &proc_macro2::Ident,
    トレイト引数: TokenStream,
    対象型: &proc_macro2::Ident,
    対象の多重度: (usize, usize),
    ノード宣言達: &[ノード宣言],
) -> TokenStream {
    let (対象下限, 対象上限) = 対象の多重度;
    let impl達 = ノード宣言達.iter().map(|n| {
        let 型 = &n.名前;
        let (下限, 上限) = if 型 == 対象型 { (対象下限, 対象上限) } else { (0usize, usize::MAX) };
        quote! {
            impl #トレイト名 #トレイト引数 for #型 {
                const 下限: usize = #下限;
                const 上限: usize = #上限;
            }
        }
    });
    quote! { #(#impl達)* }
}
