use super::super::ledger::LineCountLedger;
use super::ExceptionDeclaration;

const 定型: &str = "//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。\n\
                    //! 根拠の文。超過を許す根拠の台帳は\n\
                    //! `docs/development/line_count_ledger.md` にある。\n";

fn 台帳の1件(rationale: &str) -> LineCountLedger {
    let text = format!(
        "| ファイル | 区分 | 根拠 |\n| --- | --- | --- |\n| `a/b.rs` | 統合による超過 | {rationale} |\n"
    );
    LineCountLedger::of_text(&text)
}

fn 一致するか(header: &str, rationale: &str) -> bool {
    let ledger = 台帳の1件(rationale);
    let entry = ledger.entry_of("a/b.rs").expect("台帳の1件が引けること");
    ExceptionDeclaration::of_source_text(header).agrees_with(entry)
}

#[test]
fn 行の幅で折り返した定型を一致とみなす() {
    assert!(一致するか(定型, "根拠の文"));
}

#[test]
fn 定型の後ろにコードが続いても一致とみなす() {
    let header = format!("//! 説明。\n//!\n{定型}\nuse std::fmt;\n");
    assert!(一致するか(&header, "根拠の文"));
}

#[test]
fn 根拠が台帳と違えば一致とみなさない() {
    assert!(!一致するか(定型, "別の根拠"));
}

#[test]
fn 冒頭コメントが無ければ一致とみなさない() {
    assert!(!一致するか("use std::fmt;\n", "根拠の文"));
}

#[test]
fn コードより後ろに書いた定型を冒頭コメントとみなさない() {
    let header = format!("use std::fmt;\n{定型}");
    assert!(!一致するか(&header, "根拠の文"));
}

#[test]
fn 感嘆符を落とした冒頭コメントを一致とみなさない() {
    let header = "//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。\n\
                  //! このファイルは `flow` の構文解析一式を持つ。超過を許す根拠の台帳は\n\
                  //! `docs/development/line_count_ledger.md` にある。\n";
    assert!(!一致するか(header, "このファイルは `flow!` の構文解析一式を持つ"));
}

#[test]
fn 台帳のパスの区切りを落とした冒頭コメントを一致とみなさない() {
    let header = "//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。\n\
                  //! 根拠の文。超過を許す根拠の台帳は\n\
                  //! `docsdevelopmentline_count_ledger.md` にある。\n";
    assert!(!一致するか(header, "根拠の文"));
}

#[test]
fn 定型の固定部分を書いた冒頭コメントは例外を名乗っているとみなす() {
    assert!(ExceptionDeclaration::of_source_text(定型).claims_exception());
}

#[test]
fn 例外に触れない冒頭コメントは例外を名乗っているとみなさない() {
    let header = "//! このファイルは1ファイル100行の原則を守る。\n";
    assert!(!ExceptionDeclaration::of_source_text(header).claims_exception());
}
