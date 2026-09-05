//! `cargo xtask check-line-counts` の実処理。
//!
//! 検査は台帳 (`docs/development/line_count_ledger.md`) と実際のコード行数の両方を
//! 見る。台帳に無い超過も、台帳に残ったまま100行以内へ収まったファイルも違反に
//! する。読めなかったファイルは対象から外さず違反として数える。

mod code_line_count;
mod judgement;
mod ledger;
mod report;

use std::error::Error;

use crate::inspected_area::InspectedArea;
use crate::repository_root::RepositoryRoot;
use crate::rust_source::RustSource;
use code_line_count::CodeLineCount;
use ledger::LineCountLedger;
use report::LineCountReport;

// 数える対象の領域。この4つの配下の `.rs` を全部数える。
const INSPECTED_AREAS: [&str; 4] = ["crates", "xtask", "examples", "verification"];

// 生成物は人が分割する対象ではないため、この名前のディレクトリの配下を除く。
const GENERATED_DIRECTORY_NAME: &str = "generated";

// 対象の決め方を1箇所へ閉じた検査。リポジトリルートを保持する。
pub(crate) struct LineCountInspection<'a> {
    root: &'a RepositoryRoot,
}

impl<'a> LineCountInspection<'a> {
    pub(crate) fn new(root: &'a RepositoryRoot) -> Self {
        Self { root }
    }

    pub(crate) fn run(&self) -> Result<(), Box<dyn Error>> {
        let ledger = LineCountLedger::read_from(self.root)?;
        let mut report = LineCountReport::default();
        for source in self.sources()? {
            match source.read_text() {
                Ok(text) => {
                    report.record(source.spelling(), &CodeLineCount::of_text(&text), &ledger)
                }
                Err(reason) => report.record_unreadable(source.spelling(), &reason),
            }
        }
        report.close(&ledger);
        print!("{}", report.render(&ledger));
        if report.is_clean() {
            return Ok(());
        }
        Err("1ファイル100行の原則の検査に違反があります(上の一覧を参照してください)".into())
    }

    // 対象領域の `.rs` を綴り順で集める。生成物だけを除く。
    fn sources(&self) -> Result<Vec<RustSource>, Box<dyn Error>> {
        let mut sources = Vec::new();
        for area in INSPECTED_AREAS {
            for source in self.root.rust_source_files(&InspectedArea::at(area))? {
                if !is_generated(source.spelling()) {
                    sources.push(source);
                }
            }
        }
        Ok(sources)
    }
}

// 綴りが `generated` ディレクトリを含むか。
fn is_generated(spelling: &str) -> bool {
    spelling
        .split('/')
        .any(|segment| segment == GENERATED_DIRECTORY_NAME)
}
