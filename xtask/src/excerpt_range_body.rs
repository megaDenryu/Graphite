//! 行範囲の妥当性の判定。引用が「どの範囲から取られたか」だけを見る。
//!
//! 見るのは参照先の指定された行範囲の本文であり、引用の先頭3行がそこに実在するかを
//! 答える。引用が現在のコードから取られたかは `excerpt_file_body` が別に受け持つ。

use crate::quoted_excerpt::{normalize, ExcerptLine, QuotedExcerpt};
use crate::source_reference::SourceReference;

/// 照合する引用行の上限。
///
/// 先頭3行に限るのは、引用の後半が途中を畳んだ抜粋であることが多く、指定された
/// 行範囲に全行を要求すると正当な省略まで違反になるためである。issue #21 の検収で
/// オーケストレータがこのリポジトリの引用63件を実測したところ、全行の一致を要求すると
/// 19件が違反になり、その19件は先頭3行だけの照合では全件が合格した。
/// 引用の先頭は引用した宣言の頭にあたるため、ここが一致していれば「宣言した
/// 行範囲と引用本文が全く別物」という事故は止まる。
///
/// 注意: この上限は行範囲の妥当性だけのものである。引用の鮮度へ適用してはならない。
const ANCHOR_LIMIT: usize = 3;

/// 参照が指定した行範囲の本文。範囲ごとに分けて正規化した綴りを持つ。
///
/// カンマ区切りの複数範囲を1本へ連結しないのは、飛ばした区間の境界をまたぐ偽の
/// 一致が起きるためである。
pub(crate) struct ExcerptRangeBody {
    ranges: Vec<String>,
}

impl ExcerptRangeBody {
    /// 参照先ファイルの本文から、参照が指定した行範囲だけを取り出す。
    pub(crate) fn of(target: &SourceReference, source_lines: &[String]) -> Self {
        let ranges = target
            .line_span()
            .extents()
            .iter()
            .map(|extent| normalize(&extent.lines_within(source_lines).join("\n")))
            .collect();
        Self { ranges }
    }

    /// 引用の先頭3行のうち、どの範囲にも実在しなかった行。
    pub(crate) fn absent_excerpt_lines<'a>(&self, excerpt: &'a QuotedExcerpt) -> Vec<&'a str> {
        excerpt
            .lines()
            .iter()
            .take(ANCHOR_LIMIT)
            .filter(|line| !self.ranges.iter().any(|range| range.contains(line.normalized())))
            .map(ExcerptLine::written)
            .collect()
    }
}
