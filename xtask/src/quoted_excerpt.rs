//! 行番号付きソース参照の直後に書かれたコードフェンスの本文と、参照先の
//! 行範囲との照合。
//!
//! このファイルは、引用の収集 (どのフェンスを引用とみなし、どの行を照合するか)・
//! 照合 (正規化して部分文字列で比べる)・違反の整形の3つを1つにまとめて持つ。
//! 3つは「文書に書かれた引用1件」という同じ概念の3つの面であり、正規化の規則は
//! 収集と照合の両方が使う。別のファイルへ分けると、どちらのファイルも単独では
//! 引用の意味を説明できず、読み手は2つを並べて読むことになる。

use std::fmt;

use crate::document_reference::ReferenceOrigin;
use crate::repository_root::RepositoryRoot;
use crate::source_reference::SourceReference;

/// 照合する引用行の上限。
///
/// 先頭3行に限るのは、引用の後半が途中を畳んだ抜粋であることが多く、全行を
/// 要求すると正当な省略まで違反になるためである。issue #21 の検収でオーケスト
/// レータがこのリポジトリの引用63件を実測したところ、全行の一致を要求すると
/// 19件が違反になり、その19件は先頭3行だけの照合では全件が合格した。
/// 引用の先頭は引用した宣言の頭にあたるため、ここが一致していれば「宣言した
/// 行範囲と引用本文が全く別物」という事故は止まる。
const ANCHOR_LIMIT: usize = 3;

/// 照合に使う引用行1行。文書に書かれたままの綴りと、正規化した綴りを持つ。
///
/// 正規化した綴りが空でないことを不変条件にする。空の綴りはどの範囲にも
/// 含まれてしまい、照合しても何も判定しないためである。
struct ExcerptAnchor {
    written: String, // 違反として表示する、文書に書かれたままの綴り
    normalized: String, // 照合に使う、空白と区切りの違いを吸収した綴り
}

impl ExcerptAnchor {
    /// フェンス本文の1行から引用行を作る。照合の対象にしない行なら `None` を返す。
    fn from_line(line: &str) -> Option<Self> {
        if line.trim().is_empty() || Self::is_ellipsis(line) {
            return None;
        }
        let normalized = normalize(line);
        if Self::matches_any_range(&normalized) {
            return None;
        }
        Some(Self { written: line.trim().to_string(), normalized })
    }

    /// 正規化すると空になり、どの行範囲にも含まれてしまう行か。
    ///
    /// `{` だけの行・`,` だけの行のように、正規化が空白と末尾の区切りを落とした
    /// 結果として何も残らない行がこれにあたる。空の綴りはどの本文にも含まれる
    /// ため、照合しても常に一致になり検査として何も判定しない。
    fn matches_any_range(normalized: &str) -> bool {
        normalized.is_empty()
    }

    /// 省略を表すだけの行か。`...` と `// ...` のように記号と斜線だけからなる行は、
    /// 引用の途中を畳んだ印であってコードに実在しないため照合しない。
    fn is_ellipsis(line: &str) -> bool {
        line.trim().trim_matches(['/', ' ']) == "..."
    }
}

/// 文書がソース参照の直後のコードフェンスへ書いた引用1件。
pub struct QuotedExcerpt {
    origin: ReferenceOrigin,
    target: SourceReference,
    anchors: Vec<ExcerptAnchor>,
}

impl QuotedExcerpt {
    /// 参照が書かれた行の直後に、空行だけを挟んでコードフェンスが始まるなら、
    /// その本文を引用として取り込む。
    ///
    /// 照合の対象から外すのは、対象外である条件を書き下せる4つだけである。
    /// 行番号を持たない参照 (ファイル全体を指すため照合すべき範囲が無い)、
    /// 直後にコードフェンスが無い参照、フェンス本文のうち空行と省略記号だけの
    /// 行、および正規化すると空になる行 (`ExcerptAnchor::matches_any_range`)
    /// である。
    pub fn following_fence(
        lines: &[&str],
        reference_line_index: usize,
        origin: ReferenceOrigin,
        target: SourceReference,
    ) -> Option<Self> {
        if target.line_span().extents().is_empty() {
            return None;
        }
        let fence = fence_start_after(lines, reference_line_index)?;
        let anchors = anchors_in(&lines[fence + 1..]);
        (!anchors.is_empty()).then_some(Self { origin, target, anchors })
    }

