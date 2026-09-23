//! `cargo xtask check-doc-comments` の実処理。
//!
//! 検査は2つある。内部領域に項目の `///` が1件も無いこと (issue #22 の撤去の
//! 進捗そのもの) と、生成コードの公開面に doc コメントが網羅されていることである。
//! どちらも syn の構文解析で判定し、解析できなかったファイルは違反として数える。

mod attribute_facts;
mod generated_inspection;
mod internal_area_derivation;
mod internal_inspection;
mod item_facts;
mod orphan_inspection;
mod package_manifest;
mod public_item_visitor;
mod public_package_report;
#[cfg(test)]
mod temporary_repository;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write;

use crate::inspected_area::InspectedArea;
use crate::repository_root::RepositoryRoot;
use crate::rust_source::RustSource;
use generated_inspection::GeneratedAreaReport;
use internal_inspection::InternalAreaReport;
use orphan_inspection::OrphanSourceReport;
use public_package_report::PublicPackageReport;

// 生成ファイルを探す起点。この下でディレクトリ名が `generated` の場所を公開面とみなす。
const GENERATED_SEARCH_ROOTS: [&str; 3] = ["crates", "examples", "verification"];

const GENERATED_DIRECTORY_NAME: &str = "generated";

// 領域の決め方を1箇所へ閉じた検査。リポジトリルートを保持する。
pub(crate) struct DocCommentInspection<'a> {
    root: &'a RepositoryRoot,
}

impl<'a> DocCommentInspection<'a> {
    pub(crate) fn new(root: &'a RepositoryRoot) -> Self {
        Self { root }
    }

    pub(crate) fn run(&self) -> Result<(), Box<dyn Error>> {
        let classification = internal_area_derivation::classify_packages(self.root)?;
        let internal = self.internal_reports(classification.internal_areas())?;
        let public = PublicPackageReport::new(classification.public_areas(), internal.len());
        let generated = self.generated_reports()?;
        let orphan_sources = self.search_root_sources()?;
        let orphans = OrphanSourceReport::inspect(orphan_sources, &classification.known_areas());

        let mut text = String::from("内部領域 (項目の `///` が1件も無いこと):\n");
        for report in &internal {
            text.push_str(&report.render());
        }
        text.push_str(&public.render());
        let _ = writeln!(
            text,
            "生成コードの公開面 (非 #[doc(hidden)] な公開項目に doc があること):"
        );
        for report in &generated {
            text.push_str(&report.render());
        }
        text.push_str(&orphans.render());
        print!("{text}");

        if internal.iter().all(InternalAreaReport::is_clean)
            && generated.iter().all(GeneratedAreaReport::is_clean)
            && orphans.is_clean()
        {
            return Ok(());
        }
        Err("doc コメントの検査に違反があります(上の一覧を参照してください)".into())
    }

    fn internal_reports(
        &self,
        areas: &[InspectedArea],
    ) -> Result<Vec<InternalAreaReport>, Box<dyn Error>> {
        let mut reports = Vec::new();
        for area in areas {
            let mut sources = self.root.rust_source_files(area)?;
            sources.retain(|source| generated_area_of(source.spelling()).is_none());
            let spelling = area.spelling().to_string();
            reports.push(InternalAreaReport::inspect(spelling, sources));
        }
        Ok(reports)
    }

    fn generated_reports(&self) -> Result<Vec<GeneratedAreaReport>, Box<dyn Error>> {
        let mut grouped: BTreeMap<String, Vec<RustSource>> = BTreeMap::new();
        for source in self.search_root_sources()? {
            if let Some(area) = generated_area_of(source.spelling()) {
                grouped.entry(area).or_default().push(source);
            }
        }
        Ok(grouped
            .into_iter()
            .map(|(spelling, sources)| GeneratedAreaReport::inspect(spelling, sources))
            .collect())
    }

    // この関数は `crates`・`examples`・`verification` 配下の Rust ソースを
    // 集める。`generated_reports` と `orphan` 検出の両方がこの結果を使う。
    fn search_root_sources(&self) -> Result<Vec<RustSource>, Box<dyn Error>> {
        let mut sources = Vec::new();
        for spelling in GENERATED_SEARCH_ROOTS {
            sources.extend(self.root.rust_source_files(&InspectedArea::at(spelling))?);
        }
        Ok(sources)
    }
}

// 綴りが `generated` ディレクトリを含むなら、そこまでを領域の綴りとして返す。
fn generated_area_of(spelling: &str) -> Option<String> {
    let segments: Vec<&str> = spelling.split('/').collect();
    let index = segments
        .iter()
        .position(|segment| *segment == GENERATED_DIRECTORY_NAME)?;
    Some(segments[..=index].join("/"))
}
