use super::super::ledger::LineCountLedger;
use super::LineCountReport;

// 綴り1件だけを載せた台帳を組み立てる。
fn 台帳(spelling: &str) -> LineCountLedger {
    let text = format!(
        "| ファイル | 区分 | 根拠 |\n| --- | --- | --- |\n| `{spelling}` | 統合による超過 | 根拠 |\n"
    );
    LineCountLedger::of_text(&text)
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