    /// 引用行が参照先の行範囲に実在するかを照合する。
    ///
    /// 参照先が実在しないときは `None` を返す。ファイルの不在は
    /// `SourceCodeReference::evaluate` が違反として報告するため、ここで
    /// 二重に報告しない。
    pub(crate) fn evaluate(&self, root: &RepositoryRoot) -> Option<ExcerptMismatch<'_>> {
        let source_lines = root.source_file_lines(&self.target)?;
        self.compare_with(&source_lines)
    }

    /// 参照先ファイルの本文と引用を突き合わせる。
    ///
    /// 範囲ごとに本文を分けて持つ。カンマ区切りの複数範囲を1本へ連結すると、
    /// 飛ばした区間の境界をまたぐ偽の一致が起きる。
    fn compare_with(&self, source_lines: &[String]) -> Option<ExcerptMismatch<'_>> {
        let ranges: Vec<String> = self
            .target
            .line_span()
            .extents()
            .iter()
            .map(|extent| normalize(&extent.lines_within(source_lines).join("\n")))
            .collect();
        let missing: Vec<&str> = self
            .anchors
            .iter()
            .filter(|anchor| !ranges.iter().any(|range| range.contains(&anchor.normalized)))
            .map(|anchor| anchor.written.as_str())
            .collect();
        (!missing.is_empty()).then_some(ExcerptMismatch { excerpt: self, missing })
    }
}

impl fmt::Display for QuotedExcerpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} -> {}", self.origin, self.target)
    }
}

/// 参照の行の次から、空行だけを読み飛ばしてコードフェンスの開始行を探す。
/// 空行以外の行が先に現れたら、その参照に引用は続いていない。
fn fence_start_after(lines: &[&str], reference_line_index: usize) -> Option<usize> {
    for (offset, line) in lines.iter().enumerate().skip(reference_line_index + 1) {
        if line.trim().is_empty() {
            continue;
        }
        return line.trim_start().starts_with("```").then_some(offset);
    }
    None
}

/// フェンスの開始行の次から終了行までを、照合する引用行へ変える。
fn anchors_in(after_fence_start: &[&str]) -> Vec<ExcerptAnchor> {
    let mut anchors = Vec::new();
    for line in after_fence_start {
        if line.trim_start().starts_with("```") {
            break;
        }
        anchors.extend(ExcerptAnchor::from_line(line));
        if anchors.len() == ANCHOR_LIMIT {
            break;
        }
    }
    anchors
}

/// 空白と、rustfmt が入れる区切りの違いを吸収した綴りにする。
///
/// 空白を全て落とすのは、rustfmt が複数行へ折った署名を、文書が1行へ畳んで
/// 引用する形 (「署名のみ抜粋」と注記された引用) を受理するためである。
/// `,)` を `)` へ畳むのは、rustfmt が引数列の末尾へ付けるコンマを吸収する
/// ためである。末尾の `{` `;` `,` を落とすのは、本体が `{` で始まる関数を
/// `;` で打ち切って署名だけ引用する形を受理するためである。
fn normalize(text: &str) -> String {
    let without_space: String = text
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    without_space
        .replace(",)", ")")
        .trim_end_matches(['{', ';', ','])
        .to_string()
}

/// 引用1件の照合結果のうち、参照先の行範囲に見つからなかった引用行。
pub(crate) struct ExcerptMismatch<'a> {
    excerpt: &'a QuotedExcerpt,
    missing: Vec<&'a str>,
}

/// 参照先の行範囲に実在しない引用行を持つ引用の一覧。整形は `Display` へ閉じる。
///
/// 照合するのは引用の先頭3行までであり、それより後ろ・省略された部分・
/// 畳み込まれた空白の差は検査しない。検査の範囲と限界は `main.rs` の使い方の
/// 説明にも明記する。
pub struct MismatchedExcerpts<'a> {
    mismatches: Vec<ExcerptMismatch<'a>>,
}

impl<'a> MismatchedExcerpts<'a> {
    pub(crate) fn new(mismatches: Vec<ExcerptMismatch<'a>>) -> Self {
        Self { mismatches }
    }

    pub fn is_empty(&self) -> bool {
        self.mismatches.is_empty()
    }
}

