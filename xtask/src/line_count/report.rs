//! 検査結果の集計。
//!
//! 集計する数は6種類の違反と、検査が届いた範囲 (検査したファイル数・台帳の件数・
//! 再設計待ちの件数) である。範囲を出すのは、通ったことを「全部を見た」と
//! 読み違えないようにするためである。表示は `rendering.rs` が持つ。

mod rendering;
#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use super::code_line_count::CodeLineCount;
use super::judgement::FileJudgement;
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
    // ファイル1件の数え上げを台帳と突き合わせ、判定ごとの一覧へ振り分ける。
    pub(crate) fn record(
        &mut self,
        spelling: &str,
        count: &CodeLineCount,
        ledger: &LineCountLedger,
    ) {
        self.inspected_file_count += 1;
        self.inspected_spellings.insert(spelling.to_string());
        let measured = format!("{spelling} ({}行)", count.value());
        match FileJudgement::of(count, ledger.category_of(spelling)) {
            FileJudgement::Acceptable => {}
            FileJudgement::Unregistered => self.unregistered_excesses.push(measured),
            FileJudgement::Shrunk => self.shrunk_entries.push(measured),
            FileJudgement::AwaitingRedesign => self.awaiting_redesign.push(measured),
            FileJudgement::UpperLimitExceeded => self.upper_limit_excesses.push(measured),
        }
    }

    // 読めなかったファイルも検査した綴りとして登録する。登録しないと `close` が同じ綴りを
    // 「台帳にありますが検査対象に実在しません」へも積み、1件の事故を2件に見せる。
    pub(crate) fn record_unreadable(&mut self, spelling: &str, reason: &str) {
        self.inspected_file_count += 1;
        self.inspected_spellings.insert(spelling.to_string());
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
}
