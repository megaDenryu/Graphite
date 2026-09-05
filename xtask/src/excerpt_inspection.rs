//! 引用全件へ2つの判定を掛け、違反と、検査が届いた範囲を集計する。
//!
//! 集計を判定と同じ場所へ置かないのは、報告する数が「判定の結果」ではなく
//! 「判定がどこまで届いたか」だからである。照合できた引用の件数と行数、参照先が
//! 実在せず照合できなかった引用の件数を、違反0件のときも報告へ出す。

use std::fmt;

use crate::quoted_excerpt::QuotedExcerpt;
use crate::quoted_excerpt_check::ExcerptMismatch;
use crate::repository_root::RepositoryRoot;

// 引用全件の照合結果。違反と、照合が届いた範囲を持つ。
pub struct ExcerptInspection<'a> {
    compared_excerpts: usize,
    compared_lines: usize,
    unreadable_excerpts: usize, // 参照先が実在せず照合できなかった引用
    mismatches: Vec<ExcerptMismatch<'a>>,
}

impl<'a> ExcerptInspection<'a> {
    // 引用を1件ずつ参照先の本文と突き合わせる。
    //
    // 参照先が実在しない引用は照合できないため、違反にはせず件数だけを数える。
    // ファイルの不在は `SourceCodeReference::evaluate` が別に違反として報告する
    // ので、ここで二重に報告しない。
    pub(crate) fn over(excerpts: &'a [QuotedExcerpt], root: &RepositoryRoot) -> Self {
        let mut inspection = Self {
            compared_excerpts: 0,
            compared_lines: 0,
            unreadable_excerpts: 0,
            mismatches: Vec::new(),
        };
        for excerpt in excerpts {
            inspection.compare_one(excerpt, root);
        }
        inspection
    }

    fn compare_one(&mut self, excerpt: &'a QuotedExcerpt, root: &RepositoryRoot) {
        let target = excerpt.target();
        let (Some(source_lines), Some(source_text)) =
            (root.source_file_lines(target), root.source_file_text(target))
        else {
            self.unreadable_excerpts += 1;
            return;
        };
        self.compared_excerpts += 1;
        self.compared_lines += excerpt.line_count();
        self.mismatches.extend(ExcerptMismatch::judge(excerpt, &source_lines, &source_text));
    }

    // 参照先の本文と突き合わせた引用の件数。
    pub fn compared_excerpt_count(&self) -> usize {
        self.compared_excerpts
    }

    // 参照先の本文と突き合わせた引用行の総数。
    pub fn compared_line_count(&self) -> usize {
        self.compared_lines
    }

    // 参照先が実在せず照合できなかった引用の件数。
    pub fn unreadable_excerpt_count(&self) -> usize {
        self.unreadable_excerpts
    }

    pub fn is_empty(&self) -> bool {
        self.mismatches.is_empty()
    }
}

impl fmt::Display for ExcerptInspection<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mismatches.is_empty() {
            return Ok(());
        }
        writeln!(
            formatter,
            "参照先と一致しない引用が{}件あります:",
            self.mismatches.len()
        )?;
        for mismatch in &self.mismatches {
            write!(formatter, "{mismatch}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::ExcerptInspection;
    use crate::document_reference::ReferenceOrigin;
    use crate::quoted_excerpt::QuotedExcerpt;
    use crate::repository_root::RepositoryRoot;
    use crate::source_reference::SourceReference;

    fn root() -> RepositoryRoot {
        let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        RepositoryRoot::at(repository_root)
            .expect("xtaskの実行場所からリポジトリルートを解決できること")
    }

    // `lines[0]` を参照が書かれた行とみなして引用1件を取り込む。
    fn excerpt(token: &str, lines: &[&str]) -> Vec<QuotedExcerpt> {
        let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
        let target = SourceReference::parse(token).unwrap();
        vec![QuotedExcerpt::following_fence(lines, 0, origin, target)
            .expect("引用として取り込むこと")]
    }

    #[test]
    fn 参照先が実在する引用は照合した件数と行数に数える() {
        let lines = ["参照", "```rust", "mod quoted_excerpt;", "```"];
        let excerpts = excerpt("xtask/src/lib.rs:9-25", &lines);
        let inspection = ExcerptInspection::over(&excerpts, &root());
        assert_eq!(inspection.compared_excerpt_count(), 1);
        assert_eq!(inspection.compared_line_count(), 1);
        assert_eq!(inspection.unreadable_excerpt_count(), 0);
        assert!(inspection.is_empty());
    }

    #[test]
    fn 参照先が実在しない引用は照合した行数に数えない() {
        let lines = ["参照", "```rust", "mod quoted_excerpt;", "```"];
        let excerpts = excerpt("xtask/src/存在しない.rs:1-5", &lines);
        let inspection = ExcerptInspection::over(&excerpts, &root());
        assert_eq!(inspection.compared_excerpt_count(), 0);
        assert_eq!(inspection.compared_line_count(), 0);
        assert_eq!(inspection.unreadable_excerpt_count(), 1);
        assert!(inspection.is_empty());
    }
}
