//! 検査結果の文字列表現。
//!
//! 表示の書式をこの1箇所へ集める。検査が届いた範囲を先に書き、その後ろへ違反を
//! 種類ごとに件数付きで並べる。

use std::fmt::Write;

use super::super::ledger::LineCountLedger;
use super::LineCountReport;

impl LineCountReport {
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
        let sections: [(&str, &Vec<String>); 6] = [
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
            render_section(text, title, entries);
        }
    }
}

// 違反1種類分を、件数付きの見出しと一覧で書く。空なら何も書かない。
fn render_section(text: &mut String, title: &str, entries: &[String]) {
    if entries.is_empty() {
        return;
    }
    let _ = writeln!(text, "{title} ({}件):", entries.len());
    for entry in entries {
        let _ = writeln!(text, "  {entry}");
    }
}
