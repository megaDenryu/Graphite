// 役割名キー (subordinate・superior 等) の多重度trait生成。役割ごとに別の
// 非ジェネリックtraitを1本生成し、schemaが知る全node実体型へimplする
// (ドキュメント的価値・将来の直接参照用途のため。中身は位置キー版と同じ値)。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::input::{ノード宣言, 辺形状, 辺宣言};

pub(super) fn 役割キーtrait達を生成する(辺宣言: &辺宣言, ノード宣言達: &[ノード宣言]) -> Vec<TokenStream> {
    let (始点役割, 始点型, 終点役割, 終点型) = match &辺宣言.形状 {
        辺形状::有向 { 始点役割, 始点型, 終点役割, 終点型, .. } => (始点役割, 始点型, 終点役割, 終点型),
        辺形状::無向 { .. } => return Vec::new(),
    };
    let 種別 = &辺宣言.名前;
    vec![
        役割キーtraitを1本生成する(種別, 始点役割, 始点型, 辺宣言, ノード宣言達),
        役割キーtraitを1本生成する(種別, 終点役割, 終点型, 辺宣言, ノード宣言達),
    ]
}

fn 役割キーtraitを1本生成する(
    種別: &proc_macro2::Ident,
    役割: &proc_macro2::Ident,
    対象型: &proc_macro2::Ident,
    辺宣言: &辺宣言,
    ノード宣言達: &[ノード宣言],
) -> TokenStream {
    let トレイト名 = format_ident!("{}の{}多重度", 種別, 役割);
    let impl達 = super::実体型ごとのimpl達を生成する(
        &トレイト名,
        quote! {},
        対象型,
        辺宣言.多重度を求める(役割),
        ノード宣言達,
    );
    quote! {
        trait #トレイト名 {
            const 下限: usize;
            const 上限: usize;
        }
        #impl達
    }
}
