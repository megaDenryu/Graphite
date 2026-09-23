//! 名簿の名前が、他のマクロ呼び出しの入力の中に埋め込まれて書かれていない
//! かの検出 (issue #41 段階3 §2 の4)。instanceは文の位置に直接書く記法だけ
//! を受理し、他のマクロの引数の中に埋め込む書き方は対象外にする (schemaの
//! 探索がそこまで踏み込むと、無関係なマクロの入力を誤って解析しにいくため)。

use std::error::Error;

use proc_macro2::{TokenStream, TokenTree};

use super::schema_registry::静的schema名簿;
use super::FileMacros;
use crate::schema_macro_collector::MacroCall;

pub(crate) fn 埋め込まれたinstanceを検査する(
    files: &[FileMacros],
    名簿: &静的schema名簿,
) -> Result<(), Box<dyn Error>> {
    for file in files {
        for call in &file.calls {
            埋め込みを探す(&call.tokens, 名簿, file, call)?;
        }
    }
    Ok(())
}

fn 埋め込みを探す(
    tokens: &TokenStream,
    名簿: &静的schema名簿,
    file: &FileMacros,
    親呼び出し: &MacroCall,
) -> Result<(), Box<dyn Error>> {
    let mut iter = tokens.clone().into_iter().peekable();
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(ident) if 名簿.探す(&ident.to_string()).is_some() => {
                let 次がビックリマークか =
                    matches!(iter.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '!');
                if 次がビックリマークか {
                    iter.next();
                    if matches!(iter.peek(), Some(TokenTree::Group(_))) {
                        return Err(format!(
                            "{}:{}: instance は他のマクロの入力の中に書けません (schema `{ident}`)",
                            file.display_path, 親呼び出し.line
                        )
                        .into());
                    }
                }
            }
            TokenTree::Group(group) => {
                埋め込みを探す(&group.stream(), 名簿, file, 親呼び出し)?;
            }
            _ => {}
        }
    }
    Ok(())
}
