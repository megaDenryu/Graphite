use std::path::PathBuf;

use crate::repository_root::RepositoryRoot;

use super::{ExceptionCategory, LedgerEntry, LineCountLedger};

const 見出しと区切り: &str = "| ファイル | 区分 | 根拠 |\n| --- | --- | --- |\n";

fn 台帳(rows: &str) -> LineCountLedger {
    LineCountLedger::of_text(&format!("{見出しと区切り}{rows}"))
}

#[test]
fn 整った行を綴りと区分として読む() {
    let ledger = 台帳("| `a/b.rs` | 統合による超過 | 根拠 |\n");
    assert_eq!(ledger.entry_count(), 1);
    assert!(ledger.invalid_rows().is_empty());
    let entry = ledger.entry_of("a/b.rs").expect("読めた行が引けること");
    assert_eq!(entry.category(), ExceptionCategory::Consolidated);
    assert!(entry
        .declaration_sentences()
        .contains("(区分: 統合による超過)。根拠。"));
}

#[test]
fn 逆引用符を落とした行を読めない行として報告する() {
    let ledger = 台帳("| a/b.rs | 統合による超過 | 根拠 |\n");
    assert_eq!(ledger.entry_count(), 0);
    assert_eq!(ledger.invalid_rows().len(), 1);
    assert_eq!(
        ledger.invalid_rows()[0],
        "| a/b.rs | 統合による超過 | 根拠 |"
    );
}

#[test]
fn 区分の綴りが違う行を読めない行として報告する() {
    let ledger = 台帳("| `a/b.rs` | 統合超過 | 根拠 |\n");
    assert_eq!(ledger.invalid_rows().len(), 1);
}

#[test]
fn 根拠が空の行を読めない行として報告する() {
    let ledger = 台帳("| `a/b.rs` | 統合による超過 |  |\n");
    assert_eq!(ledger.invalid_rows().len(), 1);
}

#[test]
fn 見出し行と区切り行を読めない行として報告しない() {
    let ledger = 台帳("");
    assert_eq!(ledger.entry_count(), 0);
    assert!(ledger.invalid_rows().is_empty());
}

#[test]
fn 表の外の散文を台帳の行として読まない() {
    let ledger = LineCountLedger::of_text("この文書は台帳である。\n\n| a | b |\n");
    assert_eq!(ledger.entry_count(), 0);
    assert!(ledger.invalid_rows().is_empty());
}

// この検査が無いと、検査器が組み立てる定型と文書が書き手へ示す定型が黙って食い違う。
// 書き手は文書のとおりに書いたのに違反を出されることになる。
#[test]
fn 検査器が組み立てる定型は台帳の文書が示す定型と一致する() {
    let root = RepositoryRoot::at(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".."))
        .expect("xtaskの実行場所からリポジトリルートを解決できること");
    let 文書 = root.line_count_ledger_text().expect("台帳の文書を読めること");
    let 文書の定型 = 文書
        .lines()
        .find(|line| line.starts_with("このファイルは1ファイル100行の原則の例外である"))
        .expect("台帳の文書の「冒頭コメントの宣言」節が定型の文を書いていること")
        .replace("<区分>", "統合による超過")
        .replace("<根拠>", "根拠");
    let entry =
        LedgerEntry::from_cells("統合による超過", "根拠").expect("台帳の1件を組み立てられること");
    assert_eq!(entry.declaration_sentences(), 文書の定型);
}
