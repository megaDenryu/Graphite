// 位置キー (始点位置/終点位置) の多重度trait生成。1種別につき1trait
// (ジェネリック引数<位置>を持つ) を生成し、schemaが知る全node実体型へimplする。

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::schema::input::{ノード宣言, 辺形状, 辺宣言};

pub(super) fn 位置キーtraitを生成する(辺宣言: &辺宣言, ノード宣言達: &[ノード宣言]) -> Option<TokenStream> {
    let (始点役割, 始点型, 終点役割, 終点型) = match &辺宣言.形状 {
        辺形状::有向 { 始点役割, 始点型, 終点役割, 終点型, .. } => (始点役割, 始点型, 終点役割, 終点型),
        辺形状::無向 { .. } => return None,
    };
    let 種別 = &辺宣言.名前;
    let トレイト名 = format_ident!("{}の多重度", 種別);

    let 始点impl達 = super::実体型ごとのimpl達を生成する(
        &トレイト名,
        quote! { <始点位置> },
        始点型,
        super::制約から多重度を取り出す(辺宣言, 始点役割),
        ノード宣言達,
    );
    let 終点impl達 = super::実体型ごとのimpl達を生成する(
        &トレイト名,
        quote! { <終点位置> },
        終点型,
        super::制約から多重度を取り出す(辺宣言, 終点役割),
        ノード宣言達,
    );

    Some(quote! {
        trait #トレイト名<位置> {
            const 下限: usize;
            const 上限: usize;
        }
        #始点impl達
        #終点impl達
    })
}
