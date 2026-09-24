//! 静的schema名簿とその組み立て口 (issue #41 段階3 §2 第1段階)。
//!
//! `static_graph_schema!` の呼び出しを1件ずつ追加し、同じCargo target
//! (`cargo_target` 参照) の中で名前が重複したら追加そのものをエラーに
//! する。target が違えば同名のschemaを許す (Rustのmoduleが実際に別なら
//! 別のマクロスコープとして扱える余地があるため。判定に使う探索の単位を
//! target にする裁定は `docs/static_graph.md`「制約」節を参照)。組み立て口は
//! `静的schema名簿ビルダー` だけであり、完成した名簿 (`静的schema名簿`) は
//! 読み取り (`探す`・`len`・`いずれかのtargetに存在するか`) しかできない。

use std::collections::BTreeMap;
use std::error::Error;

use graphite_codegen::{DeclarationSite, TrackedStaticSchema};

use crate::cargo_target::CargoTarget;
use crate::generation_plan::GenerationPlan;
use crate::generation_tree::GenerationTree;
use crate::schema_macro_collector::MacroCall;
use crate::schema_source_file::SchemaSourceFile;

struct 登録済みschema {
    tracked: TrackedStaticSchema,
    site: DeclarationSite,
}

pub(crate) struct 静的schema名簿 {
    entries: BTreeMap<(CargoTarget, String), 登録済みschema>,
}

impl 静的schema名簿 {
    pub(crate) fn 探す(
        &self,
        target: &CargoTarget,
        name: &str,
    ) -> Option<(&TrackedStaticSchema, &DeclarationSite)> {
        self.entries
            .get(&(target.clone(), name.to_string()))
            .map(|entry| (&entry.tracked, &entry.site))
    }

    // targetを問わず、この名前のschemaがどこかに存在するかを見る。見つからない
    // instance候補のエラー文で「別のtargetにある」ことを案内するために使う
    // (単なる綴り誤りとの区別)。
    pub(crate) fn 他のtargetの一致先(&self, target: &CargoTarget, name: &str) -> Option<&CargoTarget> {
        self.entries
            .keys()
            .find(|(存在target, 存在名)| 存在名 == name && 存在target != target)
            .map(|(存在target, _)| 存在target)
    }

    // targetを問わず、この名前のschemaが名簿のどこかに存在するかだけを見る。
    // 「他のマクロの入力へ埋め込まれていないか」の検査 (`embedded_detection`)
    // は、instanceの解決と違って対象ファイルのtargetを問わず、名簿にある
    // 名前を安全側に倒して広く検出する。
    pub(crate) fn 名前が存在するか(&self, name: &str) -> bool {
        self.entries.keys().any(|(_, 存在名)| 存在名 == name)
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }
}

#[derive(Default)]
pub(crate) struct 静的schema名簿ビルダー {
    entries: BTreeMap<(CargoTarget, String), 登録済みschema>,
}

impl 静的schema名簿ビルダー {
    pub(crate) fn 追加する(
        &mut self,
        tree: &GenerationTree,
        source: &SchemaSourceFile,
        display_path: &str,
        target: &CargoTarget,
        call: &MacroCall,
        plan: &mut GenerationPlan,
    ) -> Result<(), Box<dyn Error>> {
        let tracked = graphite_codegen::parse_tracked_static_schema(call.tokens.clone())
            .map_err(|errors| source.format_errors(tree, errors))?;
        let name = tracked.schema_name().to_string();
        let site = DeclarationSite::new(display_path.to_string(), call.line);
        let key = (target.clone(), name.clone());

        if let Some(既存) = self.entries.get(&key) {
            return Err(format!(
                "静的グラフのschema名 `{name}` がCargo target `{}` の中で重複しています: {} と {}",
                target.表示(),
                既存.site.display(),
                site.display()
            )
            .into());
        }

        let generated_target = source.generated_target(tree, &tracked.generated_path().value())?;
        let content = tracked
            .render_module_source(&site)
            .map_err(|error| source.format_errors(tree, vec![error]))?;
        plan.add(tree, generated_target, content)?;

        self.entries.insert(key, 登録済みschema { tracked, site });
        Ok(())
    }

    pub(crate) fn 完成する(self) -> 静的schema名簿 {
        静的schema名簿 { entries: self.entries }
    }
}
