// 個体 (instanceの `node` 宣言1件の意味モデル)。名前・実体型・初期化式を
// 持つ。式が無い (`node 名前: 型;`) 場合は実行時供給であり、`Nodes::new` の
// 引数として渡される (`file/instance_file/node_entities.rs`)。

use proc_macro2::Ident;
use syn::Expr;

#[derive(Clone)]
pub(crate) struct 個体 {
    名前: Ident,
    実体型: Ident,
    値: Option<Expr>,
}

impl 個体 {
    pub(super) fn new(名前: Ident, 実体型: Ident, 値: Option<Expr>) -> Self {
        Self { 名前, 実体型, 値 }
    }

    pub(crate) fn 名前(&self) -> &Ident {
        &self.名前
    }

    pub(crate) fn 実体型(&self) -> &Ident {
        &self.実体型
    }

    pub(crate) fn 値(&self) -> Option<&Expr> {
        self.値.as_ref()
    }

    pub(crate) fn 値なし宣言か(&self) -> bool {
        self.値.is_none()
    }

    // instanceの宣言の形を、意味カードに埋め込める正規化した文字列で返す
    // (`docs/development/design_principles.md` の原則5.2)。式の種類には
    // 依存させない: 値ありは常に `node 名前: 型 = ..` に畳み、値なしだけ
    // `node 名前: 型` にする。式の種類 (struct式かどうか) で書式を変えると、
    // 意味カードの本文だけが値の書き換えで変わり `cargo graphite generate
    // --check` が値の編集ごとに再生成を要求してしまう。
    // 宣言元ファイルの行番号は含めない。
    pub(crate) fn 宣言の形(&self) -> String {
        match &self.値 {
            None => format!("node {}: {}", self.名前, self.実体型),
            Some(_) => format!("node {}: {} = ..", self.名前, self.実体型),
        }
    }
}
