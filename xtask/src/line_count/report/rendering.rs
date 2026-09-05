//! 検査結果の文字列表現。
//!
//! 表示の書式をこの1箇所へ集める。検査が届いた範囲を先に書き、その後ろへ違反を
//! 種類ごとに件数付きで並べる。

use std::fmt::Write;

use super::super::ledger::LineCountLedger;
use super::LineCountReport;

// 台帳の行として読めなかったときに、書き手へ示す期待する形。
const LEDGER_ROW_FORM: &str = "期待する形: 綴りを逆引用符で囲む。\
    区分を3語 (統合による超過・宣言的データリテラル・再設計待ち) のいずれかにする。根拠を空にしない";

// 冒頭コメントが台帳の根拠と食い違ったときに、書き手へ示す期待する形。
const DECLARATION_FORM: &str = "冒頭コメントは最初の空行で途切れます。定型の文は最初の空行より前へ書いてください";

// 台帳に無いファイルが例外を名乗ったときに、書き手へ示す期待する形。
const UNREGISTERED_DECLARATION_FORM: &str = "期待する形: 台帳へこのファイルの行を足すか、\
    冒頭コメントから例外を名乗る文を消してください";

// 違反1種類分の表示。見出しと、期待する形を書く1行 (示さないなら不在) と一覧を持つ。
struct ViolationSection<'a> {
    title: &'a str,
    expected_form: Option<&'a str>,
    entries: &'a [String],
}

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
        for section in self.violation_sections() {
            section.render(text);
        }
    }

    fn violation_sections(&self) -> [ViolationSection<'_>; 8] {
        [
            ViolationSection::of(
                "台帳に無いのに100行を超えています",
                &self.unregistered_excesses,
            ),
            ViolationSection::of("150行の上限を超えています", &self.upper_limit_excesses),
            ViolationSection::of(
                "100行以内へ収まったので台帳から削除してください",
                &self.shrunk_entries,
            ),
            ViolationSection::of(
                "台帳にありますが検査対象に実在しません",
                &self.missing_entries,
            ),
            ViolationSection {
                title: "台帳の根拠と冒頭コメントが一致しません",
                expected_form: Some(DECLARATION_FORM),
                entries: &self.declaration_mismatches,
            },
            ViolationSection {
                title: "台帳に無いのに例外を名乗っています",
                expected_form: Some(UNREGISTERED_DECLARATION_FORM),
                entries: &self.unregistered_declarations,
            },
            ViolationSection::of("読み込みに失敗しました", &self.unreadable_files),
            ViolationSection {
                title: "台帳の行として読めません",
                expected_form: Some(LEDGER_ROW_FORM),
                entries: &self.invalid_ledger_rows,
            },
        ]
    }
}

impl<'a> ViolationSection<'a> {
    fn of(title: &'a str, entries: &'a [String]) -> Self {
        Self {
            title,
            expected_form: None,
            entries,
        }
    }

    // 件数付きの見出しと一覧を書く。空なら何も書かない。
    fn render(&self, text: &mut String) {
        if self.entries.is_empty() {
            return;
        }
        let _ = writeln!(text, "{} ({}件):", self.title, self.entries.len());
        if let Some(form) = self.expected_form {
            let _ = writeln!(text, "  {form}");
        }
        for entry in self.entries {
            let _ = writeln!(text, "  {entry}");
        }
    }
}
