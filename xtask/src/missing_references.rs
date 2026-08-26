use std::fmt;

use crate::document_reference::DocumentReference;

/// 実在しない綴りを指している参照の一覧。整形は `Display` へ閉じる。
///
/// `InvalidSourceReferences` と対の形である: 検査結果を保持し、
/// 呼び出し側は組み立てと委譲だけを行う。
pub struct MissingReferences<'a> {
    references: Vec<&'a DocumentReference>,
}

impl<'a> MissingReferences<'a> {
    pub(crate) fn new(references: Vec<&'a DocumentReference>) -> Self {
        Self { references }
    }

    pub fn is_empty(&self) -> bool {
        self.references.is_empty()
    }
}

impl fmt::Display for MissingReferences<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.references.is_empty() {
            return Ok(());
        }
        writeln!(formatter, "実在しない文書を指す参照が{}件あります:", self.references.len())?;
        for reference in &self.references {
            writeln!(formatter, "  {reference}")?;
        }
        Ok(())
    }
}
