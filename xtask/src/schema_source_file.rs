use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::path::{Component, Path, PathBuf};

use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};

use crate::generation_plan::GenerationPlan;
use crate::repository_root::RepositoryRoot;

/// schema宣言を含みうる、生成元のRustファイル。
pub struct SchemaSourceFile {
    path: PathBuf,
}

impl SchemaSourceFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// このファイルの schema宣言を読み、生成すべき内容を計画へ積む。
    pub fn collect_into(
        &self,
        root: &RepositoryRoot,
        plan: &mut GenerationPlan,
    ) -> Result<(), Box<dyn Error>> {
        let source = fs::read_to_string(&self.path)?;
        let mut collector = SchemaMacroCollector::default();
        collector.visit_file(&syn::parse_file(&source)?);
        for invocation in collector.invocations {
            let schema = graphite_codegen::parse_tracked_schema(invocation.tokens)
                .map_err(|errors| self.format_errors(errors))?;
            let target = self.generated_target(Path::new(&schema.generated_path().value()))?;
            let content =
                schema.render_module_source(&root.relative_display(&self.path), invocation.line)?;
            plan.add(root, target, content)?;
        }
        Ok(())
    }

    /// 宣言元から見た相対指定を検査し、生成先の絶対パスへ変換する。
    fn generated_target(&self, relative: &Path) -> Result<PathBuf, Box<dyn Error>> {
        let mut components = relative.components();
        if components.next() != Some(Component::Normal(OsStr::new("generated")))
            || components.any(|component| !matches!(component, Component::Normal(_)))
            || relative.extension() != Some(OsStr::new("rs"))
        {
            return Err(format!(
                "{} の生成先は `generated/<名前>.rs` の形式で指定してください",
                self.path.display()
            )
            .into());
        }
        Ok(self
            .path
            .parent()
            .expect("Rustファイルには親ディレクトリがある")
            .join(relative))
    }

    fn format_errors(&self, errors: Vec<syn::Error>) -> String {
        let details = errors
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "{} のschemaを生成できません:\n{details}",
            self.path.display()
        )
    }
}

/// 追跡形式の `graph_schema!` 呼び出しと、その宣言行。
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
