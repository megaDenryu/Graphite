//! 行番号付きソース参照の直後に書かれたコードフェンスの本文を、引用として集める。
//!
//! このファイルは、引用の収集 (どのフェンスを引用とみなし、どの行を照合の対象に
//! するか) と、照合に使う正規化を持つ。2つは「文書に書かれた引用1件」という同じ
//! 概念の2つの面であり、どの行を対象にするかの判定が正規化の結果を使う。
//!
//! このファイルは判定を持たない。以前はこのファイルが収集と照合を1つにまとめて
//! おり、その冒頭コメントは、分けると引用の意味を単独で説明できなくなることを
//! 理由に挙げていた。判定が2つ (行範囲の妥当性と引用の鮮度) になり、判定ごとに
//! 照合する本文が別の型になったため、その理由は成り立たなくなった。今はここが引用を
//! 集め、`excerpt_range_body` と `excerpt_file_body` がそれぞれの本文を持ち、
//! `quoted_excerpt_check` が2つを掛ける。
//!
//! このファイルは1ファイル100行の原則の例外である (区分: 統合による超過)。引用
//! の収集と照合用の正規化が「文書に書かれた引用1件」という同じ概念に属する。本
//! 体は98行で、残りは同居する単体テストである。超過を許す根拠の台帳は
//! `docs/development/line_count_ledger.md` にある。

use std::fmt;

use crate::document_reference::ReferenceOrigin;
use crate::source_reference::SourceReference;

// 照合に使う引用行1行。文書に書かれたままの綴りと、正規化した綴りを持つ。
//
// 正規化した綴りが空でないことを不変条件にする。空の綴りはどの本文にも
// 含まれてしまい、照合しても何も判定しないためである。
pub(crate) struct ExcerptLine {
    written: String, // 違反として表示する、文書に書かれたままの綴り
    normalized: String, // 照合に使う、空白と区切りの違いを吸収した綴り
}

impl ExcerptLine {
    // フェンス本文の1行から引用行を作る。照合の対象にしない行なら `None` を返す。
    fn from_line(line: &str) -> Option<Self> {
        if line.trim().is_empty() || Self::is_ellipsis(line) {
            return None;
        }
        let normalized = normalize(line);
        if Self::matches_any_body(&normalized) {
            return None;
        }
        Some(Self { written: line.trim().to_string(), normalized })
    }

    pub(crate) fn written(&self) -> &str {
        &self.written
    }

    pub(crate) fn normalized(&self) -> &str {
        &self.normalized
    }

    // 正規化すると空になり、どの本文にも含まれてしまう行か。
    //
    // `{` だけの行・`,` だけの行のように、正規化が空白と末尾の区切りを落とした
    // 結果として何も残らない行がこれにあたる。空の綴りはどの本文にも含まれる
    // ため、照合しても常に一致になり検査として何も判定しない。
    fn matches_any_body(normalized: &str) -> bool {
        normalized.is_empty()
    }

    // 省略を表すだけの行か。`...` と `// ...` のように記号と斜線だけからなる行は、
    // 引用の途中を畳んだ印であってコードに実在しないため照合しない。
    fn is_ellipsis(line: &str) -> bool {
        line.trim().trim_matches(['/', ' ']) == "..."
    }
}

// 文書がソース参照の直後のコードフェンスへ書いた引用1件。
pub struct QuotedExcerpt {
    origin: ReferenceOrigin,
    target: SourceReference,
    lines: Vec<ExcerptLine>,
}

impl QuotedExcerpt {
    // 参照が書かれた行の直後に、空行だけを挟んでコードフェンスが始まるなら、
    // その本文を引用として取り込む。
    //
    // 照合の対象から外すのは、対象外である条件を書き下せる5つだけである。
    // 行番号を持たない参照 (ファイル全体を指すため照合すべき範囲が無い)、
    // 直後にコードフェンスが無い参照、直後のフェンスの情報文字列が `rust` で
    // ないもの、フェンス本文のうち空行と省略記号だけの行、および正規化すると
    // 空になる行 (`ExcerptLine::matches_any_body`) である。照合しなかった
    // フェンスの件数は検査の報告に出る。
    pub fn following_fence(
        lines: &[&str],
        reference_line_index: usize,
        origin: ReferenceOrigin,
        target: SourceReference,
    ) -> Option<Self> {
        if target.line_span().extents().is_empty() {
            return None;
        }
        let fence = rust_fence_start_after(lines, reference_line_index)?;
        let excerpt_lines = excerpt_lines_in(&lines[fence + 1..]);
        (!excerpt_lines.is_empty()).then_some(Self { origin, target, lines: excerpt_lines })
    }

