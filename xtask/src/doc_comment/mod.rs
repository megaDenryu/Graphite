//! `cargo xtask check-doc-comments` の実処理。
//!
//! 検査は2つある。内部領域に項目の `///` が1件も無いこと (issue #22 の撤去の
//! 進捗そのもの) と、生成コードの公開面に doc コメントが網羅されていることである。
//! どちらも syn の構文解析で判定し、解析できなかったファイルは違反として数える。

mod area;
mod attribute_facts;
mod generated_inspection;
mod internal_inspection;
mod item_facts;
mod public_item_visitor;
mod rust_source;

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write;

use crate::repository_root::RepositoryRoot;
use generated_inspection::GeneratedAreaReport;
use internal_inspection::InternalAreaReport;

pub(crate) use area::InspectedArea;
pub(crate) use rust_source::RustSource;

// 撤去の対象になる内部領域。`graphite` と生成コードだけが公開面である。
const INTERNAL_AREAS: [&str; 5] = [
    "crates/graphite-codegen",
    "crates/graphite-cli",
    "crates/graphite-macros",
    "xtask",
    "examples",
];

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
        let internal = self.internal_reports()?;
        let generated = self.generated_reports()?;
        let mut text = String::from("内部領域 (項目の `///` が1件も無いこと):\n");
        for report in &internal {
            text.push_str(&report.render());
        }
        let _ = writeln!(
            text,
            "生成コードの公開面 (非 #[doc(hidden)] な公開項目に doc があること):"
        );
        for report in &generated {
            text.push_str(&report.render());
        }
        print!("{text}");
        if internal.iter().all(InternalAreaReport::is_clean)
            && generated.iter().all(GeneratedAreaReport::is_clean)
        {
            return Ok(());
        }
        Err("doc コメントの検査に違反があります(上の一覧を参照してください)".into())
    }

    fn internal_reports(&self) -> Result<Vec<InternalAreaReport>, Box<dyn Error>> {
        let mut reports = Vec::new();
        for spelling in INTERNAL_AREAS {
            let mut sources = self.root.rust_source_files(&InspectedArea::at(spelling))?;
            sources.retain(|source| generated_area_of(source.spelling()).is_none());
            reports.push(InternalAreaReport::inspect(spelling.to_string(), sources));
        }
        Ok(reports)
    }

    fn generated_reports(&self) -> Result<Vec<GeneratedAreaReport>, Box<dyn Error>> {
        let mut grouped: BTreeMap<String, Vec<RustSource>> = BTreeMap::new();
        for spelling in GENERATED_SEARCH_ROOTS {
            for source in self.root.rust_source_files(&InspectedArea::at(spelling))? {
                if let Some(area) = generated_area_of(source.spelling()) {
                    grouped.entry(area).or_default().push(source);
                }
            }
        }
        Ok(grouped
            .into_iter()
            .map(|(spelling, sources)| GeneratedAreaReport::inspect(spelling, sources))
            .collect())
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
