use std::fmt;

use crate::document_reference::SourceCodeReference;
use crate::repository_root::RepositoryRoot;

/// ソース参照1件の検査結果。実在しないファイルを指すか、行番号(範囲なら
/// 終了行)が実ファイルの行数を超えるかのいずれか。
enum Violation<'a> {
    FileMissing(&'a SourceCodeReference),
    LineOutOfRange {
        reference: &'a SourceCodeReference,
        actual_lines: usize,
    },
}

/// 実在しないか、行番号がファイルの行数を超えるソース参照の一覧。
///
/// 引用本文とコードの一致は検査しない。行番号の実在と行数範囲までが
/// 検査の範囲であることは `main.rs` の使い方の説明にも明記する。
pub struct InvalidSourceReferences<'a> {
    violations: Vec<Violation<'a>>,
}

impl<'a> InvalidSourceReferences<'a> {
    pub fn collect(references: &'a [SourceCodeReference], root: &RepositoryRoot) -> Self {
        let mut violations = Vec::new();
        for reference in references {
            match root.source_file_line_count(reference.target().path()) {
                None => violations.push(Violation::FileMissing(reference)),
                Some(actual_lines) => {
                    if let Some(last_line) = reference.target().line_span().last_line() {
                        if last_line > actual_lines {
                            violations.push(Violation::LineOutOfRange {
                                reference,
                                actual_lines,
                            });
                        }
                    }
                }
            }
        }
        Self { violations }
    }

    pub fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for InvalidSourceReferences<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.violations.is_empty() {
            return Ok(());
        }
        writeln!(
            formatter,
            "実在しないか行数を超えるソース参照が{}件あります:",
            self.violations.len()
        )?;
        for violation in &self.violations {
            match violation {
                Violation::FileMissing(reference) => {
                    writeln!(formatter, "  {reference} (ファイルが実在しません)")?;
                }
                Violation::LineOutOfRange {
                    reference,
                    actual_lines,
                } => {
                    writeln!(
                        formatter,
                        "  {reference} (実ファイルは{actual_lines}行までです)"
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::InvalidSourceReferences;
    use crate::document_reference::{ReferenceOrigin, SourceCodeReference};
    use crate::repository_root::RepositoryRoot;
    use crate::source_reference::SourceReference;

    fn root() -> RepositoryRoot {
        let repository_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
        RepositoryRoot::at(repository_root)
            .expect("xtaskの実行場所からリポジトリルートを解決できること")
    }

    fn reference(token: &str) -> SourceCodeReference {
        let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
        SourceCodeReference::new(origin, SourceReference::parse(token).unwrap())
    }

    #[test]
    fn 実在し行数内のソース参照は違反にしない() {
        let root = root();
        let references = vec![reference("xtask/src/main.rs")];
        assert!(InvalidSourceReferences::collect(&references, &root).is_empty());
    }

    #[test]
    fn 実在しないファイルは違反になる() {
        let root = root();
        let references = vec![reference("xtask/src/存在しない.rs")];
        let invalid = InvalidSourceReferences::collect(&references, &root);
        assert!(!invalid.is_empty());
        assert!(invalid.to_string().contains("ファイルが実在しません"));
    }

    #[test]
    fn 行数を超える指定は違反になる() {
        let root = root();
        let references = vec![reference("xtask/src/source_reference_check.rs:999999")];
        let invalid = InvalidSourceReferences::collect(&references, &root);
        assert!(!invalid.is_empty());
        assert!(invalid.to_string().contains("行までです"));
    }
}
