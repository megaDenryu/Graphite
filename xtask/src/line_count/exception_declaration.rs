//! ファイル冒頭のコメントが書く、例外である宣言。
//!
//! 規約は、超過を許したファイルの冒頭コメントへ根拠を書くことを要求する。台帳を正本と
//! し、台帳の区分と根拠から組み立てた定型の文が冒頭コメントに含まれるかをこの型が答える。
//! 照合の前に空白と記号を落とすのは、冒頭コメントが定型の文を行の幅で折り返すためである。

#[cfg(test)]
mod tests;

use super::ledger::LedgerEntry;

// 照合の前に落とす記号。逆引用符と、行コメントの印を作る文字である。
const DROPPED_MARKS: [char; 4] = ['`', '/', '!', '*'];

// ファイル1つの冒頭コメント。照合できる形へ均した文字列を持つ。
pub(crate) struct ExceptionDeclaration {
    normalized_header: String,
}

impl ExceptionDeclaration {
    // 冒頭の空行を飛ばし、そこから続くコメント行だけを冒頭コメントとして取る。
    pub(crate) fn of_source_text(text: &str) -> Self {
        let mut header = String::new();
        for line in text.lines().map(str::trim) {
            if header.is_empty() && line.is_empty() {
                continue;
            }
            if !line.starts_with("//") {
                break;
            }
            header.push_str(line);
        }
        Self {
            normalized_header: normalize(&header),
        }
    }

    // 台帳の1件が要求する定型の文を、冒頭コメントが書いているか。
    pub(crate) fn agrees_with(&self, entry: &LedgerEntry) -> bool {
        self.normalized_header
            .contains(&normalize(&entry.declaration_sentences()))
    }
}

// 空白と記号を落とす。行の折り返しと逆引用符の有無を照合の対象から外す。
fn normalize(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_whitespace() && !DROPPED_MARKS.contains(c))
        .collect()
}
