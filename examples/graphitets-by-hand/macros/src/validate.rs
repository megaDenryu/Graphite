// 静的グラフ入力の検証。生成対象の型・トークンを組み立てる前に、宣言同士の
// 参照関係が閉じていることを確かめる。パース (input.rs) は構文の形だけを見る
// ため、ノード名の重複や辺の端点が未宣言のノードを指す誤りはここで検出する。

use std::collections::HashSet;

use crate::input::静的グラフ入力;

pub fn 検証する(入力: &静的グラフ入力) -> syn::Result<()> {
    let mut ノード名達 = HashSet::new();
    for ノード宣言 in &入力.ノード宣言達 {
        if !ノード名達.insert(ノード宣言.名前.to_string()) {
            return Err(syn::Error::new_spanned(
                &ノード宣言.名前,
                format!("ノード `{}` が重複して宣言されています", ノード宣言.名前),
            ));
        }
    }

    let mut 辺名達 = HashSet::new();
    for 辺宣言 in &入力.辺宣言達 {
        if !辺名達.insert(辺宣言.名前.to_string()) {
            return Err(syn::Error::new_spanned(
                &辺宣言.名前,
                format!("辺 `{}` が重複して宣言されています", 辺宣言.名前),
            ));
        }
        for 端点 in [&辺宣言.始点, &辺宣言.終点] {
            if !ノード名達.contains(&端点.to_string()) {
                return Err(syn::Error::new_spanned(
                    端点,
                    format!("辺 `{}` の端点 `{}` は node として宣言されていません", 辺宣言.名前, 端点),
                ));
            }
        }
    }
    Ok(())
}
