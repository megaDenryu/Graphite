// 対一意制約 (`where unique pair`) の検証。有向は (始点, 終点) をそのまま、
// 無向は端点の順序に依らないよう正規化してから重複を見る。個体名は Ident の
// まま保持し、正規化の並べ替え判定にだけ文字列表現を使う (Ident は Ord を
// 持たないため、並べ替えの判定材料としてのみ to_string() を使い、格納・
// 比較・表示は Ident のまま行う)。

use proc_macro2::Ident;

use crate::static_graph::literal::input::{辺形状 as 具体形状, 静的グラフ入力};
use crate::static_graph::schema::input::{制約, 静的グラフ型入力};

pub(super) fn 検証する(schema: &静的グラフ型入力, instance: &静的グラフ入力) -> syn::Result<()> {
    for 型宣言 in &schema.辺宣言達 {
        if !型宣言.制約達.iter().any(|c| matches!(c, 制約::対一意)) {
            continue;
        }

        let mut 既出端点対達: Vec<(Ident, Ident)> = Vec::new();
        for 辺 in instance.辺宣言達.iter().filter(|e| e.種別 == 型宣言.名前) {
            let 端点対 = match &辺.形状 {
                具体形状::有向 { 始点, 終点, .. } => (始点.clone(), 終点.clone()),
                具体形状::無向 { 端点1, 端点2, .. } => {
                    if 端点1.to_string() <= 端点2.to_string() {
                        (端点1.clone(), 端点2.clone())
                    } else {
                        (端点2.clone(), 端点1.clone())
                    }
                }
            };
            if 既出端点対達.contains(&端点対) {
                return Err(syn::Error::new_spanned(
                    &辺.名前,
                    format!(
                        "対一意制約違反: 種別 `{}` の辺 `{}` は端点の組 ({}, {}) が既出の辺と重複しています",
                        型宣言.名前, 辺.名前, 端点対.0, 端点対.1,
                    ),
                ));
            }
            既出端点対達.push(端点対);
        }
    }
    Ok(())
}
