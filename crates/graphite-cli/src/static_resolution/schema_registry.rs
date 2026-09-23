//! 静的schema名簿とその組み立て口 (issue #41 段階3 §2 第1段階)。
//!
//! `static_graph_schema!` の呼び出しを1件ずつ追加し、名前が重複したら追加
//! そのものをエラーにする (パッケージ内に同名の静的schemaが2つ以上あるのは
//! 常に誤りであり、生成先の重複より早く検出する)。組み立て口は
//! `静的schema名簿ビルダー` だけであり、完成した名簿 (`静的schema名簿`) は
//! 読み取り (`探す`・`len`) しかできない。

use std::collections::BTreeMap;
use std::error::Error;

use graphite_codegen::{DeclarationSite, TrackedStaticSchema};

use crate::generation_plan::GenerationPlan;
use crate::generation_tree::GenerationTree;
use crate::schema_macro_collector::MacroCall;
use crate::schema_source_file::SchemaSourceFile;

struct 登録済みschema {
    tracked: TrackedStaticSchema,
    site: DeclarationSite,
}

pub(crate) struct 静的schema名簿 {
    entries: BTreeMap<String, 登録済みschema>,
}

impl 静的schema名簿 {
    pub(crate) fn 探す(&self, name: &str) -> Option<(&TrackedStaticSchema, &DeclarationSite)> {
        self.entries.get(name).map(|entry| (&entry.tracked, &entry.site))
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Default)]
pub(crate) struct 静的schema名簿ビルダー {
    entries: BTreeMap<String, 登録済みschema>,
}

impl 静的schema名簿ビルダー {
    pub(crate) fn 追加する(
        &mut self,
        tree: &GenerationTree,
        source: &SchemaSourceFile,
        display_path: &str,
        call: &MacroCall,
        plan: &mut GenerationPlan,
    ) -> Result<(), Box<dyn Error>> {
        let tracked = graphite_codegen::parse_tracked_static_schema(call.tokens.clone())
            .map_err(|errors| source.format_errors(tree, errors))?;
        let name = tracked.schema_name().to_string();
        let site = DeclarationSite::new(display_path.to_string(), call.line);

        if let Some(既存) = self.entries.get(&name) {
            return Err(format!(
                "静的グラフのschema名 `{name}` が重複しています: {} と {}",
                既存.site.display(),
                site.display()
            )
            .into());
        }

        let target = source.generated_target(tree, &tracked.generated_path().value())?;
        let content = tracked
            .render_module_source(&site)
            .map_err(|error| source.format_errors(tree, vec![error]))?;
        plan.add(tree, target, content)?;

        self.entries.insert(name, 登録済みschema { tracked, site });
        Ok(())
    }

    pub(crate) fn 完成する(self) -> 静的schema名簿 {
        静的schema名簿 { entries: self.entries }
    }
}
