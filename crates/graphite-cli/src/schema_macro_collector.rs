//! 構文木を辿って、ファイル中の全マクロ呼び出し (名前・入力トークン・
//! 宣言行) を集める走査。分類 (`dynamic_graph_schema!`・旧名
//! `graph_schema!`・`static_graph_schema!`・それ以外の静的instance候補) は
//! 呼び出し側 (`static_resolution`・`schema_source_file`) が行う。

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
