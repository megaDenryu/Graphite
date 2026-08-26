use std::error::Error;
use std::fmt;

use crate::document_reference::{DocumentPath, ReferenceTarget};
use crate::reference_scan::tokens_in;
use crate::repository_root::RepositoryRoot;

/// 索引 (docs/README.md) が表の1列目で列挙している文書の一覧。
///
/// 1列目だけを読むのは、他の列 (現行の置換先など) が同じ文書を何度でも指す
/// ためである。全列を読むと「1文書につき1行」の判定ができない。
pub struct DocumentIndex {
    listed: Vec<DocumentPath>,
}

impl DocumentIndex {
    pub fn read_from(root: &RepositoryRoot) -> Result<Self, Box<dyn Error>> {
        let text = root.document_index_text()?;
        let mut listed = Vec::new();
        for line in text.lines() {
            let Some(cell) = first_table_cell(line) else {
                continue;
            };
            for token in tokens_in(cell) {
                if let Some(ReferenceTarget::RepositoryDocument(path)) =
                    ReferenceTarget::classify(token)
                {
                    listed.push(path);
                }
            }
        }
        Ok(Self { listed })
    }

    /// 実在するファイルの一覧と突き合わせ、過不足と重複を集める。
    pub fn compare_with(&self, existing: &[DocumentPath]) -> IndexMismatch {
        let mut absent = Vec::new();
        let mut duplicated = Vec::new();
        for document in existing {
            match self.listed.iter().filter(|listed| *listed == document).count() {
                1 => {}
                0 => absent.push(document.clone()),
                _ => duplicated.push(document.clone()),
            }
        }
        let mut phantom: Vec<DocumentPath> = self
            .listed
            .iter()
            .filter(|listed| !existing.contains(listed))
            .cloned()
            .collect();
        phantom.sort();
        phantom.dedup();
        IndexMismatch {
            absent,
            duplicated,
            phantom,
        }
    }
}

/// 索引と実ファイルの食い違い。
pub struct IndexMismatch {
    absent: Vec<DocumentPath>,
    duplicated: Vec<DocumentPath>,
    phantom: Vec<DocumentPath>,
}

impl IndexMismatch {
    pub fn is_empty(&self) -> bool {
        self.absent.is_empty() && self.duplicated.is_empty() && self.phantom.is_empty()
    }
}

impl fmt::Display for IndexMismatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_group(formatter, "索引に載っていない文書", &self.absent)?;
        write_group(formatter, "索引に2行以上ある文書", &self.duplicated)?;
        write_group(formatter, "索引にあるが実在しない文書", &self.phantom)
    }
}

fn write_group(
    formatter: &mut fmt::Formatter<'_>,
    heading: &str,
    documents: &[DocumentPath],
) -> fmt::Result {
    if documents.is_empty() {
        return Ok(());
    }
    writeln!(formatter, "{heading}:")?;
    for document in documents {
        writeln!(formatter, "  {document}")?;
    }
    Ok(())
}

/// 表の行から1列目のセルを取り出す。表でない行は `None` を返す。
fn first_table_cell(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix('|')?;
    rest.split('|').next()
}

#[cfg(test)]
mod tests {
    use super::first_table_cell;

    #[test]
    fn 表の行は1列目だけを返す() {
        let line = "| `docs/schema_v4.md` | Current reference | 構文の仕様 |";
        assert_eq!(first_table_cell(line), Some(" `docs/schema_v4.md` "));
    }

    #[test]
    fn 表でない行は対象外になる() {
        assert_eq!(first_table_cell("本文の `docs/schema_v4.md` への言及"), None);
    }
}
