use super::super::code_line_count::CodeLineCount;
use super::super::exception_declaration::ExceptionDeclaration;
use super::super::ledger::LineCountLedger;
use super::LineCountReport;

// 綴り1件だけを載せた台帳を組み立てる。
fn 台帳(spelling: &str) -> LineCountLedger {
    let text = format!(
        "| ファイル | 区分 | 根拠 |\n| --- | --- | --- |\n| `{spelling}` | 統合による超過 | 根拠 |\n"
    );
    LineCountLedger::of_text(&text)
}

// 行を1件も持たない台帳を組み立てる。
fn 空の台帳() -> LineCountLedger {
    LineCountLedger::of_text("| ファイル | 区分 | 根拠 |\n| --- | --- | --- |\n")
}

// 綴り1件を、その本文とともに検査結果へ記録する。
fn 記録する(report: &mut LineCountReport, spelling: &str, text: &str, ledger: &LineCountLedger) {
    report.record(
        spelling,
        &CodeLineCount::of_text(text),
        &ExceptionDeclaration::of_source_text(text),
        ledger,
    );
}

#[test]
fn 台帳に無いのに例外を名乗るファイルを違反として報告する() {
    let ledger = 空の台帳();
    let mut report = LineCountReport::default();
    let 本文 = "//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。\n";
    記録する(&mut report, "c/d.rs", 本文, &ledger);
    report.close(&ledger);
    assert!(!report.is_clean());
    let text = report.render(&ledger);
    assert!(
        text.contains("台帳に無いのに例外を名乗っています (1件)"),
        "{text}"
    );
    assert!(text.contains("c/d.rs"), "{text}");
}

#[test]
fn 例外を名乗らない100行以内のファイルを違反として報告しない() {
    let ledger = 空の台帳();
    let mut report = LineCountReport::default();
    記録する(&mut report, "c/d.rs", "//! 説明。\nlet x = 1;\n", &ledger);
    report.close(&ledger);
    assert!(report.is_clean(), "{}", report.render(&ledger));
}

#[test]
fn 一致しない冒頭コメントの診断へ期待する文を出す() {
    let ledger = 台帳("a/b.rs");
    let mut report = LineCountReport::default();
    記録する(&mut report, "a/b.rs", "//! 別の説明。\n", &ledger);
    let text = report.render(&ledger);
    assert!(
        text.contains(
            "期待する文: このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。根拠。超過を許す根拠の台帳は"
        ),
        "{text}"
    );
    assert!(
        text.contains("冒頭コメントは最初の空行で途切れます"),
        "{text}"
    );
}

#[test]
fn 読めなかったファイルを実在しない綴りとして二重に報告しない() {
    let ledger = 台帳("a/b.rs");
    let mut report = LineCountReport::default();
    report.record_unreadable("a/b.rs", "不正なUTF-8");
    report.close(&ledger);
    let text = report.render(&ledger);
    assert!(text.contains("読み込みに失敗しました (1件)"), "{text}");
    assert!(
        !text.contains("台帳にありますが検査対象に実在しません"),
        "{text}"
    );
}

#[test]
fn 台帳にあり走査にも現れなかった綴りを実在しない綴りとして報告する() {
    let ledger = 台帳("a/b.rs");
    let mut report = LineCountReport::default();
    report.close(&ledger);
    let text = report.render(&ledger);
    assert!(
        text.contains("台帳にありますが検査対象に実在しません (1件)"),
        "{text}"
    );
}
