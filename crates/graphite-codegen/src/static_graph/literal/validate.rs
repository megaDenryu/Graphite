// 静的グラフ入力の検証。パース (input.rs) は構文の形だけを見るため、ノード名・
// 辺名の重複、辺の端点が未宣言のノードを指す誤りはここで検出する。種別・役割
// の実在はrustcの型解決に委ねる (ハンドシェイクマクロは無い設計、hello-graph
// と同じ)。
//
// 加えて、DSL上は合法でも脱糖後のRust識別子が衝突する組も検証する (issue #41
// 生成される名前の公開契約)。node名とedge名はどちらも
// `{名前}Ref`という同じ名前空間へ展開されるため横断で重複を禁じ、edge名は
// 固定語彙`entity`(`{個体}Ref::entity()`)とも衝突できない。

use std::collections::HashSet;

use proc_macro2::Ident;

use crate::static_graph::literal::input::{辺形状, 辺宣言, 静的グラフ入力};
use crate::static_graph::reserved_words::実体アクセサ名;

impl 静的グラフ入力 {
    pub fn 検証する(&self) -> syn::Result<()> {
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
            if ノード名達.contains(&辺宣言.名前.to_string()) {
                return Err(syn::Error::new_spanned(
                    &辺宣言.名前,
                    format!(
                        "名前の衝突: 辺 `{}` はノードと同じ名前です。生成される具体個体参照・具体辺参照はどちらも `{}Ref` という同じ名前になり衝突します",
                        辺宣言.名前, 辺宣言.名前
                    ),
                ));
            }
            if 辺宣言.名前 == 実体アクセサ名 {
                return Err(syn::Error::new_spanned(
                    &辺宣言.名前,
                    format!(
                        "名前の衝突: 辺 `{実体アクセサ名}` は使えません。個体の具体参照が実体を取り出す固定語彙のメソッド名 `{実体アクセサ名}()` と衝突します"
                    ),
                ));
            }
            for 端点 in 辺宣言.端点を求める() {
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
}

impl 辺宣言 {
    fn 端点を求める(&self) -> Vec<&Ident> {
        match &self.形状 {
            辺形状::有向 { 始点, 終点, .. } => vec![始点, 終点],
            辺形状::無向 { 端点1, 端点2, .. } => vec![端点1, 端点2],
        }
    }
}