impl fmt::Display for MismatchedExcerpts<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.mismatches.is_empty() {
            return Ok(());
        }
        writeln!(
            formatter,
            "参照先の行範囲に実在しない引用が{}件あります:",
            self.mismatches.len()
        )?;
        for mismatch in &self.mismatches {
            writeln!(formatter, "  {} (次の引用行が範囲内にありません)", mismatch.excerpt)?;
            for line in &mismatch.missing {
                writeln!(formatter, "      {line}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{MismatchedExcerpts, QuotedExcerpt};
    use crate::document_reference::ReferenceOrigin;
    use crate::source_reference::SourceReference;

    /// `lines[0]` を参照が書かれた行とみなして引用を取り込む。
    fn excerpt(token: &str, lines: &[&str]) -> Option<QuotedExcerpt> {
        let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
        QuotedExcerpt::following_fence(lines, 0, origin, SourceReference::parse(token).unwrap())
    }

    fn source(text: &str) -> Vec<String> {
        text.lines().map(str::to_string).collect()
    }

    /// 1行目から順に、範囲の指定と対応させて読むための試験用ソース。
    const SOURCE: &str = "fn 先頭(
    引数: usize,
) -> bool {
    true
}
struct 五行目より後;
";

    #[test]
    fn 行番号のない参照は照合の対象にならない() {
        let lines = ["参照", "```rust", "fn 先頭(", "```"];
        assert!(excerpt("xtask/src/lib.rs", &lines).is_none());
    }

    #[test]
    fn 直後にコードフェンスが無ければ照合の対象にならない() {
        let lines = ["参照", "続く散文", "```rust", "fn 先頭(", "```"];
        assert!(excerpt("xtask/src/lib.rs:1-5", &lines).is_none());
    }

    #[test]
    fn 空行を挟んだコードフェンスの引用が範囲内にあれば違反にしない() {
        let lines = ["参照", "", "```rust", "fn 先頭(", "    引数: usize,", "```"];
        let excerpt = excerpt("xtask/src/lib.rs:1-5", &lines).expect("引用として取り込むこと");
        assert!(excerpt.compare_with(&source(SOURCE)).is_none());
    }

    #[test]
    fn 範囲の外の行を引用していれば違反になる() {
        let lines = ["参照", "```rust", "struct 五行目より後;", "```"];
        let excerpt = excerpt("xtask/src/lib.rs:1-5", &lines).expect("引用として取り込むこと");
        let mismatch = excerpt.compare_with(&source(SOURCE)).expect("違反になること");
        let mismatched = MismatchedExcerpts::new(vec![mismatch]);
        assert!(mismatched.to_string().contains("struct 五行目より後;"));
    }

    #[test]
    fn 複数行へ折った署名を1行へ畳んだ引用を受理する() {
        let lines = ["参照", "```rust", "fn 先頭(引数: usize) -> bool;", "```"];
        let excerpt = excerpt("xtask/src/lib.rs:1-5", &lines).expect("引用として取り込むこと");
        assert!(excerpt.compare_with(&source(SOURCE)).is_none());
    }

    #[test]
    fn 正規化すると空になる行だけのフェンスは引用として取り込まない() {
        let lines = ["参照", "```rust", "{", ",", "```"];
        assert!(excerpt("xtask/src/lib.rs:1-5", &lines).is_none());
    }

    #[test]
    fn 省略記号の行は照合しない() {
        let lines = ["参照", "```rust", "// ...", "fn 先頭(", "...", "```"];
        let excerpt = excerpt("xtask/src/lib.rs:1-5", &lines).expect("引用として取り込むこと");
        assert!(excerpt.compare_with(&source(SOURCE)).is_none());
    }

    #[test]
    fn 複数範囲はいずれか1つに含まれれば合格になる() {
        let lines = ["参照", "```rust", "fn 先頭(", "struct 五行目より後;", "```"];
        let excerpt = excerpt("xtask/src/lib.rs:1-2, 6", &lines).expect("引用として取り込むこと");
        assert!(excerpt.compare_with(&source(SOURCE)).is_none());
    }

    #[test]
    fn 照合するのは先頭3行までである() {
        let lines = [
            "参照",
            "```rust",
            "fn 先頭(",
            "    引数: usize,",
            ") -> bool {",
            "struct 五行目より後;",
            "```",
        ];
        let excerpt = excerpt("xtask/src/lib.rs:1-5", &lines).expect("引用として取り込むこと");
        assert!(excerpt.compare_with(&source(SOURCE)).is_none());
    }
}
