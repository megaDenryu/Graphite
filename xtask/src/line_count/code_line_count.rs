//! コード行の数え方。定義は `docs/development/line_count_ledger.md` にある。
//!
//! 数える処理を行の走査で行うのは、コンパイルが通らないことを目的にしたファイル
//! (`crates/graphite/tests/ui` 配下) も対象に含めるためである。

// 1ファイル100行の原則そのもの。
const PRINCIPLE_LIMIT: usize = 100;

// 統合による超過に許される上限。
const UPPER_LIMIT: usize = 150;

// 1ファイルのコード行数。数え方をこの型へ閉じる。
pub(crate) struct CodeLineCount {
    value: usize,
}

impl CodeLineCount {
    // 空行とコメントだけの行を除いて数える。判定は行頭だけを見る。
    pub(crate) fn of_text(text: &str) -> Self {
        let mut value = 0;
        let mut inside_block_comment = false;
        for line in text.lines() {
            let trimmed = line.trim();
            if inside_block_comment {
                match trimmed.split_once("*/") {
                    None => continue,
                    Some((_, tail)) => {
                        inside_block_comment = false;
                        if !tail.trim().is_empty() {
                            value += 1;
                        }
                    }
                }
                continue;
            }
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix("/*") {
                match rest.split_once("*/") {
                    None => inside_block_comment = true,
                    Some((_, tail)) => {
                        if !tail.trim().is_empty() {
                            value += 1;
                        }
                    }
                }
                continue;
            }
            value += 1;
        }
        Self { value }
    }

    pub(crate) fn value(&self) -> usize {
        self.value
    }

    pub(crate) fn exceeds_principle(&self) -> bool {
        self.value > PRINCIPLE_LIMIT
    }

    pub(crate) fn exceeds_upper_limit(&self) -> bool {
        self.value > UPPER_LIMIT
    }
}

#[cfg(test)]
mod tests {
    use super::CodeLineCount;

    #[test]
    fn 空行とコメントだけの行を数えない() {
        let text = "fn main() {}\n\n// 説明\n/// doc\n//! module\n";
        assert_eq!(CodeLineCount::of_text(text).value(), 1);
    }

    #[test]
    fn 行末のコメントはコード行として数える() {
        assert_eq!(CodeLineCount::of_text("let x = 1; // 説明\n").value(), 1);
    }

    #[test]
    fn ブロックコメントの途中の行を数えない() {
        let text = "/* 開始\n途中\n終わり */\nlet x = 1;\n";
        assert_eq!(CodeLineCount::of_text(text).value(), 1);
    }

    #[test]
    fn ブロックコメントの後ろに残ったコードを数える() {
        assert_eq!(CodeLineCount::of_text("/* 説明 */ let x = 1;\n").value(), 1);
    }
}
