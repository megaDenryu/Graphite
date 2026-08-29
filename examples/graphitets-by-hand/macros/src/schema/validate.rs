// 静的グラフ型入力の検証。パース (input.rs) は構文の形だけを見るため、
// ノード名・辺名の重複、辺の端点が未宣言のノードを指す誤り、同一辺内で始点
// 役割と終点役割が同名になっている誤り、where節の役割名がその辺の宣言済み
// 役割名と一致しない誤りはここで検出する。

use std::collections::HashSet;

use proc_macro2::Ident;

use super::input::{制約, 辺形状, 辺宣言, 静的グラフ型入力};

impl 静的グラフ型入力 {
    pub(crate) fn 検証する(&self) -> syn::Result<()> {
        let mut ノード名達 = HashSet::new();
        for ノード宣言 in &self.ノード宣言達 {
            if !ノード名達.insert(ノード宣言.名前.to_string()) {
                return Err(syn::Error::new_spanned(
                    &ノード宣言.名前,
                    format!("ノード `{}` が重複して宣言されています", ノード宣言.名前),
                ));
            }
        }

        let mut 辺名達 = HashSet::new();
        for 辺宣言 in &self.辺宣言達 {
            if !辺名達.insert(辺宣言.名前.to_string()) {
                return Err(syn::Error::new_spanned(
                    &辺宣言.名前,
                    format!("辺 `{}` が重複して宣言されています", 辺宣言.名前),
                ));
            }
            self.辺の端点を検証する(辺宣言)?;
            辺宣言.制約を検証する()?;
        }
        Ok(())
    }

    fn 辺の端点を検証する(&self, 辺宣言: &辺宣言) -> syn::Result<()> {
        match &辺宣言.形状 {
            辺形状::有向 { 始点役割, 始点型, 積み荷: _, 終点役割, 終点型 } => {
                self.型が宣言済みか検証する(始点型)?;
                self.型が宣言済みか検証する(終点型)?;
                if 始点役割 == 終点役割 {
                    return Err(syn::Error::new_spanned(
                        終点役割,
                        format!("辺 `{}` の始点役割と終点役割が同名です", 辺宣言.名前),
                    ));
                }
            }
            辺形状::無向 { 型 } => self.型が宣言済みか検証する(型)?,
        }
        Ok(())
    }

    fn 型が宣言済みか検証する(&self, 型: &Ident) -> syn::Result<()> {
        if self.ノード宣言達.iter().any(|n| n.名前 == *型) {
            Ok(())
        } else {
            Err(syn::Error::new_spanned(型, format!("`{}` は node として宣言されていません", 型)))
        }
    }
}

impl 辺宣言 {
    fn 制約を検証する(&self) -> syn::Result<()> {
        let 役割達: Vec<&Ident> = match &self.形状 {
            辺形状::有向 { 始点役割, 終点役割, .. } => vec![始点役割, 終点役割],
            辺形状::無向 { .. } => Vec::new(),
        };
        for 制約 in &self.制約達 {
            if let 制約::多重度 { 役割, .. } = 制約 {
                if !役割達.iter().any(|r| *r == 役割) {
                    return Err(syn::Error::new_spanned(
                        役割,
                        format!("辺 `{}` に役割 `{}` はありません", self.名前, 役割),
                    ));
                }
            }
        }
        Ok(())
    }
}
