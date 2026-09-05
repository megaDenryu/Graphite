//! schema 宣言を持つソースファイル1件の読み取りと、宣言の切り出し。

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use graphite_codegen::DeclarationSite;
use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

use crate::generated_target_path::GeneratedTargetPath;
use crate::generation_plan::GenerationPlan;
use crate::io_context::with_path_context;
use crate::generation_tree::GenerationTree;

// schema宣言を含みうる、生成元のRustファイル。
pub struct SchemaSourceFile {
    path: PathBuf,
}

impl SchemaSourceFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    // このファイルの schema宣言を読み、生成すべき内容を計画へ積む。
    //
    // このファイル自体がRustとして解析できない場合 (走査対象はディレクトリ木
    // 全体であり、schemaと無関係な壊れたファイルも対象に入りうる) は、全体を
    // 止めずに警告を表示してこのファイルの処理だけを飛ばす。読み取り失敗と、
    // schema宣言を含むファイルの検証エラーは全体を止める。
    pub fn collect_into(
        &self,
        tree: &GenerationTree,
        plan: &mut GenerationPlan,
    ) -> Result<(), Box<dyn Error>> {
        let display_path = tree.relative_display(&self.path);
        let source = with_path_context(fs::read_to_string(&self.path), &display_path)?;
        let parsed_file = match syn::parse_file(&source) {
            Ok(parsed_file) => parsed_file,
            Err(error) => {
                eprintln!(
                    "警告: {display_path} をRustとして解析できないため、schema探索から除外しました: {error}"
                );
                return Ok(());
            }
        };
        let mut collector = SchemaMacroCollector::default();
        collector.visit_file(&parsed_file);
        for invocation in collector.invocations {
            let schema = graphite_codegen::parse_tracked_schema(invocation.tokens)
                .map_err(|errors| self.format_errors(tree, errors))?;
            let target = self.generated_target(tree, &schema.generated_path().value())?;
            let site = DeclarationSite::new(display_path.clone(), invocation.line);
            let content = schema
                .render_module_source(&site)
                .map_err(|error| self.format_errors(tree, vec![error]))?;
            plan.add(tree, target, content)?;
        }
        Ok(())
    }

    // 宣言元から見た相対指定を検査し、生成先の絶対パスへ変換する。
    //
    // 形式検査そのものは `graphite_codegen::validate_generated_relative_path`
    // (コンパイル時の `graph_schema!` 展開と共有する唯一の判定) に委ねる。
    // ここで改めて検査するのは、この関数がファイルシステムへの書き込み先を
    // 決める境界であり、呼び出し経路によらずこの境界自身でも安全側に倒す
    // ためである。
    fn generated_target(
        &self,
        tree: &GenerationTree,
        relative: &str,
    ) -> Result<GeneratedTargetPath, Box<dyn Error>> {
        graphite_codegen::validate_generated_relative_path(relative)
            .map_err(|reason| format!("{}: {reason}", tree.relative_display(&self.path)))?;
        let target = self
            .path
            .parent()
            .expect("Rustファイルには親ディレクトリがある")
            .join(relative);
        Ok(GeneratedTargetPath::new(target))
    }

    fn format_errors(&self, tree: &GenerationTree, errors: Vec<syn::Error>) -> String {
        let details = errors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "{} のschemaを生成できません:\n{details}",
            tree.relative_display(&self.path)
        )
    }
}

// 追跡形式の `graph_schema!` 呼び出しと、その宣言行。
struct SchemaInvocation {
    tokens: TokenStream,
    line: usize,
}

#[derive(Default)]
struct SchemaMacroCollector {
    invocations: Vec<SchemaInvocation>,
}

impl<'ast> Visit<'ast> for SchemaMacroCollector {
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        if node
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident == "graph_schema")
        {
            self.invocations.push(SchemaInvocation {
                tokens: node.tokens.clone(),
                line: node.span().start().line,
            });
        }
        visit::visit_macro(self, node);
    }
}
