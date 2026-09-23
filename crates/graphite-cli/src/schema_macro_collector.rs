//! 構文木を辿って `dynamic_graph_schema!` の呼び出しと、旧名 `graph_schema!`
//! の呼び出しを拾う走査。

#[cfg(test)]
mod tests;

use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

// 追跡形式の `dynamic_graph_schema!` 呼び出しと、その宣言行。
pub(crate) struct SchemaInvocation {
    pub(crate) tokens: TokenStream,
    pub(crate) line: usize,
}

// 走査1回分の結果。旧名の呼び出しは生成の対象にせず、宣言行だけを持ち帰る
// (呼び出し側が改名を促すエラーへ変換する。issue #40)。
#[derive(Default)]
pub(crate) struct CollectedSchemaMacros {
    pub(crate) invocations: Vec<SchemaInvocation>,
    pub(crate) legacy_invocations: Vec<usize>,
}

// ファイル1件から `dynamic_graph_schema!`/旧名 `graph_schema!` の呼び出しを集める。
pub(crate) fn collect_schema_macros(file: &syn::File) -> CollectedSchemaMacros {
    let mut collector = SchemaMacroCollector::default();
    collector.visit_file(file);
    collector.collected
}

#[derive(Default)]
struct SchemaMacroCollector {
    collected: CollectedSchemaMacros,
}

impl<'ast> Visit<'ast> for SchemaMacroCollector {
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        let last = node.path.segments.last();
        if last.is_some_and(|s| s.ident == "dynamic_graph_schema") {
            self.collected.invocations.push(SchemaInvocation {
                tokens: node.tokens.clone(),
                line: node.span().start().line,
            });
        } else if last.is_some_and(|s| s.ident == "graph_schema") {
            self.collected
                .legacy_invocations
                .push(node.span().start().line);
        }
        visit::visit_macro(self, node);
    }
}
