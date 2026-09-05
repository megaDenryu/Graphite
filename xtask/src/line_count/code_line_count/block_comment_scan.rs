//! 行をまたぐブロックコメントの状態を持つ走査。
//!
//! 走査が状態を持つのは、ブロックコメントが行の途中で開き、何行も後の行の途中で
//! 閉じるためである。状態をこの型が所有し、1行ごとにコードの有無を答える。

// ブロックコメントの中にいるかどうかを、行をまたいで持ち回る走査。
#[derive(Default)]
pub(super) struct BlockCommentScan {
    inside_block_comment: bool,
}

impl BlockCommentScan {
    // 1行を左から右へ読み、コメントの外に空白でない文字が1つでもあればコード行とする。
    // 通常の文字列リテラルの中身は読み飛ばすため、リテラルの中の `//` と `/*` を
    // コメントの開始とみなさない。
    pub(super) fn line_has_code(&mut self, line: &str) -> bool {
        let mut has_code = false;
        let mut rest = line;
        while let Some(head) = rest.chars().next() {
            let tail = &rest[head.len_utf8()..];
            if self.inside_block_comment {
                rest = match rest.strip_prefix("*/") {
                    Some(after_closing) => {
                        self.inside_block_comment = false;
                        after_closing
                    }
                    None => tail,
                };
                continue;
            }
            if rest.starts_with("//") {
                return has_code;
            }
            if let Some(after_opening) = rest.strip_prefix("/*") {
                self.inside_block_comment = true;
                rest = after_opening;
                continue;
            }
            if head == '"' {
                has_code = true;
                rest = skip_string_literal(tail);
                continue;
            }
            has_code = has_code || !head.is_whitespace();
            rest = tail;
        }
        has_code
    }
}

// 注意: 生文字列 (`r"..."`) と文字リテラル (`'"'`) を走査は区別しない。走査はどちらの
// 中の `"` も通常の文字列リテラルの引用符として扱う。この限界は
// `docs/development/line_count_ledger.md` の「数え方の限界」に書いてある。
//
// 開き引用符の直後から読み、閉じ引用符の次を返す。`\` は次の1文字を打ち消す。閉じ
// 引用符が同じ行に無ければ行末を返し、リテラルの読み飛ばしを1行の中で閉じる。
fn skip_string_literal(after_opening_quote: &str) -> &str {
    let mut rest = after_opening_quote;
    while let Some(head) = rest.chars().next() {
        let tail = &rest[head.len_utf8()..];
        match head {
            '"' => return tail,
            '\\' => {
                rest = match tail.chars().next() {
                    Some(escaped) => &tail[escaped.len_utf8()..],
                    None => tail,
                }
            }
            _ => rest = tail,
        }
    }
    rest
}
