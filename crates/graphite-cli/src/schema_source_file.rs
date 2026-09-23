//! schema宣言を持つソースファイル1件の読み取りと、動的グラフのschema宣言
//! (`dynamic_graph_schema!`) の切り出し。静的グラフのschema・instanceの
//! 2段階の解決は `static_resolution` が、パッケージ全体のファイル群を
//! 横断して行う (このファイル単体では完結しない)。

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use graphite_codegen::DeclarationSite;

use crate::generated_target_path::GeneratedTargetPath;
use crate::generation_plan::GenerationPlan;
use crate::generation_tree::GenerationTree;
use crate::io_context::with_path_context;
use crate::schema_macro_collector::MacroCall;

// schema宣言を含みうる、生成元のRustファイル。
pub struct SchemaSourceFile {
    path: PathBuf,
}

impl SchemaSourceFile {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    // ファイルを読み、構文解析する。Rustとして解析できないファイルは
    // 走査から除外せず違反にする (`generate`/`generate --check` を止める)。
    // `target`・`generated`・`ui` は `GenerationTree::schema_source_files` が
    // そもそも走査対象から外しており、ここで対象外にする規則は増やさない。
    pub(crate) fn parse(
        &self,
        tree: &GenerationTree,
    ) -> Result<(String, syn::File), Box<dyn Error>> {
        let display_path = tree.relative_display(&self.path);
        let source = with_path_context(fs::read_to_string(&self.path), &display_path)?;
        let parsed_file = syn::parse_file(&source)
            .map_err(|error| format!("{display_path} をRustとして解析できません: {error}"))?;
        Ok((display_path, parsed_file))
    }

    // このファイルの動的グラフのschema宣言を計画へ積む。`calls` は
    // `parse` で得た構文木から `schema_macro_collector::collect_macro_calls`
    // で集めたもの (呼び出し側が1回だけ集め、動的・静的の両方の解決で使い回す)。
    pub(crate) fn collect_dynamic_into(
        &self,
        tree: &GenerationTree,
        display_path: &str,
        calls: &[MacroCall],
        plan: &mut GenerationPlan,
    ) -> Result<(), Box<dyn Error>> {
        // 互換の別名として生成することはせず、改名を促すエラーで止める
        // (issue #40)。指紋の計算方法は改名の前後で変わらないため、生成
        // ファイルの作り直しは要らず、宣言の名前を置換するだけでよい。
        if let Some(call) = calls.iter().find(|call| call.name == "graph_schema") {
            return Err(format!(
                "{display_path}:{}: `graph_schema!` は `dynamic_graph_schema!` へ改名されました (issue #40)。宣言の名前を置換してください (指紋の計算方法は変わらないため、生成ファイルの作り直しは不要です)。",
                call.line
            )
            .into());
        }
        for call in calls.iter().filter(|call| call.name == "dynamic_graph_schema") {
            let schema = graphite_codegen::parse_tracked_schema(call.tokens.clone())
                .map_err(|errors| self.format_errors(tree, errors))?;
            let target = self.generated_target(tree, &schema.generated_path().value())?;
            let site = DeclarationSite::new(display_path.to_string(), call.line);
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
    // (コンパイル時の `dynamic_graph_schema!` 展開と共有する唯一の判定) に委ねる。
    // ここで改めて検査するのは、この関数がファイルシステムへの書き込み先を
    // 決める境界であり、呼び出し経路によらずこの境界自身でも安全側に倒す
    // ためである。
    pub(crate) fn generated_target(
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

    pub(crate) fn format_errors(&self, tree: &GenerationTree, errors: Vec<syn::Error>) -> String {
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
