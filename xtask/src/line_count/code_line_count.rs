//! コード行の数え方。定義は `docs/development/line_count_ledger.md` にある。
//!
//! 数える処理を行の走査で行うのは、コンパイルが通らないことを目的にしたファイル
//! (`crates/graphite/tests/ui` 配下) も対象に含めるためである。

mod block_comment_scan;
#[cfg(test)]
mod tests;

use block_comment_scan::BlockCommentScan;

// 1ファイル100行の原則そのもの。
const PRINCIPLE_LIMIT: usize = 100;

// 統合による超過に許される上限。
const UPPER_LIMIT: usize = 150;

// 1ファイルのコード行数。数え方をこの型へ閉じる。
pub(crate) struct CodeLineCount {
    value: usize,
}

impl CodeLineCount {
    // 空行とコメントだけの行を除いて数える。
    pub(crate) fn of_text(text: &str) -> Self {
        let mut scan = BlockCommentScan::default();
        let value = text.lines().filter(|line| scan.line_has_code(line)).count();
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
