//! 検査結果の集計と表示。
//!
//! 集計する数は4種類の違反と、検査が届いた範囲 (検査したファイル数・台帳の件数・
//! 再設計待ちの件数) である。範囲を出すのは、通ったことを「全部を見た」と
//! 読み違えないようにするためである。

use std::collections::BTreeSet;
use std::fmt::Write;

use super::code_line_count::CodeLineCount;
use super::ledger::LineCountLedger;

// 検査1回分の結果。違反の一覧と、検査が届いた範囲の件数を持つ。
#[derive(Default)]
pub(crate) struct LineCountReport {
    inspected_file_count: usize,
    inspected_spellings: BTreeSet<String>,
    awaiting_redesign: Vec<String>,
    unreadable_files: Vec<String>,
    unregistered_excesses: Vec<String>,
    upper_limit_excesses: Vec<String>,
    shrunk_entries: Vec<String>,
    missing_entries: Vec<String>,
    invalid_ledger_rows: Vec<String>,
}

impl LineCountReport {
    // ファイル1件の数え上げを台帳と突き合わせる。
    pub(crate) fn record(
        &mut self,
        spelling: &str,
        count: &CodeLineCount,
        ledger: &LineCountLedger,
    ) {
        self.inspected_file_count += 1;
        self.inspected_spellings.insert(spelling.to_string());
        let measured = format!("{spelling} ({}行)", count.value());
        let Some(category) = ledger.category_of(spelling) else {
            if count.exceeds_principle() {
                self.unregistered_excesses.push(measured);
            }
            return;
        };
        if !count.exceeds_principle() {
            self.shrunk_entries.push(measured);
            return;
        }
        if category.awaits_redesign() {
            self.awaiting_redesign.push(measured);
            return;
        }
        if category.applies_upper_limit() && count.exceeds_upper_limit() {
            self.upper_limit_excesses
                .push(format!("{measured} 区分: {}", category.label()));
        }
    }

    pub(crate) fn record_unreadable(&mut self, spelling: &str, reason: &str) {
        self.inspected_file_count += 1;
        self.unreadable_files.push(format!("{spelling}: {reason}"));
    }

    // 走査し終えた後に、台帳の側だけに残った行を違反として拾う。
    pub(crate) fn close(&mut self, ledger: &LineCountLedger) {
        for spelling in ledger.spellings() {
            if !self.inspected_spellings.contains(spelling) {
                self.missing_entries.push(spelling.clone());
            }
        }
        self.invalid_ledger_rows = ledger.invalid_rows().to_vec();
    }

    pub(crate) fn is_clean(&self) -> bool {
        self.unreadable_files.is_empty()
            && self.unregistered_excesses.is_empty()
            && self.upper_limit_excesses.is_empty()
            && self.shrunk_entries.is_empty()
            && self.missing_entries.is_empty()
            && self.invalid_ledger_rows.is_empty()
    }

    pub(crate) fn render(&self, ledger: &LineCountLedger) -> String {
        let mut text = format!(
            "{}ファイルのコード行を数え、台帳の{}件と突き合わせました(再設計待ち {}件)\n",
            self.inspected_file_count,
            ledger.entry_count(),
            self.awaiting_redesign.len()
        );
        for spelling in &self.awaiting_redesign {
            let _ = writeln!(text, "  再設計待ち: {spelling}");
        }
        self.render_violations(&mut text);
        text
    }

    fn render_violations(&self, text: &mut String) {
        let sections = [
            (
                "台帳に無いのに100行を超えています",
                &self.unregistered_excesses,
            ),
            ("150行の上限を超えています", &self.upper_limit_excesses),
            (
                "100行以内へ収まったので台帳から削除してください",
                &self.shrunk_entries,
            ),
            (
                "台帳にありますが検査対象に実在しません",
                &self.missing_entries,
            ),
            ("読み込みに失敗しました", &self.unreadable_files),
            ("台帳の行として読めません", &self.invalid_ledger_rows),
        ];
        for (title, entries) in sections {
            if entries.is_empty() {
                continue;
            }
            let _ = writeln!(text, "{title} ({}件):", entries.len());
            for entry in entries {
                let _ = writeln!(text, "  {entry}");
            }
        }
    }
}
