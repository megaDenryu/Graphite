//! 構文木を辿って、ファイル中の全マクロ呼び出し (名前・入力トークン・
//! 宣言行) を集める走査。分類 (`dynamic_graph_schema!`・旧名
//! `graph_schema!`・`static_graph_schema!`・それ以外の静的instance候補) は
//! 呼び出し側 (`static_resolution`・`schema_source_file`・`module_graph`) が
//! 行う。

#[cfg(test)]
mod tests;

use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

// ファイル中の1マクロ呼び出し。`name` はパスの最後の識別子、`tokens` は
// `!` に続く波括弧/丸括弧/角括弧の中身、`line` は宣言行。
pub(crate) struct MacroCall {
    pub(crate) name: String,
    pub(crate) tokens: TokenStream,
    pub(crate) line: usize,
}

// ファイル1件に含まれる全マクロ呼び出しを、出現順で集める。
pub(crate) fn collect_macro_calls(file: &syn::File) -> Vec<MacroCall> {
    let mut collector = MacroCallCollector::default();
    collector.visit_file(file);
    collector.calls
}

#[derive(Default)]
struct MacroCallCollector {
    calls: Vec<MacroCall>,
}

impl<'ast> Visit<'ast> for MacroCallCollector {
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        if let Some(last) = node.path.segments.last() {
            self.calls.push(MacroCall {
                name: last.ident.to_string(),
                tokens: node.tokens.clone(),
                line: node.span().start().line,
            });
        }
        visit::visit_macro(self, node);
    }
}

// 呼び出しの入力トークンが `generated = "文字列";` から始まるかを見る
// (完全な解析はしない)。schema宣言 (`dynamic_graph_schema!`・
// `static_graph_schema!`) とinstance宣言はどちらもこの形を先頭に持つため、
// instance宣言のようにマクロ名が利用者ごとに違う呼び出しも、名前を知らずに
// 「Graphiteの宣言らしい」と判定できる。`static_resolution::instance_resolution`
// が名簿照合の入口判定に、`module_graph::orphan_check`がmod木から辿れない
// ファイルの違反判定にそれぞれ使う。
pub(crate) fn 追跡形式らしいか(tokens: &TokenStream) -> bool {
    let probe = |input: syn::parse::ParseStream| -> syn::Result<bool> {
        let 追跡形式の先頭か = (|| -> syn::Result<()> {
            let ident: syn::Ident = input.parse()?;
            if ident != "generated" {
                return Err(input.error("先頭が generated ではない"));
            }
            input.parse::<syn::Token![=]>()?;
            input.parse::<syn::LitStr>()?;
            input.parse::<syn::Token![;]>()?;
            Ok(())
        })()
        .is_ok();
        // 残りのトークンを読み捨てる (proc-macro-dev スキルの drain_rest と
        // 同じ理由。`Parser::parse2` は末尾に未消費トークンが残ると
        // 無関係な "unexpected token" エラーを返してしまう)。
        let _ = input.parse::<TokenStream>();
        Ok(追跡形式の先頭か)
    };
    syn::parse::Parser::parse2(probe, tokens.clone()).unwrap_or(false)
}
