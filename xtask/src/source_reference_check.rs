//! ソース参照1件の実在と行数範囲の判定と、その違反の表示。

#[cfg(test)]
mod tests;

use std::fmt;

use crate::document_reference::ReferenceOrigin;
use crate::repository_root::RepositoryRoot;
use crate::source_reference::SourceReference;

// 1箇所に書かれた自リポジトリソースへの参照。
pub struct SourceCodeReference {
    origin: ReferenceOrigin,
    target: SourceReference,
}

impl SourceCodeReference {
    pub fn new(origin: ReferenceOrigin, target: SourceReference) -> Self {
        Self { origin, target }
    }

    pub fn target(&self) -> &SourceReference {
        &self.target
    }
}

impl fmt::Display for SourceCodeReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} -> {}", self.origin, self.target)
    }
}

// SOURCE_AREAS に該当し `.rs` を含むのに行指定などを解析できなかった、
// 1箇所の生の綴り。ワイルドカード・プレースホルダを含む綴り (ファイル群の
// 総称) はここに含めない。
pub struct UnparsableSourceReference {
    origin: ReferenceOrigin,
    token: String,
}

impl UnparsableSourceReference {
    pub fn new(origin: ReferenceOrigin, token: String) -> Self {
        Self { origin, token }
    }
}

impl fmt::Display for UnparsableSourceReference {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} -> {}", self.origin, self.token)
    }
}

// ソース参照1件の検査結果。実在しないファイルを指すか、行番号 (範囲や
// 複数指定なら最大の終了行) が実ファイルの行数を超えるか、行指定などが
// 解析できなかったかのいずれか。
pub(crate) enum Violation<'a> {
    FileMissing(&'a SourceCodeReference),
    LineOutOfRange { reference: &'a SourceCodeReference, actual_lines: usize },
    Unparsable(&'a UnparsableSourceReference),
}

impl SourceCodeReference {
    // この参照が `root` の実ファイルに対して妥当かを評価する。妥当なら
    // `None`、実在しないかファイルの行数を超えるなら違反を返す。
    //
    // `ReferenceScan::invalid_source_references` が自分の保持する
    // `source_references` と `self.root` を使ってこのメソッドを呼び、検査を
    // 完結させる。所有者 (`RepositoryRoot`) を外部引数として受け取る
    // 集約関数はここには置かない。
    pub(crate) fn evaluate(&self, root: &RepositoryRoot) -> Option<Violation<'_>> {
        let Some(actual_lines) = root.source_file_line_count(self.target()) else {
            return Some(Violation::FileMissing(self));
        };
        let last_line = self.target().line_span().last_line()?;
        (last_line > actual_lines).then_some(Violation::LineOutOfRange {
            reference: self,
            actual_lines,
        })
    }
}

// 実在しないか、行番号がファイルの行数を超えるか、解析できないソース参照の
// 一覧。整形は `Display` へ閉じる。
//
// 引用本文とコードの一致は検査しない。行番号の実在と行数範囲までが
// 検査の範囲であることは `main.rs` の使い方の説明にも明記する。
pub struct InvalidSourceReferences<'a> {
    violations: Vec<Violation<'a>>,
}

impl<'a> InvalidSourceReferences<'a> {
    pub(crate) fn new(violations: Vec<Violation<'a>>) -> Self {
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
            "実在しないか行数を超えるか解析できないソース参照が{}件あります:",
            self.violations.len()
        )?;
        for violation in &self.violations {
            match violation {
                Violation::FileMissing(reference) => {
                    writeln!(formatter, "  {reference} (ファイルが実在しません)")?;
                }
                Violation::LineOutOfRange { reference, actual_lines } => {
                    writeln!(
                        formatter,
                        "  {reference} (実ファイルは{actual_lines}行までです)"
                    )?;
                }
                Violation::Unparsable(reference) => {
                    writeln!(formatter, "  {reference} (解析できないソース参照です)")?;
                }
            }
        }
        Ok(())
    }
}
