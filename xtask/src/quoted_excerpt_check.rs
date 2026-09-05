//! 引用1件に2つの判定を掛け、その1件の違反を整形する。
//!
//! 判定の中身は持たない。行範囲の妥当性は `excerpt_range_body` が、引用の鮮度は
//! `excerpt_file_body` が持つ。ここが持つのは「2つの判定は同じ引用に対して同時に
//! 掛かり、違反はその引用1件の下へまとめて並ぶ」という、2つをつなぐ関係である。
//! 引用全件への適用と到達範囲の集計は `excerpt_inspection` が受け持つ。

use std::fmt;

use crate::excerpt_file_body::ExcerptFileBody;
use crate::excerpt_range_body::ExcerptRangeBody;
use crate::quoted_excerpt::QuotedExcerpt;

/// 引用1件の照合結果のうち、参照先に見つからなかった引用行。
pub(crate) struct ExcerptMismatch<'a> {
    excerpt: &'a QuotedExcerpt,
    outside_range: Vec<&'a str>, // 先頭3行のうち、指定された行範囲に無かった行
    absent_from_file: Vec<&'a str>, // 全行のうち、ファイル全体のどこにも無かった行
}

impl<'a> ExcerptMismatch<'a> {
    /// 2つの判定を掛ける。どちらも違反が無ければ `None` を返す。
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

/// 判定1つ分の違反行を、引用の出典と参照先を添えて並べる。
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

#[cfg(test)]
mod tests {
    use super::ExcerptMismatch;
    use crate::document_reference::ReferenceOrigin;
    use crate::quoted_excerpt::QuotedExcerpt;
    use crate::source_reference::SourceReference;

    /// 1行目から順に、範囲の指定と対応させて読むための試験用ソース。
    const SOURCE: &str = "fn 先頭(
    引数: usize,
) -> bool {
    true
}
struct 五行目より後;
";

    /// `lines[0]` を参照が書かれた行とみなして引用を取り込み、2つの判定を掛けた
    /// 結果の表示を返す。違反が無ければ `None`。
    fn 違反の表示(token: &str, lines: &[&str]) -> Option<String> {
        let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
        let target = SourceReference::parse(token).unwrap();
        let excerpt = QuotedExcerpt::following_fence(lines, 0, origin, target)
            .expect("引用として取り込むこと");
        let source_lines: Vec<String> = SOURCE.lines().map(str::to_string).collect();
        let mismatch = ExcerptMismatch::judge(&excerpt, &source_lines, SOURCE)?;
        Some(mismatch.to_string())
    }

    #[test]
    fn 範囲内の引用は違反にしない() {
        let lines = ["参照", "", "```rust", "fn 先頭(", "    引数: usize,", "```"];
        assert!(違反の表示("xtask/src/lib.rs:1-5", &lines).is_none());
    }

    #[test]
    fn 範囲の外の行を引用していれば行範囲の判定が違反にする() {
        let lines = ["参照", "```rust", "struct 五行目より後;", "```"];
        let 表示 = 違反の表示("xtask/src/lib.rs:1-5", &lines).expect("違反になること");
        assert!(表示.contains("指定の行範囲にありません"));
        assert!(表示.contains("struct 五行目より後;"));
    }

    #[test]
    fn 複数行へ折った署名を1行へ畳んだ引用を受理する() {
        let lines = ["参照", "```rust", "fn 先頭(引数: usize) -> bool;", "```"];
        assert!(違反の表示("xtask/src/lib.rs:1-5", &lines).is_none());
    }

    #[test]
    fn 省略記号の行は照合しない() {
        let lines = ["参照", "```rust", "// ...", "fn 先頭(", "...", "```"];
        assert!(違反の表示("xtask/src/lib.rs:1-5", &lines).is_none());
    }

    #[test]
    fn 複数範囲はいずれか1つに含まれれば合格になる() {
        let lines = ["参照", "```rust", "fn 先頭(", "struct 五行目より後;", "```"];
        assert!(違反の表示("xtask/src/lib.rs:1-2, 6", &lines).is_none());
    }

    #[test]
    fn 行範囲の判定が照合するのは先頭3行までである() {
        let lines =
            ["参照", "```rust", "fn 先頭(", "    引数: usize,", ") -> bool {", "    true", "```"];
        assert!(違反の表示("xtask/src/lib.rs:1-3", &lines).is_none());
    }

    #[test]
    fn 四行目以降がファイルのどこにも無ければ鮮度の判定が違反にする() {
        let lines = [
            "参照",
            "```rust",
            "fn 先頭(",
            "    引数: usize,",
            ") -> bool {",
            "    ファイルに無い行();",
            "```",
        ];
        let 表示 = 違反の表示("xtask/src/lib.rs:1-5", &lines).expect("違反になること");
        assert!(表示.contains("参照先ファイルのどこにもありません"));
        assert!(表示.contains("ファイルに無い行();"));
    }
}
