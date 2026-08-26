//! 生成する公開型・公開メソッドの doc へ、schema 宣言元への参照を1段落足す。
//!
//! 参照は「宣言元ファイルのパッケージ相対の綴り」と「宣言の形」で書き、行番号は
//! 含めない。行番号まで書くと、宣言の行が動くだけで全生成ファイルが再生成の
//! 対象になるためである。行番号を持つのは生成ファイル先頭の案内コメント
//! (`crate::generated_source`) だけにする。
//!
//! 宣言の形は意味モデルが組み立てる (`schema::semantic` の `宣言の形`)。この
//! module が決めるのは「どのファイルの宣言か」を添えた doc の書式だけである。

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

/// 生成の入口が渡す、宣言元ファイルのパッケージ相対の綴り。
///
/// 空文字を「綴りが無い」の意味へ流用すると、綴りが空である場合と区別できなく
/// なるため、判別共用体で表す。
pub(crate) enum 宣言元ファイルの綴り {
    /// ファイルを生成する入口 (`TrackedSchema::render_module_source`) が渡す、
    /// パッケージルートから見た宣言元ファイルの綴り。
    パッケージ相対で分かっている(String),
    /// 綴りを渡されない経路。指紋の材料を作るときと、回復診断のインライン展開が
    /// これに当たる。
    ///
    /// 注意: 宣言元への参照は指紋の材料に含めない。指紋を計算するのは
    /// `graph_schema!` であり、マクロは自分が書かれたファイルのパッケージ相対の
    /// 綴りを知らない。含めてしまうと、生成ファイルに埋め込んだ指紋とマクロが
    /// 計算する指紋が一致しなくなる。綴りのずれは
    /// `cargo graphite generate --check` の差分が検出する。
    分かっていない,
}

impl 宣言元ファイルの綴り {
    /// 宣言の形 (`node Scene`) を指す doc 段落を作る。
    pub(crate) fn 宣言への参照(&self, 宣言の形: &str) -> 宣言元への参照 {
        match self {
            Self::パッケージ相対で分かっている(綴り) => {
                宣言元への参照(Some(format!(" 宣言: `{綴り}` の `{宣言の形}`")))
            }
            Self::分かっていない => 宣言元への参照(None),
        }
    }
}

/// 生成する公開要素の doc の末尾へ足す1段落。
///
/// 既存の doc の後ろへ空行を挟んで置くため、hover と rustdoc では要素の説明とは
/// 別の段落として読める。綴りを渡されない経路では何も出さない。
pub(crate) struct 宣言元への参照(Option<String>);

impl ToTokens for 宣言元への参照 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Some(行) = &self.0 else {
            return;
        };
        tokens.extend(quote! {
            #[doc = ""]
            #[doc = #行]
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn 綴りが分かっていれば宣言元の段落を出す() {
        let 参照 = 宣言元ファイルの綴り::パッケージ相対で分かっている("src/schema.rs".to_string())
            .宣言への参照("node Scene");
        assert_eq!(
            参照.to_token_stream().to_string(),
            quote! {
                #[doc = ""]
                #[doc = " 宣言: `src/schema.rs` の `node Scene`"]
            }
            .to_string()
        );
    }

    #[test]
    fn 綴りが分かっていなければ何も出さない() {
        let 参照 = 宣言元ファイルの綴り::分かっていない.宣言への参照("node Scene");
        assert!(参照.to_token_stream().is_empty());
    }
}
