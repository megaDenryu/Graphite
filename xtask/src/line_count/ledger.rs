//! 台帳 (`docs/development/line_count_ledger.md`) の読み取り。
//!
//! 台帳の行として読むのは、区切り行 (`| --- |` の形) より後にある、縦棒で始まる表の
//! 本体行だけである。読めた行は綴りと区分になり、読めなかった行は違反として残す。
//! 綴りの綴り方 (逆引用符の有無等) を候補の条件にしないのは、綴り方を間違えた行を
//! 黙って対象から外さないためである。

mod entry;
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;
use std::error::Error;

use crate::repository_root::RepositoryRoot;

pub(crate) use entry::{ExceptionCategory, LedgerEntry};

// 台帳1つ分。綴りから1件を引く表と、読めなかった行を持つ。
pub(crate) struct LineCountLedger {
    entries: BTreeMap<String, LedgerEntry>,
    invalid_rows: Vec<String>,
}

impl LineCountLedger {
    pub(crate) fn read_from(root: &RepositoryRoot) -> Result<Self, Box<dyn Error>> {
        Ok(Self::of_text(&root.line_count_ledger_text()?))
    }

    // 区切り行を見つけてから後ろを本体行として読む。区切り行の直前の行が見出しである。
    pub(super) fn of_text(text: &str) -> Self {
        let mut entries = BTreeMap::new();
        let mut invalid_rows = Vec::new();
        let mut reached_body = false;
        for line in text.lines().map(str::trim) {
            if !line.starts_with('|') {
                continue;
            }
            if is_separator_row(line) {
                reached_body = true;
                continue;
            }
            if !reached_body {
                continue;
            }
            match read_row(line) {
                Some((spelling, entry)) => {
                    entries.insert(spelling, entry);
                }
                None => invalid_rows.push(line.to_string()),
            }
        }
        Self {
            entries,
            invalid_rows,
        }
    }

    pub(crate) fn entry_of(&self, spelling: &str) -> Option<&LedgerEntry> {
        self.entries.get(spelling)
    }

    pub(crate) fn spellings(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }

    pub(crate) fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn invalid_rows(&self) -> &[String] {
        &self.invalid_rows
    }
}

// 枠に分ける。行頭と行末の縦棒を落としてから縦棒で切る。
fn cells_of(line: &str) -> Vec<&str> {
    line.trim_matches('|').split('|').map(str::trim).collect()
}

// 全ての枠が `-` と `:` だけでできている行を区切り行とみなす。
fn is_separator_row(line: &str) -> bool {
    cells_of(line)
        .iter()
        .all(|cell| !cell.is_empty() && cell.chars().all(|c| c == '-' || c == ':'))
}

// `| `綴り` | 区分 | 根拠 |` の1行から綴りと1件を読む。綴りを囲む逆引用符が欠けた行と
// 根拠が空の行は、黙って読み飛ばさずに読めない行として返す。
fn read_row(line: &str) -> Option<(String, LedgerEntry)> {
    let cells = cells_of(line);
    if cells.len() < 3 || cells[2].is_empty() {
        return None;
    }
    let spelling = cells[0].strip_prefix('`')?.strip_suffix('`')?;
    let entry = LedgerEntry::from_cells(cells[1], cells[2])?;
    (!spelling.is_empty()).then(|| (spelling.to_string(), entry))
}
