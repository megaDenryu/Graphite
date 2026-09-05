//! ファイル冒頭のコメントが書く、例外である宣言。
//!
//! 規約は、超過を許したファイルの冒頭コメントへ根拠を書くことを要求する。台帳を正本と
//! し、台帳の区分と根拠から組み立てた定型の文が冒頭コメントに含まれるかをこの型が答える。
//! この型は行ごとにコメントの印だけを剥がしてから連結し、照合の前に空白と逆引用符を
//! 落とす。冒頭コメントが定型の文を行の幅で折り返すためである。この型は逆引用符以外の
//! 記号を落とさない。落とすと、`flow!` を `flow` と書いた冒頭コメントまで一致とみなす。

#[cfg(test)]
mod tests;

use super::ledger::LedgerEntry;

// 台帳に無いファイルが例外を名乗っているかを見るための、定型の文の固定部分。
const EXCEPTION_CLAIM: &str = "このファイルは1ファイル100行の原則の例外である";

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
            let Some(body) = comment_body(line) else { break };
            header.push_str(body);
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

    // 冒頭コメントが、定型の文の固定部分を書いて例外を名乗っているか。
    pub(crate) fn claims_exception(&self) -> bool {
        self.normalized_header.contains(&normalize(EXCEPTION_CLAIM))
    }
}

// 行からコメントの印を剥がして本文を返す。コメント行でなければ何も返さない。
fn comment_body(line: &str) -> Option<&str> {
    let rest = line.trim_start();
    if !rest.starts_with("//") {
        return None;
    }
    let rest = rest.trim_start_matches('/');
    Some(rest.strip_prefix('!').unwrap_or(rest).trim_start())
}

// 空白と逆引用符を落とす。行の折り返しと逆引用符の有無を照合の対象から外す。
fn normalize(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_whitespace() && *c != '`')
        .collect()
}