    pub(crate) fn target(&self) -> &SourceReference {
        &self.target
    }

    pub(crate) fn lines(&self) -> &[ExcerptLine] {
        &self.lines
    }

    // この引用が照合の対象にする引用行の本数。
    pub(crate) fn line_count(&self) -> usize {
        self.lines.len()
    }

}

impl fmt::Display for QuotedExcerpt {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} -> {}", self.origin, self.target)
    }
}

// 参照の行の次から、空行だけを読み飛ばして Rust のコードフェンスの開始行を探す。
// 空行以外の行が先に現れたら、その参照に引用は続いていない。
fn rust_fence_start_after(lines: &[&str], reference_line_index: usize) -> Option<usize> {
    for (offset, line) in lines.iter().enumerate().skip(reference_line_index + 1) {
        if line.trim().is_empty() {
            continue;
        }
        return is_rust_fence_start(line).then_some(offset);
    }
    None
}

// 情報文字列が `rust` のコードフェンスの開始行か。
//
// 引用として照合するのは Rust のフェンスだけである。文書の引用は全てこの形で
// 書かれており、`powershell` や `text` のフェンスは出典を持つ引用ではない。
pub(crate) fn is_rust_fence_start(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("```") && trimmed.trim_start_matches('`').trim_end() == "rust"
}

// フェンスの開始行の次から終了行までを、照合する引用行へ変える。
fn excerpt_lines_in(after_fence_start: &[&str]) -> Vec<ExcerptLine> {
    let mut excerpt_lines = Vec::new();
    for line in after_fence_start {
        if line.trim_start().starts_with("```") {
            break;
        }
        excerpt_lines.extend(ExcerptLine::from_line(line));
    }
    excerpt_lines
}

// 空白と、rustfmt が入れる区切りの違いを吸収した綴りにする。
//
// 空白を全て落とすのは、rustfmt が複数行へ折った署名を、文書が1行へ畳んで
// 引用する形 (「署名のみ抜粋」と注記された引用) を受理するためである。
// `,)` を `)` へ畳むのは、rustfmt が引数列の末尾へ付けるコンマを吸収する
// ためである。末尾の `{` `;` `,` を落とすのは、本体が `{` で始まる関数を
// `;` で打ち切って署名だけ引用する形を受理するためである。
pub(crate) fn normalize(text: &str) -> String {
    let without_space: String = text
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    without_space
        .replace(",)", ")")
        .trim_end_matches(['{', ';', ','])
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::QuotedExcerpt;
    use crate::document_reference::ReferenceOrigin;
    use crate::source_reference::SourceReference;

    // `lines[0]` を参照が書かれた行とみなして引用を取り込む。
    fn excerpt(token: &str, lines: &[&str]) -> Option<QuotedExcerpt> {
        let origin = ReferenceOrigin::new("テスト用の出典".to_string(), 1);
        QuotedExcerpt::following_fence(lines, 0, origin, SourceReference::parse(token).unwrap())
    }

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
    fn 正規化すると空になる行だけのフェンスは引用として取り込まない() {
        let lines = ["参照", "```rust", "{", ",", "```"];
        assert!(excerpt("xtask/src/lib.rs:1-5", &lines).is_none());
    }

    #[test]
    fn 情報文字列がrustでないフェンスは引用として取り込まない() {
        let lines = ["参照", "```text", "fn 先頭(", "```"];
        assert!(excerpt("xtask/src/lib.rs:1-5", &lines).is_none());
    }

    #[test]
    fn 空行と省略記号の行は照合の対象にしない() {
        let lines = ["参照", "", "```rust", "// ...", "", "fn 先頭(", "...", "```"];
        let excerpt = excerpt("xtask/src/lib.rs:1-5", &lines).expect("引用として取り込むこと");
        assert_eq!(excerpt.line_count(), 1);
    }
}
