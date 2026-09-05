//! 引用の鮮度の判定。引用が「現在のコードから取られたか」だけを見る。
//!
//! 見るのは参照先のファイル全体の本文であり、引用の全行がそこに実在するかを答える。
//! 引用がどの範囲から取られたかは `excerpt_range_body` が別に受け持つ。

use crate::quoted_excerpt::{normalize, ExcerptLine, QuotedExcerpt};

// 参照先ファイル全体の本文を1本へ正規化した綴り。
//
// 行ごとに分けずに1本で持つ。分けて行ごとに比べると、rustfmt が複数行へ折った署名を
// 文書が1行へ畳んで引用した形が一致しなくなる。
//
// 対象をファイル全体にするため、照合する引用行に上限を置かない。途中を省略した引用も
// 畳んだ署名も、省略しなかった行はファイルのどこかに実在する。ファイルのどこにも
// 現れなくなるのは、コードが変わって古くなった行だけである。
pub(crate) struct ExcerptFileBody {
    whole: String,
}

impl ExcerptFileBody {
    // 参照先ファイルの本文全体を受け取る。
    pub(crate) fn of(source_text: &str) -> Self {
        Self { whole: normalize(source_text) }
    }

    // 引用の全行のうち、ファイルのどこにも実在しなかった行。
    pub(crate) fn absent_excerpt_lines<'a>(&self, excerpt: &'a QuotedExcerpt) -> Vec<&'a str> {
        excerpt
            .lines()
            .iter()
            .filter(|line| !self.whole.contains(line.normalized()))
            .map(ExcerptLine::written)
            .collect()
    }
}
