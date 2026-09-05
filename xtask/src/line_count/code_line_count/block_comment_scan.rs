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
    pub(super) fn line_has_code(&mut self, line: &str) -> bool {
        let mut rest = line;
        let mut has_code = false;
        loop {
            if self.inside_block_comment {
                match rest.split_once("*/") {
                    None => return has_code,
                    Some((_, tail)) => {
                        self.inside_block_comment = false;
                        rest = tail;
                    }
                }
                continue;
            }
            match next_comment_opening(rest) {
                None => return has_code || !rest.trim().is_empty(),
                Some(opening) => {
                    has_code = has_code || !rest[..opening.position].trim().is_empty();
                    if opening.is_line_comment {
                        return has_code;
                    }
                    self.inside_block_comment = true;
                    rest = &rest[opening.position + 2..];
                }
            }
        }
    }
}

// 行コメントとブロックコメントのうち、先に現れる方の開始位置。
struct CommentOpening {
    position: usize,
    is_line_comment: bool,
}

// 注意: 文字列リテラルの中身を解析しないため、リテラル内の `//` と `/*` も開始とみなす。
// この限界は `docs/development/line_count_ledger.md` の「数え方の限界」に書いてある。
fn next_comment_opening(rest: &str) -> Option<CommentOpening> {
    let line_comment = rest.find("//");
    let block_comment = rest.find("/*");
    match (line_comment, block_comment) {
        (None, None) => None,
        (Some(position), None) => Some(CommentOpening {
            position,
            is_line_comment: true,
        }),
        (None, Some(position)) => Some(CommentOpening {
            position,
            is_line_comment: false,
        }),
        (Some(line), Some(block)) => Some(CommentOpening {
            position: line.min(block),
            is_line_comment: line < block,
        }),
    }
}
