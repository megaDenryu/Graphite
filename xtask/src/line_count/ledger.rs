//! 台帳 (`docs/development/line_count_ledger.md`) の読み取り。
//!
//! 台帳の行として読むのは、行頭が「縦棒・空白・逆引用符」で始まる表の行だけで
//! ある。読めた行は綴りと区分になり、読めなかった行は違反として残す。

use std::collections::BTreeMap;
use std::error::Error;

use crate::repository_root::RepositoryRoot;

const ROW_PREFIX: &str = "| `";

// 台帳が定める、超過を許す区分。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExceptionCategory {
    Consolidated,
    DeclarativeData,
    AwaitingRedesign,
}

impl ExceptionCategory {
    fn from_cell(text: &str) -> Option<Self> {
        match text {
            "統合による超過" => Some(Self::Consolidated),
            "宣言的データリテラル" => Some(Self::DeclarativeData),
            "再設計待ち" => Some(Self::AwaitingRedesign),
            _ => None,
        }
    }

    // 宣言的データリテラルだけは150行の上限を適用しない (規約の別枠)。
    pub(crate) fn applies_upper_limit(self) -> bool {
        !matches!(self, Self::DeclarativeData)
    }

    pub(crate) fn awaits_redesign(self) -> bool {
        matches!(self, Self::AwaitingRedesign)
    }
}

// 台帳1つ分。綴りから区分を引く表と、読めなかった行を持つ。
pub(crate) struct LineCountLedger {
    categories: BTreeMap<String, ExceptionCategory>,
    invalid_rows: Vec<String>,
}

impl LineCountLedger {
    pub(crate) fn read_from(root: &RepositoryRoot) -> Result<Self, Box<dyn Error>> {
        let text = root.line_count_ledger_text()?;
        let mut categories = BTreeMap::new();
        let mut invalid_rows = Vec::new();
        for line in text.lines() {
            if !line.starts_with(ROW_PREFIX) {
                continue;
            }
            match read_row(line) {
                Some((spelling, category)) => {
                    categories.insert(spelling, category);
                }
                None => invalid_rows.push(line.trim().to_string()),
            }
        }
        Ok(Self {
            categories,
            invalid_rows,
        })
    }

    pub(crate) fn category_of(&self, spelling: &str) -> Option<ExceptionCategory> {
        self.categories.get(spelling).copied()
    }

    pub(crate) fn spellings(&self) -> impl Iterator<Item = &String> {
        self.categories.keys()
    }

    pub(crate) fn entry_count(&self) -> usize {
        self.categories.len()
    }

    pub(crate) fn invalid_rows(&self) -> &[String] {
        &self.invalid_rows
    }
}

// `| `綴り` | 区分 | 根拠 |` の1行から綴りと区分を読む。根拠が空なら読めない行とする。
fn read_row(line: &str) -> Option<(String, ExceptionCategory)> {
    let cells: Vec<&str> = line.trim().trim_matches('|').split('|').collect();
    if cells.len() < 3 {
        return None;
    }
    let spelling = cells[0].trim().trim_matches('`');
    let category = ExceptionCategory::from_cell(cells[1].trim())?;
    if spelling.is_empty() || cells[2].trim().is_empty() {
        return None;
    }
    Some((spelling.to_string(), category))
}
