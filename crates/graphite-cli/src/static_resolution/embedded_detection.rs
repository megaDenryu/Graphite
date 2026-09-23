//! 名簿の名前が、他のマクロ呼び出しの入力の中に埋め込まれて書かれていない
//! かの検出 (issue #41 段階3 §2 の4)。instanceは文の位置に直接書く記法だけ
//! を受理し、他のマクロの引数の中に埋め込む書き方は対象外にする (schemaの
//! 探索がそこまで踏み込むと、無関係なマクロの入力を誤って解析しにいくため)。

use std::error::Error;

use proc_macro2::{TokenStream, TokenTree};

use graphite_codegen::DeclarationSite;

use super::schema_registry::静的schema名簿;
use super::FileMacros;

pub(crate) fn 埋め込まれたinstanceを検査する(
    files: &[FileMacros],
    名簿: &静的schema名簿,
) -> Result<(), Box<dyn Error>> {
    for file in files {
        for call in &file.calls {
            埋め込みを探す(&call.tokens, 名簿, &file.display_path)?;
        }
    }
    Ok(())
}

// `display_path` の行番号は、埋め込みが見つかった識別子自身のspan
// (`ident.span().start().line`) から取る。呼び出し元 (`埋め込まれた
// instanceを検査する`) が持つ外側の呼び出しの行番号 (`call.line`) を使うと、
// 複数行にまたがる呼び出しの中の後ろの方で埋め込みが見つかった場合に
// エラーが呼び出し全体の開始行を指してしまい、実際にどの行を直せばよいかが
// 分かりにくくなる。
fn 埋め込みを探す(
    tokens: &TokenStream,
    名簿: &静的schema名簿,
    display_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut iter = tokens.clone().into_iter().peekable();
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(ident) if 名簿.名前が存在するか(&ident.to_string()) => {
                let 次がビックリマークか =
                    matches!(iter.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '!');
                if 次がビックリマークか {
                    iter.next();
                    if matches!(iter.peek(), Some(TokenTree::Group(_))) {
                        let site =
                            DeclarationSite::new(display_path.to_string(), ident.span().start().line);
                        return Err(format!(
                            "{}: instance は他のマクロの入力の中に書けません (schema `{ident}`)",
                            site.display()
                        )
                        .into());
                    }
                }
            }
            TokenTree::Group(group) => {
                埋め込みを探す(&group.stream(), 名簿, display_path)?;
            }
            _ => {}
        }
    }
    Ok(())
}
