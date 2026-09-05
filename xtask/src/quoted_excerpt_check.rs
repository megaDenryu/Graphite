//! 引用1件に2つの判定を掛け、その1件の違反を整形する。
//!
//! 判定の中身は持たない。行範囲の妥当性は `excerpt_range_body` が、引用の鮮度は
//! `excerpt_file_body` が持つ。ここが持つのは「2つの判定は同じ引用に対して同時に
//! 掛かり、違反はその引用1件の下へまとめて並ぶ」という、2つをつなぐ関係である。
//! 引用全件への適用と到達範囲の集計は `excerpt_inspection` が受け持つ。

#[cfg(test)]
mod tests;

use std::fmt;

use crate::excerpt_file_body::ExcerptFileBody;
use crate::excerpt_range_body::ExcerptRangeBody;
use crate::quoted_excerpt::QuotedExcerpt;

// 引用1件の照合結果のうち、参照先に見つからなかった引用行。
pub(crate) struct ExcerptMismatch<'a> {
    excerpt: &'a QuotedExcerpt,
    outside_range: Vec<&'a str>, // 先頭3行のうち、指定された行範囲に無かった行
    absent_from_file: Vec<&'a str>, // 全行のうち、ファイル全体のどこにも無かった行
}

impl<'a> ExcerptMismatch<'a> {
    // 2つの判定を掛ける。どちらも違反が無ければ `None` を返す。
    pub(crate) fn judge(
        excerpt: &'a QuotedExcerpt,
        source_lines: &[String],
        source_text: &str,
    ) -> Option<Self> {
        let outside_range =
            ExcerptRangeBody::of(excerpt.target(), source_lines).absent_excerpt_lines(excerpt);
        let absent_from_file = ExcerptFileBody::of(source_text).absent_excerpt_lines(excerpt);
        (!outside_range.is_empty() || !absent_from_file.is_empty()).then_some(Self {
            excerpt,
            outside_range,
            absent_from_file,
        })
    }
}

impl fmt::Display for ExcerptMismatch<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_missing_lines(
            formatter,
            self.excerpt,
            "次の引用行が指定の行範囲にありません",
            &self.outside_range,
        )?;
        write_missing_lines(
            formatter,
            self.excerpt,
            "次の引用行が参照先ファイルのどこにもありません",
            &self.absent_from_file,
        )
    }
}

// 判定1つ分の違反行を、引用の出典と参照先を添えて並べる。
fn write_missing_lines(
    formatter: &mut fmt::Formatter<'_>,
    excerpt: &QuotedExcerpt,
    reason: &str,
    missing: &[&str],
) -> fmt::Result {
    if missing.is_empty() {
        return Ok(());
    }
    writeln!(formatter, "  {excerpt} ({reason})")?;
    for line in missing {
        writeln!(formatter, "      {line}")?;
    }
    Ok(())
}
